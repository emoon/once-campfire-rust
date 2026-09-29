# KIT-9c: page text known by its bytes, not re-hashed (2026-09-30)

`PageParts::new` BLAKE3-hashed every text part (~55 KB per room page) on every request: 7.8% of
room_show in the tip's profile (`bench/results/profile-tip-20260930/`). Now a text is looked up
by a 64-bit foldhash, confirmed by a byte comparison with the stored copy, and its stored BLAKE3
digest reused; only new text is BLAKE3-hashed. ETags and gzip bytes are unchanged. The stored
texts are a separate cache bounded at 16 MB (text plus entry overhead). ABBA (`bench/profile perf
--freq 0`, each run in `bench/quiet`) against `refactor/cleanup` at d421663, c=16, mean of eight
runs (`time-*`, `time2-*`, `time.log`):

| target | CPU ms/req base | kit-9c | change | req/s base | kit-9c | change | T |
|---|---|---|---|---|---|---|---|
| room_show | 0.1292 | 0.1232 | -4.6% | 29,759 | 31,228 | +4.9% | 3.0% |
| messages_page | 0.1093 | 0.1091 | -0.1% | 35,114 | 35,118 | +0.0% | 3.0% |
| sidebar | 0.1106 | 0.1109 | +0.3% | 34,667 | 34,606 | -0.2% | 3.0% |
| post_message | 0.3468 | 0.3493 | +0.7% | 5,355 | 5,338 | -0.3% | 3.2% |

Microbenchmark (one pinned core, warm): 55 KB 1.66 µs against 6.95 µs for BLAKE3.
