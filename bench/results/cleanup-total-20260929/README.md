# Cleanup so far: start vs tip (2026-09-29)

`cfc9144` (the cleanup's start of 2026-09-29) against `4ac2ea8` (DB-11, DB-12, KIT-10 merged), in
one session: ABBA (`bench/profile perf --freq 0`, each run in `bench/quiet`), c=16, mean of two
runs a side. "base" is the start, "tip" the current branch.

| target | CPU ms/req base | tip | change | req/s base | tip | change | T |
|---|---|---|---|---|---|---|---|
| room_show | 0.1704 | 0.1544 | -9.4% | 21,553 | 25,116 | +16.5% | 3.0% |
| messages_page | 0.1415 | 0.1285 | -9.1% | 25,863 | 30,041 | +16.2% | 3.0% |
| sidebar | 0.2703 | 0.1285 | -52.5% | 13,704 | 29,967 | +118.7% | 3.0% |
| post_message | 0.4503 | 0.3579 | -20.5% | 4,679 | 5,452 | +16.5% | 3.2% |

The sidebar target repeats one body, so it's all KIT-10 cache hits; real sidebars gain less.
