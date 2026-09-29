# LIVE-2: encode per-member broadcasts once (2026-09-29) — dropped

Branch `refactor/cleanup-live-2` (9607c65): per-member cable broadcasts encoded once
(`Server::encode_action`, `broadcast_encoded`), and `broadcast_create` reads the member ids in its
database read and broadcasts after the read returns (Rails' order; reviewed: wire bytes identical).
ABBA (`bench/profile perf --freq 0`, each run in `bench/quiet`) against `refactor/cleanup` at
8559e9c, c=16; the first pass landed in the re-run band, so this is the mean of eight runs:

| target | CPU ms/req base | live-2 | change | req/s base | live-2 | change | T |
|---|---|---|---|---|---|---|---|
| room_show | 0.1359 | 0.1350 | -0.6% | 28,307 | 28,522 | +0.8% | 3.0% |
| messages_page | 0.1081 | 0.1081 | -0.0% | 35,531 | 35,474 | -0.2% | 3.0% |
| sidebar | 0.1263 | 0.1266 | +0.2% | 30,572 | 30,433 | -0.5% | 3.0% |
| post_message | 0.3499 | 0.3432 | -1.9% | 5,562 | 5,342 | -4.0% | 3.2% |

post_message's req/s is worse than T (−4.0% against 3.2%) although its CPU/req fell 1.9%, so the
WP fails the gate; not merged. Not isolated: whether the throughput loss comes from moving the
broadcasts after the read (more latency per post at c=16) or from the encoding change. A retry
could time the two apart, and should add the cable fan-out suite, where encoding once per member
matters most (the seed's rooms are small).
