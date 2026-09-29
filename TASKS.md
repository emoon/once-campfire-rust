# Tasks

Checklist for `plans/cleanup.md`. Read that first: it has the rules, the definition of done, and
the detail behind each ID.

This file lives once, in `/home/emoon/once-campfire-rust` on `refactor/cleanup`. Edit it there,
never in a worktree copy, and never commit it on a WP branch; only the coordinator commits it, when
it merges. Re-read it before each edit.

Marks:
- `[ ]` open.
- `[~]` in progress: add the branch.
- `[r]` ready for review: add the branch, the line delta and the allocations/request delta.
- `[x]` merged into `refactor/cleanup`.
- `[-]` dropped: say why, and where the numbers are.
- `[!]` blocked: say on what.

Add anything new under "Found while working"; the coordinator triages it.

## Decisions

The human's decisions for this run. Nothing else waits on the human; agents follow "When you'd
otherwise stop" in `plans/cleanup.md`.

- S-1: Rust 1.98.1 installed by the human (2026-09-28); `cargo` and `mise exec rust@1.98.1` both work.
- S-4: one-time `cargo fmt` commit at `max_width = 140` approved (2026-09-28).
- X-6: typed ids approved "where it makes sense" (2026-09-28); criteria in `plans/cleanup.md` X-6.
- A coordinator agent reviews (with a fresh reviewer agent) and merges WPs into `refactor/cleanup`
  (2026-09-28).
- Perf tolerance: CPU/request and throughput may be worse than base by at most T = max(3%, the
  S-2 run-to-run spread) per target; perf WPs must gain more than T or are dropped. Allocations
  per request must not go up (2026-09-28).
- Approved outside the repo (2026-09-28): worktrees in `../campfire-wt/`, removed after merge;
  Docker images (the base images, plus `campfire-rust:<id>` and `campfire-candidate-<id>` per WP,
  each pair removed after its WP merges; no other image is touched).
- Not approved: `cargo install inferno` (skip flamegraph SVGs), and freeing disk any other way
  (below 60 GB free, the coordinator stops and asks).
- The run ends after P-5 with everything committed on `refactor/cleanup` and the PR description in
  `plans/cleanup-pr.md`. Nothing is pushed (2026-09-28).

## Phase 0: safety net (serial; blocks everything)

- [x] S-1 Toolchain on Rust 1.98.1; clippy clean on it (refactor/cleanup). cargo and mise both
      report 1.98.1; default clippy is clean with no change (the richtext `nonminimal_bool`
      warning seen on 1.93 no longer fires). Tests with the seed: 652 passed, 0 failed, 7 ignored
      (need a live reference app or a Ruby-made DB); no seed skips left, only the storage vectors'
      libvips/ffmpeg version-dependent byte comparisons (host 8.18.6 / n9.0.1).
- [x] S-2 (refactor/cleanup) Seed, reference and candidate images; integration tests actually
      run; baselines (parity lean gate, `bench/profile` alloc and cpu, `bench/run --apps rust`)
      in `bench/results/cleanup-baseline-20260928/`. Seed-backed tests: 53 skipped before the seed,
      0 after (652 pass, 7 ignored). Parity 873/874 (manifest allowed). T = 3% (room_show,
      messages_page, sidebar), 3.2% (post_message); allocs noise 0.1/req, post_message bimodal.
      `bench/profile cpu` now writes `cpu.json`. Gate dry run (worktree `dryrun`, removed with
      its branch and images) fixed the plan: copy the seed instead of symlinking it (the parity
      container can't follow the link, so every cell that reads the seed's labels errored), `git worktree remove --force`, copy the
      base binary, post_message twice, explicit cpu and `bench/run` recipes, `LOAD_WAIT_SECS=60`,
      cargo pinned off the benchmark CCD (`taskset -c 0-7,16-23`), DB-5's differential runner.
- [x] S-3 (refactor/cleanup) `bench/loc` script, with the baseline recorded (in
      `bench/results/cleanup-baseline-20260928/README.md`: 33,807 production lines incl. 1,771 of
      templates, 4,557 comment lines, 15,449 test lines before S-4's `cargo fmt`; 36,407 /
      4,557 / 17,537 after it).
      `bench/loc --against refactor/cleanup` prints the per-crate delta for commit messages.
- [x] S-4 (refactor/cleanup) (approved) `rustfmt.toml` + one `cargo fmt` commit + `.git-blame-ignore-revs`.
      fmt commit 63b72b1 (233 files, 1,635 hunks; production lines 34,234 → 36,776). The vendored
      html5ever is a workspace member, so `crates/richtext/vendor/rustfmt.toml` disables formatting
      there; `cargo fmt` is safe to run anywhere now. Tests (seed built) and clippy clean after.
- [-] S-6 Parity compare on its own lock, pinned off the benchmark cores (prove perf spread stays within T first) (refactor/cleanup-s-6, 7b1ab33)
      Dropped: outside T. The pinned compare passed (874 cells: 873 pass, 1 allowed, 2,061 s with
      PARITY_WORKERS=8), but `bench/profile cpu` beside it on one binary was room_show +4.1%
      CPU/req and -4.4% req/s, messages_page +3.6/-3.6%, sidebar +4.5/-4.8%, post_message
      +6.5/-17.0% (clean runs, before the screensaver). The benchmark cores' clock drops 2.3%
      (shared boost budget), and disk I/O kernel workers can't be pinned. Allocations unchanged
      (C/SQLite only; see S-7).
      Branch keeps only `bench/results/s-6-20260929/` and a plan note; the pinning diff
      (`PARITY_CPUSET`) is saved there, not applied. Gate recipes unchanged. Images removed.
- [x] S-5 (refactor/cleanup) Workspace `[lints.clippy]` floor (warn), existing hits allowed.
      The 8 lints are `warn` in `[workspace.lints.clippy]`; every crate but html5ever has
      `[lints] workspace = true`. The 99 existing hits carry `#[expect(clippy::…, reason = "existing
      hit under the S-5 lint floor")]` on their 81 enclosing fns (48 format_push_string, 25
      needless_pass_by_value, 11 significant_drop_tightening, 5 redundant_clone, 5 needless_collect,
      5 trivially_copy_pass_by_ref; 0 large_enum_variant, 0 inefficient_to_string). `expect`
      rather than `allow`, so fixing a hit makes clippy flag the stale attribute; find a lane's
      with `grep -rn "S-5 lint floor" crates/<lane>`.

## Phase 1: shared foundations

- [x] F-1 `rails_compat::erb` escaper; delete 5 copies (refactor/cleanup-f-1, rebased on a0df4b6; re-running parity with the cache fix).
      Production lines -22 (assets -13, cable -14, rails_compat +47, richtext -12, views -30);
      allocs/req unchanged (18.0/19.0/53.1, post_message 78.6 → 78.8 high mode, noise); CPU and
      req/s within T; parity 873/874 (1 allowed), as baseline; tests 654 pass, 7 ignored (seed
      built), clippy clean. Results: `bench/results/f-1-20260929/`. Plan corrections: 4 copies
      replaced, not 5. kit `front/tls.rs` is Go's `net/http` htmlEscape (`"` → `&#34;`, and hyper
      accepts `"` in a path), so it stays, with a comment. assets `build.rs` has a 6th copy, kept:
      a build script would compile rails_compat a second time. rails_compat's crypto is now behind
      a `crypto` feature: with crypto always on, a clean build of views/richtext/assets used 50-200%
      more CPU (numbers in the commit message).
      kit, cable and campfire enable the feature. Note for F-2: private `json` is inside
      `crypto` too, and `serde_json` is optional; un-gate both when json gains public API (lib.rs
      and Cargo.toml will conflict; keep both sides).
- [~] F-2 `rails_compat::json` Float formatting + `EncodedJson`; delete 3 JSON string encoders (refactor/cleanup-f-2)
- [~] F-3 `rails_compat::rack::byte_ranges` + multipart; fix the assets overflow range (refactor/cleanup-f-3)
- [x] F-4 `rails_compat::ruby::{to_i, cast_integer}`; delete 3 copies (refactor/cleanup-f-4, 4704c8f
      on a0df4b6, review nits folded in; parity re-run after 2895add: 873 pass + 1 allowed,
      compare-2026-09-29T05-04-43-722Z). Production lines +4 (campfire −41, rails_compat +45; 11 are new imports),
      tests +37. Allocs/req unchanged (room_show 18.0/18.1, messages_page 19.0/19.0, sidebar
      53.1/53.1, post_message higher run 79.0/78.6); CPU/req and req/s within ±0.8%
      (`bench/results/f-4-20260929/`). Tests 654 pass / 0 fail / 7 ignored with the seed built;
      clippy clean; parity 873 pass + 1 allowed (baseline). **Plan correction:** Rails'
      id cast doesn't "differ on underscores" from `to_i`. `ActiveModel::Type::Integer#serialize`
      (activemodel `type/integer.rb`) calls `String#to_i`, gated only by `non_numeric_string?`
      (`NUMERIC_REGEX = /\A\s*[+-]?\d/`, `type/helpers/numeric.rb`), and is out of range past i64.
      The three copies disagreed with each other, so `cast_integer` now matches Rails. I checked
      that in the reference image (Ruby 3.4.10) with `Room.find`, `find_by(id:)` and
      `Type::Integer#serialize` on crafted strings. **Behavior changes, all toward Rails, crafted
      ids only:** controller id params and the `last_room` cookie: "1_0" → 10 (was 1), "0d5" → 5
      (was 0), a leading NBSP or U+2003 ("\u{a0}5") → no match (was 5); cable `room_id`: "0d5" → 5
      (was 0), a leading "\v" is whitespace (was no match). `to_i` of a negative past i64 gives
      i64::MIN (was −i64::MAX; the page clamp and `since` saturation hide it). Cross-lane: one
      import path in `campfire/src/active_storage.rs` (STORE). The images `campfire-rust:f-4` and
      `campfire-candidate-f-4` are still there.
- [~] F-5 `Patch<T>` + `Assignments`; `User`/`Account`/`Room::update`; 7 callers (refactor/cleanup-f-5)
      Review round done (reviewer approves 789ffe8, one squashed commit); waiting on the re-run
      parity and alloc gates. Production lines +43 (db +41, campfire +2), tests +28. Per-call
      allocations base → now: `User::update` 20 → 9, `Account::update` 18 → 7, `Room::update`
      rename 6 → 4, type change 6 → 6. First-round gates (before the review fix): allocations/request
      unchanged, CPU and req/s within T/2, parity 873/874. Tests 655 pass / 7 ignored with the seed;
      clippy clean. Plan notes: `Assignments` is crate-private in `sql.rs` and gained `patch`;
      `write` takes `(tx, table: &'static str, id, &mut updated_at)` so "touch updated_at only when
      something changed" lives in one place. `Room::update` keeps its literal UPDATE, via
      `Patch::apply`. `Role`/`Status` gained `From<_> for rusqlite::types::Value` (DB-3's
      `sql_enum!` should generate it).

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
- [x] DB-10 `differential.sh`: ignore the app-added `index_messages_on_room_id_and_created_at` (before DB-5)
      (refactor/cleanup-db-10). +0 production lines (script only), allocs n/a (no production
      code; parity and perf gates don't apply). `differential.sh` runs all four steps, ending in
      "rollback ok" (exit 0). Tests with the seed: 654 passed, 0 failed, 7 ignored; clippy clean.

### KIT (`crates/kit`, `crates/routes`)
- [~] KIT-1 perf: params merged once (touches `campfire/src/controllers.rs:351`) (refactor/cleanup-kit-1)
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
- [ ] WEB-10 Redirect helper, bool params, precomputed version headers; `update_message` matches on `Assignment` (F-5)
- [ ] WEB-11 De-flake `presenters::accounts::tests::manages_bots` (302 under load; F-2)

### LIVE (`crates/cable`, campfire channels, integrations, jobs)
- [ ] LIVE-1 perf: hub lock not held while sending
- [ ] LIVE-2 perf: encode per-member broadcasts once
- [ ] LIVE-3 `Broadcasting` newtype; `Typing` enum (with VIEW-5)
- [ ] LIVE-4 Split `connection::run`; `Command` enum
- [ ] LIVE-5 Web push: no boxed handler; `permitted_endpoint_host`; tokio worker (optional)
- [ ] LIVE-6 opengraph `Target` enum; `Endpoint::from_uri`; `transport` bool
- [ ] LIVE-8 De-flake `channels_test::room_channel_streams_for_member_rooms_only` (under load; F-5)
- [ ] LIVE-7 `Attr` enum, one `BoxFuture`, `FrameHead`, `Writer::send` allocs, dead turbo fns, `unix_now`, `ChannelError`

### VIEW (`crates/views`, `crates/richtext`)
- [~] VIEW-1 perf: `raw` without copy; fragment cache returns `Arc`; borrowed sidebar partial (refactor/cleanup-view-1)
- [ ] VIEW-2 perf: `Attrs` with `Cow` keys and `write!`; the `push_str(&format!)` sites
- [ ] VIEW-3 perf: `SafeList` statics, no double sanitize, `qualified_name` without allocation, `NodeId`
- [ ] VIEW-4 `Cached<V>` + `CachedPartial` for messages/users/boosts
- [ ] VIEW-5 One `DomId` and one `param_key` (with LIVE-3)
- [ ] VIEW-6 Borrowed `ViewContext`; static asset paths; `Platform` enums
- [ ] VIEW-7 `render_mention` agrees with `_mention.html` (test or render)
- [ ] VIEW-8 perf (last): `ParsedBody`, parse each message body once
- [ ] VIEW-9 views `rails_json_escape`/`to_rails_json` and richtext `to_json_string` on `rails_compat::json` (after F-1, F-2)

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

## For the human

Items the coordinator moved here from "Found while working": behavior changes or calls it
won't make. They don't block the run and go into the final report.

- (S-2) `post_message` allocations are bimodal: the same binary measures 78.7 or 77.2 per request
  (5 of 14 runs landed low). The gate works around it (run twice, compare the higher run); the cause
  (probably a timing-dependent job or broadcast path) is unexplained.
- Verify: `Layout::render` reads (and so sweeps) the flash even for `layout false` renders; Rails
  wouldn't. Possible parity edge (see WEB-5).
- (F-4) Direct upload `byte_size` (`campfire/src/active_storage.rs`, `cast_integer`): Rails
  casts it as an attribute (`Type::Integer#cast`, `"abc".to_i` → 0, then validations), not as a
  condition, so `"abc"` would create a 0-byte blob where the port answers 422. This predates F-4;
  fixing it would change behavior (STORE, or for the human).

## Found while working

- (F-3) kit `response.rs` has a fourth byte-range parser, `parse_range` (single range, strict
  `u64` parse, not Rack's rules), behind `SendOptions::ranges`. No production caller sets `ranges`
  (only `kit/tests/http.rs`; Active Storage's proxy serves ranges through `rails_compat::rack`), so
  the flag, `parse_range`, `RangeResult` and their test look dead: delete them, or switch them to
  `rails_compat::rack::byte_ranges` if something should keep them (KIT lane).
