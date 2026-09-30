```
date: 2026-09-30T11:04:52+02:00
host: 7.2.3-arch1-3, AMD Ryzen 9 9950X3D 16-Core Processor, 32 threads, 60GB
server cpus: 8-11 (nproc 4); loadgen cpus: 12-15; network: host
env: WEB_CONCURRENCY=3 JOB_CONCURRENCY=3 RAILS_MAX_THREADS=5 
rust extra env: 
rust-base image: campfire-rust:v0.1.1 sha256:bba76e9ee0be84d204f0b8285a61a77aece7b66873d14d5c977ff2c33121e688 2026-09-29T11:34:38.632554571+02:00
rust image: campfire-rust:live-9 sha256:f4f39763092193a68bdd43532a5f05e63646498da3a1ed0fb362c54da9a835a0 2026-09-30T11:04:29.721305905+02:00
rust-alt image: campfire-rust:tip sha256:94dc5b48db44758ca64f5d2bcba9908d84797591473f41a3cdf71a42ca6623ac 2026-09-30T02:14:11.782024898+02:00
rust HEAD: 1fe6d71 (dirty: 0 files)
```

Reps: rust-base 3, rust 3, rust-alt 3. Cells: median [min–max].

### Startup and memory

| Metric | Rust (base) | Rust | Rust (alt) | Rust adv. | vs base | alt vs base |
|---|---|---|---|---|---|---|
| cold start: docker run → /up 200 (ms) | 440 [427–523] | 403 [392–415] | 423 [405–483] | – | 1.092× | 1.040× |
| idle memory.current (MB) | 14.0 [14.0–14.0] | 13.0 [13.0–14.0] | 14.0 [14.0–14.0] | – | 1.077× | 1.000× |
| idle anon (MB) | 11.0 [11.0–11.0] | 11.0 [11.0–11.0] | 11.0 [11.0–11.0] | – | 1.000× | 1.000× |
| peak memory.current under load (MB) | 857 [832–919] | 928 [790–941] | 940 [868–998] | – | 0.923× | 0.912× |
| peak anon under load (MB) | 252 [249–262] | 252 [249–260] | 245 [240–247] | – | 1.000× | 1.029× |

### HTTP (signed in as david; keep-alive; c = concurrent connections)

| Metric | Rust (base) | Rust | Rust (alt) | Rust adv. | vs base | alt vs base |
|---|---|---|---|---|---|---|
| room_show c=16 req/s | 22,662 [22,533–22,838] | 36,927 [36,724–37,018] | 36,554 [36,543–36,925] | – | 1.629× | 1.613× |
| room_show c=16 p50 ms | 0.67 [0.67–0.68] | 0.42 [0.42–0.42] | 0.42 [0.42–0.42] | – | 1.599× | 1.584× |
| room_show c=16 p99 ms | 1.52 [1.50–1.54] | 0.76 [0.76–0.77] | 0.77 [0.76–0.77] | – | 2.011× | 1.982× |
| messages_page c=16 req/s | 26,624 [26,568–26,652] | 42,571 [41,944–42,726] | 42,352 [42,332–42,878] | – | 1.599× | 1.591× |
| messages_page c=16 p50 ms | 0.56 [0.56–0.56] | 0.37 [0.37–0.37] | 0.37 [0.36–0.37] | – | 1.534× | 1.526× |
| messages_page c=16 p99 ms | 1.44 [1.40–1.44] | 0.62 [0.62–0.64] | 0.63 [0.62–0.63] | – | 2.304× | 2.293× |
| sidebar c=16 req/s | 14,045 [13,964–14,086] | 35,958 [35,654–36,183] | 35,995 [35,732–36,170] | – | 2.560× | 2.563× |
| sidebar c=16 p50 ms | 1.10 [1.10–1.11] | 0.43 [0.43–0.43] | 0.43 [0.43–0.43] | – | 2.557× | 2.557× |
| sidebar c=16 p99 ms | 2.26 [2.23–2.26] | 0.80 [0.79–0.80] | 0.80 [0.79–0.80] | – | 2.823× | 2.844× |
| search c=16 req/s | 26,842 [26,441–26,874] | 33,167 [33,078–33,314] | 32,739 [32,492–33,468] | – | 1.236× | 1.220× |
| search c=16 p50 ms | 0.55 [0.55–0.56] | 0.45 [0.45–0.45] | 0.45 [0.44–0.46] | – | 1.233× | 1.214× |
| search c=16 p99 ms | 1.49 [1.48–1.50] | 1.17 [1.16–1.18] | 1.18 [1.15–1.19] | – | 1.270× | 1.259× |
| avatar c=16 req/s | 355,414 [355,195–356,847] | 354,891 [354,635–356,790] | 354,018 [352,652–355,436] | – | 0.999× | 0.996× |
| avatar c=16 p50 ms | 0.03 [0.03–0.03] | 0.03 [0.03–0.03] | 0.03 [0.03–0.03] | – | 1.000× | 1.000× |
| avatar c=16 p99 ms | 0.13 [0.13–0.13] | 0.13 [0.13–0.13] | 0.13 [0.13–0.13] | – | 1.015× | 1.023× |
| static_css c=16 req/s | 380,839 [379,083–385,383] | 381,523 [379,602–384,680] | 380,687 [375,166–380,916] | – | 1.002× | 1.000× |
| static_css c=16 p50 ms | 0.03 [0.03–0.03] | 0.03 [0.03–0.03] | 0.03 [0.03–0.03] | – | 1.000× | 0.970× |
| static_css c=16 p99 ms | 0.12 [0.12–0.12] | 0.12 [0.12–0.12] | 0.12 [0.12–0.12] | – | 0.984× | 0.992× |
| up c=16 req/s | 150,323 [148,803–151,505] | 231,802 [231,252–232,176] | 230,215 [227,660–231,764] | – | 1.542× | 1.531× |
| up c=16 p50 ms | 0.11 [0.11–0.11] | 0.07 [0.07–0.07] | 0.07 [0.07–0.07] | – | 1.574× | 1.574× |
| up c=16 p99 ms | 0.19 [0.19–0.19] | 0.15 [0.13–0.16] | 0.14 [0.13–0.16] | – | 1.268× | 1.370× |
| post_message c=16 req/s | 4,892 [4,813–4,932] | 5,491 [5,461–5,594] | 5,424 [5,329–5,520] | – | 1.122× | 1.109× |
| post_message c=16 p50 ms | 2.27 [2.24–2.27] | 1.87 [1.87–1.87] | 1.88 [1.86–1.88] | – | 1.216× | 1.213× |
| post_message c=16 p99 ms | 41.2 [41.2–45.0] | 40.0 [38.8–40.8] | 40.3 [38.1–44.9] | – | 1.030× | 1.024× |

### HTTP errors / non-2xx-3xx (first rep, per app)

| Metric | Rust (base) | Rust | Rust (alt) | Rust adv. | vs base | alt vs base |
|---|---|---|---|---|---|---|
- rust-base: none
- rust: none
- rust-alt: none

### Action Cable fan-out (one room; chatter.js subscriptions per client)

| Metric | Rust (base) | Rust | Rust (alt) | Rust adv. | vs base | alt vs base |
|---|---|---|---|---|---|---|
| 100 clients: subscribed | 100 [100–100] | 100 [100–100] | 100 [100–100] | – | 1.000× | 1.000× |
| 100 clients: connect+subscribe all (s) | 0.06 [0.06–0.06] | 0.06 [0.06–0.06] | 0.07 [0.07–0.07] | – | 1.000× | 0.857× |
| 100 clients: paced post→one client p50 ms | 1.13 [1.12–1.13] | 1.06 [1.05–1.06] | 1.07 [1.07–1.07] | – | 1.062× | 1.058× |
| 100 clients: paced post→all clients p50 ms | 1.37 [1.36–1.38] | 1.31 [1.30–1.31] | 1.31 [1.30–1.33] | – | 1.045× | 1.040× |
| 100 clients: paced post→all clients p99 ms | 2.16 [2.01–8.65] | 8.97 [8.91–9.49] | 8.34 [8.34–12.40] | – | 0.241× | 0.259× |
| 100 clients: max sustained msgs/s (delivered to all) | 2,718 [2,687–2,755] | 3,402 [3,373–3,409] | 3,375 [3,314–3,435] | – | 1.252× | 1.242× |
| 100 clients: deliveries/s (client×message) | 271,748 [268,710–275,493] | 340,245 [337,329–340,943] | 337,467 [331,376–343,502] | – | 1.252× | 1.242× |
| 100 clients: saturated post→all p50 ms | 1.13 [1.12–1.13] | 1.10 [1.09–1.10] | 1.12 [1.12–1.12] | – | 1.023× | 1.002× |
| 100 clients: saturated POST p50 ms | 1.08 [1.08–1.09] | 0.79 [0.79–0.79] | 0.78 [0.78–0.80] | – | 1.368× | 1.382× |
| 1000 clients: subscribed | 1,000 [1,000–1,000] | 1,000 [1,000–1,000] | 1,000 [1,000–1,000] | – | 1.000× | 1.000× |
| 1000 clients: connect+subscribe all (s) | 1.15 [0.15–1.34] | 0.18 [0.13–0.19] | 0.13 [0.13–0.13] | – | 6.389× | 8.846× |
| 1000 clients: paced post→one client p50 ms | 3.55 [3.47–3.60] | 3.44 [3.33–3.46] | 3.44 [3.33–3.51] | – | 1.032× | 1.033× |
| 1000 clients: paced post→all clients p50 ms | 6.31 [6.31–6.54] | 6.19 [5.90–6.25] | 6.28 [6.15–6.46] | – | 1.020× | 1.004× |
| 1000 clients: paced post→all clients p99 ms | 12.8 [11.4–13.7] | 13.9 [10.6–14.2] | 13.5 [13.4–15.5] | – | 0.922× | 0.948× |
| 1000 clients: max sustained msgs/s (delivered to all) | 412 [409–413] | 422 [419–423] | 415 [414–418] | – | 1.025× | 1.008× |
| 1000 clients: deliveries/s (client×message) | 412,124 [408,874–413,250] | 422,380 [418,688–422,817] | 415,323 [413,741–418,538] | – | 1.025× | 1.008× |
| 1000 clients: saturated post→all p50 ms | 19.9 [19.7–20.0] | 19.3 [19.2–19.4] | 19.1 [19.0–19.5] | – | 1.030× | 1.042× |
| 1000 clients: saturated POST p50 ms | 9.33 [9.30–9.36] | 9.22 [9.14–9.23] | 9.38 [9.29–9.46] | – | 1.011× | 0.995× |
| 5000 clients: subscribed | 5,000 [5,000–5,000] | 5,000 [5,000–5,000] | 5,000 [5,000–5,000] | – | 1.000× | 1.000× |
| 5000 clients: connect+subscribe all (s) | 1.41 [1.17–1.69] | 0.41 [0.29–1.33] | 1.20 [0.65–1.34] | – | 3.439× | 1.175× |
| 5000 clients: paced post→one client p50 ms | 13.0 [12.8–13.1] | 12.6 [12.6–12.7] | 12.7 [12.6–12.8] | – | 1.029× | 1.025× |
| 5000 clients: paced post→all clients p50 ms | 27.9 [26.5–28.2] | 27.5 [27.2–27.6] | 27.7 [27.2–28.3] | – | 1.014× | 1.005× |
| 5000 clients: paced post→all clients p99 ms | 44.2 [44.1–45.4] | 44.2 [42.9–45.6] | 47.7 [44.3–47.8] | – | 1.000× | 0.926× |
| 5000 clients: max sustained msgs/s (delivered to all) | 87.9 [86.0–88.0] | 90.1 [88.7–90.6] | 89.5 [89.0–90.2] | – | 1.025× | 1.018× |
| 5000 clients: deliveries/s (client×message) | 439,354 [430,197–439,945] | 450,421 [443,734–452,803] | 447,512 [444,826–451,060] | – | 1.025× | 1.019× |
| 5000 clients: saturated post→all p50 ms | 143 [142–143] | 142 [139–144] | 146 [144–149] | – | 1.006× | 0.974× |
| 5000 clients: saturated POST p50 ms | 28.0 [27.6–28.4] | 24.6 [24.3–25.8] | 26.0 [24.7–26.0] | – | 1.138× | 1.079× |
| 10000 clients: subscribed | 10,000 [10,000–10,000] | 10,000 [10,000–10,000] | 10,000 [10,000–10,000] | – | 1.000× | 1.000× |
| 10000 clients: connect+subscribe all (s) | 1.43 [1.25–1.84] | 1.51 [1.39–1.71] | 1.37 [0.42–1.41] | – | 0.947× | 1.044× |
| 10000 clients: paced post→one client p50 ms | 24.7 [24.6–25.0] | 24.3 [24.2–24.5] | 24.2 [24.1–24.4] | – | 1.017× | 1.020× |
| 10000 clients: paced post→all clients p50 ms | 55.9 [55.2–56.3] | 54.3 [53.6–54.4] | 55.6 [53.4–55.8] | – | 1.029× | 1.005× |
| 10000 clients: paced post→all clients p99 ms | 83.6 [79.4–87.6] | 83.6 [80.4–87.4] | 81.3 [79.6–83.2] | – | 1.000× | 1.028× |
| 10000 clients: max sustained msgs/s (delivered to all) | 47.5 [46.0–49.7] | 48.5 [46.6–49.4] | 49.7 [47.9–50.7] | – | 1.021× | 1.046× |
| 10000 clients: deliveries/s (client×message) | 474,565 [460,257–496,934] | 485,108 [466,275–493,513] | 497,371 [479,253–506,930] | – | 1.022× | 1.048× |
| 10000 clients: saturated post→all p50 ms | 302 [288–323] | 303 [285–332] | 305 [299–335] | – | 0.997× | 0.990× |
| 10000 clients: saturated POST p50 ms | 38.7 [37.0–40.1] | 35.1 [32.2–35.4] | 34.6 [34.5–35.0] | – | 1.102× | 1.119× |

### Upload + thumbnail (black_hole.jpg, 505 KB)

| Metric | Rust (base) | Rust | Rust (alt) | Rust adv. | vs base | alt vs base |
|---|---|---|---|---|---|---|
| POST with attachment (ms) | – | – | – | – | – | – |
| then GET thumb → 200 (ms) | – | – | – | – | – | – |
| POST → thumbnail served (ms) | – | – | – | – | – | – |

### Memory during cable fan-out, by process (MB, peak within the phase)

App process: Rails' Puma master and workers (Action Cable runs in them), or Rust's one campfire
process (its front server included). Pss counts pages shared between forked workers once;
RssAnon counts them in every process.

| Metric | Rust (base) | Rust | Rust (alt) | Rust adv. | vs base | alt vs base |
|---|---|---|---|---|---|---|
| 100 clients, all subscribed, idle: app process Pss | 123 [122–129] | 139 [136–142] | 140 [136–143] | – | 0.884× | 0.878× |
| 100 clients, all subscribed, idle: app process RssAnon | 99.9 [99.2–106.2] | 116 [113–119] | 117 [114–120] | – | 0.864× | 0.853× |
| 100 clients, all subscribed, idle: app + Redis + Thruster Pss | 123 [122–129] | 139 [136–142] | 140 [136–143] | – | 0.884× | 0.878× |
| 100 clients, all subscribed, idle: whole container Pss | 123 [122–129] | 139 [136–142] | 140 [136–143] | – | 0.884× | 0.878× |
| 100 clients, saturated fan-out: app process Pss | 130 [128–137] | 135 [131–138] | 135 [134–138] | – | 0.965× | 0.962× |
| 100 clients, saturated fan-out: app process RssAnon | 107 [105–114] | 112 [108–116] | 113 [112–116] | – | 0.962× | 0.954× |
| 100 clients, saturated fan-out: app + Redis + Thruster Pss | 130 [128–137] | 135 [131–138] | 135 [134–138] | – | 0.965× | 0.962× |
| 100 clients, saturated fan-out: whole container Pss | 130 [128–137] | 135 [131–138] | 135 [134–138] | – | 0.965× | 0.962× |
| 1000 clients, all subscribed, idle: app process Pss | 146 [145–154] | 142 [141–145] | 147 [146–148] | – | 1.021× | 0.990× |
| 1000 clients, all subscribed, idle: app process RssAnon | 121 [121–130] | 120 [118–123] | 124 [124–125] | – | 1.015× | 0.975× |
| 1000 clients, all subscribed, idle: app + Redis + Thruster Pss | 146 [145–154] | 142 [141–145] | 147 [146–148] | – | 1.021× | 0.990× |
| 1000 clients, all subscribed, idle: whole container Pss | 146 [145–154] | 142 [141–145] | 147 [146–148] | – | 1.021× | 0.990× |
| 1000 clients, saturated fan-out: app process Pss | 138 [137–144] | 141 [135–144] | 140 [140–143] | – | 0.974× | 0.981× |
| 1000 clients, saturated fan-out: app process RssAnon | 113 [113–120] | 118 [112–122] | 118 [117–120] | – | 0.958× | 0.956× |
| 1000 clients, saturated fan-out: app + Redis + Thruster Pss | 138 [137–144] | 141 [135–144] | 140 [140–143] | – | 0.974× | 0.981× |
| 1000 clients, saturated fan-out: whole container Pss | 138 [137–144] | 141 [135–144] | 140 [140–143] | – | 0.974× | 0.981× |
| 5000 clients, all subscribed, idle: app process Pss | 204 [203–208] | 203 [198–208] | 197 [194–201] | – | 1.005× | 1.033× |
| 5000 clients, all subscribed, idle: app process RssAnon | 179 [179–183] | 180 [175–186] | 175 [172–179] | – | 0.998× | 1.027× |
| 5000 clients, all subscribed, idle: app + Redis + Thruster Pss | 204 [203–208] | 203 [198–208] | 197 [194–201] | – | 1.005× | 1.033× |
| 5000 clients, all subscribed, idle: whole container Pss | 204 [203–208] | 203 [198–208] | 197 [194–201] | – | 1.005× | 1.033× |
| 5000 clients, saturated fan-out: app process Pss | 196 [194–197] | 198 [189–198] | 191 [189–193] | – | 0.988× | 1.024× |
| 5000 clients, saturated fan-out: app process RssAnon | 171 [170–172] | 175 [166–175] | 169 [167–170] | – | 0.978× | 1.011× |
| 5000 clients, saturated fan-out: app + Redis + Thruster Pss | 196 [194–197] | 198 [189–198] | 191 [189–193] | – | 0.988× | 1.024× |
| 5000 clients, saturated fan-out: whole container Pss | 196 [194–197] | 198 [189–198] | 191 [189–193] | – | 0.988× | 1.024× |
| 10000 clients, all subscribed, idle: app process Pss | 277 [272–286] | 275 [272–283] | 268 [263–270] | – | 1.006× | 1.031× |
| 10000 clients, all subscribed, idle: app process RssAnon | 252 [248–262] | 252 [249–260] | 246 [240–248] | – | 1.001× | 1.026× |
| 10000 clients, all subscribed, idle: app + Redis + Thruster Pss | 277 [272–286] | 275 [272–283] | 268 [263–270] | – | 1.006× | 1.031× |
| 10000 clients, all subscribed, idle: whole container Pss | 277 [272–286] | 275 [272–283] | 268 [263–270] | – | 1.006× | 1.031× |
| 10000 clients, saturated fan-out: app process Pss | 268 [265–272] | 269 [263–272] | 261 [257–263] | – | 0.999× | 1.026× |
| 10000 clients, saturated fan-out: app process RssAnon | 244 [241–248] | 246 [240–249] | 240 [235–241] | – | 0.994× | 1.019× |
| 10000 clients, saturated fan-out: app + Redis + Thruster Pss | 268 [265–272] | 269 [263–272] | 261 [257–263] | – | 0.999× | 1.026× |
| 10000 clients, saturated fan-out: whole container Pss | 268 [265–272] | 269 [263–272] | 261 [257–263] | – | 0.999× | 1.026× |
