# DB-13 on the newer tip (2026-09-30)

Dropped on 2026-09-29 (`../db-13-20260929/`: best gain −2.9%, under T). A `perf` profile of the
tip after VIEW-2 (`../profile-tip-20260930/`) showed the name lookups it removes had grown as a
share once other costs fell: `room_from_prefixed_row` alone was 6.5% of sidebar, and
`columnName`/`strlen`/`sqlite3_column_name` ~3% of room_show. Re-timed after merging
`refactor/cleanup` at d421663 into the branch: ABBA (`bench/profile perf --freq 0`, each run in
`bench/quiet`), c=16; the first pass landed in the re-run band, so this is the mean of eight runs
(`time-*`, `time2-*`, `time.log`):

| target | CPU ms/req base | db-13 | change | req/s base | db-13 | change | T |
|---|---|---|---|---|---|---|---|
| room_show | 0.1290 | 0.1258 | -2.5% | 29,822 | 30,583 | +2.6% | 3.0% |
| messages_page | 0.1098 | 0.1055 | -3.9% | 34,911 | 36,329 | +4.1% | 3.0% |
| sidebar | 0.1104 | 0.1062 | -3.8% | 34,714 | 36,004 | +3.7% | 3.0% |
| post_message | 0.3499 | 0.3550 | +1.5% | 5,252 | 5,135 | -2.2% | 3.2% |

messages_page and sidebar now gain more than T; post_message is within T.
