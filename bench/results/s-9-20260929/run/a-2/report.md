```
date: 2026-09-29T13:12:45+02:00
host: 7.2.3-arch1-3, AMD Ryzen 9 9950X3D 16-Core Processor, 32 threads, 60GB
server cpus: 8-11 (nproc 4); loadgen cpus: 12-15; network: host
env: WEB_CONCURRENCY=3 JOB_CONCURRENCY=3 RAILS_MAX_THREADS=5 
rust extra env: 
rust image: campfire-rust:s-9-a sha256:bba76e9ee0be84d204f0b8285a61a77aece7b66873d14d5c977ff2c33121e688 2026-09-29T11:34:38.632554571+02:00
rust HEAD: 0f3bac7 (dirty: 0 files)
```

Reps: rust 1. Cells: median [min–max].

### Startup and memory

| Metric | Rust | Rust adv. |
|---|---|---|
| cold start: docker run → /up 200 (ms) | 413 | – |
| idle memory.current (MB) | 62.0 | – |
| idle anon (MB) | 11.0 | – |
| peak memory.current under load (MB) | 900 | – |
| peak anon under load (MB) | 441 | – |

### HTTP (signed in as david; keep-alive; c = concurrent connections)

| Metric | Rust | Rust adv. |
|---|---|---|
| room_show c=1 req/s | 4,956 | – |
| room_show c=1 p50 ms | 0.19 | – |
| room_show c=1 p99 ms | 0.33 | – |
| room_show c=16 req/s | 19,898 | – |
| room_show c=16 p50 ms | 0.76 | – |
| room_show c=16 p99 ms | 1.86 | – |
| room_show c=64 req/s | 20,371 | – |
| room_show c=64 p50 ms | 3.08 | – |
| room_show c=64 p99 ms | 5.66 | – |
| messages_page c=1 req/s | 5,459 | – |
| messages_page c=1 p50 ms | 0.17 | – |
| messages_page c=1 p99 ms | 0.30 | – |
| messages_page c=16 req/s | 24,266 | – |
| messages_page c=16 p50 ms | 0.61 | – |
| messages_page c=16 p99 ms | 1.62 | – |
| messages_page c=64 req/s | 25,749 | – |
| messages_page c=64 p50 ms | 2.41 | – |
| messages_page c=64 p99 ms | 4.61 | – |
| sidebar c=1 req/s | 2,988 | – |
| sidebar c=1 p50 ms | 0.32 | – |
| sidebar c=1 p99 ms | 0.48 | – |
| sidebar c=16 req/s | 13,361 | – |
| sidebar c=16 p50 ms | 1.15 | – |
| sidebar c=16 p99 ms | 2.42 | – |
| sidebar c=64 req/s | 13,387 | – |
| sidebar c=64 p50 ms | 4.75 | – |
| sidebar c=64 p99 ms | 8.19 | – |
| search c=1 req/s | 6,147 | – |
| search c=1 p50 ms | 0.15 | – |
| search c=1 p99 ms | 0.27 | – |
| search c=16 req/s | 25,046 | – |
| search c=16 p50 ms | 0.59 | – |
| search c=16 p99 ms | 1.52 | – |
| search c=64 req/s | 26,430 | – |
| search c=64 p50 ms | 2.35 | – |
| search c=64 p99 ms | 4.36 | – |
| avatar c=1 req/s | 77,229 | – |
| avatar c=1 p50 ms | 0.01 | – |
| avatar c=1 p99 ms | 0.02 | – |
| avatar c=16 req/s | 342,806 | – |
| avatar c=16 p50 ms | 0.03 | – |
| avatar c=16 p99 ms | 0.14 | – |
| avatar c=64 req/s | 344,596 | – |
| avatar c=64 p50 ms | 0.14 | – |
| avatar c=64 p99 ms | 0.88 | – |
| static_css c=1 req/s | 78,688 | – |
| static_css c=1 p50 ms | 0.01 | – |
| static_css c=1 p99 ms | 0.03 | – |
| static_css c=16 req/s | 377,377 | – |
| static_css c=16 p50 ms | 0.03 | – |
| static_css c=16 p99 ms | 0.12 | – |
| static_css c=64 req/s | 392,552 | – |
| static_css c=64 p50 ms | 0.13 | – |
| static_css c=64 p99 ms | 1.07 | – |
| up c=1 req/s | 31,397 | – |
| up c=1 p50 ms | 0.03 | – |
| up c=1 p99 ms | 0.06 | – |
| up c=16 req/s | 134,321 | – |
| up c=16 p50 ms | 0.12 | – |
| up c=16 p99 ms | 0.25 | – |
| up c=64 req/s | 135,087 | – |
| up c=64 p50 ms | 0.46 | – |
| up c=64 p99 ms | 1.03 | – |
| post_message c=1 req/s | 1,911 | – |
| post_message c=1 p50 ms | 0.41 | – |
| post_message c=1 p99 ms | 1.73 | – |
| post_message c=16 req/s | 4,626 | – |
| post_message c=16 p50 ms | 2.41 | – |
| post_message c=16 p99 ms | 40.3 | – |
| post_message c=64 req/s | 4,798 | – |
| post_message c=64 p50 ms | 10.1 | – |
| post_message c=64 p99 ms | 56.5 | – |

### HTTP errors / non-2xx-3xx (first rep, per app)

| Metric | Rust | Rust adv. |
|---|---|---|
- rust: none

### Action Cable fan-out (one room; chatter.js subscriptions per client)

| Metric | Rust | Rust adv. |
|---|---|---|
| 100 clients: subscribed | 100 | – |
| 100 clients: connect+subscribe all (s) | 0.06 | – |
| 100 clients: paced post→one client p50 ms | 1.19 | – |
| 100 clients: paced post→all clients p50 ms | 1.49 | – |
| 100 clients: paced post→all clients p99 ms | 1.80 | – |
| 100 clients: max sustained msgs/s (delivered to all) | 2,694 | – |
| 100 clients: deliveries/s (client×message) | 269,382 | – |
| 100 clients: saturated post→all p50 ms | 1.16 | – |
| 100 clients: saturated POST p50 ms | 1.12 | – |
| 1000 clients: subscribed | 1,000 | – |
| 1000 clients: connect+subscribe all (s) | 0.13 | – |
| 1000 clients: paced post→one client p50 ms | 3.91 | – |
| 1000 clients: paced post→all clients p50 ms | 6.49 | – |
| 1000 clients: paced post→all clients p99 ms | 11.6 | – |
| 1000 clients: max sustained msgs/s (delivered to all) | 390 | – |
| 1000 clients: deliveries/s (client×message) | 390,043 | – |
| 1000 clients: saturated post→all p50 ms | 20.6 | – |
| 1000 clients: saturated POST p50 ms | 9.78 | – |
| 5000 clients: subscribed | 5,000 | – |
| 5000 clients: connect+subscribe all (s) | 1.29 | – |
| 5000 clients: paced post→one client p50 ms | 13.2 | – |
| 5000 clients: paced post→all clients p50 ms | 29.3 | – |
| 5000 clients: paced post→all clients p99 ms | 43.4 | – |
| 5000 clients: max sustained msgs/s (delivered to all) | 87.4 | – |
| 5000 clients: deliveries/s (client×message) | 436,941 | – |
| 5000 clients: saturated post→all p50 ms | 151 | – |
| 5000 clients: saturated POST p50 ms | 28.2 | – |
| 10000 clients: subscribed | 10,000 | – |
| 10000 clients: connect+subscribe all (s) | 2.45 | – |
| 10000 clients: paced post→one client p50 ms | 25.4 | – |
| 10000 clients: paced post→all clients p50 ms | 54.7 | – |
| 10000 clients: paced post→all clients p99 ms | 86.6 | – |
| 10000 clients: max sustained msgs/s (delivered to all) | 47.1 | – |
| 10000 clients: deliveries/s (client×message) | 471,165 | – |
| 10000 clients: saturated post→all p50 ms | 338 | – |
| 10000 clients: saturated POST p50 ms | 41.3 | – |

### Upload + thumbnail (black_hole.jpg, 505 KB)

| Metric | Rust | Rust adv. |
|---|---|---|
| POST with attachment (ms) | 34.3 | – |
| then GET thumb → 200 (ms) | 0.20 | – |
| POST → thumbnail served (ms) | 34.5 | – |

### Memory during cable fan-out, by process (MB, peak within the phase)

App process: Rails' Puma master and workers (Action Cable runs in them), or Rust's one campfire
process (its front server included). Pss counts pages shared between forked workers once;
RssAnon counts them in every process.

| Metric | Rust | Rust adv. |
|---|---|---|
| 100 clients, all subscribed, idle: app process Pss | 158 | – |
| 100 clients, all subscribed, idle: app process RssAnon | 133 | – |
| 100 clients, all subscribed, idle: app + Redis + Thruster Pss | 158 | – |
| 100 clients, all subscribed, idle: whole container Pss | 158 | – |
| 100 clients, saturated fan-out: app process Pss | 168 | – |
| 100 clients, saturated fan-out: app process RssAnon | 144 | – |
| 100 clients, saturated fan-out: app + Redis + Thruster Pss | 168 | – |
| 100 clients, saturated fan-out: whole container Pss | 168 | – |
| 1000 clients, all subscribed, idle: app process Pss | 181 | – |
| 1000 clients, all subscribed, idle: app process RssAnon | 154 | – |
| 1000 clients, all subscribed, idle: app + Redis + Thruster Pss | 181 | – |
| 1000 clients, all subscribed, idle: whole container Pss | 181 | – |
| 1000 clients, saturated fan-out: app process Pss | 180 | – |
| 1000 clients, saturated fan-out: app process RssAnon | 153 | – |
| 1000 clients, saturated fan-out: app + Redis + Thruster Pss | 180 | – |
| 1000 clients, saturated fan-out: whole container Pss | 180 | – |
| 5000 clients, all subscribed, idle: app process Pss | 245 | – |
| 5000 clients, all subscribed, idle: app process RssAnon | 219 | – |
| 5000 clients, all subscribed, idle: app + Redis + Thruster Pss | 245 | – |
| 5000 clients, all subscribed, idle: whole container Pss | 245 | – |
| 5000 clients, saturated fan-out: app process Pss | 242 | – |
| 5000 clients, saturated fan-out: app process RssAnon | 216 | – |
| 5000 clients, saturated fan-out: app + Redis + Thruster Pss | 242 | – |
| 5000 clients, saturated fan-out: whole container Pss | 242 | – |
| 10000 clients, all subscribed, idle: app process Pss | 322 | – |
| 10000 clients, all subscribed, idle: app process RssAnon | 296 | – |
| 10000 clients, all subscribed, idle: app + Redis + Thruster Pss | 322 | – |
| 10000 clients, all subscribed, idle: whole container Pss | 322 | – |
| 10000 clients, saturated fan-out: app process Pss | 320 | – |
| 10000 clients, saturated fan-out: app process RssAnon | 294 | – |
| 10000 clients, saturated fan-out: app + Redis + Thruster Pss | 320 | – |
| 10000 clients, saturated fan-out: whole container Pss | 320 | – |
