# KIT-10: reuse gzip output for repeated bodies (2026-09-29)

ABBA (`bench/profile perf --freq 0`, each run in `bench/quiet`) against `refactor/cleanup` at
4b3b460 (DB-11 and DB-12 merged), c=16. Two passes (`time-*`, `time2-*`; `time.log`), because
messages_page's first pass landed in the re-run band (+1.8% CPU). Mean of all eight runs:

| target | CPU ms/req base | kit-10 | change | req/s base | kit-10 | change | T |
|---|---|---|---|---|---|---|---|
| room_show | 0.1550 | 0.1558 | +0.5% | 25,012 | 24,877 | -0.5% | 3.0% |
| messages_page | 0.1299 | 0.1312 | +1.0% | 29,707 | 29,448 | -0.9% | 3.0% |
| sidebar | 0.2528 | 0.1301 | -48.5% | 15,506 | 29,625 | +91.1% | 3.0% |
| post_message | 0.3596 | 0.3616 | +0.6% | 5,383 | 5,457 | +1.4% | 3.2% |

The sidebar target requests the same sidebar repeatedly, so every body after the first is a
cache hit; real sidebars differ per user and change with unread state, so they gain less.
`miss/` is the miss-path microbenchmark (+0.4 µs of 108), `run-*` and `*-room_show`/`*-messages_page`
are earlier runs from the WP agent.
