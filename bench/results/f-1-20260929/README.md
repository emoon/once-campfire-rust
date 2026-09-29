# F-1: one ERB escaper (`rails_compat::erb`)

Base: `refactor/cleanup` at `3c7a173`. WP: `refactor/cleanup-f-1` at `b006a9f`. Both release builds
with `CARGO_PROFILE_RELEASE_DEBUG=line-tables-only`, measured in the same session with the gate
commands in `plans/cleanup.md`. T from `bench/results/cleanup-baseline-20260928/README.md`.

The branch was since rebased onto `a0df4b6`, which changed only `parity/bin/candidate` and
`plans/cleanup.md`, and review nits were folded in: a `tls.rs` test, the `cpuprof.py` category
regex, and `build.rs` escaping through `rails_compat::erb` (build time only; the embedded
importmap tags are byte-identical). No code that runs per request changed, so these numbers stand.

## Allocations per request (`bench/profile alloc`)

| target | base | F-1 |
|---|---|---|
| room_show | 18.0 | 18.0 |
| messages_page | 19.0 | 19.0 |
| sidebar | 53.1 | 53.1 |
| post_message (two runs; compare the higher) | 77.3, **78.6** | 77.2, **78.8** |

No change: post_message's higher runs differ by 0.2, inside its 0.3 noise in the high mode.

## CPU and throughput (`bench/profile cpu`, ABBA: base, F-1, F-1, base)

| target | base CPU ms/req | F-1 CPU ms/req | Δ | base req/s | F-1 req/s | Δ | T |
|---|---|---|---|---|---|---|---|
| room_show | 0.1727, 0.1724 | 0.1711, 0.1727 | -0.4% | 21,443, 21,412 | 21,551, 21,400 | +0.2% | 3% |
| messages_page | 0.1462, 0.1451 | 0.1449, 0.1430 | -1.2% | 25,130, 25,255 | 25,263, 25,579 | +0.9% | 3% |
| sidebar | 0.2774, 0.2763 | 0.2786, 0.2780 | +0.5% | 13,375, 13,464 | 13,358, 13,438 | -0.2% | 3% |
| post_message | 0.4648, 0.4605 | 0.4655, 0.4641 | +0.5% | 4,700, 4,761 | 4,628, 4,708 | -1.3% | 3.2% |

Every target is within T and outside the T/2 re-run band. Gate passes.

## Parity

`parity/bin/candidate compare`: 874 cells: 873 pass (0 flaky), 0 fail, 1 allowed, 0 error, the
same as the Phase 0 baseline.
