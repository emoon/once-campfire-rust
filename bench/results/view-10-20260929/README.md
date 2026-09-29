# VIEW-10: page buffers sized up front (2026-09-29)

`render_sized!` renders pages with `render_into` into a `String` reserved from the last size rendered
at that call site (+1/8, capped at 1 MiB), instead of askama's small size hint grown by `realloc`.
Used for the framed pages (room show and ~20 others), the messages index and the sidebar.
ABBA (`bench/profile perf --freq 0`, each run in `bench/quiet`) against `refactor/cleanup` at
6b98426, c=16. The first pass landed in the re-run band; mean of all eight runs (`time-*`,
`time2-*`, `time.log`):

| target | CPU ms/req base | view-10 | change | req/s base | view-10 | change | T |
|---|---|---|---|---|---|---|---|
| room_show | 0.1484 | 0.1418 | -4.4% | 26,099 | 27,251 | +4.4% | 3.0% |
| messages_page | 0.1242 | 0.1203 | -3.1% | 31,096 | 32,081 | +3.2% | 3.0% |
| sidebar | 0.1285 | 0.1276 | -0.7% | 30,000 | 30,207 | +0.7% | 3.0% |
| post_message | 0.3481 | 0.3490 | +0.3% | 5,538 | 5,583 | +0.8% | 3.2% |
