```
date: 2026-09-29T12:26:26+02:00
host: 7.2.3-arch1-3, AMD Ryzen 9 9950X3D 16-Core Processor, 32 threads, 60GB
server cpus: 8-11 (nproc 4); loadgen cpus: 12-15; network: host
env: WEB_CONCURRENCY=3 JOB_CONCURRENCY=3 RAILS_MAX_THREADS=5 
rust extra env: 
rust image: campfire-rust:s-9-b sha256:776069d599316097c45bdd60306bfee9538e5a0c869e623914eccb7f56a8c282 2026-09-28T21:35:23.074165457+02:00
rust HEAD: 0f3bac7 (dirty: 0 files)
```

Reps: rust 1. Cells: median [min–max].

### Startup and memory

| Metric | Rust | Rust adv. |
|---|---|---|
| cold start: docker run → /up 200 (ms) | 405 | – |
| idle memory.current (MB) | 47.0 | – |
| idle anon (MB) | 11.0 | – |
| peak memory.current under load (MB) | 821 | – |
| peak anon under load (MB) | 435 | – |

### HTTP (signed in as david; keep-alive; c = concurrent connections)

| Metric | Rust | Rust adv. |
|---|---|---|
| room_show c=1 req/s | 4,946 | – |
| room_show c=1 p50 ms | 0.19 | – |
| room_show c=1 p99 ms | 0.33 | – |
| room_show c=16 req/s | 20,949 | – |
| room_show c=16 p50 ms | 0.73 | – |
| room_show c=16 p99 ms | 1.66 | – |
| room_show c=64 req/s | 21,812 | – |
| room_show c=64 p50 ms | 2.90 | – |
| room_show c=64 p99 ms | 5.11 | – |
| messages_page c=1 req/s | 6,261 | – |
| messages_page c=1 p50 ms | 0.15 | – |
| messages_page c=1 p99 ms | 0.25 | – |
| messages_page c=16 req/s | 25,750 | – |
| messages_page c=16 p50 ms | 0.57 | – |
| messages_page c=16 p99 ms | 1.49 | – |
| messages_page c=64 req/s | 26,573 | – |
| messages_page c=64 p50 ms | 2.35 | – |
| messages_page c=64 p99 ms | 4.18 | – |
| sidebar c=1 req/s | 3,143 | – |
| sidebar c=1 p50 ms | 0.31 | – |
| sidebar c=1 p99 ms | 0.45 | – |
| sidebar c=16 req/s | 13,681 | – |
| sidebar c=16 p50 ms | 1.13 | – |
| sidebar c=16 p99 ms | 2.35 | – |
| sidebar c=64 req/s | 14,079 | – |
| sidebar c=64 p50 ms | 4.52 | – |
| sidebar c=64 p99 ms | 7.63 | – |
| search c=1 req/s | 6,515 | – |
| search c=1 p50 ms | 0.14 | – |
| search c=1 p99 ms | 0.24 | – |
| search c=16 req/s | 25,694 | – |
| search c=16 p50 ms | 0.57 | – |
| search c=16 p99 ms | 1.51 | – |
| search c=64 req/s | 27,385 | – |
| search c=64 p50 ms | 2.26 | – |
| search c=64 p99 ms | 4.29 | – |
| avatar c=1 req/s | 78,720 | – |
| avatar c=1 p50 ms | 0.01 | – |
| avatar c=1 p99 ms | 0.02 | – |
| avatar c=16 req/s | 361,328 | – |
| avatar c=16 p50 ms | 0.03 | – |
| avatar c=16 p99 ms | 0.13 | – |
| avatar c=64 req/s | 376,672 | – |
| avatar c=64 p50 ms | 0.13 | – |
| avatar c=64 p99 ms | 1.31 | – |
| static_css c=1 req/s | 83,966 | – |
| static_css c=1 p50 ms | 0.01 | – |
| static_css c=1 p99 ms | 0.02 | – |
| static_css c=16 req/s | 391,531 | – |
| static_css c=16 p50 ms | 0.03 | – |
| static_css c=16 p99 ms | 0.12 | – |
| static_css c=64 req/s | 394,674 | – |
| static_css c=64 p50 ms | 0.13 | – |
| static_css c=64 p99 ms | 1.33 | – |
| up c=1 req/s | 32,344 | – |
| up c=1 p50 ms | 0.03 | – |
| up c=1 p99 ms | 0.05 | – |
| up c=16 req/s | 148,842 | – |
| up c=16 p50 ms | 0.11 | – |
| up c=16 p99 ms | 0.19 | – |
| up c=64 req/s | 147,990 | – |
| up c=64 p50 ms | 0.42 | – |
| up c=64 p99 ms | 0.91 | – |
| post_message c=1 req/s | 1,968 | – |
| post_message c=1 p50 ms | 0.41 | – |
| post_message c=1 p99 ms | 1.68 | – |
| post_message c=16 req/s | 4,702 | – |
| post_message c=16 p50 ms | 2.35 | – |
| post_message c=16 p99 ms | 41.4 | – |
| post_message c=64 req/s | 4,906 | – |
| post_message c=64 p50 ms | 9.94 | – |
| post_message c=64 p99 ms | 56.0 | – |

### HTTP errors / non-2xx-3xx (first rep, per app)

| Metric | Rust | Rust adv. |
|---|---|---|
- rust: none

### Action Cable fan-out (one room; chatter.js subscriptions per client)

| Metric | Rust | Rust adv. |
|---|---|---|
| 100 clients: subscribed | 100 | – |
| 100 clients: connect+subscribe all (s) | 0.06 | – |
| 100 clients: paced post→one client p50 ms | 1.13 | – |
| 100 clients: paced post→all clients p50 ms | 1.41 | – |
| 100 clients: paced post→all clients p99 ms | 9.69 | – |
| 100 clients: max sustained msgs/s (delivered to all) | 2,682 | – |
| 100 clients: deliveries/s (client×message) | 268,231 | – |
| 100 clients: saturated post→all p50 ms | 1.15 | – |
| 100 clients: saturated POST p50 ms | 1.11 | – |
| 1000 clients: subscribed | 1,000 | – |
| 1000 clients: connect+subscribe all (s) | 0.14 | – |
| 1000 clients: paced post→one client p50 ms | 3.48 | – |
| 1000 clients: paced post→all clients p50 ms | 6.27 | – |
| 1000 clients: paced post→all clients p99 ms | 11.4 | – |
| 1000 clients: max sustained msgs/s (delivered to all) | 412 | – |
| 1000 clients: deliveries/s (client×message) | 411,829 | – |
| 1000 clients: saturated post→all p50 ms | 20.3 | – |
| 1000 clients: saturated POST p50 ms | 9.10 | – |
| 5000 clients: subscribed | 5,000 | – |
| 5000 clients: connect+subscribe all (s) | 1.31 | – |
| 5000 clients: paced post→one client p50 ms | 13.1 | – |
| 5000 clients: paced post→all clients p50 ms | 27.5 | – |
| 5000 clients: paced post→all clients p99 ms | 41.7 | – |
| 5000 clients: max sustained msgs/s (delivered to all) | 86.9 | – |
| 5000 clients: deliveries/s (client×message) | 434,305 | – |
| 5000 clients: saturated post→all p50 ms | 148 | – |
| 5000 clients: saturated POST p50 ms | 27.7 | – |
| 10000 clients: subscribed | 10,000 | – |
| 10000 clients: connect+subscribe all (s) | 1.42 | – |
| 10000 clients: paced post→one client p50 ms | 24.8 | – |
| 10000 clients: paced post→all clients p50 ms | 56.5 | – |
| 10000 clients: paced post→all clients p99 ms | 77.6 | – |
| 10000 clients: max sustained msgs/s (delivered to all) | 47.2 | – |
| 10000 clients: deliveries/s (client×message) | 472,496 | – |
| 10000 clients: saturated post→all p50 ms | 300 | – |
| 10000 clients: saturated POST p50 ms | 37.7 | – |

### Upload + thumbnail (black_hole.jpg, 505 KB)

| Metric | Rust | Rust adv. |
|---|---|---|
| POST with attachment (ms) | 25.0 | – |
| then GET thumb → 200 (ms) | 0.20 | – |
| POST → thumbnail served (ms) | 25.1 | – |

### Memory during cable fan-out, by process (MB, peak within the phase)

App process: Rails' Puma master and workers (Action Cable runs in them), or Rust's one campfire
process (its front server included). Pss counts pages shared between forked workers once;
RssAnon counts them in every process.

| Metric | Rust | Rust adv. |
|---|---|---|
| 100 clients, all subscribed, idle: app process Pss | 162 | – |
| 100 clients, all subscribed, idle: app process RssAnon | 137 | – |
| 100 clients, all subscribed, idle: app + Redis + Thruster Pss | 162 | – |
| 100 clients, all subscribed, idle: whole container Pss | 162 | – |
| 100 clients, saturated fan-out: app process Pss | 175 | – |
| 100 clients, saturated fan-out: app process RssAnon | 151 | – |
| 100 clients, saturated fan-out: app + Redis + Thruster Pss | 175 | – |
| 100 clients, saturated fan-out: whole container Pss | 175 | – |
| 1000 clients, all subscribed, idle: app process Pss | 188 | – |
| 1000 clients, all subscribed, idle: app process RssAnon | 162 | – |
| 1000 clients, all subscribed, idle: app + Redis + Thruster Pss | 188 | – |
| 1000 clients, all subscribed, idle: whole container Pss | 188 | – |
| 1000 clients, saturated fan-out: app process Pss | 188 | – |
| 1000 clients, saturated fan-out: app process RssAnon | 162 | – |
| 1000 clients, saturated fan-out: app + Redis + Thruster Pss | 188 | – |
| 1000 clients, saturated fan-out: whole container Pss | 188 | – |
| 5000 clients, all subscribed, idle: app process Pss | 254 | – |
| 5000 clients, all subscribed, idle: app process RssAnon | 228 | – |
| 5000 clients, all subscribed, idle: app + Redis + Thruster Pss | 254 | – |
| 5000 clients, all subscribed, idle: whole container Pss | 254 | – |
| 5000 clients, saturated fan-out: app process Pss | 251 | – |
| 5000 clients, saturated fan-out: app process RssAnon | 225 | – |
| 5000 clients, saturated fan-out: app + Redis + Thruster Pss | 251 | – |
| 5000 clients, saturated fan-out: whole container Pss | 251 | – |
| 10000 clients, all subscribed, idle: app process Pss | 324 | – |
| 10000 clients, all subscribed, idle: app process RssAnon | 298 | – |
| 10000 clients, all subscribed, idle: app + Redis + Thruster Pss | 324 | – |
| 10000 clients, all subscribed, idle: whole container Pss | 324 | – |
| 10000 clients, saturated fan-out: app process Pss | 324 | – |
| 10000 clients, saturated fan-out: app process RssAnon | 299 | – |
| 10000 clients, saturated fan-out: app + Redis + Thruster Pss | 324 | – |
| 10000 clients, saturated fan-out: whole container Pss | 324 | – |
