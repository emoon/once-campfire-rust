# DB-13: column indices once per query (2026-09-29) — dropped

Branch `refactor/cleanup-db-13` (a `columns!` macro; `from_row(row, &cols)` resolves each column's
index once per query instead of per `Row::get(&str)`). S-8 put the name lookups at 5-6% of the
reads. ABBA (`bench/profile perf --freq 0`, each run in `bench/quiet`) against
`refactor/cleanup` at fc1aef6 (DB-11, DB-12, KIT-10 merged), c=16. The first pass landed in the
re-run band, so this is the mean of all eight runs (`time-*`, `time2-*`, `time.log`):

| target | CPU ms/req base | db-13 | change | req/s base | db-13 | change | T |
|---|---|---|---|---|---|---|---|
| room_show | 0.1553 | 0.1532 | -1.4% | 24,948 | 25,211 | +1.1% | 3.0% |
| messages_page | 0.1305 | 0.1267 | -2.9% | 29,594 | 30,318 | +2.4% | 3.0% |
| sidebar | 0.1290 | 0.1264 | -2.0% | 29,822 | 30,401 | +1.9% | 3.0% |
| post_message | 0.3601 | 0.3679 | +2.2% | 5,424 | 5,323 | -1.9% | 3.2% |

No target gains more than T, and post_message is 2.2% worse, so the WP is dropped as perf. Its
code isn't merged; the branch stays for reference.
