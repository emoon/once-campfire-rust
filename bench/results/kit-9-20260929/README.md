# KIT-9: splice without rehashing (2026-09-29)

Text parts, fragment digests and the page ETag use BLAKE3 instead of SHA-256; the gzip CRC is
combined from each stored piece's CRC (zlib's `crc32_combine_op`: one GF(2) multiply per part)
instead of CRC-ing the whole body. Output bytes are unchanged (`the_same_bytes_as_before_kit_9`);
ETags on room, messages and search pages change once. Recording fragment offsets (S-8's third
part, ~6%) was split out as KIT-9b.

ABBA (`bench/profile perf --freq 0`, each run in `bench/quiet`) against `refactor/cleanup` at
907aa89, c=16. The first pass landed in the re-run band; mean of all eight runs (`time-*`,
`time2-*`, `time.log`):

| target | CPU ms/req base | kit-9 | change | req/s base | kit-9 | change | T |
|---|---|---|---|---|---|---|---|
| room_show | 0.1528 | 0.1478 | -3.3% | 25,285 | 25,997 | +2.8% | 3.0% |
| messages_page | 0.1270 | 0.1228 | -3.3% | 30,427 | 31,451 | +3.4% | 3.0% |
| sidebar | 0.1275 | 0.1281 | +0.5% | 30,268 | 30,117 | -0.5% | 3.0% |
| post_message | 0.3539 | 0.3518 | -0.6% | 5,083 | 5,341 | +5.1% | 3.2% |

room_show and messages_page gain more than T. post_message's req/s moves with its bimodal base;
its CPU/req is within noise. The WP agent's single-core microbenchmark: 37 KB + 18 KB of layout
hashed in 7.2 µs instead of 20.6; 82 CRCs combined in 1.3 µs, against 5.5 µs to CRC the 466 KB
body (and 11.9 µs with `crc32fast::Hasher::combine`).
