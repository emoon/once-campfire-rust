```
date: 2026-09-29T13:47:11+02:00
host: 7.2.3-arch1-3, AMD Ryzen 9 9950X3D 16-Core Processor, 32 threads, 60GB
server cpus: 8-11 (nproc 4); loadgen cpus: 12-15; network: host
env: WEB_CONCURRENCY=3 JOB_CONCURRENCY=3 RAILS_MAX_THREADS=5 
rust extra env: 
rust image: campfire-rust:perf-1 sha256:645c0efeaa03ac38ed0b12718b20f5f79a38fa50edb37d557d470e7c307625f4 2026-09-29T13:46:57.797791654+02:00
rust HEAD: a49dc9b (dirty: 0 files)
```

Reps: rust 3. Cells: median [min–max].

### Startup and memory

| Metric | Rust | Rust adv. |
|---|---|---|
| cold start: docker run → /up 200 (ms) | 406 [393–412] | – |
| idle memory.current (MB) | 15.0 [13.0–23.0] | – |
| idle anon (MB) | 11.0 [11.0–13.0] | – |
| peak memory.current under load (MB) | 334 [316–345] | – |
| peak anon under load (MB) | 135 [135–144] | – |

### HTTP (signed in as david; keep-alive; c = concurrent connections)

| Metric | Rust | Rust adv. |
|---|---|---|
| room_show c=1 req/s | 6,626 [6,253–6,637] | – |
| room_show c=1 p50 ms | 0.14 [0.14–0.15] | – |
| room_show c=1 p99 ms | 0.24 [0.24–0.28] | – |
| room_show c=16 req/s | 26,682 [26,257–26,860] | – |
| room_show c=16 p50 ms | 0.60 [0.60–0.61] | – |
| room_show c=16 p99 ms | 1.06 [1.05–1.11] | – |
| room_show c=64 req/s | 26,825 [26,322–26,832] | – |
| room_show c=64 p50 ms | 2.32 [2.31–2.35] | – |
| room_show c=64 p99 ms | 4.89 [4.87–5.05] | – |
| messages_page c=1 req/s | 7,975 [7,929–8,272] | – |
| messages_page c=1 p50 ms | 0.12 [0.12–0.12] | – |
| messages_page c=1 p99 ms | 0.21 [0.20–0.22] | – |
| messages_page c=16 req/s | 32,739 [32,238–32,851] | – |
| messages_page c=16 p50 ms | 0.49 [0.49–0.50] | – |
| messages_page c=16 p99 ms | 0.86 [0.86–0.88] | – |
| messages_page c=64 req/s | 32,929 [32,284–33,003] | – |
| messages_page c=64 p50 ms | 1.89 [1.89–1.92] | – |
| messages_page c=64 p99 ms | 3.98 [3.96–4.09] | – |
| sidebar c=1 req/s | 3,807 [3,668–3,850] | – |
| sidebar c=1 p50 ms | 0.25 [0.25–0.26] | – |
| sidebar c=1 p99 ms | 0.41 [0.40–0.43] | – |
| sidebar c=16 req/s | 15,746 [15,666–15,815] | – |
| sidebar c=16 p50 ms | 1.02 [1.02–1.03] | – |
| sidebar c=16 p99 ms | 1.78 [1.77–1.80] | – |
| sidebar c=64 req/s | 15,782 [15,727–15,804] | – |
| sidebar c=64 p50 ms | 3.95 [3.94–3.96] | – |
| sidebar c=64 p99 ms | 8.26 [8.20–8.31] | – |
| search c=1 req/s | 7,780 [7,734–7,810] | – |
| search c=1 p50 ms | 0.12 [0.12–0.12] | – |
| search c=1 p99 ms | 0.21 [0.21–0.22] | – |
| search c=16 req/s | 30,601 [30,093–30,951] | – |
| search c=16 p50 ms | 0.50 [0.50–0.51] | – |
| search c=16 p99 ms | 1.22 [1.19–1.25] | – |
| search c=64 req/s | 29,743 [29,123–30,256] | – |
| search c=64 p50 ms | 2.06 [2.04–2.10] | – |
| search c=64 p99 ms | 4.80 [4.66–4.97] | – |
| avatar c=1 req/s | 78,161 [77,528–78,426] | – |
| avatar c=1 p50 ms | 0.01 [0.01–0.01] | – |
| avatar c=1 p99 ms | 0.02 [0.02–0.02] | – |
| avatar c=16 req/s | 359,025 [354,460–359,556] | – |
| avatar c=16 p50 ms | 0.03 [0.03–0.03] | – |
| avatar c=16 p99 ms | 0.13 [0.13–0.13] | – |
| avatar c=64 req/s | 367,723 [366,752–368,202] | – |
| avatar c=64 p50 ms | 0.13 [0.13–0.13] | – |
| avatar c=64 p99 ms | 1.28 [1.24–1.31] | – |
| static_css c=1 req/s | 84,292 [84,144–84,816] | – |
| static_css c=1 p50 ms | 0.01 [0.01–0.01] | – |
| static_css c=1 p99 ms | 0.02 [0.02–0.02] | – |
| static_css c=16 req/s | 382,330 [382,324–384,321] | – |
| static_css c=16 p50 ms | 0.03 [0.03–0.03] | – |
| static_css c=16 p99 ms | 0.12 [0.12–0.12] | – |
| static_css c=64 req/s | 392,195 [388,249–392,293] | – |
| static_css c=64 p50 ms | 0.13 [0.13–0.13] | – |
| static_css c=64 p99 ms | 1.25 [1.18–1.34] | – |
| up c=1 req/s | 32,020 [31,915–32,150] | – |
| up c=1 p50 ms | 0.03 [0.03–0.03] | – |
| up c=1 p99 ms | 0.06 [0.06–0.06] | – |
| up c=16 req/s | 150,131 [149,019–150,942] | – |
| up c=16 p50 ms | 0.11 [0.11–0.11] | – |
| up c=16 p99 ms | 0.19 [0.19–0.19] | – |
| up c=64 req/s | 150,820 [150,364–151,302] | – |
| up c=64 p50 ms | 0.41 [0.41–0.41] | – |
| up c=64 p99 ms | 0.88 [0.88–0.89] | – |
| post_message c=1 req/s | 2,408 [2,382–2,419] | – |
| post_message c=1 p50 ms | 0.32 [0.32–0.32] | – |
| post_message c=1 p99 ms | 1.45 [1.44–1.47] | – |
| post_message c=16 req/s | 5,055 [4,985–5,167] | – |
| post_message c=16 p50 ms | 2.11 [2.10–2.13] | – |
| post_message c=16 p99 ms | 43.3 [41.8–46.4] | – |
| post_message c=64 req/s | 5,122 [5,032–5,139] | – |
| post_message c=64 p50 ms | 9.14 [9.13–9.15] | – |
| post_message c=64 p99 ms | 55.8 [55.6–60.6] | – |

### HTTP errors / non-2xx-3xx (first rep, per app)

| Metric | Rust | Rust adv. |
|---|---|---|
- rust: none

### Action Cable fan-out (one room; chatter.js subscriptions per client)

| Metric | Rust | Rust adv. |
|---|---|---|
| 100 clients: subscribed | 100 [100–100] | – |
| 100 clients: connect+subscribe all (s) | 0.06 [0.06–0.06] | – |
| 100 clients: paced post→one client p50 ms | 1.03 [1.02–1.05] | – |
| 100 clients: paced post→all clients p50 ms | 1.27 [1.26–1.28] | – |
| 100 clients: paced post→all clients p99 ms | 2.62 [1.60–16.31] | – |
| 100 clients: max sustained msgs/s (delivered to all) | 3,170 [3,072–3,286] | – |
| 100 clients: deliveries/s (client×message) | 317,008 [307,248–328,652] | – |
| 100 clients: saturated post→all p50 ms | 1.23 [1.20–1.27] | – |
| 100 clients: saturated POST p50 ms | 0.84 [0.83–0.87] | – |
| 500 clients: subscribed | 500 [500–500] | – |
| 500 clients: connect+subscribe all (s) | 0.11 [0.10–0.13] | – |
| 500 clients: paced post→one client p50 ms | 2.16 [2.05–2.20] | – |
| 500 clients: paced post→all clients p50 ms | 3.77 [3.31–3.87] | – |
| 500 clients: paced post→all clients p99 ms | 11.1 [5.7–16.2] | – |
| 500 clients: max sustained msgs/s (delivered to all) | 823 [819–827] | – |
| 500 clients: deliveries/s (client×message) | 411,681 [409,686–413,374] | – |
| 500 clients: saturated post→all p50 ms | 11.3 [11.3–11.4] | – |
| 500 clients: saturated POST p50 ms | 4.27 [4.23–4.34] | – |
| 1000 clients: subscribed | 1,000 [1,000–1,000] | – |
| 1000 clients: connect+subscribe all (s) | 0.20 [0.13–0.21] | – |
| 1000 clients: paced post→one client p50 ms | 3.55 [3.51–3.56] | – |
| 1000 clients: paced post→all clients p50 ms | 6.44 [6.32–6.49] | – |
| 1000 clients: paced post→all clients p99 ms | 10.2 [8.5–15.2] | – |
| 1000 clients: max sustained msgs/s (delivered to all) | 417 [416–418] | – |
| 1000 clients: deliveries/s (client×message) | 417,393 [415,689–418,455] | – |
| 1000 clients: saturated post→all p50 ms | 19.4 [19.1–19.8] | – |
| 1000 clients: saturated POST p50 ms | 9.33 [9.26–9.37] | – |

### Upload + thumbnail (black_hole.jpg, 505 KB)

| Metric | Rust | Rust adv. |
|---|---|---|
| POST with attachment (ms) | – | – |
| then GET thumb → 200 (ms) | – | – |
| POST → thumbnail served (ms) | – | – |

### Memory during cable fan-out, by process (MB, peak within the phase)

App process: Rails' Puma master and workers (Action Cable runs in them), or Rust's one campfire
process (its front server included). Pss counts pages shared between forked workers once;
RssAnon counts them in every process.

| Metric | Rust | Rust adv. |
|---|---|---|
| 100 clients, all subscribed, idle: app process Pss | 154 [153–156] | – |
| 100 clients, all subscribed, idle: app process RssAnon | 129 [128–132] | – |
| 100 clients, all subscribed, idle: app + Redis + Thruster Pss | 154 [153–156] | – |
| 100 clients, all subscribed, idle: whole container Pss | 154 [153–156] | – |
| 100 clients, saturated fan-out: app process Pss | 160 [160–162] | – |
| 100 clients, saturated fan-out: app process RssAnon | 136 [135–137] | – |
| 100 clients, saturated fan-out: app + Redis + Thruster Pss | 160 [160–162] | – |
| 100 clients, saturated fan-out: whole container Pss | 160 [160–162] | – |
| 500 clients, all subscribed, idle: app process Pss | 158 [157–160] | – |
| 500 clients, all subscribed, idle: app process RssAnon | 132 [131–134] | – |
| 500 clients, all subscribed, idle: app + Redis + Thruster Pss | 158 [157–160] | – |
| 500 clients, all subscribed, idle: whole container Pss | 158 [157–160] | – |
| 500 clients, saturated fan-out: app process Pss | 159 [159–160] | – |
| 500 clients, saturated fan-out: app process RssAnon | 133 [133–134] | – |
| 500 clients, saturated fan-out: app + Redis + Thruster Pss | 159 [159–160] | – |
| 500 clients, saturated fan-out: whole container Pss | 159 [159–160] | – |
| 1000 clients, all subscribed, idle: app process Pss | 160 [158–170] | – |
| 1000 clients, all subscribed, idle: app process RssAnon | 134 [132–144] | – |
| 1000 clients, all subscribed, idle: app + Redis + Thruster Pss | 160 [158–170] | – |
| 1000 clients, all subscribed, idle: whole container Pss | 160 [158–170] | – |
| 1000 clients, saturated fan-out: app process Pss | 160 [158–170] | – |
| 1000 clients, saturated fan-out: app process RssAnon | 134 [132–143] | – |
| 1000 clients, saturated fan-out: app + Redis + Thruster Pss | 160 [158–170] | – |
| 1000 clients, saturated fan-out: whole container Pss | 160 [158–170] | – |
