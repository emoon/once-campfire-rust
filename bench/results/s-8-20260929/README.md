# S-8: where the time goes (perf profiles, 2026-09-29)

The question (`plans/cleanup.md`, S-8): the perf WPs were picked by reading code. Measure where CPU
time actually goes on room_show, messages_page, sidebar, post_message and the cable fan-out, rank
the hot spots, map each to a WP (or propose one), and order the perf work by measured cost. Also
compare `perf` with `bench/profile cpu` (gperftools) and say which the CPU gate should use.

## The answer in short

1. **The biggest single cost is the `spawn_blocking` hop per DB read**, not any code the plan
   targeted as perf. Every `Database::read` wakes a blocking-pool thread (futex), which runs the
   query, goes back to sleep (futex) and wakes the worker. room_show makes 6 of them, messages_page
   7, sidebar 5, post_message 10. Running the reads on the worker instead (a two-line experiment,
   [`experiment.patch`](experiment.patch)) cut CPU per request by **12-23% on all four targets**
   and raised req/s by 16-30%. Context switches per request dropped from ~10 to ~0.5. That is most
   of the "~17% `syscall`" bucket in the S-2 rollup. New WP **DB-11**.
2. **The deflater costs far more than it looks.** sidebar spends ~40% of its CPU deflating a body
   that repeats (new **KIT-10**). The splice that serves room pages from pre-compressed pieces
   still costs room_show ~19% of its CPU: SHA-256 of the page's text parts (7%), a whole-fragment
   `memcmp` to find each fragment (6%), and a CRC of the whole body (3%) (new **KIT-9**). That
   SHA-256 is most of the "9% crypto" in the S-2 rollup; signing is ~2.5%.
3. **Decoding rows and building cache keys**, per message per request, costs more than rendering:
   rusqlite's by-name `Row::get` is ~6% of the reads (new **DB-13**). Building each message's
   fragment-cache key with three `format!`s and a `strftime` is 10-12% of messages_page (new
   **VIEW-11**).
4. **post_message pays ~9% for SQLite's `mmap`**: after every commit, each reader unmaps and
   remaps the file. `mmap_size = 0` cut post_message by 14%, but may cost reads 2-3% (new
   **DB-12**, measure first).
5. **The cable fan-out is ~70% kernel**: one `writev` per client per message (0.86 per delivery),
   on loopback. User-space fixes there (LIVE-1, LIVE-2) can reach at most the other 30%.
6. Several planned perf WPs measure as noise here: DB-4 (`format!` of paging SQL: 0.0%), KIT-1
   (0.4-0.7%), WEB-2 (≤0.9%), STORE-2's verifier construction (0.2-0.3%).
7. **Gate:** take CPU/request and req/s from an unprofiled run (`bench/profile perf --freq 0`),
   not under gperftools. Profile with `perf` when a WP needs to explain a change. Details
   [below](#perf-vs-bench-profile-cpu-gperftools).

The ranked order is in [Perf order](#perf-order); the plan's "Perf order (S-8)" section repeats it
for scheduling.

## What was run

Code: `refactor/cleanup` at `2855a0c` (Phase 1 partly merged). Host: Ryzen 9 9950X3D; app pinned
to CPUs 8-11, load generator to 12-15, as in `bench/profile`. Everything below ran in one hold of
`/tmp/campfire-bench.lock`, 12:34-12:47, with the screensaver off before and after every step
([`scripts/session.sh`](scripts/session.sh); each step counts only if `pgrep -f omarchy-screensaver`
was empty right before and right after it). That check can match other agents' commands that
mention the screensaver, so its false positives delayed the session, but a false negative isn't
possible. The human confirmed the screensaver wasn't running. The plan's check is now
`pgrep -a -x foot | grep -q org.omarchy.screensaver`.

**Binaries** (all release, `CARGO_PROFILE_RELEASE_DEBUG=line-tables-only`, built with
`taskset -c 0-7,16-23`):

| label | flags | used for |
|---|---|---|
| `base` | the default (rust-lld) | timing |
| `fp` | `RUSTFLAGS=-Cforce-frame-pointers=yes`, `CFLAGS=-fno-omit-frame-pointer -mno-omit-leaf-frame-pointer` | timing, `perf --call-graph fp` |
| `fpbfd` | as `fp`, plus `-Clinker-features=-lld -Clink-self-contained=-linker` (GNU ld) | `perf --call-graph dwarf` |
| `exp` | `base` plus [`experiment.patch`](experiment.patch) (env toggles `S8_INLINE_READS`, `S8_MMAP_SIZE`; not committed) | the two experiments |

**Tooling added** (committed on this branch):

- `bench/profile perf`: the same load as `bench/profile cpu`, with no preload. It attaches
  `perf record -e cpu-clock:u -F 1000 --call-graph fp|dwarf -p <app>` (started disabled, and
  enabled through perf's `--control` FIFO around each target's load window, like `cpu`'s SIGUSR2).
  `perf.json` has, per target, CPU ms/req and req/s, plus per request: user and system CPU,
  voluntary and involuntary context switches per thread kind (/proc), and read/write syscalls.
  `--freq 0` measures without recording; `--app-env K=V` passes environment to the app.
- `bench/lib/perfprof.py`: `perf script` → the same folded stacks and `.top.md` rollup as
  `cpuprof.py` (which it reuses: same llvm-symbolizer, same inlined-frame expansion, same
  categories), so the two profilers' reports compare line for line.
- `bench/lib/stacks.py`: queries on folded stacks (`incl`, `callers`, `callees`, `self`).
  [`scripts/wp_shares.py`](scripts/wp_shares.py) uses it to compute every hot-spot share below.

**perf on this machine needed two workarounds**, both now handled in the tools:

- **DWARF unwinding fails on rust-lld binaries**: 77% of samples got "address range overlaps an
  existing module" from perf's libdw unwinder. lld lays out the PIE with segment vaddr = file
  offset + 0x1000, and GNU ld doesn't. Linking with GNU ld (`fpbfd`) fixes it.
- **perf 7.2's libdw unwinder only unwinds one thread per process.** It keeps one Dwfl per
  process, attached to the first thread it unwinds, and fails every other thread's samples with
  "No such process". `perfprof.py --per-thread` runs `perf script --tid` once per thread.

Frame-pointer unwinding (`fp`) needs neither: 100% of stacks reach `clone3`, through std, glibc,
jemalloc and SQLite. It has one blind spot: a frameless leaf (glibc's `memcpy`/`memcmp`/`strlen`)
loses its immediate caller. So the ranked tables below use the DWARF profiles for the four HTTP
targets and the FP profile for cable (DWARF wasn't recorded for cable). The FP share table is kept
as a cross-check.

**Per-request call counts** come from gdb ([`scripts/count.py`](scripts/count.py), `counts/`). The
app runs under gdb with ignore-counted breakpoints, for 0 and for 20 requests at c=1, and the
difference is divided by 20. Only counts that break on a C function's entry, or that agree with
the code, are used below. Line breakpoints inside inlined Rust code over-count (they land on shared
instructions), and `break write` also matches Rust functions named `write`; those rows in
`counts/` are ignored.

## Timings without a profiler

CPU ms per request and req/s at c=16, 10 s per target. `base` and `fp` ran ABBA (base, fp, fp,
base); the `exp` runs are the experiment's base arm (same code). System CPU and context switches are
per request, from /proc ([`timing-profilers.md`](timing-profilers.md),
[`timing-experiments.md`](timing-experiments.md), JSON in `timing/`).

| target | base CPU ms/req (4 runs) | req/s (mean) | system CPU share | vol. + invol. context switches /req | write syscalls /req |
|---|---|---|---|---|---|
| room_show | 0.1787*, 0.1666, 0.1659, 0.1676 | 21,512 | 15% | 7.2 + 3.4 | 2.0 |
| messages_page | 0.1447, 0.1389, 0.1416, 0.1418 | 25,818 | 20% | 8.6 + 3.0 | 2.0 |
| sidebar | 0.2651, 0.2687, 0.2741, 0.2779 | 13,700 | 11% | 6.6 + 3.0 | 2.0 |
| post_message | 0.4520, 0.4524, 0.4544, 0.4489 | 4,835 | 35% | 24.5 + 7.1 | 38 (WAL) |
| cable, 1,000 clients | 5.2, 4.9 ms per message delivered to all | 384-410 msg/s | 70% | 40 + 9 per message | 862 per message |

\* The first run after taking the lock was 7% slower on room_show than every later run of the same
binary. ABBA absorbs that, but a gate session should start with one throwaway run.

The two `write` syscalls per page request are the response (`writev`) and the front server's
request log line to stdout.

**Frame pointers cost nothing measurable:** `fp` (2 runs) against `base` (2 runs, ABBA):
messages_page +0.0%, sidebar +1.5%, post_message −0.2%, cable per message −1.1%. room_show reads
−4.1%, but that's base's slow first run; against the other three runs of the same code it's −0.7%.
All are within T.

## The S-2 buckets, explained (room_show)

The S-2 gperftools rollup attributed room_show to `syscall`/`__syscall_cancel_arch` ~17%, crypto
~9%, memcpy ~8%, memcmp ~4%, askama ~18% and sqlite ~16%. What those are:

| S-2 bucket | what it is | per request |
|---|---|---|
| **~17% syscalls** | gperftools charges kernel time to the user frame the syscall returns to. By caller (clean gperftools run): **8.9% is the `spawn_blocking` hop**: 4.7% waking an idle pool thread (the futex wake in `Spawner::spawn_task`'s unlock) and 4.2% threads going to sleep on the pool's condvar. 4.2% is the response `writev`, 2.1% the request log's `write` to stdout, 0.8% socket reads, 0.7% SQLite's `fcntl` WAL-index locks and 0.4% the reader pool's condvar. | 6 hops, 7.2 voluntary + 3.4 involuntary context switches, 1 `writev`, 1 log `write` |
| **~9% crypto** | Mostly not signing. **8.2% (user) is SHA-256 in the deflater splice** (`splice::text_part`, hashing the layout text around the messages). The `_mm512_clmulepi64` samples are crc32fast's CRC of the gzip body (3.3%), not AES-GCM. Actual signing is ~2.7%: cookie verify 1.2%, avatar signed ids 1.0%, turbo stream name 0.3%, GlobalId 0.2%. Building the verifiers (`generate_key` lookups) is 0.2%. | 2 text parts hashed; 4 HMACs (1 SHA1 cookie verify, 2 avatar `signed_id`s, 1 turbo stream name), each on a verifier built for the call (4 `generate_key` lookups); 0 AES-GCM; 0 PBKDF2 (cached) |
| **~8% memcpy** | 6.5% (user) is copying each cached message fragment into the page (`askama_write` → `Vec::extend_from_slice`) and 2.0% is `realloc` growing that page `String` (jemalloc copying on `rallocx`). The rest is small (SQLite, hyper, the splice's gzip assembly 0.3%). | ~40 fragments (the page) |
| **~4% memcmp** | 7.2% (user) in DWARF: the splice's `find` confirming each fragment's position with a whole-fragment `starts_with` (3.3% of it the `memcmp` leaf), and 1.2% the assets' `digested_path` binary search comparing paths. | ~40 fragment compares; one binary search per asset tag in the layout |
| **~18% askama** | The `rooms::Show` template (19.2% user) is mostly other things. Fragment copying 6.5% and page-buffer growth 3.8% (above), fragment cache keys 5.8% (below), asset paths 4.0%, `Attrs::render` 3.1%. Actual template code is a few %. | 1 page, ~40 fragment keys |
| **~16% sqlite** | 19.2% (user) in DWARF, but **SQLite itself is under half of it**. `Message::last_page` is 17.6%: stepping the query ~6% (49 `sqlite3_step` per request); rusqlite's by-name column lookup (`Row::get("…")` scans the columns with `sqlite3_column_name` + `strlen` + a case-insensitive compare, per column per row) 6.4%; `Timestamp` parsing ~2%; the `String` per row. Also `Room::original` 1.9% (a scan plus a temp B-tree for `ORDER BY created_at LIMIT 1`; `rooms` has no `created_at` index) and the layout's `Blob::attached` logo check 1.1%. | 6 reads, 49 steps, 0 prepares (the statement cache hits) |

## Ranked hot spots per target

"CPU %" is the share of all the target's CPU (user + kernel): user-space shares from the DWARF
profile scaled by the target's user share (room_show 0.85, messages_page 0.80, sidebar 0.89,
post_message 0.65, cable 0.30). Kernel shares come from gperftools, which charges kernel time to
the caller. "Measured" means an experiment moved the target by that much. "self" is the leaf share
where one function dominates. Rows ≥ ~2% are listed. Full share tables:
[`shares-perf-dwarf.md`](shares-perf-dwarf.md), [`shares-perf-fp.md`](shares-perf-fp.md),
[`shares-gperf.md`](shares-gperf.md). Rollups per profile: `perf-dwarf/`, `perf-fp/`, `gperf/`.

### room_show (0.167 ms/req, 15% kernel)

| # | CPU % | self | call path | what it is | per request | WP |
|---|---|---|---|---|---|---|
| 1 | **12.4% measured** | futex (kernel) | `Database::read` → `spawn_blocking` → pool thread → `ReaderPool::with` | a thread hop per read: futex wake + sleep, 2 context switches | 6 reads, 10.6 ctx switches | **DB-11** (new) |
| 2 | 18.8% | SHA-256 7.0, memcmp ~3, CRC 2.8 | `Ctx::finish` → `rack_etag` → `PageParts::new` (`text_part`, `locate`/`find`) ; deflater → `PageParts::gzip` → `crc32fast` | the splice rehashes the layout, re-finds each fragment, CRCs the whole body | 2 text parts, ~40 fragments | **KIT-9** (new) |
| 3 | 15.0% | `sqlite3VdbeExec` 2.0, `strlen` 2.0, `columnName` 1.4 | `rooms::render_show` → `Message::last_page` → `query_all` → `Message::from_row` → `Row::get(&str)` | the page query: ~5% stepping, 5.4% by-name column lookup, ~2% `Timestamp` parsing | 40 rows × 6 columns | **DB-13** (new), DB-8 |
| 4 | 5.5% + 3.2% | memcpy | `rooms::Show::render` → `askama_write` → `extend_from_slice` ; `realloc` | cached fragments copied into the page; the page `String` grown from askama's small size hint | ~40 fragments | **VIEW-10** (new) for growth; copying is inherent while the body is one `String` |
| 5 | 4.9% | `Display::fmt`, realloc | `cached_message_fragment` → `message_fragment_key` → `cache_key_with_version` → `cache_version` (jiff `strftime`) | each message's cache key: 3 `format!` + `strftime`, hit or miss | ~40 keys | **VIEW-11** (new) |
| 6 | 4.2% | kernel | hyper → `writev` | writing the response | 1 | inherent |
| 7 | 3.4% | memcmp | `Layout::render` → `campfire_assets::asset_path` → `try_asset_path` → `digested_path` (binary search) + `helpers::compute` (a `String`) | asset tags in the layout, looked up per request | one per asset tag | **VIEW-6** (add a perf item) |
| 8 | 3.3% (gperf) | kernel 2.1 | front `RequestLog` → tracing `fmt` → stdout `write` | the per-request log line: formatting + one `write(2)` on the worker | 1 line | **WEB-12** (new) |
| 9 | 2.6% | alloc | `Attrs::render` (`image_tag`, `builder_tag`) | attribute maps built and escaped per tag | per tag | VIEW-2 |
| 10 | 2.3% (2.7% user) | HMAC | `restore_authentication` → `verify_signed_cookie` ; `avatar_token` → `signed_id::generate` ; `signed_stream_name` | signing and verifying | 4 HMACs | STORE-2 (verifier construction: 0.2%), WEB-8 (cache `UserView` with its signed avatar id) |
| 11 | 2.2% | — | `recognize` / `normalize_path` | the router trying regexes in order | 1 | WEB-9 |
| 12 | 1.6% (gperf 1.8%) | sqlite | `render_show` → `Room::original` | `SELECT * FROM rooms ORDER BY created_at LIMIT 1`: scan + temp B-tree | 1 | **DB-14** (new) |

Below 2%: `Blob::attached` (logo) 1.1% (WEB-5), hyper outside the app 1.7%, user agent 0.8%
(WEB-2), params 0.4% (KIT-1), DB-4's `format!` 0.0%. The allocator is 10% self across the target,
spread over the rows above (page growth 1.7%, keys 1.2%, `Attrs` 0.8%, …).

### messages_page (0.142 ms/req, 20% kernel)

| # | CPU % | self | call path | what it is | per request | WP |
|---|---|---|---|---|---|---|
| 1 | **18.8% measured** | futex (kernel) | `Database::read` → `spawn_blocking` | the hop, as above | 7 reads, 11.6 ctx switches | **DB-11** |
| 2 | 12.2% | jiff `write_item` 1.1, `i32`/`i64` `Display` | `message_item` → `cached_message_fragment` → `message_fragment_key` | fragment cache keys | 40 keys | **VIEW-11** |
| 3 | 16.2% | `strlen` 2.3, `columnName` 1.6 | `find_paged_messages` → `Message::page_before` → `Message::from_row` | the page query: 6.1% by-name lookup, ~2% `Timestamp` parsing | 40 rows, 57 steps | **DB-13**, DB-8 |
| 4 | 11.9% | memcmp 3.0, CRC 3.0 | `rack_etag` → `PageParts::new` → `locate`/`find` ; `PageParts::gzip` → `crc32fast` | the splice (no text hashing here: 1 short text part) | ~40 fragments | **KIT-9** |
| 5 | 5.0% | memcpy | `askama_write` | fragments copied into the page | 40 | inherent |
| 6 | 4.5% + 1.8% | kernel | `writev` ; request log `write` | response and log line | 1 + 1 | inherent ; **WEB-12** |
| 7 | 1.5% (gperf 2.7%) | sqlite | `Room::original` | as above | 1 | **DB-14** |
| 8 | 1.8% | — | router | | 1 | WEB-9 |

### sidebar (0.272 ms/req, 11% kernel)

| # | CPU % | self | call path | what it is | per request | WP |
|---|---|---|---|---|---|---|
| 1 | **40%** | `longest_match_help` 10.4, `is_match` 7.6 | deflater → `gzip_stream` → flate2 → zlib-rs `deflate_medium` | the whole sidebar deflated at level 6 on every request, although it repeats | 1 body | **KIT-10** (new) |
| 2 | **12.2% measured** | futex | `Database::read` → `spawn_blocking` | the hop | 5 reads, 9.6 ctx switches | **DB-11** |
| 3 | 6.5% | `strlen` 2.4 | `Membership::with_rooms…` → `room_from_prefixed_row` / `Membership::from_row` → `Row::get(&str)` | membership + room rows, by-name lookups | one row per room | **DB-13** |
| 4 | 4.3% | SHA-256 | `rack_etag` → `Sha256::digest(body)` | Rack::ETag over the whole body (Rails does the same) | 1 | inherent (KIT-10 reuses the digest as its key) |
| 5 | 3.9% | alloc | `Attrs::render` | tag attributes | per tag | VIEW-2 |
| 6 | 3.1% + 1.5% | kernel | `writev` ; log `write` | | | inherent ; **WEB-12** |
| 7 | 1.4% | HMAC | `signed_id::generate` (avatars) | | 8 HMACs (5 signed ids, 2 stream names, 1 cookie), 8 `generate_key` | STORE-2, WEB-8 |

### post_message (0.452 ms/req, 35% kernel)

| # | CPU % | self | call path | what it is | per request | WP |
|---|---|---|---|---|---|---|
| 1 | **22.9% measured** | futex | `Database::read` → `spawn_blocking` | the hop | 10 reads + 1 write, 31.6 ctx switches | **DB-11** |
| 2 | **14.1% measured** (gperf 8.6%) | `munmap`/`mmap` (kernel) | `sqlite3PagerSharedLock` → `pagerBeginReadTransaction` → `unixUnfetch` → `munmap` ; `unixFetch` → `unixRemapfile` → `mmap` | with `mmap_size = 128 MB`, each reader remaps the file after every commit | per reader per commit | **DB-12** (new, measure) |
| 3 | 8.9% | kernel | `seekAndWrite` → `pwrite` ; `unixRead` → `pread` | WAL writes and reads | 32 `pwrite64`, 16 `pread64` | inherent |
| 4 | 9.2% | `pqdownheap`, `memset` 1.8 | deflater → `gzip_stream` → flate2 | gzipping the response (unique each time; a fresh encoder state is zeroed per response) | 1 | inherent, except encoder reuse (~1%, **KIT-10**) |
| 5 | 6.6% | — | `Presenter::message` → `RichTextRecord::find_for` ; `Message::find_by_id` | re-reading the message and its body just written | 2+ reads | WEB-8 |
| 6 | 5.1% | `next_code_point`, `next_match` | `present_message`, `to_plain_text` ×2, `mentioned_users`, `canonical_body` → `Content::load` → html5ever | the new body parsed ~4 times | 4 parses | VIEW-8, VIEW-3 |
| 7 | 4.4% | char scanning 2.0 | `Broadcasts::to` → `broadcast_stream_to` → `campfire_cable::json::encode` / `escape_html_entities` (twice) | encoding and re-escaping the broadcast | per broadcast | F-2 (in flight; `EncodedJson` stops the re-escape), LIVE-2 |
| 8 | 3.6% | sqlite | `Message::create` → `create_in_index` | the FTS insert | 1 | inherent (Rails does it) |
| 9 | 3.0% | — | web push: `PushSubscription` reads, job | | | LIVE-5 |
| 10 | 4.5% (gperf) | kernel | `read`s | request body, wakers | | inherent |

### Cable fan-out, 1,000 clients (≈5 ms CPU per message, 70% kernel)

| # | CPU % | self | call path | what it is | per message | WP |
|---|---|---|---|---|---|---|
| 1 | **66%** (gperf) | kernel | cable runtime → `socket::Writer::send` → `write_vectored` → `writev` | one socket write per client per message; on loopback the write also runs the receiver's TCP path | 862 writes (0.86 per delivery) | **LIVE-9** (new, measure) |
| 2 | ~8% (27% of user) | `futures_core::register`, atomics | `connection::run` → `SelectAll<Deliveries>` / `FuturesUnordered` | each connection polls its subscriptions as boxed, abortable streams | per delivery | LIVE-9 |
| 3 | ~4% (12-15% of user) | `parking_lot::lock/unlock` 9% of user | `broadcast::Receiver::recv_ref` | 1,000 receivers on one tokio `broadcast` channel contend on its tail lock | per delivery | LIVE-1 |
| 4 | ~4% (13% of user) | — | `Hub::broadcast` / pubsub | | per message | LIVE-1 |
| 5 | ~3% | sqlite, render | the four posters' requests | | | (post_message's WPs) |

## perf vs `bench/profile cpu` (gperftools)

| | `perf` (`cpu-clock:u`, 1 kHz) | gperftools (`bench/profile cpu`) |
|---|---|---|
| **What it sees** | User space only (`perf_event_paranoid=2`: kernel-mode samples are dropped). Kernel time is invisible: post_message's `munmap` shows as 0.3%. | User time plus kernel time, charged to the frame the syscall returns to (ITIMER_PROF counts both). That is why S-2's rollup showed a `syscall` bucket at all. |
| **Sample capture** | Complete: room_show recorded 32.7 CPU-s of user samples against 29.8 s of measured user CPU. | **Loses ~40% of samples** under load. One process-wide ITIMER_PROF delivers SIGPROFs that coalesce while one is pending: room_show recorded 22.2 CPU-s of the ~37 CPU-s it used. The loss looks uniform (shares agree with perf's within ~1-2 points), but it isn't guaranteed. |
| **Stacks** | FP: complete, but frameless glibc leaves lose their immediate caller. DWARF: exact, but needs a GNU-ld build and `--per-thread` (above), and 8 KB stack copies truncate deep stacks. | libgcc unwinding from the signal frame: exact callers for leaves, full depth. |
| **Overhead** (vs the unprofiled mean) | room_show +4.6%, messages_page +4.8%, sidebar +1.9%, **post_message +19%**, cable +13%. The overhead grows with context switches (post_message has 32 per request), most likely because perf switches its per-thread events on each one (not confirmed). | room_show +2.8%, messages_page +0.8%, sidebar +3.0%, post_message +4.0%. |
| **Stability** | 13 recordings today (9 in the clean session), no failure. | Crashed the app twice today (SIGSEGV in libgcc `_Unwind_Backtrace` from the SIGPROF handler; `TASKS.md`, "Found while working"). No crash in this session. |
| **Setup** | Nothing to preload; needs the FP build (or a GNU-ld build for DWARF) for good stacks. | `LD_PRELOAD` of libprofiler, SIGUSR2 toggling, works with any build. |

**Recommendation for the CPU gate:** measure CPU ms/request and req/s **without a profiler**:
`bench/profile perf --freq 0 --label <label> --targets room_show,messages_page,sidebar,post_message
--out …`, which writes `perf.json` with the same `rps`/`cpu_ms_per_req` fields `cpu.json` has, plus
user/system CPU and context switches per request. Keep the ABBA order and start each session with a
throwaway run. Both profilers perturb what the gate measures (gperftools by 1-4%, perf by up to
19%), and gperftools can crash the run. Profiles are for explaining, not gating. Use
`bench/profile perf --call-graph fp` with an `fp` build (frame pointers cost ≤1.5%, within noise),
the `fpbfd` build with `--call-graph dwarf` and `perfprof.py --per-thread` when leaf callers
matter, and `bench/profile cpu` when kernel time needs attributing to call sites. The coordinator
should switch the recipe between WPs, never between a WP's base and WP runs.

## Perf order

Biggest measured cost first. Gains are % CPU per request; "measured" means an experiment in this
session, otherwise it's the hot spot's share (an upper bound). Expect less where a fix can't remove
all of it.

| # | WP | targets and expected gain | basis |
|---|---|---|---|
| 1 | **DB-11** reads without the blocking-pool hop | room_show −12%, messages_page −19%, sidebar −12%, post_message −23% (req/s +16…+30%) | measured |
| 2 | **KIT-10** reuse gzip output for repeated bodies (+ encoder reuse) | sidebar up to −40%; post_message ~−1% | 44-45% user share; `memset` 1.8% |
| 3 | **KIT-9** splice without rehashing the page | room_show up to −19% (realistically −12…−15%), messages_page up to −12% | 22% / 15% user share |
| 4 | **DB-12** `mmap_size` (measure; after DB-11) | post_message −14%; reads +2-3% (so maybe readers only) | measured |
| 5 | **VIEW-11** fragment cache keys without formatting (after VIEW-4) | messages_page up to −12%, room_show up to −5% | 15% / 6% user share |
| 6 | **DB-13** column indices once per query | reads −5…−6%, post_message −2% | 6.4-7.6% user share |
| 7 | WEB-8 (body once, `UserView` cache) + VIEW-8 (parse once) | post_message up to −12% together | 6.6% + 5.1% |
| 8 | **VIEW-10** page buffers sized up front | room_show −3%, sidebar −2% | realloc under render |
| 9 | VIEW-6, as perf (asset paths) | room_show −3% | 4.0% user share |
| 10 | **WEB-12** request log off the request path | −2…−3% on every request | 2.1% kernel + formatting |
| 11 | F-2 (in flight) → LIVE-2 | post_message −4% | cable JSON escaping twice + encoding |
| 12 | VIEW-2 (`Attrs`) | sidebar −4%, room_show −3% | share |
| 13 | DB-8 timestamps | reads −2% | `Timestamp::column_result` |
| 14 | WEB-9 router | reads −2% | share |
| 15 | **DB-14** `Room::original` | reads −1.5…−2% | share |
| 16 | LIVE-1, **LIVE-9** (cable) | cable: LIVE-1 ≤ ~8% of CPU; LIVE-9 is where the 66% kernel is | share |
| 17 | WEB-5 (fewer reads) | re-measure after DB-11: its gain was mostly the hops | — |
| — | KIT-1 (0.4-0.7%), WEB-2 (≤0.9%), STORE-2 (0.2-0.3% construction), VIEW-1 (its copies <0.5%: `raw` 0.2%, fragment `to_vec` 0.1%), KIT-2 (each item too small to see) | below T on these targets; keep them for allocations and readability, not as perf | shares |
| — | DB-4 (0.0%) | no measurable cost: drop "perf" from it | share |
| — | DB-1, DB-5, DB-6, STORE-3, WEB-1, VIEW-3 (cache misses only) | not on these targets (new DMs, bulk inserts, uploads, static assets); no data here | — |

Interactions: DB-11 first, because it changes what everything else is measured against (the
hops also inflate every other WP's variance). KIT-9 and KIT-10 both touch `deflater`, so do them
one after the other. VIEW-11 lands after VIEW-4, which reshapes the same keys. S-9 (the write path
and cable vs v0.1.1) should use these profiles: post_message's top items (hops, mmap, WAL I/O)
and cable's `writev` are where a regression would show.

## Files

- `README.md`: this.
- `experiment.patch`: the `exp` binary's two env toggles (not committed as code).
- `timing/*.json`, `timing-profilers.md`, `timing-experiments.md`: every timing run (perf mode
  `perf.json`, with /proc counters).
- `perf-fp/`, `perf-dwarf/`, `gperf/`: rollups (`*.top.md`) and the runs' `perf.json`/`cpu.json`.
  The folded stacks (30-190 MB each, ~4 MB gzipped even with short names) stay out of git. Dropping
  the long tail to shrink them skewed the shares by up to 5 points, so they're regenerated instead:
  `bench/profile perf` writes them.
- `shares-*.md`: the hot-spot share tables (`scripts/wp_shares.py`) per profile.
- `counts/`: gdb call counts (`scripts/count.py`). Use only the rows the text above uses.
- `scripts/`: the session, count, share and timing scripts, as run (paths are this worktree's).
