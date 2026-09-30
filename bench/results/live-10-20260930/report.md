```
date: 2026-09-30T09:54:24+02:00
host: 7.2.3-arch1-3, AMD Ryzen 9 9950X3D 16-Core Processor, 32 threads, 60GB
server cpus: 8-11 (nproc 4); loadgen cpus: 12-15; network: host
env: WEB_CONCURRENCY=3 JOB_CONCURRENCY=3 RAILS_MAX_THREADS=5 
rust extra env: 
rust-base image: campfire-rust:v0.1.1 sha256:bba76e9ee0be84d204f0b8285a61a77aece7b66873d14d5c977ff2c33121e688 2026-09-29T11:34:38.632554571+02:00
rust image: campfire-rust:tip sha256:94dc5b48db44758ca64f5d2bcba9908d84797591473f41a3cdf71a42ca6623ac 2026-09-30T02:14:11.782024898+02:00
rust-alt image: campfire-rust:live-10 sha256:e77fd2d242cff333ecae64ad39ec4b970401ea92c7d5671c6933c49ae94f82e7 2026-09-30T09:54:03.095009805+02:00
rust HEAD: 2e0dc37 (dirty: 0 files)
```

Reps: rust-base 4, rust 4, rust-alt 4. Cells: median [min–max].

### Startup and memory

| Metric | Rust (base) | Rust | Rust (alt) | Rust adv. | vs base | alt vs base |
|---|---|---|---|---|---|---|
| cold start: docker run → /up 200 (ms) | 425 [402–450] | 446 [409–458] | 432 [360–448] | – | 0.954× | 0.985× |
| idle memory.current (MB) | 18.0 [13.0–42.0] | 13.0 [13.0–25.0] | 13.0 [13.0–14.0] | – | 1.385× | 1.385× |
| idle anon (MB) | 11.0 [11.0–11.0] | 11.0 [11.0–11.0] | 11.0 [11.0–11.0] | – | 1.000× | 1.000× |
| peak memory.current under load (MB) | 866 [783–900] | 892 [813–1,012] | 1,047 [787–1,189] | – | 0.971× | 0.827× |
| peak anon under load (MB) | 257 [253–273] | 238 [234–242] | 251 [246–255] | – | 1.082× | 1.024× |

### HTTP (signed in as david; keep-alive; c = concurrent connections)

| Metric | Rust (base) | Rust | Rust (alt) | Rust adv. | vs base | alt vs base |
|---|---|---|---|---|---|---|
| room_show c=16 req/s | 22,551 [22,416–22,623] | 36,764 [36,421–36,834] | 36,763 [35,967–37,196] | – | 1.630× | 1.630× |
| room_show c=16 p50 ms | 0.68 [0.68–0.68] | 0.42 [0.42–0.43] | 0.42 [0.42–0.43] | – | 1.600× | 1.600× |
| room_show c=16 p99 ms | 1.56 [1.54–1.57] | 0.76 [0.76–0.77] | 0.77 [0.75–0.80] | – | 2.038× | 2.025× |
| messages_page c=16 req/s | 26,287 [26,054–26,683] | 42,208 [42,022–42,282] | 42,367 [42,199–42,857] | – | 1.606× | 1.612× |
| messages_page c=16 p50 ms | 0.57 [0.56–0.57] | 0.37 [0.37–0.37] | 0.37 [0.36–0.37] | – | 1.541× | 1.545× |
| messages_page c=16 p99 ms | 1.43 [1.42–1.49] | 0.63 [0.63–0.64] | 0.63 [0.62–0.63] | – | 2.263× | 2.268× |
| sidebar c=16 req/s | 13,910 [13,782–14,003] | 35,730 [35,517–36,083] | 35,949 [35,691–36,169] | – | 2.569× | 2.584× |
| sidebar c=16 p50 ms | 1.11 [1.11–1.12] | 0.43 [0.43–0.43] | 0.43 [0.43–0.43] | – | 2.573× | 2.576× |
| sidebar c=16 p99 ms | 2.28 [2.24–2.30] | 0.81 [0.79–0.85] | 0.80 [0.79–0.80] | – | 2.825× | 2.850× |
| search c=16 req/s | 26,460 [26,137–26,539] | 32,898 [32,570–33,142] | 33,198 [32,834–33,645] | – | 1.243× | 1.255× |
| search c=16 p50 ms | 0.56 [0.56–0.56] | 0.45 [0.45–0.45] | 0.45 [0.44–0.45] | – | 1.243× | 1.251× |
| search c=16 p99 ms | 1.52 [1.48–1.53] | 1.18 [1.15–1.21] | 1.18 [1.15–1.19] | – | 1.290× | 1.294× |
| avatar c=16 req/s | 355,562 [352,770–359,340] | 354,204 [353,003–355,550] | 353,673 [352,968–356,446] | – | 0.996× | 0.995× |
| avatar c=16 p50 ms | 0.03 [0.03–0.03] | 0.03 [0.03–0.03] | 0.03 [0.03–0.03] | – | 1.000× | 1.000× |
| avatar c=16 p99 ms | 0.13 [0.13–0.13] | 0.13 [0.13–0.13] | 0.13 [0.13–0.13] | – | 1.004× | 1.008× |
| static_css c=16 req/s | 378,812 [377,224–379,426] | 378,601 [377,802–379,195] | 378,138 [377,461–380,954] | – | 0.999× | 0.998× |
| static_css c=16 p50 ms | 0.03 [0.03–0.03] | 0.03 [0.03–0.03] | 0.03 [0.03–0.03] | – | 0.970× | 0.985× |
| static_css c=16 p99 ms | 0.12 [0.12–0.12] | 0.12 [0.12–0.12] | 0.12 [0.12–0.12] | – | 1.000× | 1.000× |
| up c=16 req/s | 149,500 [149,246–149,704] | 225,245 [223,258–230,882] | 228,186 [217,879–230,902] | – | 1.507× | 1.526× |
| up c=16 p50 ms | 0.11 [0.11–0.11] | 0.07 [0.07–0.07] | 0.07 [0.07–0.07] | – | 1.540× | 1.551× |
| up c=16 p99 ms | 0.19 [0.19–0.19] | 0.17 [0.14–0.18] | 0.14 [0.12–0.18] | – | 1.127× | 1.328× |
| post_message c=16 req/s | 4,796 [4,699–4,890] | 5,303 [5,082–5,589] | 5,442 [5,345–5,461] | – | 1.106× | 1.135× |
| post_message c=16 p50 ms | 2.27 [2.24–2.28] | 1.86 [1.84–1.95] | 1.87 [1.85–1.88] | – | 1.224× | 1.218× |
| post_message c=16 p99 ms | 42.1 [41.9–42.3] | 41.6 [38.3–51.7] | 43.7 [41.8–46.0] | – | 1.012× | 0.963× |

### HTTP errors / non-2xx-3xx (first rep, per app)

| Metric | Rust (base) | Rust | Rust (alt) | Rust adv. | vs base | alt vs base |
|---|---|---|---|---|---|---|
- rust-base: none
- rust: none
- rust-alt: none

### Action Cable fan-out (one room; chatter.js subscriptions per client)

| Metric | Rust (base) | Rust | Rust (alt) | Rust adv. | vs base | alt vs base |
|---|---|---|---|---|---|---|
| 1000 clients: subscribed | 1,000 [1,000–1,000] | 1,000 [1,000–1,000] | 1,000 [1,000–1,000] | – | 1.000× | 1.000× |
| 1000 clients: connect+subscribe all (s) | 0.17 [0.13–1.15] | 0.15 [0.14–0.20] | 0.15 [0.14–0.23] | – | 1.207× | 1.207× |
| 1000 clients: paced post→one client p50 ms | 3.46 [3.39–3.57] | 3.46 [3.34–3.61] | 3.43 [3.33–3.55] | – | 1.001× | 1.010× |
| 1000 clients: paced post→all clients p50 ms | 6.34 [6.30–6.40] | 6.34 [6.24–6.59] | 6.21 [6.05–6.43] | – | 1.000× | 1.021× |
| 1000 clients: paced post→all clients p99 ms | 13.4 [12.3–15.5] | 14.0 [12.4–14.3] | 17.0 [14.5–19.9] | – | 0.958× | 0.789× |
| 1000 clients: max sustained msgs/s (delivered to all) | 410 [404–412] | 417 [416–419] | 414 [413–420] | – | 1.018× | 1.010× |
| 1000 clients: deliveries/s (client×message) | 410,088 [404,431–411,864] | 417,462 [415,695–419,156] | 414,192 [412,653–420,422] | – | 1.018× | 1.010× |
| 1000 clients: saturated post→all p50 ms | 20.3 [19.6–20.4] | 19.2 [18.9–19.4] | 19.5 [19.2–19.9] | – | 1.056× | 1.042× |
| 1000 clients: saturated POST p50 ms | 9.23 [9.21–9.28] | 9.32 [9.30–9.34] | 9.44 [9.38–9.46] | – | 0.990× | 0.978× |
| 10000 clients: subscribed | 10,000 [10,000–10,000] | 10,000 [10,000–10,000] | 10,000 [10,000–10,000] | – | 1.000× | 1.000× |
| 10000 clients: connect+subscribe all (s) | 1.69 [1.41–1.89] | 1.49 [1.37–1.68] | 1.85 [0.40–2.44] | – | 1.131× | 0.911× |
| 10000 clients: paced post→one client p50 ms | 24.2 [23.8–24.6] | 24.2 [24.0–24.3] | 24.2 [24.1–24.5] | – | 1.002× | 0.999× |
| 10000 clients: paced post→all clients p50 ms | 53.6 [53.0–54.6] | 54.2 [53.0–56.2] | 54.2 [53.1–56.3] | – | 0.989× | 0.990× |
| 10000 clients: paced post→all clients p99 ms | 83.2 [80.4–85.8] | 85.5 [85.1–86.4] | 84.3 [82.2–84.9] | – | 0.973× | 0.987× |
| 10000 clients: max sustained msgs/s (delivered to all) | 47.9 [47.2–48.7] | 49.7 [48.1–49.9] | 48.2 [47.6–49.1] | – | 1.037× | 1.006× |
| 10000 clients: deliveries/s (client×message) | 478,746 [471,727–487,052] | 496,574 [481,275–499,000] | 482,382 [476,022–490,605] | – | 1.037× | 1.008× |
| 10000 clients: saturated post→all p50 ms | 300 [279–309] | 328 [296–332] | 295 [281–302] | – | 0.915× | 1.018× |
| 10000 clients: saturated POST p50 ms | 39.1 [38.1–41.4] | 35.3 [33.9–36.9] | 38.6 [36.7–44.3] | – | 1.107× | 1.013× |

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
| 1000 clients, all subscribed, idle: app process Pss | 140 [136–144] | 148 [143–152] | 149 [147–152] | – | 0.945× | 0.940× |
| 1000 clients, all subscribed, idle: app process RssAnon | 117 [114–121] | 126 [121–130] | 127 [125–129] | – | 0.931× | 0.926× |
| 1000 clients, all subscribed, idle: app + Redis + Thruster Pss | 140 [136–144] | 148 [143–152] | 149 [147–152] | – | 0.945× | 0.940× |
| 1000 clients, all subscribed, idle: whole container Pss | 140 [136–144] | 148 [143–152] | 149 [147–152] | – | 0.945× | 0.940× |
| 1000 clients, saturated fan-out: app process Pss | 139 [136–142] | 138 [136–141] | 145 [145–146] | – | 1.009× | 0.956× |
| 1000 clients, saturated fan-out: app process RssAnon | 116 [114–120] | 115 [114–118] | 123 [122–124] | – | 1.006× | 0.944× |
| 1000 clients, saturated fan-out: app + Redis + Thruster Pss | 139 [136–142] | 138 [136–141] | 145 [145–146] | – | 1.009× | 0.956× |
| 1000 clients, saturated fan-out: whole container Pss | 139 [136–142] | 138 [136–141] | 145 [145–146] | – | 1.009× | 0.956× |
| 10000 clients, all subscribed, idle: app process Pss | 280 [275–296] | 260 [255–265] | 274 [268–278] | – | 1.077× | 1.022× |
| 10000 clients, all subscribed, idle: app process RssAnon | 257 [253–273] | 237 [233–242] | 251 [246–256] | – | 1.082× | 1.023× |
| 10000 clients, all subscribed, idle: app + Redis + Thruster Pss | 280 [275–296] | 260 [255–265] | 274 [268–278] | – | 1.077× | 1.022× |
| 10000 clients, all subscribed, idle: whole container Pss | 280 [275–296] | 260 [255–265] | 274 [268–278] | – | 1.077× | 1.022× |
| 10000 clients, saturated fan-out: app process Pss | 267 [265–274] | 258 [255–261] | 266 [265–272] | – | 1.036× | 1.004× |
| 10000 clients, saturated fan-out: app process RssAnon | 244 [242–251] | 236 [233–239] | 244 [243–249] | – | 1.037× | 1.002× |
| 10000 clients, saturated fan-out: app + Redis + Thruster Pss | 267 [265–274] | 258 [255–261] | 266 [265–272] | – | 1.036× | 1.004× |
| 10000 clients, saturated fan-out: whole container Pss | 267 [265–274] | 258 [255–261] | 266 [265–272] | – | 1.036× | 1.004× |
