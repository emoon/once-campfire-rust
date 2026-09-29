# WEB-1: static files without a copy

`static_response` (`crates/campfire/src/app.rs`) passes the `Cow<'static, [u8]>` from
`campfire_assets::serve` to `Body::from`, which keeps a borrowed body borrowed
(`Bytes::from_static`), instead of copying it into a `Vec` with `into_owned()` per request.

Base: `refactor/cleanup` at `3c2ba5a`. WP: `refactor/cleanup-web-1`. Both release builds with
`CARGO_PROFILE_RELEASE_DEBUG=line-tables-only`. T from
`bench/results/cleanup-baseline-20260928/README.md`: 3% (room_show, messages_page, sidebar),
3.2% (post_message); allocation noise 0.1/request.

Two targets are new in `bench/lib/benchlib.py`:

- `static_css`: the page's stylesheet through the front server. After the first request the front
  server's response cache answers it (`Cache-Control: public, max-age=2592000`), so the changed
  code doesn't run. It's a control.
- `static_css_app`: the same file from the bare app on PORT + 1, behind the front server. Every
  request takes the changed path. This is what production does on a response-cache miss.

## Allocations per request

`bench/profile alloc` today counts only C (SQLite) allocations. Rust's are what this WP changes,
so both binaries were also built with `--features tikv-jemallocator/stats` and measured with
S-7's `bench/profile` (branch `refactor/cleanup-s-7` at 6632271, copied in as an untracked
`bench/profile-s7`), which counts both. They are provisional until S-7 lands, and for information only.

| target | C, base | C, WP | Rust, base | Rust, WP |
|---|---|---|---|---|
| room_show | 18.0 | 18.0 | 1619.7 ×3 | 1620.3, 1619.7, 1620.3 |
| messages_page | 19.0 | 19.0 | 1188.7 | 1188.7 |
| sidebar | 53.0 | 53.0 | 2196.4 | 2196.5 |
| post_message (higher of two) | 79.4 | 78.7 | 1146.2 | 1146.8 |
| **static_css_app** | 0.0 | 0.0 | **98.0** | **97.0** |
| static_css | 0.0 | 0.0 | 40.6 | 40.6 |

static_css_app drops by 1.0 allocation per request: the `Vec` the copy made. room_show runs the
same code on both sides: `static_response` returns at `serve(..)?` for a non-file path, before
the changed line. Its 1619.7 and 1620.3 are two modes of one code path, and S-7 is still
measuring that spread: its runs of one binary range from 1619.7 to 1621.2. The same goes for
post_message's 0.6.

Files: `<side>-<route>/` (C only, today's tool) and `rust-alloc/<side>-<route>[-n]/` (S-7's tool).

## CPU per request and throughput

`bench/profile cpu`, 10 s at c=16 per target, in two ABBA rounds (base, WP, WP, base). A run
counts only if the screensaver was off and the benchmark cores (8-15, 24-31) were under 5% busy,
both right before and right after it. An earlier second round, at 14:17-14:24, was discarded:
another project's unpinned cargo builds loaded all 32 CPUs (load average 28), and room_show fell
to 14k req/s. Those runs are kept in `discarded/`.
The comparison started at 12:47, before the plan moved to `bench/profile perf --freq 0` inside
`bench/quiet`, so it stays on `bench/profile cpu` throughout. Round 2 used this WP's own guard
(`pgrep -f 'omarchy[-]screensaver'` plus bench cores under 5% busy before and after each run), not
`bench/quiet`.
Round 1 alone had post_message req/s at -5.8% (base 4,866/4,920, WP 4,612/4,610) with CPU/req
+1.6%. That's outside 1.5 T, but it's the same code path, and round 2 showed the opposite
(+5.1%). So the decision is on the mean of all eight runs.

| target | CPU ms/req base | WP | Δ | req/s base | WP | Δ |
|---|---|---|---|---|---|---|
| room_show | 0.1703 | 0.1708 | +0.3% | 21,431 | 21,352 | -0.4% |
| messages_page | 0.1436 | 0.1440 | +0.2% | 25,471 | 25,443 | -0.1% |
| sidebar | 0.2737 | 0.2716 | -0.8% | 13,555 | 13,623 | +0.5% |
| post_message | 0.4555 | 0.4575 | +0.4% | 4,755 | 4,732 | -0.5% |
| static_css_app | 0.0180 | 0.0181 | +0.3% | 204,718 | 202,930 | -0.9% |
| static_css | 0.0071 | 0.0071 | -1.1% | 431,053 | 440,665 | +2.2% |

Every target is within T, and none gains more than T in CPU or req/s, static_css_app included.
Copying the stylesheet once and making one allocation cost well under 1% of an 18 µs request, so
this change can't show up above T. Under the plan's rule as of 7ac1aae/7ef98d2, only time counts
as a gain, and allocation counts are for information only. The one fewer allocation on
static_css_app therefore doesn't keep the WP. **WEB-1 is dropped: no measurable gain.** In
production the change would matter even less, because the front server's response cache answers
repeat asset requests.

The code change was commit `8e3df3e` on `refactor/cleanup-web-1`.

Files: `cpu-<side>-<n>/cpu.json` and the `.top.md` rollups.

No `bench/run` suite: the http suite's `static_css` goes through the front server's cache (373k
req/s at the baseline), so it can't see the changed path. static_css_app above is the direct
measurement.
