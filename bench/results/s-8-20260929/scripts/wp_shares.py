#!/usr/bin/env python3
"""Inclusive share of each hot spot / WP pattern per target, from folded stacks (fp)."""
import sys, re, os
sys.path.insert(0, "/home/emoon/campfire-wt/s-8/bench/lib")
import stacks as S
d = sys.argv[1]
targets = sys.argv[2].split(",")
PAT = [
 ("splice PageParts::new (all)", r"PageParts>::new|^campfire_kit::new$"),
 ("  splice text SHA-256", r"campfire_kit::text_part"),
 ("  splice locate memcmp", r"campfire_kit::locate"),
 ("splice gzip (CRC + copy)", r"PageParts>::gzip"),
 ("splice etag", r"PageParts>::etag"),
 ("full deflate (zlib-rs)", r"zlib_rs::"),
 ("views (campfire_views::, all)", r"campfire_views::"),
 ("askama templates (all)", r"askama::Template"),
 ("  realloc under render (output/key growth)", ("leaf", r"realloc|grow_amortized|finish_grow|ralloc", r"askama::Template", 3)),
 ("  memcpy under render (fragments into page)", ("leaf", r"^__memcpy", r"askama::Template", 1)),
 ("  fragment cache keys", r"message_fragment_key|cache_key_with_version"),
 ("  asset digest lookup", r"digested_path|campfire_assets::serve::file|try_asset_path"),
 ("  Attrs render", r"Attrs>::render"),
 ("sqlite + rusqlite (all)", r"^libsqlite3_sys::|^rusqlite::"),
 ("  rusqlite by-name column lookup (Row::get(&str))", ("or", r"rusqlite::column_index|rusqlite::column_name|sqlite3_column_count", ("leaf", r"^__strlen", r"Row>::get"))),
 ("  Message::last_page / page_* (query + rows)", r"Message>::(last_page|page_before|page_after|page_around|first_page)"),
 ("  Room::original", r"campfire_db::original"),
 ("  Blob::attached", r"Blob>::attached"),
 ("  mmap/munmap remap", r"unixRemapfile|unixUnfetch|unixMapfile"),
 ("  FTS insert (create_in_index)", r"create_in_index"),
 ("  RichTextRecord::find_for", r"RichTextRecord>::find_for"),
 ("DB-4 format! in paging SQL", ("leaf", r"format_inner|map_or_else", r"Message>::(last_page|page_before|page_after|page_around)", 4)),
 ("DB-8 Timestamp encode/parse", r"campfire_db::time::|Timestamp as rusqlite"),
 ("signing: cookie verify", r"verify_signed_cookie|verify_signed_value|campfire_kit::signed"),
 ("signing: signed_id / avatar", r"signed_id::generate|avatar_token"),
 ("signing: turbo stream name", r"signed_stream_name"),
 ("signing: GlobalId", r"GlobalId>::to_param|global_id::"),
 ("STORE-2 verifier construction (generate_key etc)", r"generate_key|signed_cookie_verifier|encrypted_cookie_encryptor|turbo::verifier|signed_id::verifier|global_id::verifier"),
 ("request log (tracing fmt)", r"RequestLog|tracing_subscriber|tracing_core"),
 ("WEB-9 routing (recognize, normalize_path)", r"campfire::controllers::recognize|recognize|normalize_path"),
 ("WEB-2 user agent", r"user_agent|allow_browser|ApplicationPlatform"),
 ("KIT-1 params (Ctx::new, install_path_params)", r"Ctx>::new|install_path_params|ParamMap"),
 ("hyper/axum outside the app (excl. deflater, Ctx, handler)", ("not", r"^hyper::|<hyper::", r"ActionHandler|campfire_kit::deflater|Ctx>|campfire_kit::adapter|campfire_kit::front::handler|campfire_kit::{async_fn")),
 ("spawn_blocking (spawn side, user)", r"spawn_blocking"),
 ("DB reader pool checkout", r"ReaderPool|campfire_db::database::Checkout"),
 ("malloc/free/realloc (self)", None),
 ("memcpy (self)", None),
 ("memcmp (self)", None),
 ("strlen (self)", None),
 ("cable: tokio broadcast", r"tokio::sync::broadcast|<tokio::sync::broadcast"),
 ("cable: SelectAll / FuturesUnordered", r"SelectAll|futures_unordered"),
 ("cable: hub / pubsub", r"campfire_cable::pubsub|Hub"),
 ("cable: socket writer", r"campfire_cable::socket"),
]
SELF = {"malloc/free/realloc (self)": r"^(_rjem|je_|imalloc|cache_bin|sz_|free_fastpath|do_rallocx|iralloc|arena_|tcache|isfree|ifree|ialloc)|tikv_jemallocator|__rust_(alloc|dealloc|realloc)|realloc_nonnull",
        "memcpy (self)": r"^__mem(cpy|move)", "memcmp (self)": r"^__memcmp|^__bcmp", "strlen (self)": r"^__strlen"}
def match(pat, fr):
    if isinstance(pat, str):
        return any(re.search(pat, f) for f in fr)
    kind = pat[0]
    if kind == "or":
        return any(match(p, fr) for p in pat[1:])
    if kind == "not":
        return match(pat[1], fr) and not match(pat[2], fr)
    if kind == "leaf":
        depth = pat[3] if len(pat) > 3 else 1
        return any(re.search(pat[1], f) for f in fr[-depth:]) and match(pat[2], fr)
data = {t: S.load(os.path.join(d, f"{os.environ.get('PREFIX', 'perf')}-{t}.folded{os.environ.get('SUFFIX', '')}")) for t in targets if os.path.exists(os.path.join(d, f"{os.environ.get('PREFIX', 'perf')}-{t}.folded{os.environ.get('SUFFIX', '')}"))}
print("| hot spot | " + " | ".join(data) + " |")
print("|---|" + "---|" * len(data))
for name, pat in PAT:
    row = []
    for t, st in data.items():
        total = sum(n for _, n in st)
        if pat is None:
            rx = re.compile(SELF[name]); n = sum(c for fr, c in st if rx.search(fr[-1]))
        else:
            n = sum(c for fr, c in st if match(pat, fr))
        row.append(f"{100*n/max(total,1):.1f}%")
    print(f"| {name} | " + " | ".join(row) + " |")
