# VIEW-3: richtext statics, fewer sanitize passes, allocation-free name matching (2026-09-29)

The four standard targets serve messages from the fragment cache, so they barely run richtext.
To reach the code, the cold targets start both binaries with `CAMPFIRE_FRAGMENT_CACHE_MB=0` (the
cache keeps nothing, so every message renders its rich text on every request; `cold-wrapper.sh`).
ABBA (`bench/profile perf --freq 0`, each run in `bench/quiet`) against `refactor/cleanup` at
4ac2ea8, c=16. Both comparisons landed in the re-run band after one pass, so each is the mean of
eight runs.

Cold fragment cache (`time-cold-*`, `time-cold2-*`, `time-cold.log`):

| target | CPU ms/req base | view-3 | change | req/s base | view-3 | change | T |
|---|---|---|---|---|---|---|---|
| room_show | 1.2098 | 1.1538 | -4.6% | 3,062 | 3,371 | +10.1% | 3.0% |
| messages_page | 1.4974 | 1.4324 | -4.3% | 2,539 | 2,733 | +7.6% | 3.0% |
| post_message | 0.3982 | 0.3878 | -2.6% | 4,760 | 4,940 | +3.8% | 3.2% |

Standard targets, no-regression check (`time-*`, `time2-*`, `time.log`):

| target | CPU ms/req base | view-3 | change | req/s base | view-3 | change | T |
|---|---|---|---|---|---|---|---|
| room_show | 0.1551 | 0.1540 | -0.7% | 24,983 | 25,121 | +0.6% | 3.0% |
| messages_page | 0.1288 | 0.1275 | -1.0% | 30,028 | 30,322 | +1.0% | 3.0% |
| sidebar | 0.1287 | 0.1285 | -0.2% | 29,921 | 30,005 | +0.3% | 3.0% |
| post_message | 0.3565 | 0.3638 | +2.0% | 5,497 | 5,358 | -2.5% | 3.2% |

post_message is within T; it's +2.0% here but −2.6% on the cold runs, as bimodal as S-2 found it.
`perf-stages.md`: the WP agent's `perf` microbenchmark, −16% cycles on the richtext path.

Parity on the merged branch (18e9b8b + batch 1): 874 cells, 873 pass, 0 fail, 1 allowed.
