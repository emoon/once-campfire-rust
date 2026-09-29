//! `Rack::Deflater` (rack 3.2), which the reference installs around the whole app in `config.ru`:
//! it gzips every response with a body when the client accepts gzip, whatever its size or type,
//! and adds `Accept-Encoding` to `Vary`. A gzipped body has no `Content-Length`, so it goes out
//! chunked (and the front server's compression leaves it alone).

use std::collections::HashMap;
use std::hash::Hash;
use std::io::Write;
use std::sync::{Arc, LazyLock, Mutex, MutexGuard};

use axum::body::Body;
use axum::extract::Request;
use axum::http::{HeaderValue, StatusCode, header};
use axum::middleware::Next;
use axum::response::Response;
use bytes::Bytes;
use flate2::write::GzEncoder;
use flate2::{Compression, GzBuilder};
use futures_util::StreamExt;
use http_body_util::BodyExt;

pub mod splice;

/// A response `ActionDispatch::Static` served (a public file or an asset). Marks responses the
/// middleware below `Static` in the reference (`Rack::Runtime`, `ActionDispatch::RequestId`)
/// never saw.
#[derive(Debug, Clone, Copy)]
pub struct StaticFile;

/// A `Content-Length` the app set itself (`PublicExceptions`, `ShowExceptions#pass_response`),
/// as opposed to one hyper adds after this middleware from the body's size.
#[derive(Debug, Clone, Copy)]
pub struct AppContentLength;

/// The SHA-256 `Rack::ETag` took of a response's whole body: the body's identity, so the gzip of a
/// body that repeats (the sidebar) comes from [`GZIPPED`] instead of being deflated again.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct BodyDigest(pub(crate) [u8; 32]);

/// A bound on the bytes of kept gzip members (a sidebar's is ~6 KB).
const MAX_GZIPPED_BYTES: usize = 16 << 20;

/// The `Rack::Deflater` middleware.
pub async fn deflater(request: Request, next: Next) -> Response {
    let accept_encoding = request
        .headers()
        .get(header::ACCEPT_ENCODING)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("")
        .to_string();
    let path = request.uri().path_and_query().map(|p| p.as_str().to_string()).unwrap_or_default();
    let mut response = next.run(request).await;
    if !should_deflate(&response) {
        return response;
    }

    let encoding = select_best_encoding(&["gzip", "identity"], &parse_accept_encoding(&accept_encoding));

    let vary: Vec<String> = response
        .headers()
        .get_all(header::VARY)
        .iter()
        .filter_map(|v| v.to_str().ok())
        .flat_map(|v| v.split(',').map(|t| t.trim().to_string()).collect::<Vec<_>>())
        .collect();
    if !vary.iter().any(|v| v == "*" || v.eq_ignore_ascii_case("accept-encoding")) {
        let mut vary: Vec<String> = if response.headers().contains_key(header::VARY) {
            vary
        } else {
            Vec::new()
        };
        vary.push("Accept-Encoding".into());
        if let Ok(value) = HeaderValue::from_str(&vary.join(",")) {
            response.headers_mut().insert(header::VARY, value);
        }
    }

    match encoding {
        Some("gzip") => {
            let mtime = response
                .headers()
                .get(header::LAST_MODIFIED)
                .and_then(|v| v.to_str().ok())
                .and_then(crate::clock::parse_httpdate)
                .map(|t| t.as_second() as u32)
                .unwrap_or(0);
            let headers = response.headers_mut();
            headers.insert(header::CONTENT_ENCODING, HeaderValue::from_static("gzip"));
            headers.remove(header::CONTENT_LENGTH);
            let (mut parts, body) = response.into_parts();
            let body = if let Some(page_parts) = parts.extensions.remove::<Arc<splice::PageParts>>() {
                gzip_page_parts(body, &page_parts, mtime).await
            } else if let Some(digest) = parts.extensions.remove::<BodyDigest>() {
                gzip_digested(body, digest, mtime).await
            } else {
                gzip_stream(body, mtime)
            };
            Response::from_parts(parts, body)
        }
        Some(_) => response,
        None => {
            let message = format!("An acceptable encoding for the requested resource {path} could not be found.");
            let mut response = Response::new(Body::from(message.clone()));
            *response.status_mut() = StatusCode::NOT_ACCEPTABLE;
            response
                .headers_mut()
                .insert(header::CONTENT_TYPE, HeaderValue::from_static("text/plain"));
            response
                .headers_mut()
                .insert(header::CONTENT_LENGTH, HeaderValue::from(message.len()));
            response
        }
    }
}

/// `should_deflate?` (rack 3.2.6; the reference passes no `:include` or `:if`): not for statuses
/// without a body (1xx, 204, 304), `no-transform`, non-identity `Content-Encoding`, or a
/// `Content-Length: 0` the app set (static files and error pages; an app response's length is
/// otherwise the server's doing, after this middleware, so empty rendered bodies are gzipped).
fn should_deflate(response: &Response) -> bool {
    let status = response.status().as_u16();
    if matches!(status, 100..=199 | 204 | 304) {
        return false;
    }
    let headers = response.headers();
    let get = |name| headers.get(name).and_then(|v: &HeaderValue| v.to_str().ok());
    if get(header::CACHE_CONTROL).is_some_and(|cc| has_word(cc, "no-transform")) {
        return false;
    }
    if get(header::CONTENT_ENCODING).is_some_and(|ce| !has_word(ce, "identity")) {
        return false;
    }
    let extensions = response.extensions();
    let app_set_length = extensions.get::<StaticFile>().is_some() || extensions.get::<AppContentLength>().is_some();
    !(app_set_length && get(header::CONTENT_LENGTH) == Some("0"))
}

/// `/\bword\b/`
fn has_word(haystack: &str, word: &str) -> bool {
    let is_word = |c: char| c.is_ascii_alphanumeric() || c == '_';
    haystack.match_indices(word).any(|(i, _)| {
        let before = haystack[..i].chars().next_back();
        let after = haystack[i + word.len()..].chars().next();
        !before.is_some_and(is_word) && !after.is_some_and(is_word)
    })
}

/// `Rack::Request#accept_encoding` (`parse_http_accept_header`).
fn parse_accept_encoding(header: &str) -> Vec<(String, f64)> {
    header
        .split(',')
        .map(str::trim)
        .filter(|part| !part.is_empty())
        .map(|part| {
            let (attribute, parameters) = match part.split_once(';') {
                Some((a, p)) => (a.trim(), Some(p.trim())),
                None => (part, None),
            };
            let quality = parameters
                .and_then(|p| p.strip_prefix("q="))
                .map(|q| {
                    let digits: String = q.chars().take_while(|c| c.is_ascii_digit() || *c == '.').collect();
                    ruby_to_f(&digits)
                })
                .unwrap_or(1.0);
            (attribute.to_string(), quality)
        })
        .collect()
}

/// `String#to_f` on `[\d.]+`: the longest leading float.
fn ruby_to_f(s: &str) -> f64 {
    let mut end = 0;
    let mut dot = false;
    for (i, c) in s.char_indices() {
        if c == '.' {
            if dot {
                break;
            }
            dot = true;
        }
        end = i + 1;
    }
    s[..end].trim_end_matches('.').parse().unwrap_or(0.0)
}

/// `Rack::Utils.select_best_encoding`
fn select_best_encoding(available: &[&'static str], accept: &[(String, f64)]) -> Option<&'static str> {
    let accept = &accept[..accept.len().min(16)];
    let mut expanded: Vec<(String, f64, usize)> = Vec::new();
    let mut wildcard_seen = false;
    for (m, q) in accept {
        let preference = available.iter().position(|a| a == m).unwrap_or(available.len());
        if m == "*" {
            if !wildcard_seen {
                for m2 in available.iter().filter(|a| !accept.iter().any(|(m, _)| m == *a)) {
                    expanded.push((m2.to_string(), *q, preference));
                }
                wildcard_seen = true;
            }
        } else {
            expanded.push((m.clone(), *q, preference));
        }
    }
    let mut sorted = expanded.clone();
    sorted.sort_by(|(_, q1, p1), (_, q2, p2)| q2.partial_cmp(q1).unwrap_or(std::cmp::Ordering::Equal).then(p1.cmp(p2)));
    let mut candidates: Vec<String> = sorted.into_iter().map(|(m, _, _)| m).collect();
    if !candidates.iter().any(|c| c == "identity") {
        candidates.push("identity".into());
    }
    for (m, q, _) in &expanded {
        if *q == 0.0 {
            candidates.retain(|c| c != m);
        }
    }
    candidates.iter().find_map(|c| available.iter().copied().find(|a| a == c))
}

/// `GzipStream` with `sync: true`: each body chunk is compressed and flushed as it arrives.
/// `Zlib::GzipWriter` writes the header with the given mtime and the Unix OS code.
fn gzip_stream(body: Body, mtime: u32) -> Body {
    let encoder = gzip_encoder(mtime);
    let chunks = body.into_data_stream();
    let stream = futures_util::stream::unfold(Some((chunks, encoder)), |state| async move {
        let (mut chunks, mut encoder) = state?;
        loop {
            match chunks.next().await {
                Some(Ok(chunk)) => {
                    if chunk.is_empty() {
                        continue;
                    }
                    let output = compress(&mut encoder, &chunk);
                    return Some((output, Some((chunks, encoder))));
                }
                Some(Err(error)) => return Some((Err(std::io::Error::other(error)), None)),
                None => return Some((encoder.finish().map(Bytes::from), None)),
            }
        }
    });
    Body::from_stream(stream)
}

fn gzip_encoder(mtime: u32) -> GzEncoder<Vec<u8>> {
    GzBuilder::new()
        .mtime(mtime)
        .operating_system(3)
        .write(Vec::new(), Compression::default())
}

/// A body `Rack::ETag` digested (always a single buffer), gzipped once while it keeps repeating.
async fn gzip_digested(body: Body, digest: BodyDigest, mtime: u32) -> Body {
    let bytes = match collect(body).await {
        Ok(bytes) => bytes,
        Err(error) => return error,
    };
    let key = (digest, mtime);
    let cached = lock(&GZIPPED).get(&key);
    let gzipped = match cached {
        Some(gzipped) => gzipped,
        None => match gzip_member(&bytes, mtime) {
            Ok(gzipped) => {
                // One huge body mustn't push out everything else.
                if gzipped.len() <= MAX_GZIPPED_BYTES / 4 {
                    lock(&GZIPPED).insert(key, gzipped.clone(), gzipped.len());
                }
                gzipped
            }
            Err(error) => return error_body(error),
        },
    };
    single_chunk(gzipped)
}

/// What [`gzip_stream`] sends for a single-buffer body, in one piece.
fn gzip_member(body: &[u8], mtime: u32) -> std::io::Result<Bytes> {
    let mut encoder = gzip_encoder(mtime);
    if !body.is_empty() {
        encoder.write_all(body)?;
        encoder.flush()?;
    }
    let mut member = encoder.finish()?;
    // Kept for as long as it's used, so without the spare capacity growing it left.
    member.shrink_to_fit();
    Ok(member.into())
}

/// A body split into [`splice::PageParts`] (always a single buffer), as their stored pieces; the
/// same decoded bytes as [`gzip_stream`].
async fn gzip_page_parts(body: Body, page_parts: &splice::PageParts, mtime: u32) -> Body {
    let bytes = match collect(body).await {
        Ok(bytes) => bytes,
        Err(error) => return error,
    };
    if !page_parts.fits(&bytes) {
        return gzip_stream(Body::from(bytes), mtime);
    }
    single_chunk(page_parts.gzip(&bytes, mtime).into())
}

/// A single-buffer body's bytes, or a body that fails with its error.
async fn collect(body: Body) -> Result<Bytes, Body> {
    body.collect()
        .await
        .map(http_body_util::Collected::to_bytes)
        .map_err(|error| error_body(std::io::Error::other(error)))
}

/// Streamed like [`gzip_stream`]'s output, so no `Content-Length` goes with it.
fn single_chunk(gzipped: Bytes) -> Body {
    Body::from_stream(futures_util::stream::once(async move { Ok::<_, std::io::Error>(gzipped) }))
}

fn error_body(error: std::io::Error) -> Body {
    Body::from_stream(futures_util::stream::once(async move { Err::<Bytes, _>(error) }))
}

fn compress(encoder: &mut GzEncoder<Vec<u8>>, chunk: &[u8]) -> std::io::Result<Bytes> {
    encoder.write_all(chunk)?;
    encoder.flush()?;
    Ok(Bytes::from(std::mem::take(encoder.get_mut())))
}

/// A map bounded by bytes, in two generations: a read promotes an old entry, and when the young
/// generation fills half the budget it becomes the old one (dropping the previous old one). What's
/// in use stays, and each generation overshoots half the budget by at most the entry that filled it.
struct Generations<K, V> {
    max_bytes: usize,
    young: HashMap<K, (V, usize)>,
    old: HashMap<K, (V, usize)>,
    young_bytes: usize,
}

impl<K: Copy + Eq + Hash, V: Clone> Generations<K, V> {
    fn new(max_bytes: usize) -> Self {
        Self {
            max_bytes,
            young: HashMap::new(),
            old: HashMap::new(),
            young_bytes: 0,
        }
    }

    fn get(&mut self, key: &K) -> Option<V> {
        if let Some((value, _)) = self.young.get(key) {
            return Some(value.clone());
        }
        let (value, size) = self.old.remove(key)?;
        self.insert(*key, value.clone(), size);
        Some(value)
    }

    /// Keeps `value`, which holds `size` bytes.
    fn insert(&mut self, key: K, value: V, size: usize) {
        self.young_bytes += size;
        self.young.insert(key, (value, size));
        if self.young_bytes > self.max_bytes / 2 {
            self.old = std::mem::take(&mut self.young);
            self.young_bytes = 0;
        }
    }
}

/// Gzip members by body digest and gzip mtime.
static GZIPPED: LazyLock<Mutex<Generations<(BodyDigest, u32), Bytes>>> = LazyLock::new(|| Mutex::new(Generations::new(MAX_GZIPPED_BYTES)));

fn lock<T>(mutex: &'static Mutex<T>) -> MutexGuard<'static, T> {
    mutex.lock().unwrap_or_else(|poisoned| poisoned.into_inner())
}

#[cfg(test)]
mod tests {
    use super::*;
    use sha2::Digest;

    fn best(header: &str) -> Option<&'static str> {
        select_best_encoding(&["gzip", "identity"], &parse_accept_encoding(header))
    }

    #[test]
    fn encoding_selection() {
        assert_eq!(best("gzip, deflate, br"), Some("gzip"));
        assert_eq!(best(""), Some("identity"));
        assert_eq!(best("br"), Some("identity"));
        assert_eq!(best("gzip;q=0"), Some("identity"));
        assert_eq!(best("identity;q=0.5, gzip;q=0.1"), Some("identity"));
        assert_eq!(best("*"), Some("gzip"));
        assert_eq!(best("identity;q=0, *;q=0"), None);
        assert_eq!(best("gzip;q=0, identity;q=0"), None);
    }

    #[test]
    fn words() {
        assert!(has_word("no-transform, public", "no-transform"));
        assert!(!has_word("xno-transform", "no-transform"));
        assert!(has_word("identity", "identity"));
    }

    #[tokio::test]
    async fn gzip_round_trip_and_empty_bodies() {
        use http_body_util::BodyExt;
        use std::io::Read;
        for input in [&b""[..], b"hello world"] {
            let body = gzip_stream(Body::from(Bytes::copy_from_slice(input)), 0);
            let bytes = body.collect().await.unwrap().to_bytes();
            assert!(bytes.len() >= 20);
            let mut out = Vec::new();
            flate2::read::GzDecoder::new(&bytes[..]).read_to_end(&mut out).unwrap();
            assert_eq!(out, input);
        }
    }

    async fn gzipped(body: Body) -> Bytes {
        body.collect().await.unwrap().to_bytes()
    }

    fn digest(body: &[u8]) -> BodyDigest {
        BodyDigest(sha2::Sha256::digest(body).into())
    }

    #[tokio::test]
    async fn digested_bodies_are_gzipped_once() {
        let body = Bytes::from("<a href=\"/rooms/1\">Room</a>".repeat(300));
        let first = gzipped(gzip_digested(Body::from(body.clone()), digest(&body), 0).await).await;
        assert_eq!(
            first,
            gzipped(gzip_stream(Body::from(body.clone()), 0)).await,
            "what gzip_stream sends"
        );
        let kept = lock(&GZIPPED).get(&(digest(&body), 0)).expect("kept");
        let again = gzipped(gzip_digested(Body::from(body.clone()), digest(&body), 0).await).await;
        assert_eq!(again.as_ptr(), kept.as_ptr(), "not gzipped again");

        let other = Bytes::from("<a href=\"/rooms/2\">Room</a>".repeat(300));
        let gzip = gzipped(gzip_digested(Body::from(other.clone()), digest(&other), 0).await).await;
        assert_eq!(gzip, gzipped(gzip_stream(Body::from(other), 0)).await);
    }

    #[test]
    fn generations_stay_within_their_budget() {
        let max_bytes = 16 << 20;
        let mut generations = Generations::new(max_bytes);
        let size = 1 << 20;
        for n in 0..100u8 {
            generations.insert(n, (), size);
            let held = (generations.young.len() + generations.old.len()) * size;
            // Each generation may overshoot half the budget by the entry that filled it.
            assert!(held <= max_bytes + 2 * size, "{held} bytes held");
        }
        assert!(generations.get(&99).is_some(), "the latest is kept");
    }

    /// KIT-10's gate: what a body that never repeats costs, gzipped as before (a fresh `GzEncoder`)
    /// and through the cache (lookup, gzip, insert); and what a stream of distinct bodies leaves
    /// held. The SHA-256, which `Rack::ETag` takes either way, isn't timed. `GZIP_BENCH_BODY` names a
    /// saved body (a sidebar) that each round varies by a counter; run pinned with
    /// `cargo test --release -p campfire_kit --lib gzip_miss_overhead -- --ignored --nocapture`.
    #[test]
    #[ignore = "a benchmark"]
    fn gzip_miss_overhead() {
        use std::time::{Duration, Instant};
        let path = std::env::var("GZIP_BENCH_BODY").expect("GZIP_BENCH_BODY");
        let mut body = std::fs::read(path).unwrap();
        let rss_mib = || {
            let statm = std::fs::read_to_string("/proc/self/statm").unwrap();
            statm.split(' ').nth(1).unwrap().parse::<f64>().unwrap() * 4096.0 / f64::from(1 << 20)
        };
        let (bodies, rounds) = (2_000, 20);
        let mut counter = 0u64;
        let mut next_key = |body: &mut Vec<u8>| {
            counter += 1;
            body[..20].copy_from_slice(format!("{counter:020}").as_bytes());
            (digest(body), 0)
        };
        let (mut before, mut cached) = (Duration::ZERO, Duration::ZERO);
        let rss_start = rss_mib();
        let mut rss_half = 0.0;
        for round in 0..rounds {
            for _ in 0..bodies {
                next_key(&mut body);
                let start = Instant::now();
                let mut encoder = gzip_encoder(0);
                encoder.write_all(&body).unwrap();
                encoder.flush().unwrap();
                std::hint::black_box(encoder.finish().unwrap());
                before += start.elapsed();
            }
            for _ in 0..bodies {
                let key = next_key(&mut body);
                let start = Instant::now();
                assert!(lock(&GZIPPED).get(&key).is_none());
                let gzipped = gzip_member(&body, 0).unwrap();
                lock(&GZIPPED).insert(key, gzipped.clone(), gzipped.len());
                std::hint::black_box(gzipped);
                cached += start.elapsed();
            }
            if round == rounds / 2 {
                rss_half = rss_mib() - rss_start;
            }
        }
        let per = |total: Duration| total.as_secs_f64() * 1e6 / f64::from(bodies * rounds);
        println!(
            "body {} B; per distinct body: before {:.1} us, through the cache {:.1} us ({:+.1} us, {:+.1}%)",
            body.len(),
            per(before),
            per(cached),
            per(cached) - per(before),
            (per(cached) / per(before) - 1.0) * 100.0
        );
        let held: usize = {
            let gzipped = lock(&GZIPPED);
            gzipped.young.values().chain(gzipped.old.values()).map(|(_, size)| size).sum()
        };
        println!(
            "after {} distinct bodies: {:.1} MiB held (bound {} MiB); RSS {rss_half:+.1} MiB halfway, {:+.1} MiB at the end",
            bodies * rounds,
            held as f64 / f64::from(1 << 20),
            MAX_GZIPPED_BYTES >> 20,
            rss_mib() - rss_start
        );
        assert!(held <= MAX_GZIPPED_BYTES + 2 * MAX_GZIPPED_BYTES / 4);
    }
}
