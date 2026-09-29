//! The two JSON encoders Rails uses. Both emit keys in insertion order, which relies on
//! serde_json's `preserve_order` feature (enabled in the workspace manifest).
//!
//! The JSON gem escapes the same characters serde_json does (quote, backslash and control
//! characters, lowercase hex), and neither escapes `/`. Floats are written the JSON gem's way
//! (see `float.rs`), not serde_json's.
use std::fmt;
use std::io;

use serde::Serialize;
use serde_json::Value;
use serde_json::ser::Formatter;

mod float;

/// JSON that [`encode`] produced, so it's already escaped the way `ActiveSupport::JSON.encode`
/// escapes, and can be sent as is.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EncodedJson(String);

impl EncodedJson {
    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_string(self) -> String {
        self.0
    }
}

impl fmt::Display for EncodedJson {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// `::JSON.generate` / `JSON.dump`: plain JSON, non-ASCII left as UTF-8.
pub fn generate<T: Serialize + ?Sized>(value: &T) -> String {
    write(value, RubyFormatter { escape_html: false })
}

/// `ActiveSupport::JSON.encode` with `escape_html_entities_in_json` (the default): like
/// `JSON.generate`, plus `<`, `>` and `&` escaped as `\uXXXX`. U+2028/U+2029 are *not* escaped:
/// `load_defaults` 8.1+ turns `escape_js_separators_in_json` off, so
/// `JSONGemCoderEncoder#encode` (activesupport/lib/active_support/json/encoding.rb) only gsubs
/// `HTML_ENTITIES_REGEX`.
pub fn encode<T: Serialize + ?Sized>(value: &T) -> EncodedJson {
    EncodedJson(write(value, RubyFormatter { escape_html: true }))
}

/// A float as `ActiveSupport::JSON.encode` writes it: `null` for NaN and the infinities
/// (`Float#as_json`), the JSON gem's format otherwise.
pub fn encode_float(value: f64) -> String {
    if value.is_finite() {
        float::dtoa(value, &mut [0; 32]).to_string()
    } else {
        "null".to_string()
    }
}

pub fn parse(bytes: &[u8]) -> Option<Value> {
    serde_json::from_slice(bytes).ok()
}

fn write<T: Serialize + ?Sized>(value: &T, formatter: RubyFormatter) -> String {
    let mut out = Vec::with_capacity(128);
    value
        .serialize(&mut serde_json::Serializer::with_formatter(&mut out, formatter))
        .expect("JSON serialization cannot fail");
    String::from_utf8(out).expect("serde_json writes UTF-8")
}

/// serde_json's compact output, with the JSON gem's floats and, for Active Support, `<`, `>` and
/// `&` escaped inside strings (the only place they can occur, keys included). serde_json writes
/// `null` for non-finite floats itself.
struct RubyFormatter {
    escape_html: bool,
}

impl Formatter for RubyFormatter {
    fn write_f64<W: ?Sized + io::Write>(&mut self, writer: &mut W, value: f64) -> io::Result<()> {
        writer.write_all(float::dtoa(value, &mut [0; 32]).as_bytes())
    }

    fn write_string_fragment<W: ?Sized + io::Write>(&mut self, writer: &mut W, fragment: &str) -> io::Result<()> {
        if !self.escape_html {
            return writer.write_all(fragment.as_bytes());
        }
        let bytes = fragment.as_bytes();
        let mut last = 0;
        for (index, byte) in bytes.iter().enumerate() {
            let escaped = match byte {
                b'<' => "\\u003c",
                b'>' => "\\u003e",
                b'&' => "\\u0026",
                _ => continue,
            };
            writer.write_all(&bytes[last..index])?;
            writer.write_all(escaped.as_bytes())?;
            last = index + 1;
        }
        writer.write_all(&bytes[last..])
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use serde_json::json;

    use super::*;

    /// `vectors/rails_compat_json.json`, from `reference-tools/rails_compat_json_vectors.rb`.
    fn vectors() -> Value {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../vectors/rails_compat_json.json");
        serde_json::from_str(&std::fs::read_to_string(path).expect("vectors/rails_compat_json.json")).expect("valid JSON")
    }

    fn float(bits: &Value) -> f64 {
        f64::from_bits(u64::from_str_radix(bits.as_str().unwrap(), 16).unwrap())
    }

    #[test]
    fn floats_match_the_json_gem() {
        let vectors = vectors();
        for case in vectors["floats"]
            .as_array()
            .unwrap()
            .iter()
            .chain(vectors["non_finite"].as_array().unwrap())
        {
            let (value, expected) = (float(&case[0]), case[1].as_str().unwrap());
            assert_eq!(encode_float(value), expected, "{}", case[0]);
            assert_eq!(encode(&value).as_str(), expected, "{}", case[0]);
        }
    }

    #[test]
    fn documents_match_active_support_and_the_json_gem() {
        for case in vectors()["documents"].as_array().unwrap() {
            assert_eq!(encode(&case[0]).as_str(), case[1].as_str().unwrap());
            assert_eq!(generate(&case[0]), case[2].as_str().unwrap());
        }
    }

    #[test]
    fn escapes_html_entities_but_not_separators_or_slashes() {
        assert_eq!(
            encode(&json!({ "key": "<a href=\"/x\">&</a>\u{2028}" })).as_str(),
            "{\"key\":\"\\u003ca href=\\\"/x\\\"\\u003e\\u0026\\u003c/a\\u003e\u{2028}\"}"
        );
    }

    #[test]
    fn escapes_control_characters_like_the_json_gem() {
        assert_eq!(encode("\u{1f}\n\t\u{8}\u{c}").as_str(), "\"\\u001f\\n\\t\\b\\f\"");
    }
}
