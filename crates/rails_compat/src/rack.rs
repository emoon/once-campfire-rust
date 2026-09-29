//! HTTP byte ranges as Rack serves them (rack 3.2): `Rack::Utils.get_byte_ranges`, and the
//! `multipart/byteranges` body `Rack::Files` builds for several ranges, which Active Storage's
//! `send_blob_byte_range_data` builds too, spelled its own way.

use crate::ruby;

/// `Rack::Files::MULTIPART_BOUNDARY`
pub const MULTIPART_BOUNDARY: &str = "AaB03x";

/// `Rack::Utils.get_byte_ranges(http_range, size)`: inclusive `(start, end)` ranges. `None` means
/// serve the whole body (no header, a malformed one, or an empty body), and an empty list means 416.
///
/// Numbers are Ruby's `String#to_i`, which saturates past `i64` here where Ruby has Bignums. That
/// only shows in a backwards range between two such numbers, which is dropped where Rack ignores
/// the header.
pub fn byte_ranges(header: Option<&str>, size: u64) -> Option<Vec<(u64, u64)>> {
    if size == 0 {
        return None;
    }
    let spec = range_spec(header?)?;
    if spec.matches(',').count() >= 100 {
        return None;
    }
    let last = size - 1;
    let mut ranges = Vec::new();
    for range_spec in split_ranges(spec) {
        // `range_spec.split('-')`, whose trailing empty fields Ruby drops.
        let (r0, rest) = range_spec.split_once('-')?;
        let r1 = rest
            .contains(|c: char| c != '-')
            .then(|| &rest[..rest.find('-').unwrap_or(rest.len())]);
        let (start, end) = if r0.is_empty() {
            (size.saturating_sub(number(r1?)), last)
        } else {
            let start = number(r0);
            match r1.map(number) {
                None => (start, last),
                Some(end) if end < start => return None,
                Some(end) => (start, end.min(last)),
            }
        };
        if start <= end {
            ranges.push((start, end));
        }
    }
    let total: u128 = ranges.iter().map(|&(start, end)| u128::from(end - start + 1)).sum();
    if total > u128::from(size) {
        return Some(Vec::new());
    }
    Some(ranges)
}

/// The first match of `/bytes=([^;]+)/`.
fn range_spec(header: &str) -> Option<&str> {
    header.match_indices("bytes=").find_map(|(i, prefix)| {
        let rest = &header[i + prefix.len()..];
        let spec = &rest[..rest.find(';').unwrap_or(rest.len())];
        (!spec.is_empty()).then_some(spec)
    })
}

/// `spec.split(/,[ \t]*/)`: Ruby drops the trailing empty fields, so none at all for a spec
/// that is only separators.
fn split_ranges(spec: &str) -> impl Iterator<Item = &str> {
    let mut spec = spec;
    while let Some(rest) = spec.trim_end_matches([' ', '\t']).strip_suffix(',') {
        spec = rest;
    }
    let fields = (!spec.is_empty()).then(|| spec.split(','));
    fields
        .into_iter()
        .flatten()
        .enumerate()
        .map(|(i, field)| if i == 0 { field } else { field.trim_start_matches([' ', '\t']) })
}

/// A field's `to_i`. It's never negative: the fields were split on `-`.
fn number(field: &str) -> u64 {
    u64::try_from(ruby::to_i(field)).unwrap_or(0)
}

/// A `multipart/byteranges` body: each range after its heading, then the closing delimiter.
pub struct Multipart<'a> {
    pub boundary: &'a str,
    pub spelling: Spelling,
    /// Every part's content type.
    pub content_type: &'a str,
    /// The size of the whole body the ranges are from.
    pub size: u64,
}

impl Multipart<'_> {
    /// The response's content type.
    pub fn content_type_header(&self) -> String {
        format!("multipart/byteranges; boundary={}", self.boundary)
    }

    pub fn parts(&self, ranges: &[(u64, u64)]) -> Vec<Part> {
        let (content_type, content_range) = match self.spelling {
            Spelling::Rack => ("content-type", "content-range"),
            Spelling::ActiveStorage => ("Content-Type", "Content-Range"),
        };
        let mut parts = Vec::with_capacity(ranges.len() * 2 + 1);
        for &(start, end) in ranges {
            parts.push(Part::Text(format!(
                "\r\n--{}\r\n{content_type}: {}\r\n{content_range}: bytes {start}-{end}/{}\r\n\r\n",
                self.boundary, self.content_type, self.size
            )));
            parts.push(Part::Range(start, end));
        }
        parts.push(Part::Text(format!("\r\n--{}--\r\n", self.boundary)));
        parts
    }
}

/// How the part headings spell their header names.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Spelling {
    /// `content-type`, `content-range` (`Rack::Files::BaseIterator#multipart_heading`).
    Rack,
    /// `Content-Type`, `Content-Range` (`ActiveStorage::Streaming#send_blob_byte_range_data`).
    ActiveStorage,
}

/// One piece of a response body, in order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Part {
    /// Text between the ranges: a heading or the closing delimiter.
    Text(String),
    /// An inclusive byte range of the body being served.
    Range(u64, u64),
}

impl Part {
    fn len(&self) -> u64 {
        match self {
            Part::Text(text) => text.len() as u64,
            Part::Range(start, end) => end - start + 1,
        }
    }
}

/// The `content-length` of a body made of `parts`.
pub fn content_length(parts: &[Part]) -> u64 {
    parts.iter().map(Part::len).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    // Expected values are rack 3.2.6's `Rack::Utils.get_byte_ranges(header, 10)` on the reference
    // image.

    #[test]
    fn byte_ranges_match_rack() {
        assert_eq!(byte_ranges(None, 10), None);
        assert_eq!(byte_ranges(Some("bytes=0-4"), 10), Some(vec![(0, 4)]));
        assert_eq!(byte_ranges(Some("bytes=5-"), 10), Some(vec![(5, 9)]));
        assert_eq!(byte_ranges(Some("bytes=-3"), 10), Some(vec![(7, 9)]));
        assert_eq!(byte_ranges(Some("bytes=-30"), 10), Some(vec![(0, 9)]));
        assert_eq!(byte_ranges(Some("bytes=0-1, 3-4"), 10), Some(vec![(0, 1), (3, 4)]));
        assert_eq!(byte_ranges(Some("bytes=4-1"), 10), None);
        assert_eq!(byte_ranges(Some("bytes=20-30"), 10), Some(vec![]));
        assert_eq!(byte_ranges(Some("bytes=0-9,0-9"), 10), Some(vec![]));
        assert_eq!(byte_ranges(Some("bytes=-"), 10), None);
        assert_eq!(byte_ranges(Some("items=0-1"), 10), None);
        assert_eq!(byte_ranges(Some("Bytes=0-1"), 10), None);
        assert_eq!(byte_ranges(Some("xbytes=2-3"), 10), Some(vec![(2, 3)]));
        assert_eq!(byte_ranges(Some("bytes=0-100"), 10), Some(vec![(0, 9)]));
        assert_eq!(byte_ranges(Some("bytes=0-1"), 0), None);
    }

    #[test]
    fn numbers_are_to_i() {
        assert_eq!(byte_ranges(Some("bytes=0-99999999999999999999"), 10), Some(vec![(0, 9)]));
        assert_eq!(byte_ranges(Some("bytes=0d5-"), 10), Some(vec![(5, 9)]));
        assert_eq!(byte_ranges(Some("bytes=1_0-"), 10), Some(vec![]));
        assert_eq!(byte_ranges(Some("bytes=+2-+3"), 10), Some(vec![(2, 3)]));
        assert_eq!(byte_ranges(Some("bytes= 2- 3"), 10), Some(vec![(2, 3)]));
        assert_eq!(byte_ranges(Some("bytes=x-3"), 10), Some(vec![(0, 3)]));
        assert_eq!(byte_ranges(Some("bytes=2-x"), 10), None);
    }

    #[test]
    fn splits_like_ruby() {
        // Whitespace before the first range isn't a separator, so this isn't a suffix range.
        assert_eq!(byte_ranges(Some("bytes= -5"), 10), Some(vec![(0, 5)]));
        assert_eq!(byte_ranges(Some("bytes=\t-5"), 10), Some(vec![(0, 5)]));
        assert_eq!(byte_ranges(Some("bytes=0-1, -2"), 10), Some(vec![(0, 1), (8, 9)]));
        assert_eq!(byte_ranges(Some("bytes=0-1,\t -2"), 10), Some(vec![(0, 1), (8, 9)]));
        assert_eq!(byte_ranges(Some("bytes=0-1, ,"), 10), Some(vec![(0, 1)]));
        assert_eq!(byte_ranges(Some("bytes=0-1 ,"), 10), Some(vec![(0, 1)]));
        assert_eq!(byte_ranges(Some("bytes=,"), 10), Some(vec![]));
        assert_eq!(byte_ranges(Some("bytes=0-1,,2-3"), 10), None);
        assert_eq!(byte_ranges(Some("bytes=;bytes=0-1"), 10), Some(vec![(0, 1)]));
        assert_eq!(byte_ranges(Some("bytes=0-1;foo"), 10), Some(vec![(0, 1)]));
        assert_eq!(byte_ranges(Some("bytes=5--6"), 10), None);
        assert_eq!(byte_ranges(Some("bytes=--5"), 10), Some(vec![]));
        assert_eq!(byte_ranges(Some("bytes=---"), 10), None);
        assert_eq!(byte_ranges(Some("bytes=5-6-7"), 10), Some(vec![(5, 6)]));
    }

    #[test]
    fn at_most_a_hundred_ranges() {
        let ranges = |n: usize| format!("bytes={}", vec!["0-0"; n].join(","));
        assert_eq!(byte_ranges(Some(&ranges(100)), 1000), Some(vec![(0, 0); 100]));
        assert_eq!(byte_ranges(Some(&ranges(101)), 1000), None);
    }

    #[test]
    fn multipart_body() {
        let multipart = Multipart {
            boundary: MULTIPART_BOUNDARY,
            spelling: Spelling::Rack,
            content_type: "text/plain",
            size: 10,
        };
        assert_eq!(multipart.content_type_header(), "multipart/byteranges; boundary=AaB03x");
        let heading = |range: &str| format!("\r\n--AaB03x\r\ncontent-type: text/plain\r\ncontent-range: bytes {range}/10\r\n\r\n");
        let closing = "\r\n--AaB03x--\r\n";
        let parts = multipart.parts(&[(0, 1), (5, 9)]);
        assert_eq!(
            parts,
            [
                Part::Text(heading("0-1")),
                Part::Range(0, 1),
                Part::Text(heading("5-9")),
                Part::Range(5, 9),
                Part::Text(closing.into()),
            ]
        );
        let length = heading("0-1").len() + 2 + heading("5-9").len() + 5 + closing.len();
        assert_eq!(content_length(&parts), length as u64);

        let active_storage = Multipart {
            spelling: Spelling::ActiveStorage,
            ..multipart
        };
        assert!(matches!(
            &active_storage.parts(&[(0, 1)])[0],
            Part::Text(heading) if heading.contains("\r\nContent-Type: text/plain\r\nContent-Range: bytes 0-1/10\r\n")
        ));
    }
}
