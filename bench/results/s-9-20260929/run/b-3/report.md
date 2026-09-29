```
date: 2026-09-29T14:56:46+02:00
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
| cold start: docker run → /up 200 (ms) | 18,421 | – |
| idle memory.current (MB) | 66.0 | – |
| idle anon (MB) | 11.0 | – |
| peak memory.current under load (MB) | 1,239 | – |
| peak anon under load (MB) | 361 | – |

### HTTP (signed in as david; keep-alive; c = concurrent connections)

| Metric | Rust | Rust adv. |
|---|---|---|
| room_show c=1 req/s | 4,624 | – |
| room_show c=1 p50 ms | 0.19 | – |
| room_show c=1 p99 ms | 0.58 | – |
| room_show c=16 req/s | 21,886 | – |
| room_show c=16 p50 ms | 0.70 | – |
| room_show c=16 p99 ms | 1.57 | – |
| room_show c=64 req/s | 18,491 | – |
| room_show c=64 p50 ms | 3.20 | – |
| room_show c=64 p99 ms | 7.07 | – |
| messages_page c=1 req/s | 5,154 | – |
| messages_page c=1 p50 ms | 0.17 | – |
| messages_page c=1 p99 ms | 0.61 | – |
| messages_page c=16 req/s | 22,944 | – |
| messages_page c=16 p50 ms | 0.64 | – |
| messages_page c=16 p99 ms | 1.71 | – |
| messages_page c=64 req/s | 17,377 | – |
| messages_page c=64 p50 ms | 2.84 | – |
| messages_page c=64 p99 ms | 7.64 | – |
| sidebar c=1 req/s | 2,281 | – |
| sidebar c=1 p50 ms | 0.36 | – |
| sidebar c=1 p99 ms | 1.30 | – |
| sidebar c=16 req/s | 2,825 | – |
| sidebar c=16 p50 ms | 1.31 | – |
| sidebar c=16 p99 ms | 4.33 | – |
| sidebar c=64 req/s | 5,465 | – |
| sidebar c=64 p50 ms | 5.08 | – |
| sidebar c=64 p99 ms | 77.1 | – |
| search c=1 req/s | 5,719 | – |
| search c=1 p50 ms | 0.15 | – |
| search c=1 p99 ms | 0.38 | – |
| search c=16 req/s | 24,899 | – |
| search c=16 p50 ms | 0.58 | – |
| search c=16 p99 ms | 1.57 | – |
| search c=64 req/s | 26,803 | – |
| search c=64 p50 ms | 2.29 | – |
| search c=64 p99 ms | 4.65 | – |
| avatar c=1 req/s | 76,376 | – |
| avatar c=1 p50 ms | 0.01 | – |
| avatar c=1 p99 ms | 0.02 | – |
| avatar c=16 req/s | 261,897 | – |
| avatar c=16 p50 ms | 0.04 | – |
| avatar c=16 p99 ms | 0.23 | – |
| avatar c=64 req/s | 345,627 | – |
| avatar c=64 p50 ms | 0.14 | – |
| avatar c=64 p99 ms | 0.78 | – |
| static_css c=1 req/s | 66,412 | – |
| static_css c=1 p50 ms | 0.01 | – |
| static_css c=1 p99 ms | 0.03 | – |
| static_css c=16 req/s | 152,684 | – |
| static_css c=16 p50 ms | 0.04 | – |
| static_css c=16 p99 ms | 0.24 | – |
| static_css c=64 req/s | 11,944 | – |
| static_css c=64 p50 ms | 0.30 | – |
| static_css c=64 p99 ms | 160 | – |
| up c=1 req/s | 21,296 | – |
| up c=1 p50 ms | 0.03 | – |
| up c=1 p99 ms | 0.09 | – |
| up c=16 req/s | 137,703 | – |
| up c=16 p50 ms | 0.11 | – |
| up c=16 p99 ms | 0.22 | – |
| up c=64 req/s | 129,983 | – |
| up c=64 p50 ms | 0.45 | – |
| up c=64 p99 ms | 1.42 | – |
| post_message c=1 req/s | 1,406 | – |
| post_message c=1 p50 ms | 0.44 | – |
| post_message c=1 p99 ms | 2.19 | – |
| post_message c=16 req/s | 2,604 | – |
| post_message c=16 p50 ms | 2.48 | – |
| post_message c=16 p99 ms | 63.8 | – |
| post_message c=64 req/s | 4,563 | – |
| post_message c=64 p50 ms | 9.81 | – |
| post_message c=64 p99 ms | 67.6 | – |

### HTTP errors / non-2xx-3xx (first rep, per app)

| Metric | Rust | Rust adv. |
|---|---|---|
- rust: none

### Action Cable fan-out (one room; chatter.js subscriptions per client)

| Metric | Rust | Rust adv. |
|---|---|---|
| 100 clients: subscribed | 100 | – |
| 100 clients: connect+subscribe all (s) | 0.06 | – |
| 100 clients: paced post→one client p50 ms | 1.40 | – |
| 100 clients: paced post→all clients p50 ms | 1.74 | – |
| 100 clients: paced post→all clients p99 ms | 5.72 | – |
| 100 clients: max sustained msgs/s (delivered to all) | 1,967 | – |
| 100 clients: deliveries/s (client×message) | 196,657 | – |
| 100 clients: saturated post→all p50 ms | 1.38 | – |
| 100 clients: saturated POST p50 ms | 1.30 | – |
| 1000 clients: subscribed | 1,000 | – |
| 1000 clients: connect+subscribe all (s) | 0.12 | – |
| 1000 clients: paced post→one client p50 ms | 4.33 | – |
| 1000 clients: paced post→all clients p50 ms | 7.03 | – |
| 1000 clients: paced post→all clients p99 ms | 19.7 | – |
| 1000 clients: max sustained msgs/s (delivered to all) | 332 | – |
| 1000 clients: deliveries/s (client×message) | 331,825 | – |
| 1000 clients: saturated post→all p50 ms | 21.0 | – |
| 1000 clients: saturated POST p50 ms | 10.6 | – |
| 5000 clients: subscribed | 5,000 | – |
| 5000 clients: connect+subscribe all (s) | 1.56 | – |
| 5000 clients: paced post→one client p50 ms | 13.0 | – |
| 5000 clients: paced post→all clients p50 ms | 27.9 | – |
| 5000 clients: paced post→all clients p99 ms | 49.1 | – |
| 5000 clients: max sustained msgs/s (delivered to all) | 84.5 | – |
| 5000 clients: deliveries/s (client×message) | 422,532 | – |
| 5000 clients: saturated post→all p50 ms | 148 | – |
| 5000 clients: saturated POST p50 ms | 28.0 | – |
| 10000 clients: subscribed | 10,000 | – |
| 10000 clients: connect+subscribe all (s) | 1.58 | – |
| 10000 clients: paced post→one client p50 ms | 27.9 | – |
| 10000 clients: paced post→all clients p50 ms | 55.9 | – |
| 10000 clients: paced post→all clients p99 ms | 211 | – |
| 10000 clients: max sustained msgs/s (delivered to all) | 26.2 | – |
| 10000 clients: deliveries/s (client×message) | 261,854 | – |
| 10000 clients: saturated post→all p50 ms | 374 | – |
| 10000 clients: saturated POST p50 ms | 47.8 | – |

### Upload + thumbnail (black_hole.jpg, 505 KB)

| Metric | Rust | Rust adv. |
|---|---|---|
| POST with attachment (ms) | 30.9 | – |
| then GET thumb → 200 (ms) | 0.20 | – |
| POST → thumbnail served (ms) | 31.1 | – |

### Memory during cable fan-out, by process (MB, peak within the phase)

App process: Rails' Puma master and workers (Action Cable runs in them), or Rust's one campfire
process (its front server included). Pss counts pages shared between forked workers once;
RssAnon counts them in every process.

| Metric | Rust | Rust adv. |
|---|---|---|
| 100 clients, all subscribed, idle: app process Pss | 147 | – |
| 100 clients, all subscribed, idle: app process RssAnon | 131 | – |
| 100 clients, all subscribed, idle: app + Redis + Thruster Pss | 147 | – |
| 100 clients, all subscribed, idle: whole container Pss | 147 | – |
| 100 clients, saturated fan-out: app process Pss | 156 | – |
| 100 clients, saturated fan-out: app process RssAnon | 140 | – |
| 100 clients, saturated fan-out: app + Redis + Thruster Pss | 156 | – |
| 100 clients, saturated fan-out: whole container Pss | 156 | – |
| 1000 clients, all subscribed, idle: app process Pss | 168 | – |
| 1000 clients, all subscribed, idle: app process RssAnon | 151 | – |
| 1000 clients, all subscribed, idle: app + Redis + Thruster Pss | 168 | – |
| 1000 clients, all subscribed, idle: whole container Pss | 168 | – |
| 1000 clients, saturated fan-out: app process Pss | 167 | – |
| 1000 clients, saturated fan-out: app process RssAnon | 150 | – |
| 1000 clients, saturated fan-out: app + Redis + Thruster Pss | 167 | – |
| 1000 clients, saturated fan-out: whole container Pss | 167 | – |
| 5000 clients, all subscribed, idle: app process Pss | 221 | – |
| 5000 clients, all subscribed, idle: app process RssAnon | 205 | – |
| 5000 clients, all subscribed, idle: app + Redis + Thruster Pss | 221 | – |
| 5000 clients, all subscribed, idle: whole container Pss | 221 | – |
| 5000 clients, saturated fan-out: app process Pss | 221 | – |
| 5000 clients, saturated fan-out: app process RssAnon | 204 | – |
| 5000 clients, saturated fan-out: app + Redis + Thruster Pss | 221 | – |
| 5000 clients, saturated fan-out: whole container Pss | 221 | – |
| 10000 clients, all subscribed, idle: app process Pss | 307 | – |
| 10000 clients, all subscribed, idle: app process RssAnon | 290 | – |
| 10000 clients, all subscribed, idle: app + Redis + Thruster Pss | 307 | – |
| 10000 clients, all subscribed, idle: whole container Pss | 307 | – |
| 10000 clients, saturated fan-out: app process Pss | 307 | – |
| 10000 clients, saturated fan-out: app process RssAnon | 290 | – |
| 10000 clients, saturated fan-out: app + Redis + Thruster Pss | 307 | – |
| 10000 clients, saturated fan-out: whole container Pss | 307 | – |
