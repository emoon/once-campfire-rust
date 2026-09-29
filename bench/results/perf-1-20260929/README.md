# DB-11: reads on the calling worker (2026-09-29)

`Database::read` runs a read inline on the tokio worker when a reader connection is free, then
`yield_now()`, instead of hopping to the blocking pool and back. Reads that grow with the whole
database use `read_offloaded`. 9950X3D, server on CPUs 8-11, load generator on 12-15, c=16.

## CPU and throughput (`bench/profile perf --freq 0`, ABBA after a warm-up, each run in `bench/quiet`)

Final branch (`time-*`, `time.log`); base is `refactor/cleanup` at cfc9144. Mean of two runs a side.

| target | CPU ms/req base | perf-1 | change | req/s base | perf-1 | change | T |
|---|---|---|---|---|---|---|---|
| room_show | 0.1663 | 0.1494 | -10.2% | 22,204 | 26,029 | +17.2% | 3.0% |
| messages_page | 0.1399 | 0.1227 | -12.3% | 26,282 | 31,592 | +20.2% | 3.0% |
| sidebar | 0.2726 | 0.2464 | -9.6% | 13,598 | 15,909 | +17.0% | 3.0% |
| post_message | 0.4555 | 0.4157 | -8.7% | 4,676 | 5,038 | +7.7% | 3.2% |

Every target gains more than T (`bench/results/cleanup-baseline-20260928/README.md`).

## The cap on inline reads was dropped

Review asked for a cap: inline only while fewer than half the runtime's workers are inside a read,
so a stalled read can't hold every worker. Measured (`capped/`, `tails-cap/`, `tails-cap3/`), it
gave back most of the gain, because every read past the cap pays the blocking-pool hop again.

The CPU ABBA with the half-the-workers cap (`capped/time.log`):

| target | CPU ms/req base | perf-1 | change | req/s base | perf-1 | change | T |
|---|---|---|---|---|---|---|---|
| room_show | 0.1698 | 0.1589 | -6.4% | 21,711 | 22,074 | +1.7% | 3.0% |
| messages_page | 0.1444 | 0.1341 | -7.1% | 25,300 | 24,979 | -1.3% | 3.0% |
| sidebar | 0.2735 | 0.2537 | -7.3% | 13,591 | 14,301 | +5.2% | 3.0% |
| post_message | 0.4561 | 0.4368 | -4.2% | 4,645 | 4,856 | +4.5% | 3.2% |

Native tails (`tails.py`, fresh app and seed per label, two runs a side in the order base, X, Y,
Y, X, base). Change against base:

| route | half the workers: req/s | p99 | all but one: req/s | p99 | no cap: req/s | p99 |
|---|---|---|---|---|---|---|
| room_show c=16 | −3.7% | +38.4% | +16.9% | −2.1% | +23.5…25.4% | −34…−36% |
| messages_page c=16 | −5.8% | +33.8% | +14.8% | −7.2% | +25.1…25.2% | −42% |
| sidebar c=16 | +2.4% | +22.3% | +16.9% | −14.7% | +12.4…18.4% | −11…−18% |
| search c=16 | +2.5% | +1.5% | +11.7% | −11.5% | +10.6…14.0% | −9…−15% |
| room_show c=64 | −0.5% | +27.8% | +15.2% | −2.9% | +19.7…20.0% | −16…−17% |
| messages_page c=64 | −0.1% | +19.9% | +8.5% | +10.3% | +20.9…23.1% | −21…−27% |

(no-cap ranges span the two sessions.) The `bench/run` pair in `quiet-run-*` (Docker, one rep a
side) was taken with the half-the-workers cap and shows the same p99 rise.

Older runs from before the review, on a busier machine: `run*-*` (`bench/run`), `tails/` (the
yield fixing a c=64 p99 regression), `alloc-*` (allocations, informational).
