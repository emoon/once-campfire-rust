//! Page renders into a buffer sized up front. Askama's `render()` reserves the template's
//! `SIZE_HINT`, a guess from its literal text, and a page with ~40 cached message fragments then
//! grows its `String` by `realloc` several times while they're written in. A [`RenderSize`]
//! remembers the length of the last render at its call site and reserves that (plus some
//! headroom) for the next one. The page is [`recorded`](crate::recorded), so the cached fragments
//! it holds don't count: only its text is written into the buffer.

use std::sync::atomic::{AtomicUsize, Ordering};

use askama::Template;

use crate::recorded::{self, RecordedPage};

/// The most a render reserves up front. Only the last render's length is kept, so one huge page
/// sizes just the next render; this cap bounds even that one.
const MAX_RESERVE: usize = 1 << 20;

/// The length of the last page rendered at one call site (see [`render_sized!`]).
#[derive(Debug, Default)]
pub struct RenderSize(AtomicUsize);

impl RenderSize {
    pub const fn new() -> Self {
        Self(AtomicUsize::new(0))
    }

    /// `template.render()`, recorded into a text buffer sized from the last render here.
    pub fn render<T: Template>(&self, template: &T) -> askama::Result<RecordedPage> {
        let page = recorded::render(template, self.capacity(T::SIZE_HINT))?;
        self.0.store(page.text().len(), Ordering::Relaxed);
        Ok(page)
    }

    /// The last length plus an eighth, so a page a little longer than the last still fits.
    fn capacity(&self, size_hint: usize) -> usize {
        let last = self.0.load(Ordering::Relaxed);
        (last + last / 8).min(MAX_RESERVE).max(size_hint)
    }
}

/// `template.render()`, recorded, sized from the last render at this call site (each expansion has its own
/// [`RenderSize`]): `render_sized!(rooms::Show { ctx, show: &show })`.
#[macro_export]
macro_rules! render_sized {
    ($template:expr) => {{
        static SIZE: $crate::sized::RenderSize = $crate::sized::RenderSize::new();
        SIZE.render(&$template)
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Template)]
    #[template(source = "<p>{{ body }}</p>", ext = "html")]
    struct Page<'a> {
        body: &'a str,
    }

    #[test]
    fn renders_what_render_does() {
        let page = Page { body: "a & b" };
        assert_eq!(RenderSize::new().render(&page).unwrap().to_string(), page.render().unwrap());
    }

    #[test]
    fn reserves_the_last_length_with_headroom() {
        let size = RenderSize::new();
        let body = "x".repeat(8000);
        size.render(&Page { body: &body }).unwrap();
        assert_eq!(size.capacity(0), 8007 + 8007 / 8);
        // A shorter page sizes the next render: one long page doesn't stay reserved.
        size.render(&Page { body: &body[..7000] }).unwrap();
        assert_eq!(size.capacity(0), 7007 + 7007 / 8);
    }

    #[test]
    fn caps_the_reservation() {
        let size = RenderSize::new();
        let body = "x".repeat(2 * MAX_RESERVE);
        size.render(&Page { body: &body }).unwrap();
        assert_eq!(size.capacity(0), MAX_RESERVE);
    }

    #[test]
    fn each_call_site_has_its_own_size() {
        let short = || render_sized!(Page { body: "short" }).unwrap();
        let long = "x".repeat(4000);
        render_sized!(Page { body: &long }).unwrap();
        assert!(short().into_parts().0.capacity() < 100);
    }
}
