# Tasks

Checklist for `plans/cleanup.md`. Read that first: it has the rules, the definition of done, and
the detail behind each ID.

This file lives once, in `/home/emoon/once-campfire-rust` on `refactor/cleanup`. Edit it there,
never in a worktree copy, and never commit it on a WP branch. Re-read it before each edit.

Marks:
- `[ ]` open.
- `[~]` in progress: add the branch.
- `[r]` ready for review: add the branch, the line delta and the allocations/request delta.
- `[x]` merged into `refactor/cleanup`.
- `[!]` blocked: say on what.

Add anything new under "Found while working".

## Waiting on the human

- [x] S-1: Rust 1.98.1 installed by the human (2026-09-28); `cargo` and `mise exec rust@1.98.1` both work.
- [x] S-4: one-time `cargo fmt` commit at `max_width = 140` approved (2026-09-28).
- [x] X-6: typed ids approved "where it makes sense" (2026-09-28); criteria in `plans/cleanup.md` X-6.

## Phase 0: safety net (serial; blocks everything)

- [ ] S-1 Toolchain on Rust 1.98.1; clippy clean on it
- [ ] S-2 Seed, reference and candidate images; integration tests actually run; baselines (parity
      lean gate, `bench/profile` alloc and cpu, `bench/run --apps rust`) in
      `bench/results/cleanup-baseline-<date>/`
- [ ] S-3 `bench/loc` script, with the baseline recorded
- [ ] S-4 (approved) `rustfmt.toml` + one `cargo fmt` commit + `.git-blame-ignore-revs`
- [ ] S-5 Workspace `[lints.clippy]` floor (warn), existing hits allowed

## Phase 1: shared foundations

- [ ] F-1 `rails_compat::erb` escaper; delete 5 copies
- [ ] F-2 `rails_compat::json` Float formatting + `EncodedJson`; delete 3 JSON string encoders
- [ ] F-3 `rails_compat::rack::byte_ranges` + multipart; fix the assets overflow range
- [ ] F-4 `rails_compat::ruby::{to_i, cast_integer}`; delete 3 copies
- [ ] F-5 `Patch<T>` + `Assignments`; `User`/`Account`/`Room::update`; 7 callers

## Phase 2: lanes

### DB (`crates/db`)
- [ ] DB-1 perf: N+1 in `Room::find_direct_for`
- [ ] DB-2 Existing enums instead of string/integer literals
- [ ] DB-3 `sql_enum!` for Role/Status/Involvement/RoomType + value-pinning tests
- [ ] DB-4 perf: static SQL for message/room paging
- [ ] DB-5 perf: one-statement bulk membership inserts
- [ ] DB-6 perf (measure first): `json_each` for IN lists; single-statement `trim_recent_searches`
- [ ] DB-7 `NewRoom`, `NewPushSubscription`, `ClientInfo`, `Pushes`; consistent id order
- [ ] DB-8 perf: allocation-free timestamp encode/parse; signed counter update
- [ ] DB-9 Clock read once, `is_*` predicates, dead code

### KIT (`crates/kit`, `crates/routes`)
- [ ] KIT-1 perf: params merged once (touches `campfire/src/controllers.rs:351`)
- [ ] KIT-2 perf: per-request allocations (deflater, cache variant, compression, host, remote_ip, formats, log, timeouts)
- [ ] KIT-3 Session dead state
- [ ] KIT-4 `dispatch` error flow; delete `clone_error`
- [ ] KIT-5 `PendingEntry` instead of boxed closure; `anyhow` in acme
- [ ] KIT-6 Typed `FrontConfig`, `Disposition`, `Redirect.status`, bool structs
- [ ] KIT-7 Routes: `&'static str` for constant paths
- [ ] KIT-8 Split `compression::apply`; dead `Pair`; `MediaType`; consistent `is_xhr`

### WEB (`crates/campfire` controllers, concerns, app)
- [ ] WEB-1 perf: static assets via `Bytes::from_static`
- [ ] WEB-2 perf: memoized user-agent parse; byte-offset parser
- [ ] WEB-3 UA matchers → `LazyLock<Regex>`; `BrowserRule` enum
- [ ] WEB-4 `c.read`/`c.write` helpers (keep 404 vs 500 mapping per site)
- [ ] WEB-5 perf: ~6 → ~3 DB round trips on room show; `page::bare` without layout load
- [ ] WEB-6 `is_administrator` / `can_administer(creator_id)`; `forbid_unless`
- [ ] WEB-7 `page::render_partial` for the 12 detached renders
- [ ] WEB-8 Presenters: SQL to db (after DB-2), dedupe, per-message perf fixes
- [ ] WEB-9 perf (measure): `RegexSet` router (after KIT-1)
- [ ] WEB-10 Redirect helper, bool params, precomputed version headers

### LIVE (`crates/cable`, campfire channels, integrations, jobs)
- [ ] LIVE-1 perf: hub lock not held while sending
- [ ] LIVE-2 perf: encode per-member broadcasts once
- [ ] LIVE-3 `Broadcasting` newtype; `Typing` enum (with VIEW-5)
- [ ] LIVE-4 Split `connection::run`; `Command` enum
- [ ] LIVE-5 Web push: no boxed handler; `permitted_endpoint_host`; tokio worker (optional)
- [ ] LIVE-6 opengraph `Target` enum; `Endpoint::from_uri`; `transport` bool
- [ ] LIVE-7 `Attr` enum, one `BoxFuture`, `FrameHead`, `Writer::send` allocs, dead turbo fns, `unix_now`, `ChannelError`

### VIEW (`crates/views`, `crates/richtext`)
- [ ] VIEW-1 perf: `raw` without copy; fragment cache returns `Arc`; borrowed sidebar partial
- [ ] VIEW-2 perf: `Attrs` with `Cow` keys and `write!`; the `push_str(&format!)` sites
- [ ] VIEW-3 perf: `SafeList` statics, no double sanitize, `qualified_name` without allocation, `NodeId`
- [ ] VIEW-4 `Cached<V>` + `CachedPartial` for messages/users/boosts
- [ ] VIEW-5 One `DomId` and one `param_key` (with LIVE-3)
- [ ] VIEW-6 Borrowed `ViewContext`; static asset paths; `Platform` enums
- [ ] VIEW-7 `render_mention` agrees with `_mention.html` (test or render)
- [ ] VIEW-8 perf (last): `ParsedBody`, parse each message body once

### STORE (`crates/storage`, `crates/assets`, rails_compat crypto, `campfire/src/active_storage.rs`)
- [ ] STORE-1 One Active Storage verifier; vectors test the production one
- [ ] STORE-2 perf: verifiers built once in `Secrets`; decrypt once
- [ ] STORE-3 perf: hash while copying uploads and opens
- [ ] STORE-4 `BlobKey`, `Checksum`, `Purpose`, `Disposition`, `MediaKind`, `ContentTypeSource`
- [ ] STORE-5 `Variation` `Operation` enum; checked casts
- [ ] STORE-6 Storage JSON on the shared encoder (vectors for floats and U+2028 first; after F-2)
- [ ] STORE-7 `find_or_process`; `purge` SQL to storage; consistent `FileNotFound`
- [ ] STORE-8 vips `SAFETY` comments; warm ffmpeg check; split long functions

## Phase 3: cross-cutting migrations (one at a time)

- [ ] X-1 `Format` enum + `Accepted`
- [ ] X-2 Static `Permit` lists; `ParamMap::id`; `Param::to_s` → `Cow`
- [ ] X-3 `Html` newtype with private field; helpers take `&Html`
- [ ] X-4 Involvement/RoomKind/Role enums in view models (after VIEW-5)
- [ ] X-5 `Ctx` split (incremental)
- [ ] X-6a Typed ids: db struct fields and finders (`UserId`, `RoomId`, `MessageId` first; after DB-7)
- [ ] X-6b Typed ids: campfire controllers, presenters, channels
- [ ] X-6c Typed ids: views and routes helpers
- [ ] X-6d Typed ids: param parsing and GlobalID/signed-id edges
- [ ] X-7 One blob model; one `Record` type

## Phase 4: size and polish

- [ ] P-1 Test-support dedupe
- [ ] P-2 Comment pass, per crate
- [ ] P-3 Lints to `deny`
- [ ] P-4 (Optional) MIME tables generated at build time
- [ ] P-5 Final report + README

## Found while working

- Verify: `Layout::render` reads (and so sweeps) the flash even for `layout false` renders; Rails
  wouldn't. Possible parity edge (see WEB-5).
