# F-4: `rails_compat::ruby::{to_i, cast_integer}`

Perf gate for F-4 (`plans/cleanup.md`, "Commands for the gates"). Base: `refactor/cleanup` at
`3c7a173`, built in `$MAIN` and copied to `target/base-campfire`. WP: `refactor/cleanup-f-4`,
copied to `target/f4-campfire`. Both release builds with `CARGO_PROFILE_RELEASE_DEBUG=line-tables-only`.
T is from `bench/results/cleanup-baseline-20260928/README.md`: 3% for room_show, messages_page and
sidebar, 3.2% for post_message.

## Allocations per request (`bench/profile alloc`)

| target | base | F-4 | verdict |
|---|---|---|---|
| room_show | 18.0 | 18.1 | noise (0.1) |
| messages_page | 19.0 | 19.0 | same |
| sidebar | 53.1 | 53.1 | same |
| post_message (two runs each; compare the higher) | 79.0, 78.7 | 78.6, 77.5 | 79.0 → 78.6, within the high mode's 0.3 noise |

## CPU per request and throughput (`bench/profile cpu`, ABBA: base, F-4, F-4, base)

| target | base CPU ms/req | F-4 CPU ms/req | Δ | base req/s | F-4 req/s | Δ | T |
|---|---|---|---|---|---|---|---|
| room_show | 0.1701, 0.1702 | 0.1695, 0.1712 | +0.12% | 21,681, 21,604 | 21,862, 21,540 | +0.27% | 3% |
| messages_page | 0.1449, 0.1444 | 0.1434, 0.1442 | −0.59% | 25,251, 25,434 | 25,566, 25,380 | +0.52% | 3% |
| sidebar | 0.2766, 0.2753 | 0.2727, 0.2750 | −0.76% | 13,428, 13,498 | 13,627, 13,488 | +0.70% | 3% |
| post_message | 0.4594, 0.4633 | 0.4622, 0.4601 | −0.04% | 4,810, 4,806 | 4,729, 4,833 | −0.56% | 3.2% |

Every target is well inside T, so the gate passes. F-4 isn't a perf WP. The new parser doesn't
allocate, where the old `cast_integer` collected its digits into a `String`, but the difference
doesn't show at this resolution.

Files: `{base,f-4}-<route>/alloc-<route>.json` (and the `jeprof` top sites),
`cpu-{base,f-4}-{1,2}/cpu.json` and `cpu-*/cpu-<target>.top.md`.

## Parity lean gate

`parity/bin/candidate compare`, with images built after `2895add`/`a0df4b6` (no shared cargo cache)
from `69a9ac2`: **874 cells: 873 pass (0 flaky), 0 fail, 1 allowed, 0 error** in 2,191 s, the
same as the baseline (`parity/out/compare-2026-09-29T05-04-43-722Z/`).
