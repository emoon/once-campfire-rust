# WEB-12: request log off the request path (2026-09-29) — dropped

Branch `refactor/cleanup-web-12` (3c596fe, 967b0c2): request log lines go to a batching writer
thread instead of one `write(2)` per line on the request's worker. ABBA (`bench/profile perf
--freq 0`, each run in `bench/quiet`) against `refactor/cleanup` at 6b98426, c=16, mean of two runs
a side:

| target | CPU ms/req base | web-12 | change | req/s base | web-12 | change | T |
|---|---|---|---|---|---|---|---|
| room_show | 0.1484 | 0.1497 | +0.9% | 26,076 | 25,852 | -0.9% | 3.0% |
| messages_page | 0.1243 | 0.1263 | +1.5% | 31,059 | 30,445 | -2.0% | 3.0% |
| sidebar | 0.1291 | 0.1304 | +1.0% | 29,851 | 29,531 | -1.1% | 3.0% |
| post_message | 0.3460 | 0.3493 | +0.9% | 5,423 | 5,473 | +0.9% | 3.2% |

No gain; every target is slightly worse (the hand-off to the writer thread costs about what the
write saved). Dropped as perf; the code isn't merged.

Not measured: the bench sends the app's stdout to a file (`bench/.work/.../app.log`), where a write
is cheap. In production under Docker stdout is a pipe to the log driver, where a write can cost
more or block; the result there may differ.
