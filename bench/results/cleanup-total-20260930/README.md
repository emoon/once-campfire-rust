# Cleanup so far: start vs tip (2026-09-30)

`cfc9144` (the cleanup's start of 2026-09-29) against `efea14c` (DB-11, DB-12, KIT-10, batch 1,
VIEW-3, KIT-9, VIEW-11, VIEW-10, VIEW-2 merged), in one session: ABBA (`bench/profile perf
--freq 0`, each run in `bench/quiet`), c=16, mean of two runs a side. "base" is the start.

| target | CPU ms/req base | tip | change | req/s base | tip | change | T |
|---|---|---|---|---|---|---|---|
| room_show | 0.1667 | 0.1285 | -22.9% | 22,047 | 29,965 | +35.9% | 3.0% |
| messages_page | 0.1409 | 0.1095 | -22.3% | 25,995 | 34,995 | +34.6% | 3.0% |
| sidebar | 0.2692 | 0.1104 | -59.0% | 13,810 | 34,716 | +151.4% | 3.0% |
| post_message | 0.4532 | 0.3448 | -23.9% | 4,744 | 5,595 | +17.9% | 3.2% |

The sidebar target repeats one body, so KIT-10's gzip reuse hits on every request; real sidebars
gain less. The previous snapshot is `bench/results/cleanup-total-20260929/`.
