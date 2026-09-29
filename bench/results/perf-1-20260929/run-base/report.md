```
date: 2026-09-29T13:27:04+02:00
host: 7.2.3-arch1-3, AMD Ryzen 9 9950X3D 16-Core Processor, 32 threads, 60GB
server cpus: 8-11 (nproc 4); loadgen cpus: 12-15; network: host
env: WEB_CONCURRENCY=3 JOB_CONCURRENCY=3 RAILS_MAX_THREADS=5 
rust extra env: 
rust image: campfire-rust:perf-1 sha256:2f8906ede435c36f438a2039c69e898ed67b84868b40e30c22eb1f7eaff6198e 2026-09-29T13:26:50.200167137+02:00
rust HEAD: a49dc9b (dirty: 0 files)
```

Reps: rust 3. Cells: median [min–max].

### Startup and memory

| Metric | Rust | Rust adv. |
|---|---|---|
| cold start: docker run → /up 200 (ms) | 413 [403–424] | – |
| idle memory.current (MB) | 16.0 [13.0–19.0] | – |
| idle anon (MB) | 11.0 [11.0–11.0] | – |
| peak memory.current under load (MB) | 333 [283–354] | – |
| peak anon under load (MB) | 149 [144–158] | – |

### HTTP (signed in as david; keep-alive; c = concurrent connections)

| Metric | Rust | Rust adv. |
|---|---|---|
| room_show c=1 req/s | 5,119 [5,019–5,165] | – |
| room_show c=1 p50 ms | 0.18 [0.18–0.18] | – |
| room_show c=1 p99 ms | 0.32 [0.31–0.34] | – |
| room_show c=16 req/s | 22,192 [22,146–22,245] | – |
| room_show c=16 p50 ms | 0.69 [0.69–0.69] | – |
| room_show c=16 p99 ms | 1.57 [1.55–1.59] | – |
| room_show c=64 req/s | 21,846 [21,060–22,323] | – |
| room_show c=64 p50 ms | 2.89 [2.83–3.00] | – |
| room_show c=64 p99 ms | 5.09 [4.96–5.40] | – |
| messages_page c=1 req/s | 5,778 [5,636–5,886] | – |
| messages_page c=1 p50 ms | 0.16 [0.16–0.16] | – |
| messages_page c=1 p99 ms | 0.30 [0.28–0.30] | – |
| messages_page c=16 req/s | 25,782 [25,615–25,842] | – |
| messages_page c=16 p50 ms | 0.58 [0.58–0.58] | – |
| messages_page c=16 p99 ms | 1.48 [1.47–1.50] | – |
| messages_page c=64 req/s | 25,960 [25,834–26,225] | – |
| messages_page c=64 p50 ms | 2.40 [2.39–2.42] | – |
| messages_page c=64 p99 ms | 4.25 [4.25–4.43] | – |
| sidebar c=1 req/s | 2,876 [2,767–2,971] | – |
| sidebar c=1 p50 ms | 0.33 [0.32–0.35] | – |
| sidebar c=1 p99 ms | 0.52 [0.52–0.54] | – |
| sidebar c=16 req/s | 13,206 [13,067–13,379] | – |
| sidebar c=16 p50 ms | 1.16 [1.15–1.17] | – |
| sidebar c=16 p99 ms | 2.53 [2.48–2.57] | – |
| sidebar c=64 req/s | 13,778 [13,650–13,792] | – |
| sidebar c=64 p50 ms | 4.61 [4.60–4.65] | – |
| sidebar c=64 p99 ms | 7.96 [7.93–8.05] | – |
| search c=1 req/s | 5,967 [5,873–6,046] | – |
| search c=1 p50 ms | 0.15 [0.15–0.15] | – |
| search c=1 p99 ms | 0.30 [0.29–0.30] | – |
| search c=16 req/s | 25,127 [24,873–25,226] | – |
| search c=16 p50 ms | 0.58 [0.58–0.59] | – |
| search c=16 p99 ms | 1.55 [1.53–1.58] | – |
| search c=64 req/s | 25,983 [24,382–26,703] | – |
| search c=64 p50 ms | 2.34 [2.33–2.40] | – |
| search c=64 p99 ms | 4.32 [4.19–4.50] | – |
| avatar c=1 req/s | 76,933 [75,633–77,259] | – |
| avatar c=1 p50 ms | 0.01 [0.01–0.01] | – |
| avatar c=1 p99 ms | 0.03 [0.02–0.03] | – |
| avatar c=16 req/s | 351,993 [326,040–357,505] | – |
| avatar c=16 p50 ms | 0.03 [0.03–0.04] | – |
| avatar c=16 p99 ms | 0.13 [0.13–0.15] | – |
| avatar c=64 req/s | 361,279 [346,989–374,357] | – |
| avatar c=64 p50 ms | 0.14 [0.13–0.14] | – |
| avatar c=64 p99 ms | 1.24 [0.99–1.27] | – |
| static_css c=1 req/s | 82,738 [79,293–82,865] | – |
| static_css c=1 p50 ms | 0.01 [0.01–0.01] | – |
| static_css c=1 p99 ms | 0.02 [0.02–0.03] | – |
| static_css c=16 req/s | 377,734 [360,680–386,162] | – |
| static_css c=16 p50 ms | 0.03 [0.03–0.03] | – |
| static_css c=16 p99 ms | 0.12 [0.12–0.13] | – |
| static_css c=64 req/s | 384,029 [383,959–395,468] | – |
| static_css c=64 p50 ms | 0.13 [0.13–0.13] | – |
| static_css c=64 p99 ms | 1.18 [1.01–1.18] | – |
| up c=1 req/s | 30,192 [29,576–31,535] | – |
| up c=1 p50 ms | 0.03 [0.03–0.03] | – |
| up c=1 p99 ms | 0.08 [0.06–0.08] | – |
| up c=16 req/s | 145,453 [138,912–146,026] | – |
| up c=16 p50 ms | 0.11 [0.11–0.11] | – |
| up c=16 p99 ms | 0.20 [0.20–0.22] | – |
| up c=64 req/s | 147,712 [146,139–148,279] | – |
| up c=64 p50 ms | 0.42 [0.42–0.42] | – |
| up c=64 p99 ms | 0.90 [0.90–0.91] | – |
| post_message c=1 req/s | 2,014 [1,979–2,020] | – |
| post_message c=1 p50 ms | 0.40 [0.40–0.40] | – |
| post_message c=1 p99 ms | 1.63 [1.59–1.65] | – |
| post_message c=16 req/s | 4,611 [4,513–4,774] | – |
| post_message c=16 p50 ms | 2.37 [2.31–2.37] | – |
| post_message c=16 p99 ms | 44.6 [41.6–45.3] | – |
| post_message c=64 req/s | 4,670 [4,661–4,784] | – |
| post_message c=64 p50 ms | 10.1 [9.8–10.2] | – |
| post_message c=64 p99 ms | 58.0 [56.6–63.3] | – |

### HTTP errors / non-2xx-3xx (first rep, per app)

| Metric | Rust | Rust adv. |
|---|---|---|
- rust: none

### Action Cable fan-out (one room; chatter.js subscriptions per client)

| Metric | Rust | Rust adv. |
|---|---|---|
| 100 clients: subscribed | 100 [100–100] | – |
| 100 clients: connect+subscribe all (s) | 0.06 [0.06–0.06] | – |
| 100 clients: paced post→one client p50 ms | 1.18 [1.14–1.18] | – |
| 100 clients: paced post→all clients p50 ms | 1.43 [1.39–1.47] | – |
| 100 clients: paced post→all clients p99 ms | 1.89 [1.83–2.03] | – |
| 100 clients: max sustained msgs/s (delivered to all) | 2,524 [2,489–2,556] | – |
| 100 clients: deliveries/s (client×message) | 252,441 [248,861–255,634] | – |
| 100 clients: saturated post→all p50 ms | 1.19 [1.19–1.23] | – |
| 100 clients: saturated POST p50 ms | 1.16 [1.15–1.18] | – |
| 500 clients: subscribed | 500 [500–500] | – |
| 500 clients: connect+subscribe all (s) | 0.11 [0.11–0.12] | – |
| 500 clients: paced post→one client p50 ms | 2.46 [2.43–2.48] | – |
| 500 clients: paced post→all clients p50 ms | 4.20 [3.88–4.22] | – |
| 500 clients: paced post→all clients p99 ms | 17.4 [13.7–18.4] | – |
| 500 clients: max sustained msgs/s (delivered to all) | 724 [638–748] | – |
| 500 clients: deliveries/s (client×message) | 362,148 [318,877–374,239] | – |
| 500 clients: saturated post→all p50 ms | 11.4 [10.9–11.9] | – |
| 500 clients: saturated POST p50 ms | 4.78 [4.67–5.17] | – |
| 1000 clients: subscribed | 1,000 [1,000–1,000] | – |
| 1000 clients: connect+subscribe all (s) | 0.18 [0.16–1.18] | – |
| 1000 clients: paced post→one client p50 ms | 3.74 [3.63–3.87] | – |
| 1000 clients: paced post→all clients p50 ms | 6.76 [6.45–6.77] | – |
| 1000 clients: paced post→all clients p99 ms | 22.5 [15.7–23.2] | – |
| 1000 clients: max sustained msgs/s (delivered to all) | 395 [389–397] | – |
| 1000 clients: deliveries/s (client×message) | 394,696 [388,912–396,617] | – |
| 1000 clients: saturated post→all p50 ms | 20.8 [20.1–21.1] | – |
| 1000 clients: saturated POST p50 ms | 9.65 [9.59–9.93] | – |

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
| 100 clients, all subscribed, idle: app process Pss | 159 [159–168] | – |
| 100 clients, all subscribed, idle: app process RssAnon | 132 [132–140] | – |
| 100 clients, all subscribed, idle: app + Redis + Thruster Pss | 159 [159–168] | – |
| 100 clients, all subscribed, idle: whole container Pss | 159 [159–168] | – |
| 100 clients, saturated fan-out: app process Pss | 172 [170–180] | – |
| 100 clients, saturated fan-out: app process RssAnon | 144 [142–152] | – |
| 100 clients, saturated fan-out: app + Redis + Thruster Pss | 172 [170–180] | – |
| 100 clients, saturated fan-out: whole container Pss | 172 [170–180] | – |
| 500 clients, all subscribed, idle: app process Pss | 172 [172–182] | – |
| 500 clients, all subscribed, idle: app process RssAnon | 144 [143–153] | – |
| 500 clients, all subscribed, idle: app + Redis + Thruster Pss | 172 [172–182] | – |
| 500 clients, all subscribed, idle: whole container Pss | 172 [172–182] | – |
| 500 clients, saturated fan-out: app process Pss | 172 [172–182] | – |
| 500 clients, saturated fan-out: app process RssAnon | 143 [143–153] | – |
| 500 clients, saturated fan-out: app + Redis + Thruster Pss | 172 [172–182] | – |
| 500 clients, saturated fan-out: whole container Pss | 172 [172–182] | – |
| 1000 clients, all subscribed, idle: app process Pss | 178 [173–188] | – |
| 1000 clients, all subscribed, idle: app process RssAnon | 150 [144–159] | – |
| 1000 clients, all subscribed, idle: app + Redis + Thruster Pss | 178 [173–188] | – |
| 1000 clients, all subscribed, idle: whole container Pss | 178 [173–188] | – |
| 1000 clients, saturated fan-out: app process Pss | 179 [173–188] | – |
| 1000 clients, saturated fan-out: app process RssAnon | 150 [144–159] | – |
| 1000 clients, saturated fan-out: app + Redis + Thruster Pss | 179 [173–188] | – |
| 1000 clients, saturated fan-out: whole container Pss | 179 [173–188] | – |
