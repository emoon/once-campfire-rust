| run | target | CPU ms/req | req/s | user ms | sys ms | sys share | vol cs | invol cs |
|---|---|---|---|---|---|---|---|---|
| time-base-1 | room_show | 0.1787 | 19,870 | 0.1477 | 0.0267 | 15% | 7.2 | 3.4 |
| time-base-1 | messages_page | 0.1447 | 25,291 | 0.1160 | 0.0287 | 20% | 8.6 | 3.0 |
| time-base-1 | sidebar | 0.2651 | 14,063 | 0.2367 | 0.0290 | 11% | 6.6 | 3.0 |
| time-base-1 | post_message | 0.4520 | 4,725 | 0.2943 | 0.1583 | 35% | 24.5 | 7.1 |
| time-base-1 | cable1000 (per message, 383.4 msg/s) | 5.219 | 383.4 | 1.540 | 3.679 | 70% | 40 | 9 |
| time-fp-1 | room_show | 0.1649 | 22,352 | 0.1357 | 0.0250 | 16% | 7.2 | 3.4 |
| time-fp-1 | messages_page | 0.1405 | 26,277 | 0.1121 | 0.0277 | 20% | 8.6 | 3.0 |
| time-fp-1 | sidebar | 0.2674 | 14,021 | 0.2375 | 0.0298 | 11% | 6.6 | 3.0 |
| time-fp-1 | post_message | 0.4520 | 4,782 | 0.2927 | 0.1575 | 35% | 24.6 | 7.1 |
| time-fp-1 | cable1000 (per message, 395.5 msg/s) | 5.136 | 395.5 | 1.541 | 3.596 | 70% | 40 | 9 |
| time-fp-2 | room_show | 0.1661 | 22,062 | 0.1357 | 0.0257 | 16% | 7.2 | 3.4 |
| time-fp-2 | messages_page | 0.1432 | 25,498 | 0.1136 | 0.0288 | 20% | 8.6 | 2.9 |
| time-fp-2 | sidebar | 0.2746 | 13,423 | 0.2447 | 0.0298 | 11% | 6.6 | 2.9 |
| time-fp-2 | post_message | 0.4504 | 4,653 | 0.2881 | 0.1616 | 36% | 24.5 | 7.1 |
| time-fp-2 | cable1000 (per message, 410.7 msg/s) | 4.863 | 410.7 | 1.415 | 3.447 | 71% | 40 | 9 |
| time-base-2 | room_show | 0.1666 | 21,967 | 0.1378 | 0.0248 | 15% | 7.2 | 3.3 |
| time-base-2 | messages_page | 0.1389 | 26,327 | 0.1106 | 0.0276 | 20% | 8.6 | 3.0 |
| time-base-2 | sidebar | 0.2687 | 13,830 | 0.2391 | 0.0290 | 11% | 6.5 | 3.0 |
| time-base-2 | post_message | 0.4524 | 4,954 | 0.2954 | 0.1568 | 35% | 24.5 | 7.1 |
| time-base-2 | cable1000 (per message, 409.9 msg/s) | 4.891 | 409.9 | 1.460 | 3.431 | 70% | 40 | 9 |
| perf-fp | room_show | 0.1731 | 21,395 | 0.1394 | 0.0302 | 18% | 7.2 | 3.3 |
| perf-fp | messages_page | 0.1487 | 24,867 | 0.1137 | 0.0342 | 23% | 8.5 | 3.0 |
| perf-fp | sidebar | 0.2761 | 13,563 | 0.2425 | 0.0343 | 12% | 6.5 | 3.0 |
| perf-fp | post_message | 0.5378 | 4,148 | 0.3272 | 0.2112 | 39% | 24.3 | 7.1 |
| perf-fp | cable1000 (per message, 363.1 msg/s) | 5.664 | 363.1 | 1.671 | 3.993 | 71% | 39 | 9 |
| gperf | room_show | 0.1714 | 21,585 |  | | | |  |
| gperf | messages_page | 0.1429 | 25,687 |  | | | |  |
| gperf | sidebar | 0.2797 | 13,373 |  | | | |  |
| gperf | post_message | 0.4702 | 4,723 |  | | | |  |
| gperf | cable1000 | | 383.0 msg/s | | | | | |
