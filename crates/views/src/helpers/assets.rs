//! `image_tag` and asset resolution (`AssetTagHelper`, `AssetUrlHelper`).

use std::borrow::Borrow;

use super::html::Html;
use super::tag::{Attrs, legacy_tag};
use crate::ViewContext;

/// `asset_path(source)`: URLs and absolute paths pass through; logical asset paths are digested.
pub fn asset_path(ctx: &ViewContext, source: &str) -> String {
    let is_url = source.starts_with('/')
        || source.starts_with("data:")
        || source.starts_with("cid:")
        || source
            .split_once("://")
            .is_some_and(|(scheme, _)| !scheme.is_empty() && scheme.chars().all(|c| c.is_ascii_alphabetic() || c == '-'));
    if is_url { source.to_string() } else { ctx.asset(source) }
}

/// `image_tag(source, options)`: the options in order, then `src`, then `width`/`height` from
/// `size:` ("20" or "20x30").
pub fn image_tag(ctx: &ViewContext, source: impl std::fmt::Display, options: impl Borrow<Attrs>) -> Html {
    let src = asset_path(ctx, &source.to_string());
    let mut options = options.borrow().view();
    let size = options.remove("size");
    options.set("src", Some(src.as_str().into()));
    if let Some(size) = size {
        let size = size.as_str();
        let (width, height) = size.split_once('x').unwrap_or((size, size));
        options.set("width", Some(width.into()));
        options.set("height", Some(height.into()));
    }
    legacy_tag("img", &options)
}
