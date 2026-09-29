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
- Parity is batched (2026-09-29): per-WP only for WPs that change rendered output (views,
  richtext, templates, asset overrides); otherwise one compare on the tip after every three merges.
- The human disables the desktop screensaver and the spinning `gh auth token` themselves
  (2026-09-29); agents still check `pgrep -f omarchy-screensaver` around CPU runs.
- Performance means time (2026-09-29): the gate is CPU per request and req/s at c=16; allocations
  are information only and never block a merge. Perf work is ordered by S-8's measured costs.
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
- [x] S-8 Profile with `perf` (call graphs) on the targets; rank hot spots; map to WPs, new WPs, perf order (refactor/cleanup-s-8, b9214c3 on 2c2fd4a)
      Results: `bench/results/s-8-20260929/README.md`. Biggest cost: the `spawn_blocking` hop per DB
      read; reads inline cut CPU/req −12…−23% on all four targets (experiment). Then the deflater
      (sidebar ~40% deflating a repeating body; the splice ~19% of room_show), per-message cache
      keys and by-name row decoding, and SQLite `mmap` remapping on post_message (−14% with
      `mmap_size = 0`). The "9% crypto" is mostly the splice's SHA-256; signing is ~2.5%. New WPs
      DB-11…14, KIT-9/10, WEB-12, LIVE-9, VIEW-10/11 (lines added below) and "Perf order (S-8)" in
      the plan (on the branch). Gate recommendation: CPU/req and req/s from an unprofiled
      `bench/profile perf --freq 0` run (gperftools adds 1-4%, loses ~40% of samples, has crashed;
      perf adds up to 19%); not applied to the recipe. Tooling: `bench/profile perf`,
      `bench/lib/perfprof.py`, `bench/lib/stacks.py`; `cpuprof.py` split to share them. Production
      lines +0 (no Rust changes), allocations n/a. Tests and clippy not re-run (no Rust changed);
      parity n/a. Clean session under the lock, screensaver off before and after every step.
- [x] S-9 The write path and cable slower than published v0.1.1: answered, no regression (the published table is from another CPU; 080f903 ≈ 9e2a110 here). Results only: `bench/results/s-9-20260929/`
- [~] S-7 The alloc gate counts Rust allocations; re-baseline at 3c7a173, check F-1/F-4/DB-10 (refactor/cleanup-s-7)
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
- [x] F-2 `rails_compat::json` Float formatting + `EncodedJson`; delete 3 JSON string encoders (refactor/cleanup-f-2, Merged in batch 1 (21985b1; no regression, ABBA within ±1.3%).
      8b5bf92 on ca8ac66). Parity 873/874 (1 allowed, baseline) with the cache-fixed build
      (compare-2026-09-29T08-11-48-489Z). Pending: guarded ABBA CPU (screensaver), alloc with S-7's tool.
- [x] F-3 `rails_compat::rack::byte_ranges` + multipart; fix the assets overflow range (refactor/cleanup-f-3) Merged in batch 1 (21985b1; no regression, ABBA within ±1.3%).
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
- [x] F-5 `Patch<T>` + `Assignments`; `User`/`Account`/`Room::update`; 7 callers (refactor/cleanup-f-5)
      One commit (4ccf1ee on 6c76d78); reviewer approved the code. Production lines +43 (db +41,
      campfire +2), tests +28. Allocations/request unchanged on the four targets (room_show 18.1,
      messages_page 19.0, sidebar 53.1, post_message higher run 78.2 on both sides; C/SQLite only
      until S-7), CPU and req/s within T/2 (first round; no target runs a model update).
      Per-call allocations counted with a counting allocator (Rust included), base → now:
      `User::update` 20 → 9, `Account::update` 18 → 7, `Room::update` rename 6 → 4, type change
      6 → 6. Parity 873/874 (manifest allowed) on the final code; db differential passes end to
      end; tests 658 pass / 7 ignored with the seed; clippy clean. Numbers in
      `bench/results/f-5-20260929/`. Plan notes: `Assignments` is crate-private in `sql.rs` and
      gained `patch`; `write` takes `(tx, table: &'static str, id, &mut updated_at)` so "touch
      updated_at only when something changed" lives in one place. `Room::update` keeps its literal
      UPDATE, via `Patch::apply`. `Role`/`Status` gained `From<_> for rusqlite::types::Value`
      (DB-3's `sql_enum!` should generate it).

## Phase 2: lanes

Batch parity on 857121c (after DB-11, DB-12, KIT-10): 874 cells, 873 pass, 0 fail, 1 allowed, as the Phase 0 baseline (2026-09-29).
Batch parity on 6b98426 (after batch 1, VIEW-3, KIT-9): 873 pass, 0 fail, 1 allowed (2026-09-29).

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
- [x] DB-11 perf: reads without the `spawn_blocking` hop; `read_offloaded` for long reads (refactor/cleanup-perf-1, merged a493fe6). c=16 ABBA: CPU/req −8.7…−12.3%, req/s +7.7…+20.2% on all four targets; no cap on inline reads (see "For the human"). db +53, campfire +3 lines
- [x] DB-12 perf: no `mmap_size` (refactor/cleanup-db-12, merged). c=16 ABBA on DB-11: post_message CPU/req −12.8%, req/s +8.9%; reads ±1.3%. db −1 line
- [-] DB-13 perf: column indices once per query (refactor/cleanup-db-13). Dropped: no target gains more than T (8 ABBA runs on fc1aef6: messages_page CPU/req −2.9%, room_show −1.4%, sidebar −2.0%, post_message +2.2%). Results kept (`bench/results/db-13-20260929/`); code not merged.
- [ ] DB-14 perf: `Room::original` without a scan (index or id only)

### KIT (`crates/kit`, `crates/routes`)
- [x] KIT-1 perf: params merged once (touches `campfire/src/controllers.rs:351`) (refactor/cleanup-kit-1) Merged in batch 1 (21985b1; no regression, ABBA within ±1.3%).
- [ ] KIT-2 perf: per-request allocations (deflater, cache variant, compression, host, remote_ip, formats, log, timeouts)
- [x] KIT-3 Session dead state (refactor/cleanup-kit-3)
- [ ] KIT-4 `dispatch` error flow; delete `clone_error`
- [~] KIT-5 `PendingEntry` instead of boxed closure; `anyhow` in acme (refactor/cleanup-kit-5)
- [ ] KIT-6 Typed `FrontConfig`, `Disposition`, `Redirect.status`, bool structs
- [ ] KIT-7 Routes: `&'static str` for constant paths
- [ ] KIT-8 Split `compression::apply`; dead `Pair`; `MediaType`; consistent `is_xhr`
- [x] KIT-9 perf: splice without rehashing (BLAKE3 text identity, combined CRCs) (refactor/cleanup-kit-9, merged). 8 ABBA runs: room_show and messages_page CPU/req −3.3%. kit +60 lines
- [ ] KIT-9b perf: record fragment offsets while rendering instead of `find`/`starts_with` over the body (6.1% of room_show, 6.6% of messages_page in S-8). Split from KIT-9: needs a recording writer through askama's `render()`, the page helpers and `Response`, composed across the frame layout's copy of the content
- [x] KIT-10 perf: reuse gzip output for repeated bodies (refactor/cleanup-kit-10, merged). c=16 ABBA, 8 runs: sidebar CPU/req −48.5%, req/s +91%; others ±1%. Encoder pooling left out (−0.5% of post_message, under T). kit +72 lines

### WEB (`crates/campfire` controllers, concerns, app)
- [-] WEB-1 perf: static assets via `Bytes::from_static` (refactor/cleanup-web-1). Dropped: no measurable time gain, even on the path it changes (static_css_app, the bare app: CPU/req +0.3%, req/s -0.9%, 8 ABBA runs). Only the results are kept (7d9ccbc, `bench/results/web-1-20260929/`); the code commit was not merged.
- [ ] WEB-2 perf: memoized user-agent parse; byte-offset parser
- [x] WEB-3 UA matchers → `LazyLock<Regex>`; `BrowserRule` enum (refactor/cleanup-web-3) Merged in batch 1 (21985b1; no regression, ABBA within ±1.3%).
- [ ] WEB-4 `c.read`/`c.write` helpers (keep 404 vs 500 mapping per site)
- [ ] WEB-5 perf: ~6 → ~3 DB round trips on room show; `page::bare` without layout load
- [x] WEB-6 `is_administrator` / `can_administer(creator_id)`; `forbid_unless` (refactor/cleanup-web-6, 7af9f04 on d23c0b8): production lines campfire -12, db +2 (total -10), tests +50; allocations/request n/a (no perf gate, coordinator's call). Reviewer's two requests applied: room-creation test now covers member+unrestricted 200 and administrator+restricted 200 (each verified to fail against the one-sided rewrite), `can_administer` doc cites reference/app/models/user/role.rb. Tests 659 passed, 0 failed, 7 ignored (seed built, none skipped); clippy clean. Previous commit message's delta (-14/+0/+33) was miscounted; corrected.
- [ ] WEB-7 `page::render_partial` for the 12 detached renders
- [ ] WEB-8 Presenters: SQL to db (after DB-2), dedupe, per-message perf fixes
- [ ] WEB-9 perf (measure): `RegexSet` router (after KIT-1)
- [ ] WEB-10 Redirect helper, bool params, precomputed version headers; `update_message` matches on `Assignment` (F-5)
- [ ] WEB-11 De-flake `presenters::accounts::tests::manages_bots` and `accounts::tests::serves_the_account_logo_and_avatars` (302 under load; F-2, VIEW-3)
- [ ] WEB-12 perf: request log off the request path (buffered, non-blocking, lossless) (S-8)

### LIVE (`crates/cable`, campfire channels, integrations, jobs)
- [ ] LIVE-1 perf: hub lock not held while sending
- [ ] LIVE-2 perf: encode per-member broadcasts once
- [ ] LIVE-3 `Broadcasting` newtype; `Typing` enum (with VIEW-5)
- [ ] LIVE-4 Split `connection::run`; `Command` enum
- [ ] LIVE-5 Web push: no boxed handler; `permitted_endpoint_host`; tokio worker (optional)
- [ ] LIVE-6 opengraph `Target` enum; `Endpoint::from_uri`; `transport` bool
- [ ] LIVE-8 De-flake `channels_test::room_channel_streams_for_member_rooms_only` (under load; F-5)
- [ ] LIVE-7 `Attr` enum, one `BoxFuture`, `FrameHead`, `Writer::send` allocs, dead turbo fns, `unix_now`, `ChannelError`
- [ ] LIVE-9 perf (measure first; after LIVE-1, LIVE-2): fan-out writes per delivery; per-connection queue instead of `SelectAll` (S-8)

### VIEW (`crates/views`, `crates/richtext`)
- [x] VIEW-1 perf: `raw` without copy; fragment cache returns `Arc`; borrowed sidebar partial (refactor/cleanup-view-1) Merged in batch 1 (21985b1; no regression, ABBA within ±1.3%).
- [ ] VIEW-2 perf: `Attrs` with `Cow` keys and `write!`; the `push_str(&format!)` sites
- [x] VIEW-3 perf: richtext statics, fewer sanitize passes, allocation-free names (refactor/cleanup-view-3, merged b603f22). Cold fragment cache, 8 ABBA runs: room_show CPU/req −4.6%, req/s +10.1%; messages_page −4.3%, +7.6%; warm targets within T. Parity 873/874. richtext +50 lines
- [ ] VIEW-4 `Cached<V>` + `CachedPartial` for messages/users/boosts
- [ ] VIEW-5 One `DomId` and one `param_key` (with LIVE-3)
- [ ] VIEW-6 Borrowed `ViewContext`; static asset paths; `Platform` enums
- [ ] VIEW-7 `render_mention` agrees with `_mention.html` (test or render)
- [ ] VIEW-8 perf (last): `ParsedBody`, parse each message body once
- [ ] VIEW-9 views `rails_json_escape`/`to_rails_json` and richtext `to_json_string` on `rails_compat::json` (after F-1, F-2)
- [ ] VIEW-10 perf: page buffers sized up front (touches the page renders in campfire) (S-8)
- [ ] VIEW-11 perf (after VIEW-4): fragment cache keys without `format!`/`strftime` (S-8)

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

- ~~(S-2) `post_message` allocations are bimodal~~ Explained by S-7: one-time work (likely each
  reader connection preparing its statements) landed in the measured run; with S-7's 500-request
  warm-up the counts are exact (no longer an item for the human).
- Verify: `Layout::render` reads (and so sweeps) the flash even for `layout false` renders; Rails
  wouldn't. Possible parity edge (see WEB-5).
- (F-4) Direct upload `byte_size` (`campfire/src/active_storage.rs`, `cast_integer`): Rails
  casts it as an attribute (`Type::Integer#cast`, `"abc".to_i` → 0, then validations), not as a
  condition, so `"abc"` would create a 0-byte blob where the port answers 422. This predates F-4;
  fixing it would change behavior (STORE, or for the human).
- (VIEW-3 review) Possible DoS, predates the cleanup: richtext checks its depth and attribute limits
  only after the parse (`crates/richtext/src/dom.rs` ~:586), so html5ever can build a quadratic
  number of nodes from a hostile body before the depth error fires; Gumbo (Rails) stops during the
  parse. Fixing it means limiting inside the parse (behavior change on hostile input only).

- (DB-11, 2026-09-29) Merged **without** the reviewer's cap on inline reads. Capping them at half
  the workers cost room_show/messages_page 4-6% req/s and raised p99 20-38%; all-but-one cost
  5-12% req/s; uncapped is +10…+25% req/s with p99 down 9-42% (native tails, two runs a side,
  `bench/results/perf-1-20260929/README.md`). The risk the cap guarded: a read that stalls (e.g.
  SQLITE_BUSY for up to 5 s) holds a runtime worker; with WAL, readers don't wait on the writer, so
  this needs something unusual (WAL recovery). Say if you want the cap anyway.

## Found while working

- (F-3) kit `response.rs` has a fourth byte-range parser, `parse_range` (single range, strict
  `u64` parse, not Rack's rules), behind `SendOptions::ranges`. No production caller sets `ranges`
  (only `kit/tests/http.rs`; Active Storage's proxy serves ranges through `rails_compat::rack`), so
  the flag, `parse_range`, `RangeResult` and their test look dead: delete them, or switch them to
  `rails_compat::rack::byte_ranges` if something should keep them (KIT lane).
- (F-3, coordinator) `bench/profile cpu` crashed the app twice on 2026-09-29 (10:05 F-3, 10:08 VIEW-1):
  SIGSEGV in libgcc `_Unwind_Backtrace` from libprofiler's SIGPROF handler (a frame with return
  address 0xffffffffffffffff), not app code. Retrying works. If it recurs, try `PROFILER_UNWIND`
  (TCMALLOC_STACKTRACE_METHOD) other than libgcc, or make `cpu` retry a crashed target itself.
- (VIEW-1) `bench/profile alloc` doesn't count Rust allocations. campfire's global allocator is
  tikv-jemallocator (`campfire/src/main.rs`, symbols prefixed `_rjem_`), but `alloc` LD_PRELOADs
  `/usr/lib/libjemalloc.so` and reads that allocator's stats and heap profiles. So the gate's
  18/19/53/78 per request are C mallocs, mostly sqlite (`sqlite3MemMalloc` is about 107% of the
  jeprof objects), and the `post_message` bimodality is sqlite's. A Rust-side change can't move
  them. The embedded jemalloc is built without `stats`, so `_RJEM_MALLOC_CONF` alone prints no
  counters. VIEW-1 measured with binaries built `--features tikv-jemallocator/stats` and
  `_RJEM_MALLOC_CONF=stats_print:true,stats_print_opts:J,tcache:false` (script:
  `bench/results/view-1-20260929/rust_alloc.py`); `post_message` is about 1,150 Rust allocations
  per request. Suggest an S-WP: make `bench/profile alloc` count the Rust allocator (for example a
  `stats` build of the measured binaries) and re-baseline. Earlier WPs' "allocs unchanged" only
  covers C allocations.
- (KIT-10, for KIT-9) `splice::compress` builds a fresh `flate2::Compress` for every uncached piece;
  a new zlib-rs deflate state zeroes 256 KB (window, prev, head), and a reused one
  (`Compress::reset`) zeroes only the 128 KB head. KIT-10 measured pooling for `gzip_stream` at
  −5% of a 31 KB body's gzip (5.7 µs of 107), too small to show on a gate target, so it dropped the
  pool; it only matters for splice on cache misses.
- (DB-11 review) `broadcast_create` (`controllers/messages.rs:419`, `integrations/jobs.rs:164`)
  runs inside an inline `db.read` and sends one `unread_room` broadcast per room member
  (`channels/broadcasts.rs:114`); an open room is every user, on every post. The inline-read cap
  bounds it to half the workers. Measure moving it to `read_offloaded` (or reading the member ids
  and broadcasting after the read) at a large member count; post_message is the hot path.
  Also low: `rooms/closeds.rs:142` loops over `room.user_ids` inline; `sessions.rs:62` `no_users`
  counts every user where an `EXISTS` would do.
- (2026-09-29) `database::tests::the_wal_stays_bounded_under_sustained_writes` fails whenever
  TMPDIR is on the real disk (WAL 50-52 MB against a ~41 MB bound), on the base branch too (3/3);
  it passes on /tmp (tmpfs). The checkpointer can't keep up on a real fsync. Make the test not
  depend on disk speed, or bound what it asserts.
- (2026-09-29) /tmp is a tmpfs with a per-user quota (`usrquota`); seed tests failed with "Disk
  quota exceeded" at 17 GB used by this user across projects. Keep scratch small there.
