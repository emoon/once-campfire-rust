//! gzip and ETags for pages made mostly of cached fragments (a room's messages), without
//! recompressing or rehashing the whole page on every request.
//!
//! A page arrives in parts that cover it end to end: its cached fragments (each with the few
//! bytes of text before it, when that follows another fragment) and the text in between (the
//! layout). The template recorded where each fragment went as it rendered, so the page's bytes
//! are never joined into one buffer, nor searched for its fragments: a plain response sends the
//! parts one after another. Each part is compressed once, against the part before it as a preset
//! dictionary, and kept with the part's CRC-32: deflate back-references can reach anything in the
//! last 32 KB of output, so a piece is valid wherever the same predecessor comes right before it.
//! Pages render the same until what they show changes (there are no per-request CSRF tokens), so
//! from one request to the next a page is a run of stored pieces: gzip costs some copying and
//! combining their CRCs, and the ETag a hash of the parts' digests instead of the whole body.
//! Compressing each part on its own would lose what consecutive messages share and make a room
//! page ~4× larger; chained like this it's within 1% of compressing the page whole.
//!
//! Fragments are known by identity (the `Arc` the fragment cache hands out), text by its BLAKE3
//! digest (as collision-resistant as SHA-256). A stored piece that depends on a fragment holds a
//! `Weak` to it, so while the piece exists that address can't come back as a different fragment.
//! Text is hashed with BLAKE3 only the first time it's seen: after that it's found by a fast
//! non-cryptographic hash and confirmed by comparing it with the stored copy byte for byte, which
//! costs a room page's layout ~4× less than hashing it again.

use std::borrow::Cow;
use std::collections::HashMap;
use std::convert::Infallible;
use std::hash::BuildHasher;
use std::pin::Pin;
use std::sync::{Arc, LazyLock, Mutex, Weak};
use std::task::{Context, Poll};

use bytes::Bytes;
use flate2::{Compress, Compression, FlushCompress};
use hyper::body::{Body as HttpBody, Frame, SizeHint};

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
        text: Bytes,
        digest: Digest,
    },
    Fragment {
        fragment: Arc<String>,
        digest: Digest,
        /// The text since the previous fragment, which goes out just before this one.
        glue: Bytes,
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
    /// The part's bytes, the glue joined to its fragment (only a part being compressed needs that).
    fn bytes(&self) -> Cow<'_, [u8]> {
        match self {
            Part::Text { text, .. } => Cow::Borrowed(text),
            Part::Fragment { fragment, glue, .. } if glue.is_empty() => Cow::Borrowed(fragment.as_bytes()),
            Part::Fragment { fragment, glue, .. } => Cow::Owned([glue, fragment.as_bytes()].concat()),
        }
    }

    /// The part as body chunks: the text, or the glue (if any) and then the fragment.
    fn chunks(&self) -> (Bytes, Option<Bytes>) {
        match self {
            Part::Text { text, .. } => (text.clone(), None),
            Part::Fragment { fragment, glue, .. } if glue.is_empty() => (shared(fragment), None),
            Part::Fragment { fragment, glue, .. } => (glue.clone(), Some(shared(fragment))),
        }
    }

    /// The identity of the part as the part after it sees it, and the bytes its piece may use as
    /// a dictionary: a fragment's own bytes (never its glue), or the text.
    fn as_before(&self) -> (Before, &[u8]) {
        match self {
            Part::Text { text, digest } => (Before::Text(*digest), text),
            Part::Fragment { fragment, .. } => (Before::Fragment(fragment_key(fragment)), fragment.as_bytes()),
        }
    }
}

impl PageParts {
    /// The body `text` makes with each of `fragments` (cached HTML, in order) spliced in at its
    /// byte offset in `text`, split into parts; or that body whole, when no fragment is big enough
    /// for a part of its own.
    ///
    /// # Panics
    ///
    /// If the offsets go backwards or past the end of `text`.
    pub fn splice(text: &Bytes, fragments: Vec<(usize, Arc<String>)>) -> Result<Self, Bytes> {
        let len = text.len() + fragments.iter().map(|(_, fragment)| fragment.len()).sum::<usize>();
        let mut between = Gathered::default();
        let mut runs = Vec::with_capacity(fragments.len());
        let mut position = 0;
        for (offset, fragment) in fragments {
            between.push(text.slice(position..offset));
            position = offset;
            if fragment.len() < MIN_FRAGMENT {
                between.push(shared(&fragment));
            } else {
                runs.push((between.take(), fragment));
            }
        }
        between.push(text.slice(position..));
        let rest = between.take();
        if runs.is_empty() {
            return Err(rest);
        }
        let digests = fragment_digests(runs.iter().map(|(_, fragment)| fragment));
        let mut parts = Vec::with_capacity(runs.len() * 2 + 1);
        for ((gap, fragment), digest) in runs.into_iter().zip(digests) {
            let follows_fragment = matches!(parts.last(), Some(Part::Fragment { .. }));
            let glue = if follows_fragment && gap.len() <= MAX_GLUE {
                gap
            } else {
                if !gap.is_empty() {
                    parts.push(text_part(gap));
                }
                Bytes::new()
            };
            parts.push(Part::Fragment { fragment, digest, glue });
        }
        if !rest.is_empty() {
            parts.push(text_part(rest));
        }
        Ok(Self { len, parts })
    }

    /// The body's length.
    pub fn body_len(&self) -> usize {
        self.len
    }

    /// Whether `body` is still these parts' plain body (a HEAD response's is empty).
    pub fn fits(&self, body: &impl HttpBody) -> bool {
        body.size_hint().exact() == Some(self.len as u64)
    }

    /// The body, sent part by part.
    pub fn plain_body(self: &Arc<Self>) -> PlainBody {
        PlainBody {
            page: self.clone(),
            next: 0,
            fragment: None,
            remaining: self.len as u64,
        }
    }

    /// The weak ETag's value: 32 hex digits of a BLAKE3 hash over the parts (the same body split
    /// the same way always gets the same one), as `Rack::ETag`'s is of the body.
    pub fn etag(&self) -> String {
        let mut hasher = blake3::Hasher::new();
        for part in &self.parts {
            match part {
                Part::Text { text, digest } => {
                    hasher.update(b"T");
                    hasher.update(&(text.len() as u64).to_le_bytes());
                    hasher.update(digest);
                }
                Part::Fragment { digest, glue, fragment } => {
                    hasher.update(b"F");
                    hasher.update(&(glue.len() as u64).to_le_bytes());
                    hasher.update(glue);
                    hasher.update(&(fragment.len() as u64).to_le_bytes());
                    hasher.update(digest);
                }
            }
        }
        hex::encode(&hasher.finalize().as_bytes()[..16])
    }

    /// The whole gzip member for the body. `mtime` and the Unix OS code go in the header, as
    /// `Zlib::GzipWriter` writes them.
    pub fn gzip(&self, mtime: u32) -> Vec<u8> {
        let pieces = self.pieces();
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
        out.extend_from_slice(&crc.to_le_bytes());
        out.extend_from_slice(&(self.len as u32).to_le_bytes());
        out
    }

    /// Each part's piece: stored ones where they fit, the rest compressed and stored.
    fn pieces(&self) -> Vec<Piece> {
        let befores: Vec<(Before, &[u8])> = std::iter::once((Before::Nothing, &b""[..]))
            .chain(self.parts.iter().map(Part::as_before))
            .collect();
        let mut pieces: Vec<Option<Piece>> = {
            let fragments = lock(&FRAGMENTS);
            let texts = &mut lock(&TEXT_PIECES);
            self.parts
                .iter()
                .zip(&befores)
                .map(|(part, (before, _))| match part {
                    Part::Text { digest, .. } => texts.get(&(*digest, *before)).map(|text| text.piece),
                    Part::Fragment { fragment, glue, .. } => fragments
                        .get(&fragment_key(fragment))
                        .and_then(|known| known.piece_after(*before, glue)),
                })
                .collect()
        };
        let mut new_texts = Vec::new();
        let mut new_fragments = Vec::new();
        for (index, ((part, (before, dictionary)), piece)) in self.parts.iter().zip(&befores).zip(&mut pieces).enumerate() {
            if piece.is_some() {
                continue;
            }
            let bytes = part.bytes();
            let compressed = Piece {
                deflated: compress(dictionary, &bytes),
                crc: Crc::of(&bytes),
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
                Part::Fragment { fragment, glue, .. } => new_fragments.push((
                    fragment.clone(),
                    FragmentPiece {
                        before: *before,
                        _pin: pin,
                        glue: glue.to_vec().into(),
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

fn text_part(text: Bytes) -> Part {
    let digest = text_digest(&text);
    Part::Text { text, digest }
}

/// A fragment's bytes as a `Bytes` that shares them.
fn shared(fragment: &Arc<String>) -> Bytes {
    struct Shared(Arc<String>);
    impl AsRef<[u8]> for Shared {
        fn as_ref(&self) -> &[u8] {
            self.0.as_bytes()
        }
    }
    Bytes::from_owner(Shared(fragment.clone()))
}

/// Body bytes gathered piece by piece (text, and fragments too small for a part of their own),
/// joined only when there's more than one piece.
#[derive(Default)]
struct Gathered(Vec<Bytes>);

impl Gathered {
    fn push(&mut self, bytes: Bytes) {
        if !bytes.is_empty() {
            self.0.push(bytes);
        }
    }

    fn take(&mut self) -> Bytes {
        match self.0.len() {
            0 => Bytes::new(),
            1 => self.0.pop().expect("one piece"),
            _ => std::mem::take(&mut self.0).concat().into(),
        }
    }
}

/// A page's plain body: its parts' bytes one after another, never joined into one buffer.
pub struct PlainBody {
    page: Arc<PageParts>,
    /// The index of the next part to send.
    next: usize,
    /// A fragment due after the glue just sent.
    fragment: Option<Bytes>,
    remaining: u64,
}

impl HttpBody for PlainBody {
    type Data = Bytes;
    type Error = Infallible;

    fn poll_frame(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Option<Result<Frame<Bytes>, Infallible>>> {
        let this = self.get_mut();
        let chunk = this.fragment.take().or_else(|| {
            let part = this.page.parts.get(this.next)?;
            this.next += 1;
            let (chunk, fragment) = part.chunks();
            this.fragment = fragment;
            Some(chunk)
        });
        Poll::Ready(chunk.map(|chunk| {
            this.remaining -= chunk.len() as u64;
            Ok(Frame::data(chunk))
        }))
    }

    fn is_end_stream(&self) -> bool {
        self.remaining == 0
    }

    fn size_hint(&self) -> SizeHint {
        SizeHint::with_exact(self.remaining)
    }
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
    // The text plus its entry: the `Arc`'s counts, the digest and the map slot.
    lock(&KNOWN_TEXTS).insert(key, known, text.len() + std::mem::size_of::<KnownText>() + 64);
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
    use sha2::Digest as _;
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

    /// A page as a template records it (its text, and where each fragment went), and the body it
    /// stands for.
    #[derive(Default, Clone)]
    struct Page {
        plain: String,
        text: String,
        fragments: Vec<(usize, Arc<String>)>,
    }

    impl Page {
        fn text(mut self, text: &str) -> Self {
            self.plain.push_str(text);
            self.text.push_str(text);
            self
        }

        fn fragment(mut self, fragment: &Arc<String>) -> Self {
            self.plain.push_str(fragment);
            self.fragments.push((self.text.len(), fragment.clone()));
            self
        }

        fn parts(&self) -> PageParts {
            PageParts::splice(&Bytes::from(self.text.clone()), self.fragments.clone()).expect("a fragment big enough for a part")
        }

        fn gzip(&self, mtime: u32) -> Vec<u8> {
            self.parts().gzip(mtime)
        }

        fn etag(&self) -> String {
            self.parts().etag()
        }
    }

    fn page(head: &str, messages: &[Arc<String>], tail: &str) -> Page {
        messages
            .iter()
            .fold(Page::default().text(head), |page, message| page.text("  ").fragment(message))
            .text(tail)
    }

    async fn plain(parts: PageParts) -> Vec<u8> {
        use http_body_util::BodyExt;
        let body = Arc::new(parts).plain_body();
        assert_eq!(body.size_hint().exact(), Some(body.remaining));
        body.collect().await.unwrap().to_bytes().to_vec()
    }

    #[test]
    fn decodes_to_the_body_and_reuses_pieces() {
        let messages: Vec<_> = (0..30).map(message).collect();
        let page = page("<html><head>layout</head><body>", &messages, "</body></html>");
        let gz = page.gzip(1234);
        assert_eq!(gunzip(&gz), page.plain.as_bytes());
        assert_eq!(&gz[4..8], &1234u32.to_le_bytes());
        assert_eq!(gz[9], 3);
        let piece = lock(&FRAGMENTS)[&fragment_key(&messages[5])].pieces[0].clone();
        assert_eq!(page.gzip(1234), gz, "the same page is the same stored pieces");
        assert!(Arc::ptr_eq(&piece, &lock(&FRAGMENTS)[&fragment_key(&messages[5])].pieces[0]));
    }

    #[tokio::test]
    async fn the_plain_body_is_the_page() {
        let messages: Vec<_> = (900..910).map(message).collect();
        let small = Arc::new("<i>small</i>".to_string());
        let page = page("<p>", &messages[..5], "")
            .fragment(&small)
            .text(&"x".repeat(MAX_GLUE + 1))
            .fragment(&messages[5])
            .fragment(&messages[6])
            .text("</p>");
        let parts = page.parts();
        assert_eq!(parts.body_len(), page.plain.len());
        assert!(parts.fits(&http_body_util::Full::new(Bytes::from(page.plain.clone()))));
        assert!(!parts.fits(&http_body_util::Empty::<Bytes>::new()), "a HEAD response's body");
        assert_eq!(plain(parts).await, page.plain.as_bytes());
    }

    /// The ETags and gzip members the split by searching the body for its fragments gave these
    /// pages before KIT-11 (at 95aca44).
    #[test]
    fn the_same_etags_and_gzip_as_before_kit_11() {
        let sha = |gz: Vec<u8>| hex::encode(sha2::Sha256::digest(gz));
        let messages: Vec<_> = (0..30).map(message).collect();
        let whole = page("<html><head>layout</head><body>", &messages, "</body></html>");
        assert_eq!(whole.etag(), "0f21f66bd8370045fd702d07473468e2");
        assert_eq!(sha(whole.gzip(7)), "20bebddcfa29cdde1b76e1572479c130eb4cfd9bfb0e032e5332a2e3b453cfe3");

        let small = Arc::new("<i>small</i>".to_string());
        let mixed = Page::default()
            .text("<p>")
            .fragment(&messages[0])
            .text(&"x".repeat(300))
            .fragment(&messages[1])
            .text("  ")
            .fragment(&small)
            .text("  ")
            .fragment(&messages[2])
            .text("</p>");
        assert_eq!(mixed.etag(), "3d1d4208307a20a71a8ac959c24e0ec1");
        assert_eq!(sha(mixed.gzip(7)), "1de43a2a5729028f5084ac91c02de84b5518b61093f2a6896da5ed785e3ce724");
    }

    #[test]
    fn a_changed_layout_or_neighbour_decodes_correctly() {
        let messages: Vec<_> = (100..110).map(message).collect();
        page("<p>", &messages, "</p>").gzip(0);
        for page in [page("<p>changed", &messages, "</p>"), page("<p>", &messages, "</p>changed")] {
            assert_eq!(gunzip(&page.gzip(0)), page.plain.as_bytes());
        }
        // Drop one message: the one after it now follows a different predecessor.
        let mut fewer = messages.clone();
        fewer.remove(4);
        let page = page("<p>", &fewer, "</p>");
        assert_eq!(gunzip(&page.gzip(0)), page.plain.as_bytes());
        // Different glue between the same fragments.
        let page = messages
            .iter()
            .fold(Page::default().text("<p>"), |page, message| page.text("\n    ").fragment(message));
        assert_eq!(gunzip(&page.gzip(0)), page.plain.as_bytes());
    }

    #[test]
    fn small_repeated_and_far_apart_fragments() {
        let messages: Vec<_> = (200..206).map(message).collect();
        let small = Arc::new("<i>small</i>".to_string());
        let with_small = page("<p>", &messages[..1], "  ").fragment(&small);
        let with_small = messages[1..3].iter().fold(with_small, |page, message| page.text("  ").fragment(message)).text("</p>");
        assert_eq!(gunzip(&with_small.gzip(0)), with_small.plain.as_bytes());
        // A small fragment is text to the parts.
        let as_text = page("<p>", &messages[..1], &format!("  {small}"));
        let as_text = messages[1..3].iter().fold(as_text, |page, message| page.text("  ").fragment(message)).text("</p>");
        assert_eq!(with_small.etag(), as_text.etag());
        let only_small = Page::default().text("<p>").fragment(&small).text("</p>");
        let whole = PageParts::splice(&Bytes::from(only_small.text), only_small.fragments).expect_err("no part");
        assert_eq!(whole, only_small.plain.as_bytes());

        let (a, b) = (message(300), message(301));
        let page = Page::default()
            .fragment(&a)
            .fragment(&b)
            .fragment(&a)
            .text(&"x".repeat(MAX_GLUE + 1))
            .fragment(&b)
            .fragment(&a);
        for _ in 0..2 {
            assert_eq!(gunzip(&page.gzip(0)), page.plain.as_bytes());
        }
    }

    #[test]
    fn a_fragment_keeps_a_piece_for_each_predecessor() {
        let messages: Vec<_> = (600..606).map(message).collect();
        let room = page("<p>", &messages, "</p>");
        // The same last message after a different one, as in search results.
        let search = page("<q>", &[messages[1].clone(), messages[5].clone()], "</q>");
        room.gzip(0);
        search.gzip(0);
        let pieces = |fragment: &Arc<String>| lock(&FRAGMENTS)[&fragment_key(fragment)].pieces.clone();
        let before = pieces(&messages[5]);
        assert_eq!(before.len(), 2, "one after message 604, one after 601");
        room.gzip(0);
        search.gzip(0);
        let after = pieces(&messages[5]);
        assert!(before.iter().zip(&after).all(|(a, b)| Arc::ptr_eq(a, b)), "both pages reuse theirs");
    }

    #[test]
    fn etags_follow_the_content() {
        let messages: Vec<_> = (400..410).map(message).collect();
        let body = page("<p>", &messages, "</p>");
        assert_eq!(body.etag(), page("<p>", &messages, "</p>").etag());
        assert_eq!(body.etag().len(), 32);
        assert_ne!(body.etag(), page("<p>", &messages, "</p>!").etag());
        assert_ne!(body.etag(), page("<q>", &messages, "</p>").etag());
        let glued = messages
            .iter()
            .fold(Page::default().text("<p>"), |page, message| page.text(" ").fragment(message))
            .text("</p>");
        assert_ne!(body.etag(), glued.etag());
    }

    /// What `gzip` sent before KIT-9 took the CRC from the pieces': every part compressed against
    /// the one before it, and one CRC over the whole body.
    fn gzip_before_kit_9(page: &Page, mtime: u32) -> Vec<u8> {
        let parts = page.parts();
        let mut out = vec![0x1f, 0x8b, 8, 0];
        out.extend_from_slice(&mtime.to_le_bytes());
        out.extend_from_slice(&[0, 3]);
        let mut dictionary = &b""[..];
        for part in &parts.parts {
            out.extend_from_slice(&compress(dictionary, &part.bytes()));
            dictionary = part.as_before().1;
        }
        out.extend_from_slice(&[0x03, 0x00]);
        out.extend_from_slice(&crc32fast::hash(page.plain.as_bytes()).to_le_bytes());
        out.extend_from_slice(&(page.plain.len() as u32).to_le_bytes());
        out
    }

    #[test]
    fn the_same_bytes_as_before_kit_9() {
        let messages: Vec<_> = (700..780).map(message).collect();
        let head: String = (0..400).map(|n| format!("<link rel=\"preload\" href=\"/a/{n}.js\">")).collect();
        let tail: String = (0..200).map(|n| format!("<footer data-n=\"{n}\"></footer>")).collect();
        let body = page(&head, &messages, &tail);
        assert_eq!(body.gzip(77), gzip_before_kit_9(&body, 77), "first request");
        assert_eq!(body.gzip(77), gzip_before_kit_9(&body, 77), "from stored pieces");
        // A new layout around the same fragments: new text pieces, stored fragment ones.
        let changed = page(&format!("{head}!"), &messages, &tail);
        assert_eq!(changed.gzip(77), gzip_before_kit_9(&changed, 77));
    }

    #[test]
    fn a_one_byte_change_in_the_middle_is_new_text() {
        let messages: Vec<_> = (800..820).map(message).collect();
        let head: String = (0..2000).map(|n| format!("<meta name=\"m{n}\">")).collect();
        let mut changed = head.clone().into_bytes();
        changed[head.len() / 2] ^= 1;
        let changed = String::from_utf8(changed).unwrap();
        let (body, changed) = (page(&head, &messages, "</p>"), page(&changed, &messages, "</p>"));
        let first_digest = |page: &Page| match &page.parts().parts[0] {
            Part::Text { text, digest } => {
                assert_eq!(*digest, *blake3::hash(text).as_bytes());
                *digest
            }
            Part::Fragment { .. } => panic!("the page starts with text"),
        };
        for _ in 0..2 {
            assert_ne!(first_digest(&body), first_digest(&changed));
            assert_ne!(body.etag(), changed.etag());
            for page in [&body, &changed] {
                let gz = page.gzip(5);
                assert_eq!(gz, gzip_before_kit_9(page, 5));
                assert_eq!(gunzip(&gz), page.plain.as_bytes());
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
        let page = page(&"<head>layout</head>".repeat(200), &messages, &"<footer/>".repeat(300));
        let mut whole = flate2::write::GzEncoder::new(Vec::new(), Compression::default());
        whole.write_all(page.plain.as_bytes()).unwrap();
        let whole = whole.finish().unwrap().len();
        let spliced = page.gzip(0).len();
        // Each piece costs its flush marker and block header, not a recompressed message.
        assert!(
            spliced < whole + 40 * (messages.len() + 2),
            "{spliced} bytes spliced vs {whole} whole"
        );
    }

    /// KIT-11's gate: what a room-sized page (~450 KB: ~45 KB of layout, 40 messages of ~10 KB)
    /// costs the kit per request, from its stored pieces, as parts recorded while it rendered
    /// versus as before: the fragments copied into one body (as the render wrote them) and then
    /// found in it again. Run pinned with
    /// `taskset -c 0-7 cargo test --release -p campfire_kit --lib parts_vs_search -- --ignored --nocapture`.
    #[test]
    #[ignore = "a benchmark"]
    fn parts_vs_search() {
        use std::time::Instant;
        let head: String = (0..1000).map(|n| format!("<link rel=\"preload\" href=\"/a/{n}.js\">")).collect();
        let tail: String = (0..300).map(|n| format!("<footer data-n=\"{n}\"></footer>")).collect();
        let messages: Vec<_> = (0..40)
            .map(|n| Arc::new(format!("<div id=\"m{n}\">{}</div>\n", "<button>Boost</button> hello <a href=\"/x\">x</a> ".repeat(180 + n % 13))))
            .collect();
        let page = page(&head, &messages, &tail);
        let recorded = || PageParts::splice(&Bytes::from(page.text.clone()), page.fragments.clone()).unwrap();
        // The body as the render used to write it, and the fragments' offsets found in it.
        let searched = || {
            let mut body = String::with_capacity(page.plain.len() + page.plain.len() / 8);
            let mut position = 0;
            for (offset, fragment) in &page.fragments {
                body.push_str(&page.text[position..*offset]);
                body.push_str(fragment);
                position = *offset;
            }
            body.push_str(&page.text[position..]);
            let (mut text, mut fragments, mut at) = (String::new(), Vec::new(), 0);
            for fragment in &messages {
                let start = find(body.as_bytes(), at, fragment.as_bytes()).expect("in the body");
                text.push_str(&body[at..start]);
                fragments.push((text.len(), fragment.clone()));
                at = start + fragment.len();
            }
            text.push_str(&body[at..]);
            PageParts::splice(&Bytes::from(text), fragments).unwrap()
        };
        let per_request = |parts: &dyn Fn() -> PageParts| {
            let parts = parts();
            std::hint::black_box((parts.etag(), parts.gzip(0)));
        };
        assert_eq!(recorded().gzip(0), searched().gzip(0));
        let rounds = 4000;
        for (name, parts) in [("searched", &searched as &dyn Fn() -> PageParts), ("recorded", &recorded)] {
            for _ in 0..200 {
                per_request(parts);
            }
            let start = Instant::now();
            for _ in 0..rounds {
                per_request(parts);
            }
            let micros = start.elapsed().as_secs_f64() * 1e6 / f64::from(rounds);
            println!("{name}: {micros:.1} us per {} B page", page.plain.len());
        }
    }

    /// The first occurrence of `needle` in `body` at or after `from`, the way pages were split
    /// before KIT-11: a short prefix found with `memmem`, the rest compared.
    fn find(body: &[u8], from: usize, needle: &[u8]) -> Option<usize> {
        let finder = memchr::memmem::Finder::new(&needle[..needle.len().min(64)]);
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
}
