# Cleanup so far: start vs tip after KIT-11 (2026-09-30)

`cfc9144` (the cleanup's start) against `bed9289` (adds DB-13, KIT-9c, KIT-11 since
`../cleanup-total-20260930/`), one session: ABBA (`bench/profile perf --freq 0`, each run in
`bench/quiet`), c=16, mean of two runs a side. "base" is the start.

| target | CPU ms/req base | tip | change | req/s base | tip | change | T |
|---|---|---|---|---|---|---|---|
| room_show | 0.1662 | 0.1036 | -37.7% | 22,290 | 36,792 | +65.1% | 3.0% |
| messages_page | 0.1408 | 0.0890 | -36.8% | 25,867 | 42,710 | +65.1% | 3.0% |
| sidebar | 0.2685 | 0.1066 | -60.3% | 13,847 | 35,824 | +158.7% | 3.0% |
| post_message | 0.4529 | 0.3566 | -21.3% | 4,846 | 5,337 | +10.1% | 3.2% |

The sidebar target repeats one body (KIT-10's gzip reuse hits every time); real sidebars gain less.
