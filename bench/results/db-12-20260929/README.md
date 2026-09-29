# DB-12: no `mmap_size` (2026-09-29)

Rails sets `PRAGMA mmap_size = 128 MB` on every connection; this drops it. ABBA (`bench/profile
perf --freq 0`, each run in `bench/quiet`) against `refactor/cleanup` at e1a2686, which has DB-11.
Mean of two runs a side, c=16 (`time-*`, `time.log`):

| target | CPU ms/req base | db-12 | change | req/s base | db-12 | change | T |
|---|---|---|---|---|---|---|---|
| room_show | 0.1532 | 0.1528 | -0.2% | 25,350 | 25,398 | +0.2% | 3.0% |
| messages_page | 0.1270 | 0.1257 | -1.0% | 30,270 | 30,820 | +1.8% | 3.0% |
| sidebar | 0.2474 | 0.2442 | -1.3% | 15,847 | 16,055 | +1.3% | 3.0% |
| post_message | 0.4133 | 0.3604 | -12.8% | 5,010 | 5,454 | +8.9% | 3.2% |

post_message gains well past T; the reads are within ±1.3%, inside T/2.

Before DB-11 (`campfire-wt/perf-2`, variants base/off/readers-only, same recipe): post_message
−15.5% CPU/req and +12.7% req/s, but room_show and messages_page 2.5-2.7% worse; the `rdr` variant
(mmap changed for the reader connections only) came out worse than off everywhere. With DB-11's inline
reads the read cost is gone.
