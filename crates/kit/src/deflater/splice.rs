//! gzip and ETags for pages made mostly of cached fragments (a room's messages), without
//! recompressing or rehashing the whole page on every request.
//!
//! A page is split into parts that cover it end to end: its cached fragments (each with the few
//! bytes of text before it, when that follows another fragment) and the text in between (the
//! layout). Each part is compressed once, against the part before it as a preset dictionary, and
//! kept with the part's CRC-32: deflate back-references can reach anything in the last 32 KB of
//! output, so a piece is valid wherever the same predecessor comes right before it. Pages render
//! the same until what they show changes (there are no per-request CSRF tokens), so from one
//! request to the next a page is a run of stored pieces: gzip costs some copying and combining
//! their CRCs, and the ETag a hash of the parts' digests instead of the whole body. Compressing
//! each part on its own would lose what consecutive messages share and make a room page ~4×
//! larger; chained like this it's within 1% of compressing the page whole.
//!
//! Fragments are known by identity (the `Arc` the fragment cache hands out), text by its BLAKE3
//! digest (as collision-resistant as SHA-256). A stored piece that depends on a fragment holds a
//! `Weak` to it, so while the piece exists that address can't come back as a different fragment.
//! Text is hashed with BLAKE3 only the first time it's seen: after that it's found by a fast
//! non-cryptographic hash and confirmed by comparing it with the stored copy byte for byte, which
//! costs a room page's layout ~4× less than hashing it again.

use std::collections::HashMap;
use std::hash::BuildHasher;
use std::ops::Range;
use std::sync::{Arc, LazyLock, Mutex, Weak};

use bytes::Bytes;
use flate2::{Compress, Compression, FlushCompress};

use super::{Generations, lock};

/// Smaller fragments aren't worth a part of their own; they stay in the text around them.
const MIN_FRAGMENT: usize = 1024;
/// At most this much text between two fragments travels with the second; more is a text part.
const MAX_GLUE: usize = 256;
/// Deflate's window.
const WINDOW: usize = 32 * 1024;
/// A bound on remembered fragments (a 32 MB fragment cache holds a few thousand messages).
const MAX_FRAGMENTS: usize = 8 * 1024;
/// Pieces kept per fragment, for the predecessors it's seen with: a message follows the same one in
/// its room and on a page of older messages, and other ones in search results.
const PIECES_PER_FRAGMENT: usize = 4;
/// A bound on the bytes of stored text pieces (a room page's layout is ~10 KB compressed).
const MAX_TEXT_PIECE_BYTES: usize = 16 << 20;
/// A bound on the bytes of text kept to recognise it by (a room page's layout is ~55 KB).
const MAX_KNOWN_TEXT_BYTES: usize = 16 << 20;

type Digest = [u8; 32];

/// A page's body split at its cached fragments, with each part's identity.
#[derive(Debug)]
pub struct PageParts {
    len: usize,
    parts: Vec<Part>,
}

#[derive(Debug)]
enum Part {
    Text {
        range: Range<usize>,
        digest: Digest,
    },
    /// `range` starts with the glue (the text since the previous fragment) and ends with the fragment.
    Fragment {
        fragment: Arc<String>,
        digest: Digest,
        glue: usize,
        range: Range<usize>,
    },
}

/// What comes right before a part, which its piece may refer back into.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Before {
    Nothing,
    Fragment(usize),
    Text(Digest),
}

/// A part compressed: raw deflate ending on a sync flush, and the part's CRC.
#[derive(Clone)]
struct Piece {
    deflated: Bytes,
    crc: Crc,
}

impl Part {
    fn range(&self) -> &Range<usize> {
        match self {
            Part::Text { range, .. } | Part::Fragment { range, .. } => range,
        }
    }

    /// The identity of the part as the part after it sees it, and the bytes its piece may use as
    /// a dictionary: a fragment's own bytes (never its glue), or the text.
    fn as_before<'a>(&self, body: &'a [u8]) -> (Before, &'a [u8]) {
        match self {
            Part::Text { range, digest } => (Before::Text(*digest), &body[range.clone()]),
            Part::Fragment { fragment, range, .. } => (
                Before::Fragment(fragment_key(fragment)),
                &body[range.end - fragment.len()..range.end],
            ),
        }
    }
}

impl PageParts {
    /// Splits `body` at `fragments` (cached HTML, in body order), or `None` when none of them is
    /// in it as is (a fragment escaped into a turbo stream, say, isn't).
    pub fn new(body: &[u8], fragments: &[Arc<String>]) -> Option<Self> {
        let located = locate(body, fragments);
        if located.is_empty() {
            return None;
        }
        let digests = fragment_digests(located.iter().map(|(fragment, _)| *fragment));
        let mut parts = Vec::with_capacity(located.len() * 2 + 1);
        let mut position = 0;
        for ((fragment, start), digest) in located.into_iter().zip(digests) {
            let follows_fragment = matches!(parts.last(), Some(Part::Fragment { .. }));
            let glue = if follows_fragment && start - position <= MAX_GLUE {
                start - position
            } else {
                if start > position {
                    parts.push(text_part(body, position..start));
                }
                0
            };
            let end = start + fragment.len();
            parts.push(Part::Fragment {
                fragment: fragment.clone(),
                digest,
                glue,
                range: start - glue..end,
            });
            position = end;
        }
        if position < body.len() {
            parts.push(text_part(body, position..body.len()));
        }
        Some(Self { len: body.len(), parts })
    }

    /// Whether these parts were made from a body of this length (a HEAD response has none).
    pub fn fits(&self, body: &[u8]) -> bool {
        self.len == body.len()
    }

    /// The weak ETag's value: 32 hex digits of a BLAKE3 hash over the parts (the same body split
    /// the same way always gets the same one), as `Rack::ETag`'s is of the body.
    pub fn etag(&self, body: &[u8]) -> String {
        let mut hasher = blake3::Hasher::new();
        for part in &self.parts {
            match part {
                Part::Text { range, digest } => {
                    hasher.update(b"T");
                    hasher.update(&(range.len() as u64).to_le_bytes());
                    hasher.update(digest);
                }
                Part::Fragment {
                    digest,
                    glue,
                    range,
                    fragment,
                } => {
                    hasher.update(b"F");
                    hasher.update(&(*glue as u64).to_le_bytes());
                    hasher.update(&body[range.start..range.start + glue]);
                    hasher.update(&(fragment.len() as u64).to_le_bytes());
                    hasher.update(digest);
                }
            }
        }
        hex::encode(&hasher.finalize().as_bytes()[..16])
    }

    /// The whole gzip member for `body`, decoding to exactly `body`. `mtime` and the Unix OS code
    /// go in the header, as `Zlib::GzipWriter` writes them.
    pub fn gzip(&self, body: &[u8], mtime: u32) -> Vec<u8> {
        let pieces = self.pieces(body);
        let mut out = Vec::with_capacity(pieces.iter().map(|piece| piece.deflated.len()).sum::<usize>() + 20);
        out.extend_from_slice(&[0x1f, 0x8b, 8, 0]);
        out.extend_from_slice(&mtime.to_le_bytes());
        out.extend_from_slice(&[0, 3]);
        for piece in &pieces {
            out.extend_from_slice(&piece.deflated);
        }
        // An empty final block (fixed Huffman), after the sync flushes that ended every piece.
        out.extend_from_slice(&[0x03, 0x00]);
        let crc = Crc::concatenated(pieces.iter().map(|piece| piece.crc));
        // `fits` checks only the length: a body changed after the parts were taken would otherwise
        // decode to the old bytes instead of failing its CRC.
        debug_assert_eq!(crc, crc32fast::hash(body), "the body changed after its parts were taken");
        out.extend_from_slice(&crc.to_le_bytes());
        out.extend_from_slice(&(body.len() as u32).to_le_bytes());
        out
    }

    /// Each part's piece: stored ones where they fit, the rest compressed and stored.
    fn pieces(&self, body: &[u8]) -> Vec<Piece> {
        let befores: Vec<(Before, &[u8])> = std::iter::once((Before::Nothing, &b""[..]))
            .chain(self.parts.iter().map(|part| part.as_before(body)))
            .collect();
        let mut pieces: Vec<Option<Piece>> = {
            let fragments = lock(&FRAGMENTS);
            let texts = &mut lock(&TEXT_PIECES);
            self.parts
                .iter()
                .zip(&befores)
                .map(|(part, (before, _))| match part {
                    Part::Text { digest, .. } => texts.get(&(*digest, *before)).map(|text| text.piece),
                    Part::Fragment { fragment, glue, range, .. } => {
                        let glue = &body[range.start..range.start + glue];
                        fragments
                            .get(&fragment_key(fragment))
                            .and_then(|known| known.piece_after(*before, glue))
                    }
                })
                .collect()
        };
        let mut new_texts = Vec::new();
        let mut new_fragments = Vec::new();
        for (index, ((part, (before, dictionary)), piece)) in self.parts.iter().zip(&befores).zip(&mut pieces).enumerate() {
            if piece.is_some() {
                continue;
            }
            let bytes = &body[part.range().clone()];
            let compressed = Piece {
                deflated: compress(dictionary, bytes),
                crc: Crc::of(bytes),
            };
            let pin = match index.checked_sub(1).map(|previous| &self.parts[previous]) {
                Some(Part::Fragment { fragment, .. }) => Some(Arc::downgrade(fragment)),
                _ => None,
            };
            match part {
                Part::Text { digest, .. } => new_texts.push((
                    (*digest, *before),
                    TextPiece {
                        piece: compressed.clone(),
                        _pin: pin,
                    },
                )),
                Part::Fragment { fragment, glue, range, .. } => new_fragments.push((
                    fragment.clone(),
                    FragmentPiece {
                        before: *before,
                        _pin: pin,
                        glue: body[range.start..range.start + glue].into(),
                        piece: compressed.clone(),
                    },
                )),
            }
            *piece = Some(compressed);
        }
        if !new_texts.is_empty() {
            let mut texts = lock(&TEXT_PIECES);
            for (key, text) in new_texts {
                let size = text.piece.deflated.len();
                texts.insert(key, text, size);
            }
        }
        if !new_fragments.is_empty() {
            let mut fragments = lock(&FRAGMENTS);
            for (fragment, piece) in new_fragments {
                if let Some(known) = fragments.get_mut(&fragment_key(&fragment)) {
                    known.store(piece);
                }
            }
        }
        pieces.into_iter().map(|piece| piece.expect("every part has a piece")).collect()
    }
}

fn text_part(body: &[u8], range: Range<usize>) -> Part {
    let digest = text_digest(&body[range.clone()]);
    Part::Text { range, digest }
}

/// Where each fragment is, searching after the previous one; fragments not found are skipped.
fn locate<'a>(body: &[u8], fragments: &'a [Arc<String>]) -> Vec<(&'a Arc<String>, usize)> {
    let mut located = Vec::new();
    let mut position = 0;
    for fragment in fragments.iter().filter(|f| f.len() >= MIN_FRAGMENT) {
        if let Some(start) = find(body, position, fragment.as_bytes()) {
            located.push((fragment, start));
            position = start + fragment.len();
        }
    }
    located
}

/// The first occurrence of `needle` in `body` at or after `from`. Finding a short prefix and
/// comparing the rest is much cheaper than a whole-needle search, whose setup is linear in the
/// needle and is paid per fragment.
fn find(body: &[u8], from: usize, needle: &[u8]) -> Option<usize> {
    let prefix = &needle[..needle.len().min(64)];
    let finder = memchr::memmem::Finder::new(prefix);
    let mut at = from;
    while let Some(found) = finder.find(&body[at..]) {
        let start = at + found;
        if body[start..].starts_with(needle) {
            return Some(start);
        }
        at = start + 1;
    }
    None
}

/// Raw deflate of `text` at level 6 with `dictionary` (its last 32 KB) preset, sync-flushed so it
/// ends on a byte boundary with no final block.
fn compress(dictionary: &[u8], text: &[u8]) -> Bytes {
    let mut deflate = Compress::new(Compression::default(), false);
    if !dictionary.is_empty() {
        deflate
            .set_dictionary(&dictionary[dictionary.len().saturating_sub(WINDOW)..])
            .expect("a fresh raw deflate stream takes a dictionary");
    }
    let mut out = Vec::with_capacity(text.len() / 4 + 64);
    loop {
        let consumed = deflate.total_in() as usize;
        if out.capacity() - out.len() < 1024 {
            out.reserve(out.capacity().max(4096));
        }
        deflate
            .compress_vec(&text[consumed..], &mut out, FlushCompress::Sync)
            .expect("deflate doesn't fail on valid input");
        // Done once all input is in and the flush left spare room in the output.
        if deflate.total_in() as usize == text.len() && out.len() < out.capacity() {
            break;
        }
    }
    out.into()
}

// --- CRC-32 from the pieces' --------------------------------------------------------------------

/// gzip's CRC-32 of a part, kept so a page's comes from its parts' without reading the body: the
/// CRC of `a ‖ b` is the CRC of `a` times x^(8·|b|), plus the CRC of `b`, as polynomials over GF(2)
/// modulo the CRC's (zlib's `crc32_combine_op`). A part keeps its x^(8·|b|) too, so each step is
/// one multiplication; `crc32fast::Hasher::combine` works it out from the length every time, which
/// for a room page's parts costs more than a CRC of the whole page.
#[derive(Clone, Copy)]
struct Crc {
    value: u32,
    shift: u32,
}

/// The CRC-32 polynomial, in the reflected bit order gzip's CRC uses (bit 31 is x^0).
const POLYNOMIAL: u32 = 0xedb8_8320;

/// x^(2^k) modulo the polynomial, for k in 0..32 (zlib's `x2n_table`).
const X_TO_THE_2_TO_THE: [u32; 32] = {
    let mut table = [0; 32];
    let mut power = 1 << 30; // x^1
    let mut k = 0;
    while k < 32 {
        table[k] = power;
        power = multiply(power, power);
        k += 1;
    }
    table
};

impl Crc {
    fn of(bytes: &[u8]) -> Self {
        Self {
            value: crc32fast::hash(bytes),
            shift: x_to_the_8(bytes.len() as u64),
        }
    }

    /// The CRC of `crcs`' parts, one after the other.
    fn concatenated(crcs: impl Iterator<Item = Crc>) -> u32 {
        crcs.fold(0, |value, next| multiply(value, next.shift) ^ next.value)
    }
}

/// x^(8·n) modulo the polynomial (zlib's `x2nmodp(n, 3)`). x's order divides 2^32 − 1, so
/// x^(2^32) is x^(2^0).
fn x_to_the_8(mut n: u64) -> u32 {
    let mut power = 1 << 31; // x^0
    let mut k = 3;
    while n != 0 {
        if n & 1 == 1 {
            power = multiply(X_TO_THE_2_TO_THE[k % 32], power);
        }
        n >>= 1;
        k += 1;
    }
    power
}

/// `a` times `b` modulo the polynomial (zlib's `multmodp`), without branching on the bits.
const fn multiply(a: u32, mut b: u32) -> u32 {
    let mut product = 0;
    let mut bit = 32;
    while bit > 0 {
        bit -= 1;
        product ^= b & ((a >> bit) & 1).wrapping_neg();
        b = (b >> 1) ^ (POLYNOMIAL & (b & 1).wrapping_neg());
    }
    product
}

// --- What's remembered -------------------------------------------------------------------------

/// A fragment seen in a page: its digest, and its pieces for the predecessors it's followed, most
/// recently stored last.
struct KnownFragment {
    fragment: Weak<String>,
    digest: Digest,
    pieces: Vec<Arc<FragmentPiece>>,
}

struct FragmentPiece {
    before: Before,
    /// Keeps a predecessor fragment's address from being reused while this piece refers to it.
    _pin: Option<Weak<String>>,
    glue: Box<[u8]>,
    /// The glue and the fragment.
    piece: Piece,
}

impl KnownFragment {
    fn piece_after(&self, before: Before, glue: &[u8]) -> Option<Piece> {
        self.pieces
            .iter()
            .find(|piece| piece.before == before && *piece.glue == *glue)
            .map(|piece| piece.piece.clone())
    }

    /// Stores `piece`, dropping the oldest when there are already [`PIECES_PER_FRAGMENT`].
    fn store(&mut self, piece: FragmentPiece) {
        self.pieces
            .retain(|stored| stored.before != piece.before || stored.glue != piece.glue);
        if self.pieces.len() == PIECES_PER_FRAGMENT {
            self.pieces.remove(0);
        }
        self.pieces.push(Arc::new(piece));
    }
}

#[derive(Clone)]
struct TextPiece {
    piece: Piece,
    _pin: Option<Weak<String>>,
}

/// Text seen in a page, kept to tell whether text with the same fast hash is the same text.
struct KnownText {
    text: Box<[u8]>,
    digest: Digest,
}

static FRAGMENTS: LazyLock<Mutex<HashMap<usize, KnownFragment>>> = LazyLock::new(Mutex::default);
static TEXT_PIECES: LazyLock<Mutex<Generations<(Digest, Before), TextPiece>>> =
    LazyLock::new(|| Mutex::new(Generations::new(MAX_TEXT_PIECE_BYTES)));
/// Known text by its fast hash. The seed is random so that text can't be made to collide on
/// purpose; a collision would only cost a BLAKE3 hash and the other text's entry.
static KNOWN_TEXTS: LazyLock<Mutex<Generations<u64, Arc<KnownText>>>> =
    LazyLock::new(|| Mutex::new(Generations::new(MAX_KNOWN_TEXT_BYTES)));
static TEXT_HASHER: LazyLock<foldhash::fast::RandomState> = LazyLock::new(Default::default);

fn fragment_key(fragment: &Arc<String>) -> usize {
    Arc::as_ptr(fragment) as usize
}

/// The BLAKE3 digest of `text`: the one it had when it was first seen, if it's known (the same
/// bytes, not just the same fast hash), or else hashed and remembered.
fn text_digest(text: &[u8]) -> Digest {
    let key = TEXT_HASHER.hash_one(text);
    let known = lock(&KNOWN_TEXTS).get(&key);
    if let Some(known) = known
        && *known.text == *text
    {
        return known.digest;
    }
    let digest = blake3::hash(text).into();
    let known = Arc::new(KnownText { text: text.into(), digest });
    lock(&KNOWN_TEXTS).insert(key, known, text.len());
    digest
}

/// Each fragment's digest, hashing (and remembering) the ones not seen before. A remembered
/// entry at the same address is the same fragment while its `Weak` keeps the address taken.
#[expect(clippy::significant_drop_tightening, reason = "existing hit under the S-5 lint floor")]
fn fragment_digests<'a>(fragments: impl Iterator<Item = &'a Arc<String>> + Clone) -> Vec<Digest> {
    let mut known = lock(&FRAGMENTS);
    let missing = fragments.clone().filter(|f| !known.contains_key(&fragment_key(f))).count();
    if known.len() + missing > MAX_FRAGMENTS {
        known.retain(|_, entry| entry.fragment.strong_count() > 0);
        if known.len() + missing > MAX_FRAGMENTS {
            known.clear();
        }
    }
    fragments
        .map(|fragment| {
            known
                .entry(fragment_key(fragment))
                .or_insert_with(|| KnownFragment {
                    fragment: Arc::downgrade(fragment),
                    digest: blake3::hash(fragment.as_bytes()).into(),
                    pieces: Vec::new(),
                })
                .digest
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Read, Write};

    fn gunzip(bytes: &[u8]) -> Vec<u8> {
        let mut out = Vec::new();
        flate2::read::GzDecoder::new(bytes).read_to_end(&mut out).unwrap();
        out
    }

    fn message(n: usize) -> Arc<String> {
        Arc::new(format!(
            "<div id=\"message_{n}\" class=\"message\">{}</div>\n",
            "<button>Boost</button> hello there ".repeat(40 + n % 7)
        ))
    }

    fn page(head: &str, messages: &[Arc<String>], tail: &str) -> String {
        let mut page = head.to_string();
        for message in messages {
            page.push_str("  ");
            page.push_str(message);
        }
        page.push_str(tail);
        page
    }

    fn gzip(body: &str, fragments: &[Arc<String>]) -> Vec<u8> {
        PageParts::new(body.as_bytes(), fragments)
            .expect("fragments in the body")
            .gzip(body.as_bytes(), 0)
    }

    #[test]
    fn decodes_to_the_body_and_reuses_pieces() {
        let messages: Vec<_> = (0..30).map(message).collect();
        let body = page("<html><head>layout</head><body>", &messages, "</body></html>");
        let parts = PageParts::new(body.as_bytes(), &messages).unwrap();
        let gz = parts.gzip(body.as_bytes(), 1234);
        assert_eq!(gunzip(&gz), body.as_bytes());
        assert_eq!(&gz[4..8], &1234u32.to_le_bytes());
        assert_eq!(gz[9], 3);
        let piece = lock(&FRAGMENTS)[&fragment_key(&messages[5])].pieces[0].clone();
        assert_eq!(
            PageParts::new(body.as_bytes(), &messages).unwrap().gzip(body.as_bytes(), 1234),
            gz,
            "the same page is the same stored pieces"
        );
        assert!(Arc::ptr_eq(&piece, &lock(&FRAGMENTS)[&fragment_key(&messages[5])].pieces[0]));
    }

    #[test]
    fn a_changed_layout_or_neighbour_decodes_correctly() {
        let messages: Vec<_> = (100..110).map(message).collect();
        let body = page("<p>", &messages, "</p>");
        gzip(&body, &messages);
        for body in [page("<p>changed", &messages, "</p>"), page("<p>", &messages, "</p>changed")] {
            assert_eq!(gunzip(&gzip(&body, &messages)), body.as_bytes());
        }
        // Drop one message: the one after it now follows a different predecessor.
        let mut fewer = messages.clone();
        fewer.remove(4);
        let body = page("<p>", &fewer, "</p>");
        assert_eq!(gunzip(&gzip(&body, &fewer)), body.as_bytes());
        // Different glue between the same fragments.
        let body: String = std::iter::once("<p>".to_string())
            .chain(messages.iter().map(|m| format!("\n    {m}")))
            .collect();
        assert_eq!(gunzip(&gzip(&body, &messages)), body.as_bytes());
    }

    #[test]
    #[expect(clippy::redundant_clone, reason = "existing hit under the S-5 lint floor")]
    fn missing_small_repeated_and_far_apart_fragments() {
        let messages: Vec<_> = (200..206).map(message).collect();
        let small = Arc::new("<i>small</i>".to_string());
        let absent = message(999);
        let listed = vec![messages[0].clone(), small.clone(), absent, messages[1].clone(), messages[2].clone()];
        let body = page(
            "<p>",
            &[messages[0].clone(), small, messages[1].clone(), messages[2].clone()],
            "</p>",
        );
        assert_eq!(gunzip(&gzip(&body, &listed)), body.as_bytes());
        assert!(PageParts::new(b"<p>nothing cached</p>", &messages).is_none());

        let (a, b) = (message(300), message(301));
        let body = format!("{a}{b}{a}{}{b}{a}", "x".repeat(MAX_GLUE + 1));
        let listed = vec![a.clone(), b.clone(), a.clone(), b.clone(), a.clone()];
        for _ in 0..2 {
            assert_eq!(gunzip(&gzip(&body, &listed)), body.as_bytes());
        }
    }

    #[test]
    fn a_fragment_keeps_a_piece_for_each_predecessor() {
        let messages: Vec<_> = (600..606).map(message).collect();
        let room = page("<p>", &messages, "</p>");
        // The same last message after a different one, as in search results.
        let search_hits = vec![messages[1].clone(), messages[5].clone()];
        let search = page("<q>", &search_hits, "</q>");
        for body in [&room, &search] {
            gzip(body, if std::ptr::eq(body, &room) { &messages } else { &search_hits });
        }
        let pieces = |fragment: &Arc<String>| lock(&FRAGMENTS)[&fragment_key(fragment)].pieces.clone();
        let before = pieces(&messages[5]);
        assert_eq!(before.len(), 2, "one after message 604, one after 601");
        gzip(&room, &messages);
        gzip(&search, &search_hits);
        let after = pieces(&messages[5]);
        assert!(before.iter().zip(&after).all(|(a, b)| Arc::ptr_eq(a, b)), "both pages reuse theirs");
    }

    #[test]
    fn etags_follow_the_content() {
        let messages: Vec<_> = (400..410).map(message).collect();
        let etag = |body: &str| PageParts::new(body.as_bytes(), &messages).unwrap().etag(body.as_bytes());
        let body = page("<p>", &messages, "</p>");
        assert_eq!(etag(&body), etag(&body.clone()));
        assert_eq!(etag(&body).len(), 32);
        assert_ne!(etag(&body), etag(&page("<p>", &messages, "</p>!")));
        assert_ne!(etag(&body), etag(&page("<q>", &messages, "</p>")));
        let glued: String = std::iter::once("<p>".to_string())
            .chain(messages.iter().map(|m| format!(" {m}")))
            .chain(["</p>".into()])
            .collect();
        assert_ne!(etag(&body), etag(&glued));
    }

    /// What `gzip` sent before KIT-9 took the CRC from the pieces': every part compressed against
    /// the one before it, and one CRC over the whole body.
    fn gzip_before_kit_9(body: &str, fragments: &[Arc<String>], mtime: u32) -> Vec<u8> {
        let body = body.as_bytes();
        let parts = PageParts::new(body, fragments).expect("fragments in the body");
        let mut out = vec![0x1f, 0x8b, 8, 0];
        out.extend_from_slice(&mtime.to_le_bytes());
        out.extend_from_slice(&[0, 3]);
        let mut dictionary = &b""[..];
        for part in &parts.parts {
            out.extend_from_slice(&compress(dictionary, &body[part.range().clone()]));
            dictionary = part.as_before(body).1;
        }
        out.extend_from_slice(&[0x03, 0x00]);
        out.extend_from_slice(&crc32fast::hash(body).to_le_bytes());
        out.extend_from_slice(&(body.len() as u32).to_le_bytes());
        out
    }

    #[test]
    fn the_same_bytes_as_before_kit_9() {
        let messages: Vec<_> = (700..780).map(message).collect();
        let head: String = (0..400).map(|n| format!("<link rel=\"preload\" href=\"/a/{n}.js\">")).collect();
        let tail: String = (0..200).map(|n| format!("<footer data-n=\"{n}\"></footer>")).collect();
        let body = page(&head, &messages, &tail);
        let spliced = |body: &str| PageParts::new(body.as_bytes(), &messages).unwrap().gzip(body.as_bytes(), 77);
        assert_eq!(spliced(&body), gzip_before_kit_9(&body, &messages, 77), "first request");
        assert_eq!(spliced(&body), gzip_before_kit_9(&body, &messages, 77), "from stored pieces");
        // A new layout around the same fragments: new text pieces, stored fragment ones.
        let changed = page(&format!("{head}!"), &messages, &tail);
        assert_eq!(spliced(&changed), gzip_before_kit_9(&changed, &messages, 77));
    }

    #[test]
    fn a_one_byte_change_in_the_middle_is_new_text() {
        let messages: Vec<_> = (800..820).map(message).collect();
        let head: String = (0..2000).map(|n| format!("<meta name=\"m{n}\">")).collect();
        let mut changed = head.clone().into_bytes();
        changed[head.len() / 2] ^= 1;
        let changed = String::from_utf8(changed).unwrap();
        let (body, changed) = (page(&head, &messages, "</p>"), page(&changed, &messages, "</p>"));
        let parts = |body: &str| PageParts::new(body.as_bytes(), &messages).unwrap();
        let first_digest = |body: &str| match &parts(body).parts[0] {
            Part::Text { range, digest } => {
                assert_eq!(*digest, *blake3::hash(&body.as_bytes()[range.clone()]).as_bytes());
                *digest
            }
            Part::Fragment { .. } => panic!("the page starts with text"),
        };
        for _ in 0..2 {
            assert_ne!(first_digest(&body), first_digest(&changed));
            assert_ne!(parts(&body).etag(body.as_bytes()), parts(&changed).etag(changed.as_bytes()));
            for body in [&body, &changed] {
                let gz = parts(body).gzip(body.as_bytes(), 5);
                assert_eq!(gz, gzip_before_kit_9(body, &messages, 5));
                assert_eq!(gunzip(&gz), body.as_bytes());
            }
        }
    }

    #[test]
    fn text_is_known_by_its_bytes_not_its_fast_hash() {
        let (text, other) = (b"<p>known text</p>".repeat(100), b"<p>other text</p>".repeat(100));
        let digest = text_digest(&text);
        assert_eq!(digest, *blake3::hash(&text).as_bytes());
        let key = TEXT_HASHER.hash_one(&text[..]);
        assert_eq!(lock(&KNOWN_TEXTS).get(&key).unwrap().digest, digest, "remembered");
        assert_eq!(text_digest(&text), digest, "and found again");
        // Another text under the same fast hash, as if they collided.
        let impostor = Arc::new(KnownText {
            text: other[..].into(),
            digest: blake3::hash(&other).into(),
        });
        lock(&KNOWN_TEXTS).insert(key, impostor, other.len());
        assert_eq!(text_digest(&text), digest);
        assert_eq!(*lock(&KNOWN_TEXTS).get(&key).unwrap().text, text[..], "and replaced");
    }

    #[test]
    fn crcs_concatenate() {
        let bytes: Vec<u8> = (0..100_000u32).map(|n| (n.wrapping_mul(2_654_435_761) >> 24) as u8).collect();
        for cuts in [&[][..], &[0, 0, 1], &[1, 2, 3, 4, 5], &[1000, 1001, 70_000, 99_999, 100_000]] {
            let bounds: Vec<usize> = std::iter::once(0).chain(cuts.iter().copied()).chain([bytes.len()]).collect();
            let crcs = bounds.windows(2).map(|range| Crc::of(&bytes[range[0]..range[1]]));
            assert_eq!(Crc::concatenated(crcs), crc32fast::hash(&bytes), "cut at {cuts:?}");
        }
    }

    #[test]
    fn stays_close_to_whole_body_compression() {
        let messages: Vec<_> = (500..540).map(message).collect();
        let body = page(&"<head>layout</head>".repeat(200), &messages, &"<footer/>".repeat(300));
        let mut whole = flate2::write::GzEncoder::new(Vec::new(), Compression::default());
        whole.write_all(body.as_bytes()).unwrap();
        let whole = whole.finish().unwrap().len();
        let spliced = gzip(&body, &messages).len();
        // Each piece costs its flush marker and block header, not a recompressed message.
        assert!(
            spliced < whole + 40 * (messages.len() + 2),
            "{spliced} bytes spliced vs {whole} whole"
        );
    }
}
