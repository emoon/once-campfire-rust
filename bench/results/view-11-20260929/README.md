# VIEW-11: fragment cache keys without formatting (2026-09-29)

Keys are written digit by digit into a per-thread reused buffer instead of three `format!`s and
a jiff `strftime`; byte-identical to before (tested over ~20k timestamps and edge-case ids).
ABBA (`bench/profile perf --freq 0`, each run in `bench/quiet`) against `refactor/cleanup` at
6b98426, c=16, mean of two runs a side (`time-*`, `time.log`):

| target | CPU ms/req base | view-11 | change | req/s base | view-11 | change | T |
|---|---|---|---|---|---|---|---|
| room_show | 0.1487 | 0.1412 | -5.0% | 26,056 | 27,392 | +5.1% | 3.0% |
| messages_page | 0.1254 | 0.1141 | -9.0% | 30,783 | 33,657 | +9.3% | 3.0% |
| sidebar | 0.1287 | 0.1269 | -1.4% | 29,954 | 30,390 | +1.5% | 3.0% |
| post_message | 0.3503 | 0.3483 | -0.5% | 5,311 | 5,495 | +3.5% | 3.2% |

messages_page and room_show gain more than 1.5 T, so no second pass was due.
