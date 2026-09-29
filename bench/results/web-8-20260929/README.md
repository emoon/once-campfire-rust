# WEB-8 perf parts (2026-09-29) — dropped

Branch `refactor/cleanup-web-8` (e057b2c): on a fragment-cache miss `body_html` is read and
`to_plain_text` run once instead of twice; `UserView` (with its signed avatar id) cached per
presenter; `room_name` instead of `room_and_name`; `accounts::Edit` borrows its user lists.
Reviewed: output unchanged. ABBA (`bench/profile perf --freq 0`, each run in `bench/quiet`)
against `refactor/cleanup` at 8559e9c, c=16, mean of eight runs:

| target | CPU ms/req base | web-8 | change | req/s base | web-8 | change | T |
|---|---|---|---|---|---|---|---|
| room_show | 0.1350 | 0.1352 | +0.1% | 28,601 | 28,456 | -0.5% | 3.0% |
| messages_page | 0.1082 | 0.1077 | -0.4% | 35,484 | 35,600 | +0.3% | 3.0% |
| sidebar | 0.1264 | 0.1259 | -0.4% | 30,528 | 30,616 | +0.3% | 3.0% |
| post_message | 0.3477 | 0.3432 | -1.3% | 5,592 | 5,494 | -1.8% | 3.2% |

No target gains more than T (post_message −1.3%, one message rendered per post), so the WP is
dropped as perf; the code isn't merged. The non-perf parts of WEB-8 (SQL moves, duplicates) are
still open.
