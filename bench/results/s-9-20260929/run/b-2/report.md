```
date: 2026-09-29T12:58:33+02:00
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
| cold start: docker run → /up 200 (ms) | 435 | – |
| idle memory.current (MB) | 19.0 | – |
| idle anon (MB) | 11.0 | – |
| peak memory.current under load (MB) | 846 | – |
| peak anon under load (MB) | 450 | – |

### HTTP (signed in as david; keep-alive; c = concurrent connections)

| Metric | Rust | Rust adv. |
|---|---|---|
| room_show c=1 req/s | 4,860 | – |
| room_show c=1 p50 ms | 0.19 | – |
| room_show c=1 p99 ms | 0.34 | – |
| room_show c=16 req/s | 20,406 | – |
| room_show c=16 p50 ms | 0.75 | – |
| room_show c=16 p99 ms | 1.65 | – |
| room_show c=64 req/s | 20,160 | – |
| room_show c=64 p50 ms | 3.14 | – |
| room_show c=64 p99 ms | 5.49 | – |
| messages_page c=1 req/s | 6,072 | – |
| messages_page c=1 p50 ms | 0.15 | – |
| messages_page c=1 p99 ms | 0.26 | – |
| messages_page c=16 req/s | 25,763 | – |
| messages_page c=16 p50 ms | 0.58 | – |
| messages_page c=16 p99 ms | 1.50 | – |
| messages_page c=64 req/s | 26,083 | – |
| messages_page c=64 p50 ms | 2.40 | – |
| messages_page c=64 p99 ms | 4.19 | – |
| sidebar c=1 req/s | 3,168 | – |
| sidebar c=1 p50 ms | 0.30 | – |
| sidebar c=1 p99 ms | 0.45 | – |
| sidebar c=16 req/s | 13,727 | – |
| sidebar c=16 p50 ms | 1.12 | – |
| sidebar c=16 p99 ms | 2.34 | – |
| sidebar c=64 req/s | 13,860 | – |
| sidebar c=64 p50 ms | 4.58 | – |
| sidebar c=64 p99 ms | 7.98 | – |
| search c=1 req/s | 6,282 | – |
| search c=1 p50 ms | 0.15 | – |
| search c=1 p99 ms | 0.26 | – |
| search c=16 req/s | 24,289 | – |
| search c=16 p50 ms | 0.60 | – |
| search c=16 p99 ms | 1.63 | – |
| search c=64 req/s | 26,555 | – |
| search c=64 p50 ms | 2.33 | – |
| search c=64 p99 ms | 4.40 | – |
| avatar c=1 req/s | 75,818 | – |
| avatar c=1 p50 ms | 0.01 | – |
| avatar c=1 p99 ms | 0.02 | – |
| avatar c=16 req/s | 355,550 | – |
| avatar c=16 p50 ms | 0.03 | – |
| avatar c=16 p99 ms | 0.13 | – |
| avatar c=64 req/s | 371,187 | – |
| avatar c=64 p50 ms | 0.13 | – |
| avatar c=64 p99 ms | 1.32 | – |
| static_css c=1 req/s | 82,648 | – |
| static_css c=1 p50 ms | 0.01 | – |
| static_css c=1 p99 ms | 0.02 | – |
| static_css c=16 req/s | 370,937 | – |
| static_css c=16 p50 ms | 0.03 | – |
| static_css c=16 p99 ms | 0.13 | – |
| static_css c=64 req/s | 377,099 | – |
| static_css c=64 p50 ms | 0.13 | – |
| static_css c=64 p99 ms | 0.82 | – |
| up c=1 req/s | 31,234 | – |
| up c=1 p50 ms | 0.03 | – |
| up c=1 p99 ms | 0.06 | – |
| up c=16 req/s | 145,543 | – |
| up c=16 p50 ms | 0.11 | – |
| up c=16 p99 ms | 0.20 | – |
| up c=64 req/s | 148,459 | – |
| up c=64 p50 ms | 0.42 | – |
| up c=64 p99 ms | 0.90 | – |
| post_message c=1 req/s | 2,161 | – |
| post_message c=1 p50 ms | 0.37 | – |
| post_message c=1 p99 ms | 1.45 | – |
| post_message c=16 req/s | 4,866 | – |
| post_message c=16 p50 ms | 2.31 | – |
| post_message c=16 p99 ms | 41.5 | – |
| post_message c=64 req/s | 4,904 | – |
| post_message c=64 p50 ms | 9.89 | – |
| post_message c=64 p99 ms | 57.1 | – |

### HTTP errors / non-2xx-3xx (first rep, per app)

| Metric | Rust | Rust adv. |
|---|---|---|
- rust: none

### Action Cable fan-out (one room; chatter.js subscriptions per client)

| Metric | Rust | Rust adv. |
|---|---|---|
| 100 clients: subscribed | 100 | – |
| 100 clients: connect+subscribe all (s) | 0.06 | – |
| 100 clients: paced post→one client p50 ms | 1.10 | – |
| 100 clients: paced post→all clients p50 ms | 1.31 | – |
| 100 clients: paced post→all clients p99 ms | 2.37 | – |
| 100 clients: max sustained msgs/s (delivered to all) | 2,603 | – |
| 100 clients: deliveries/s (client×message) | 260,325 | – |
| 100 clients: saturated post→all p50 ms | 1.18 | – |
| 100 clients: saturated POST p50 ms | 1.14 | – |
| 1000 clients: subscribed | 1,000 | – |
| 1000 clients: connect+subscribe all (s) | 0.22 | – |
| 1000 clients: paced post→one client p50 ms | 3.52 | – |
| 1000 clients: paced post→all clients p50 ms | 6.72 | – |
| 1000 clients: paced post→all clients p99 ms | 10.0 | – |
| 1000 clients: max sustained msgs/s (delivered to all) | 399 | – |
| 1000 clients: deliveries/s (client×message) | 398,831 | – |
| 1000 clients: saturated post→all p50 ms | 19.7 | – |
| 1000 clients: saturated POST p50 ms | 9.65 | – |
| 5000 clients: subscribed | 5,000 | – |
| 5000 clients: connect+subscribe all (s) | 0.33 | – |
| 5000 clients: paced post→one client p50 ms | 13.3 | – |
| 5000 clients: paced post→all clients p50 ms | 28.4 | – |
| 5000 clients: paced post→all clients p99 ms | 41.7 | – |
| 5000 clients: max sustained msgs/s (delivered to all) | 86.5 | – |
| 5000 clients: deliveries/s (client×message) | 432,359 | – |
| 5000 clients: saturated post→all p50 ms | 139 | – |
| 5000 clients: saturated POST p50 ms | 28.8 | – |
| 10000 clients: subscribed | 10,000 | – |
| 10000 clients: connect+subscribe all (s) | 1.41 | – |
| 10000 clients: paced post→one client p50 ms | 26.3 | – |
| 10000 clients: paced post→all clients p50 ms | 56.0 | – |
| 10000 clients: paced post→all clients p99 ms | 91.3 | – |
| 10000 clients: max sustained msgs/s (delivered to all) | 37.7 | – |
| 10000 clients: deliveries/s (client×message) | 376,945 | – |
| 10000 clients: saturated post→all p50 ms | 390 | – |
| 10000 clients: saturated POST p50 ms | 49.9 | – |

### Upload + thumbnail (black_hole.jpg, 505 KB)

| Metric | Rust | Rust adv. |
|---|---|---|
| POST with attachment (ms) | 29.5 | – |
| then GET thumb → 200 (ms) | 0.20 | – |
| POST → thumbnail served (ms) | 29.7 | – |

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
| 100 clients, saturated fan-out: app process Pss | 180 | – |
| 100 clients, saturated fan-out: app process RssAnon | 155 | – |
| 100 clients, saturated fan-out: app + Redis + Thruster Pss | 180 | – |
| 100 clients, saturated fan-out: whole container Pss | 180 | – |
| 1000 clients, all subscribed, idle: app process Pss | 187 | – |
| 1000 clients, all subscribed, idle: app process RssAnon | 162 | – |
| 1000 clients, all subscribed, idle: app + Redis + Thruster Pss | 187 | – |
| 1000 clients, all subscribed, idle: whole container Pss | 187 | – |
| 1000 clients, saturated fan-out: app process Pss | 187 | – |
| 1000 clients, saturated fan-out: app process RssAnon | 161 | – |
| 1000 clients, saturated fan-out: app + Redis + Thruster Pss | 187 | – |
| 1000 clients, saturated fan-out: whole container Pss | 187 | – |
| 5000 clients, all subscribed, idle: app process Pss | 244 | – |
| 5000 clients, all subscribed, idle: app process RssAnon | 218 | – |
| 5000 clients, all subscribed, idle: app + Redis + Thruster Pss | 244 | – |
| 5000 clients, all subscribed, idle: whole container Pss | 244 | – |
| 5000 clients, saturated fan-out: app process Pss | 244 | – |
| 5000 clients, saturated fan-out: app process RssAnon | 218 | – |
| 5000 clients, saturated fan-out: app + Redis + Thruster Pss | 244 | – |
| 5000 clients, saturated fan-out: whole container Pss | 244 | – |
| 10000 clients, all subscribed, idle: app process Pss | 323 | – |
| 10000 clients, all subscribed, idle: app process RssAnon | 297 | – |
| 10000 clients, all subscribed, idle: app + Redis + Thruster Pss | 323 | – |
| 10000 clients, all subscribed, idle: whole container Pss | 323 | – |
| 10000 clients, saturated fan-out: app process Pss | 320 | – |
| 10000 clients, saturated fan-out: app process RssAnon | 294 | – |
| 10000 clients, saturated fan-out: app + Redis + Thruster Pss | 320 | – |
| 10000 clients, saturated fan-out: whole container Pss | 320 | – |
