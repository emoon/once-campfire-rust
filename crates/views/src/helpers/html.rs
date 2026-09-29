//! `ERB::Util.html_escape` and `ActiveSupport::SafeBuffer` semantics.
//!
//! Templates escape with [`ErbEscaper`] (configured in `askama.toml` for html, svg and json), so
//! output bytes match ERB's `&amp; &lt; &gt; &quot; &#39;`. Helpers return [`Html`], askama's
//! `Safe<String>`, which templates print without escaping — the equivalent of an html_safe
//! SafeBuffer. Build helper output with [`escape`] for untrusted text and `.0` for safe parts.

use std::fmt::{self, Write};

pub use askama::filters::Safe;
pub use rails_compat::erb::{escape, escape_into};

/// An html_safe string.
pub type Html = Safe<String>;

/// `raw` / `String#html_safe`: `html` as it is, not copied (a rendered page, a `&'static` tag
/// string, a cached fragment).
pub fn raw<T: fmt::Display>(html: T) -> Safe<T> {
    Safe(html)
}

/// `h(text)` as an html_safe value.
pub fn text(text: &str) -> Html {
    Safe(escape(text))
}

pub fn empty() -> Html {
    Safe(String::new())
}

/// `ActiveSupport::JSON.encode` output escaping (`escape_html_entities_in_json`): `<`, `>` and
/// `&` become `\uXXXX`. The JS line separators stay raw (`load_defaults 8.2` turns
/// `escape_js_separators_in_json` off). Apply to serde_json output for Jbuilder views.
pub fn rails_json_escape(json: &str) -> String {
    json.replace('<', "\\u003c").replace('>', "\\u003e").replace('&', "\\u0026")
}

/// `render json:` / Jbuilder: serde_json with Rails' HTML-entity escaping.
pub fn to_rails_json<T: serde::Serialize>(value: &T) -> String {
    rails_json_escape(&serde_json::to_string(value).expect("serializable"))
}

/// The askama escaper for ERB-compatible output.
#[derive(Clone, Copy, Debug, Default)]
pub struct ErbEscaper;

impl askama::filters::Escaper for ErbEscaper {
    fn write_escaped_str<W: Write>(&self, dest: W, string: &str) -> fmt::Result {
        rails_compat::erb::write_escaped(dest, string)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn escapes_json_like_rails() {
        assert_eq!(to_rails_json(&"<b>&</b>"), r#""\u003cb\u003e\u0026\u003c/b\u003e""#);
    }
}
