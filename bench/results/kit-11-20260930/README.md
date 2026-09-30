# KIT-11: pages from parts (2026-09-30)

Page renders record where each cached message fragment goes (a thread-local hand-off from
`cached_message_item` to the `campfire_views::recorded` writer, matched by pointer and length) so
the response carries the page as text spans plus fragment `Arc`s: no copying fragments into one
string, no searching the body for them in `PageParts::new`, gzip straight from stored pieces, and
identity responses streamed in parts. From the tip's profile (`../profile-tip-20260930/`: render
memcpy 5-6%, fragment search ~3%). Output bytes, Content-Length and ETags unchanged (checked on 80
responses against 95aca44; pinned in tests).

ABBA (`bench/profile perf --freq 0`, each run in `bench/quiet`) against `refactor/cleanup` at
95aca44, c=16, with `room_show_identity` (no gzip) added because identity bodies now go out in
parts. Mean of eight runs (`time-*`, `time2-*`, `time.log`):

| target | CPU ms/req base | kit-11 | change | req/s base | kit-11 | change | T |
|---|---|---|---|---|---|---|---|
| room_show | 0.1205 | 0.1040 | -13.7% | 31,905 | 36,672 | +14.9% | 3.0% |
| messages_page | 0.1063 | 0.0892 | -16.0% | 36,048 | 42,480 | +17.8% | 3.0% |
| sidebar | 0.1065 | 0.1073 | +0.8% | 35,941 | 35,610 | -0.9% | 3.0% |
| post_message | 0.3540 | 0.3545 | +0.1% | 5,593 | 5,463 | -2.3% | 3.2% |
| room_show_identity | 0.1613 | 0.1484 | -8.0% | 22,662 | 24,222 | +6.9% | 3.0% |

Microbenchmark (`splice::parts_vs_search`, release): join + search + split + ETag + gzip
24.7-25.1 µs, recorded parts 8.6-8.7 µs.

Parity on the branch merged with 1d30809: 874 cells, 873 pass, 0 fail, 1 allowed.
