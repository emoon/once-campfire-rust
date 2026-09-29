```
date: 2026-09-29T17:28:50+02:00
host: 7.2.3-arch1-3, AMD Ryzen 9 9950X3D 16-Core Processor, 32 threads, 60GB
server cpus: 8-11 (nproc 4); loadgen cpus: 12-15; network: host
env: WEB_CONCURRENCY=3 JOB_CONCURRENCY=3 RAILS_MAX_THREADS=5 
rust extra env: 
rust image: campfire-rust:s-9-b sha256:776069d599316097c45bdd60306bfee9538e5a0c869e623914eccb7f56a8c282 2026-09-28T21:35:23.074165457+02:00
rust HEAD: 7ef98d2 (dirty: 0 files)
```

Reps: rust 1. Cells: median [min–max].

### Startup and memory

| Metric | Rust | Rust adv. |
|---|---|---|
| cold start: docker run → /up 200 (ms) | 428 | – |
| idle memory.current (MB) | 48.0 | – |
| idle anon (MB) | 11.0 | – |
| peak memory.current under load (MB) | 693 | – |
| peak anon under load (MB) | 449 | – |

### HTTP (signed in as david; keep-alive; c = concurrent connections)

| Metric | Rust | Rust adv. |
|---|---|---|
| room_show c=1 req/s | 5,592 | – |
| room_show c=1 p50 ms | 0.17 | – |
| room_show c=1 p99 ms | 0.27 | – |
| room_show c=16 req/s | 22,752 | – |
| room_show c=16 p50 ms | 0.67 | – |
| room_show c=16 p99 ms | 1.50 | – |
| room_show c=64 req/s | 22,699 | – |
| room_show c=64 p50 ms | 2.79 | – |
| room_show c=64 p99 ms | 4.78 | – |
| messages_page c=1 req/s | 6,266 | – |
| messages_page c=1 p50 ms | 0.15 | – |
| messages_page c=1 p99 ms | 0.24 | – |
| messages_page c=16 req/s | 26,300 | – |
| messages_page c=16 p50 ms | 0.57 | – |
| messages_page c=16 p99 ms | 1.44 | – |
| messages_page c=64 req/s | 26,397 | – |
| messages_page c=64 p50 ms | 2.38 | – |
| messages_page c=64 p99 ms | 4.06 | – |
| sidebar c=1 req/s | 3,210 | – |
| sidebar c=1 p50 ms | 0.30 | – |
| sidebar c=1 p99 ms | 0.43 | – |
| sidebar c=16 req/s | 13,837 | – |
| sidebar c=16 p50 ms | 1.11 | – |
| sidebar c=16 p99 ms | 2.34 | – |
| sidebar c=64 req/s | 14,173 | – |
| sidebar c=64 p50 ms | 4.49 | – |
| sidebar c=64 p99 ms | 7.64 | – |
| search c=1 req/s | 6,497 | – |
| search c=1 p50 ms | 0.14 | – |
| search c=1 p99 ms | 0.24 | – |
| search c=16 req/s | 26,258 | – |
| search c=16 p50 ms | 0.56 | – |
| search c=16 p99 ms | 1.48 | – |
| search c=64 req/s | 27,801 | – |
| search c=64 p50 ms | 2.25 | – |
| search c=64 p99 ms | 3.91 | – |
| avatar c=1 req/s | 78,085 | – |
| avatar c=1 p50 ms | 0.01 | – |
| avatar c=1 p99 ms | 0.02 | – |
| avatar c=16 req/s | 355,091 | – |
| avatar c=16 p50 ms | 0.03 | – |
| avatar c=16 p99 ms | 0.13 | – |
| avatar c=64 req/s | 365,426 | – |
| avatar c=64 p50 ms | 0.13 | – |
| avatar c=64 p99 ms | 1.30 | – |
| static_css c=1 req/s | 83,688 | – |
| static_css c=1 p50 ms | 0.01 | – |
| static_css c=1 p99 ms | 0.02 | – |
| static_css c=16 req/s | 382,519 | – |
| static_css c=16 p50 ms | 0.03 | – |
| static_css c=16 p99 ms | 0.12 | – |
| static_css c=64 req/s | 393,832 | – |
| static_css c=64 p50 ms | 0.13 | – |
| static_css c=64 p99 ms | 1.12 | – |
| up c=1 req/s | 31,874 | – |
| up c=1 p50 ms | 0.03 | – |
| up c=1 p99 ms | 0.05 | – |
| up c=16 req/s | 149,211 | – |
| up c=16 p50 ms | 0.11 | – |
| up c=16 p99 ms | 0.19 | – |
| up c=64 req/s | 150,567 | – |
| up c=64 p50 ms | 0.41 | – |
| up c=64 p99 ms | 0.88 | – |
| post_message c=1 req/s | 2,141 | – |
| post_message c=1 p50 ms | 0.37 | – |
| post_message c=1 p99 ms | 1.36 | – |
| post_message c=16 req/s | 4,941 | – |
| post_message c=16 p50 ms | 2.27 | – |
| post_message c=16 p99 ms | 39.0 | – |
| post_message c=64 req/s | 4,914 | – |
| post_message c=64 p50 ms | 9.72 | – |
| post_message c=64 p99 ms | 58.5 | – |

### HTTP errors / non-2xx-3xx (first rep, per app)

| Metric | Rust | Rust adv. |
|---|---|---|
- rust: none

### Action Cable fan-out (one room; chatter.js subscriptions per client)

| Metric | Rust | Rust adv. |
|---|---|---|
| 100 clients: subscribed | 100 | – |
| 100 clients: connect+subscribe all (s) | 0.06 | – |
| 100 clients: paced post→one client p50 ms | 1.11 | – |
| 100 clients: paced post→all clients p50 ms | 1.35 | – |
| 100 clients: paced post→all clients p99 ms | 1.77 | – |
| 100 clients: max sustained msgs/s (delivered to all) | 2,622 | – |
| 100 clients: deliveries/s (client×message) | 262,239 | – |
| 100 clients: saturated post→all p50 ms | 1.18 | – |
| 100 clients: saturated POST p50 ms | 1.13 | – |
| 1000 clients: subscribed | 1,000 | – |
| 1000 clients: connect+subscribe all (s) | 0.19 | – |
| 1000 clients: paced post→one client p50 ms | 3.88 | – |
| 1000 clients: paced post→all clients p50 ms | 6.74 | – |
| 1000 clients: paced post→all clients p99 ms | 15.9 | – |
| 1000 clients: max sustained msgs/s (delivered to all) | 412 | – |
| 1000 clients: deliveries/s (client×message) | 411,614 | – |
| 1000 clients: saturated post→all p50 ms | 19.5 | – |
| 1000 clients: saturated POST p50 ms | 9.35 | – |
| 5000 clients: subscribed | 5,000 | – |
| 5000 clients: connect+subscribe all (s) | 1.49 | – |
| 5000 clients: paced post→one client p50 ms | 12.8 | – |
| 5000 clients: paced post→all clients p50 ms | 24.4 | – |
| 5000 clients: paced post→all clients p99 ms | 43.8 | – |
| 5000 clients: max sustained msgs/s (delivered to all) | 87.1 | – |
| 5000 clients: deliveries/s (client×message) | 435,353 | – |
| 5000 clients: saturated post→all p50 ms | 146 | – |
| 5000 clients: saturated POST p50 ms | 27.4 | – |
| 10000 clients: subscribed | 10,000 | – |
| 10000 clients: connect+subscribe all (s) | 1.06 | – |
| 10000 clients: paced post→one client p50 ms | 24.4 | – |
| 10000 clients: paced post→all clients p50 ms | 54.7 | – |
| 10000 clients: paced post→all clients p99 ms | 79.4 | – |
| 10000 clients: max sustained msgs/s (delivered to all) | 47.9 | – |
| 10000 clients: deliveries/s (client×message) | 478,997 | – |
| 10000 clients: saturated post→all p50 ms | 293 | – |
| 10000 clients: saturated POST p50 ms | 38.7 | – |

### Upload + thumbnail (black_hole.jpg, 505 KB)

| Metric | Rust | Rust adv. |
|---|---|---|
| POST with attachment (ms) | 25.2 | – |
| then GET thumb → 200 (ms) | 0.20 | – |
| POST → thumbnail served (ms) | 25.4 | – |

### Memory during cable fan-out, by process (MB, peak within the phase)

App process: Rails' Puma master and workers (Action Cable runs in them), or Rust's one campfire
process (its front server included). Pss counts pages shared between forked workers once;
RssAnon counts them in every process.

| Metric | Rust | Rust adv. |
|---|---|---|
| 100 clients, all subscribed, idle: app process Pss | 165 | – |
| 100 clients, all subscribed, idle: app process RssAnon | 141 | – |
| 100 clients, all subscribed, idle: app + Redis + Thruster Pss | 165 | – |
| 100 clients, all subscribed, idle: whole container Pss | 165 | – |
| 100 clients, saturated fan-out: app process Pss | 179 | – |
| 100 clients, saturated fan-out: app process RssAnon | 155 | – |
| 100 clients, saturated fan-out: app + Redis + Thruster Pss | 179 | – |
| 100 clients, saturated fan-out: whole container Pss | 179 | – |
| 1000 clients, all subscribed, idle: app process Pss | 185 | – |
| 1000 clients, all subscribed, idle: app process RssAnon | 159 | – |
| 1000 clients, all subscribed, idle: app + Redis + Thruster Pss | 185 | – |
| 1000 clients, all subscribed, idle: whole container Pss | 185 | – |
| 1000 clients, saturated fan-out: app process Pss | 184 | – |
| 1000 clients, saturated fan-out: app process RssAnon | 158 | – |
| 1000 clients, saturated fan-out: app + Redis + Thruster Pss | 184 | – |
| 1000 clients, saturated fan-out: whole container Pss | 184 | – |
| 5000 clients, all subscribed, idle: app process Pss | 252 | – |
| 5000 clients, all subscribed, idle: app process RssAnon | 226 | – |
| 5000 clients, all subscribed, idle: app + Redis + Thruster Pss | 252 | – |
| 5000 clients, all subscribed, idle: whole container Pss | 252 | – |
| 5000 clients, saturated fan-out: app process Pss | 252 | – |
| 5000 clients, saturated fan-out: app process RssAnon | 226 | – |
| 5000 clients, saturated fan-out: app + Redis + Thruster Pss | 252 | – |
| 5000 clients, saturated fan-out: whole container Pss | 252 | – |
| 10000 clients, all subscribed, idle: app process Pss | 310 | – |
| 10000 clients, all subscribed, idle: app process RssAnon | 286 | – |
| 10000 clients, all subscribed, idle: app + Redis + Thruster Pss | 310 | – |
| 10000 clients, all subscribed, idle: whole container Pss | 310 | – |
| 10000 clients, saturated fan-out: app process Pss | 311 | – |
| 10000 clients, saturated fan-out: app process RssAnon | 287 | – |
| 10000 clients, saturated fan-out: app + Redis + Thruster Pss | 311 | – |
| 10000 clients, saturated fan-out: whole container Pss | 311 | – |
