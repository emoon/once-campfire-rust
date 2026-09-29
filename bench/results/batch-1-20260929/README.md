# Batch 1: F-2, F-3, KIT-1, VIEW-1, WEB-3 (2026-09-29)

Five approved cleanups merged together and timed once for no regression. ABBA (`bench/profile
perf --freq 0`, each run in `bench/quiet`) against `refactor/cleanup` at 857121c, c=16, mean of two
runs a side:

| target | CPU ms/req base | batch-1 | change | req/s base | batch-1 | change | T |
|---|---|---|---|---|---|---|---|
| room_show | 0.1546 | 0.1533 | -0.8% | 24,860 | 25,289 | +1.7% | 3.0% |
| messages_page | 0.1279 | 0.1271 | -0.6% | 30,239 | 30,443 | +0.7% | 3.0% |
| sidebar | 0.1289 | 0.1279 | -0.8% | 29,896 | 30,173 | +0.9% | 3.0% |
| post_message | 0.3584 | 0.3539 | -1.3% | 5,494 | 5,478 | -0.3% | 3.2% |

Every target is within T/2. Tests 673 pass with the seed; clippy clean. F-2, F-3 and VIEW-1 each
passed parity on their own (873/874).
