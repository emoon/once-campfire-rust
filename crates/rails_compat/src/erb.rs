//! `ERB::Util.html_escape` (`h`, `unwrapped_html_escape`): escapes `& < > " '` as
//! `&amp; &lt; &gt; &quot; &#39;` and leaves every other byte alone.

use std::fmt;

/// Escapes like `ERB::Util.html_escape`.
pub fn escape(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    escape_into(&mut out, text);
    out
}

/// Appends `text`, escaped, to `out`.
pub fn escape_into(out: &mut String, text: &str) {
    write_escaped(out, text).expect("writing to a String can't fail");
}

/// Writes `text`, escaped, to `dest`: the loop behind [`escape`], for writers other than a
/// `String` (askama's escaper).
pub fn write_escaped<W: fmt::Write>(mut dest: W, text: &str) -> fmt::Result {
    let mut last = 0;
    for (index, byte) in text.bytes().enumerate() {
        let replacement = match byte {
            b'&' => "&amp;",
            b'<' => "&lt;",
            b'>' => "&gt;",
            b'"' => "&quot;",
            b'\'' => "&#39;",
            _ => continue,
        };
        dest.write_str(&text[last..index])?;
        dest.write_str(replacement)?;
        last = index + 1;
    }
    dest.write_str(&text[last..])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn escapes_like_erb_util() {
        assert_eq!(escape(r#"<&>"'x"#), "&lt;&amp;&gt;&quot;&#39;x");
    }

    #[test]
    fn leaves_multibyte_text_alone() {
        assert_eq!(escape("café <ü> “quoted” 👋"), "café &lt;ü&gt; “quoted” 👋");
    }

    #[test]
    fn appends_to_existing_text() {
        let mut out = String::from("<b>");
        escape_into(&mut out, "a&b");
        assert_eq!(out, "<b>a&amp;b");
    }
}
