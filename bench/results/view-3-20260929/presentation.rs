//! Times `message_presentation` and `to_plain_text` per call on typical message bodies, and counts
//! their heap allocations. Copied into crates/richtext/examples/ to run:
//! `cargo run --release -p campfire_richtext --example presentation -- [iterations]`

use std::alloc::{GlobalAlloc, Layout, System};
use std::hint::black_box;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Instant;

use campfire_richtext::{AttachableResolver, GidLookup, MentionUser, RenderContext, SignedLookup, message_presentation, to_plain_text};

struct Counting;

static ALLOCATIONS: AtomicUsize = AtomicUsize::new(0);

unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        ALLOCATIONS.fetch_add(1, Ordering::Relaxed);
        unsafe { System.alloc(layout) }
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe { System.dealloc(ptr, layout) }
    }
    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        ALLOCATIONS.fetch_add(1, Ordering::Relaxed);
        unsafe { System.realloc(ptr, layout, new_size) }
    }
}

#[global_allocator]
static GLOBAL: Counting = Counting;

/// Allocations (and reallocations) made by one call of `f`.
fn allocations<T>(f: impl FnOnce() -> T) -> usize {
    let before = ALLOCATIONS.load(Ordering::Relaxed);
    black_box(f());
    ALLOCATIONS.load(Ordering::Relaxed) - before
}

struct OneUser;

impl AttachableResolver for OneUser {
    fn locate_signed(&self, _: &str) -> SignedLookup {
        SignedLookup::User(MentionUser {
            id: 1,
            name: "David".into(),
            title: "David – Founder".into(),
            attachable_sgid: "eyJfcmFpbHMiOnsiZGF0YSI6ImdpZDovL2NhbXBmaXJlL1VzZXIvMT9leHBpcmVzX2luIiwicHVyIjoiYXR0YWNoYWJsZSJ9fQ==--f7d8e8773314d3310320f3cdd08e5597bb51ca1a".into(),
            user_path: "/users/1".into(),
            avatar_path: "/users/1/avatar?v=1".into(),
        })
    }
    fn find_gid(&self, _: &str) -> GidLookup {
        GidLookup::NotFound
    }
}

const BODIES: &[(&str, &str)] = &[
    // What bench/loadgen's post_message posts
    ("plain", "bench write 1"),
    (
        "paragraphs",
        "<p>Morning all. The deploy went out at nine and the queue has drained.</p><p>Next up is the search index rebuild.</p>",
    ),
    (
        "links",
        "<p>See https://github.com/basecamp/once-campfire/pull/123 and www.example.com/docs?x=1, or mail ops@example.com.</p>",
    ),
    (
        "formatted",
        "<p><strong>Bold</strong>, <em>italic</em> and <a href=\"https://example.com/a b\" title=\"t\">a link</a>.</p>\
         <ul><li>one</li><li>two <code>x</code></li></ul><pre data-language=\"rust\">fn main() {}</pre>\
         <blockquote>quoted <mark style=\"color: var(--highlight-1);\">marked</mark></blockquote>",
    ),
    (
        "mention",
        "<p>Hey <action-text-attachment sgid=\"eyJfcmFpbHMiOnsiZGF0YSI6ImdpZDovL2NhbXBmaXJlL1VzZXIvMT9leHBpcmVzX2luIiwicHVyIjoiYXR0YWNoYWJsZSJ9fQ==--f7d8e8773314d3310320f3cdd08e5597bb51ca1a\" content-type=\"application/vnd.campfire.mention\"></action-text-attachment>, \
         can you look?</p>",
    ),
];

fn main() {
    let iterations: u32 = std::env::args().nth(1).map_or(20_000, |a| a.parse().unwrap());
    let ctx = RenderContext {
        resolver: &OneUser,
        request_host: Some("once.campfire.test".into()),
    };
    for (name, body) in BODIES {
        for _ in 0..iterations / 10 {
            black_box(message_presentation(body, &ctx).unwrap());
        }
        let started = Instant::now();
        for _ in 0..iterations {
            black_box(message_presentation(black_box(body), &ctx).unwrap());
        }
        let presentation = started.elapsed() / iterations;
        let started = Instant::now();
        for _ in 0..iterations {
            black_box(to_plain_text(black_box(body), &ctx).unwrap());
        }
        let plain = started.elapsed() / iterations;
        let presentation_allocs = allocations(|| message_presentation(body, &ctx).unwrap());
        let plain_allocs = allocations(|| to_plain_text(body, &ctx).unwrap());
        println!(
            "{name:<10} presentation {:>6} ns {presentation_allocs:>4} allocs   plain text {:>6} ns {plain_allocs:>4} allocs",
            presentation.as_nanos(),
            plain.as_nanos()
        );
    }
}
