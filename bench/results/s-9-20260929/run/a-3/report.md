```
date: 2026-09-29T14:04:54+02:00
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
| cold start: docker run → /up 200 (ms) | 415 | – |
| idle memory.current (MB) | 30.0 | – |
| idle anon (MB) | 11.0 | – |
| peak memory.current under load (MB) | 874 | – |
| peak anon under load (MB) | 442 | – |

### HTTP (signed in as david; keep-alive; c = concurrent connections)

| Metric | Rust | Rust adv. |
|---|---|---|
| room_show c=1 req/s | 5,473 | – |
| room_show c=1 p50 ms | 0.17 | – |
| room_show c=1 p99 ms | 0.27 | – |
| room_show c=16 req/s | 22,522 | – |
| room_show c=16 p50 ms | 0.68 | – |
| room_show c=16 p99 ms | 1.53 | – |
| room_show c=64 req/s | 21,542 | – |
| room_show c=64 p50 ms | 2.87 | – |
| room_show c=64 p99 ms | 5.54 | – |
| messages_page c=1 req/s | 5,962 | – |
| messages_page c=1 p50 ms | 0.16 | – |
| messages_page c=1 p99 ms | 0.26 | – |
| messages_page c=16 req/s | 26,031 | – |
| messages_page c=16 p50 ms | 0.57 | – |
| messages_page c=16 p99 ms | 1.47 | – |
| messages_page c=64 req/s | 26,489 | – |
| messages_page c=64 p50 ms | 2.37 | – |
| messages_page c=64 p99 ms | 4.13 | – |
| sidebar c=1 req/s | 3,048 | – |
| sidebar c=1 p50 ms | 0.32 | – |
| sidebar c=1 p99 ms | 0.47 | – |
| sidebar c=16 req/s | 13,478 | – |
| sidebar c=16 p50 ms | 1.14 | – |
| sidebar c=16 p99 ms | 2.38 | – |
| sidebar c=64 req/s | 14,027 | – |
| sidebar c=64 p50 ms | 4.53 | – |
| sidebar c=64 p99 ms | 7.76 | – |
| search c=1 req/s | 6,421 | – |
| search c=1 p50 ms | 0.14 | – |
| search c=1 p99 ms | 0.25 | – |
| search c=16 req/s | 26,049 | – |
| search c=16 p50 ms | 0.57 | – |
| search c=16 p99 ms | 1.48 | – |
| search c=64 req/s | 27,641 | – |
| search c=64 p50 ms | 2.26 | – |
| search c=64 p99 ms | 3.90 | – |
| avatar c=1 req/s | 77,509 | – |
| avatar c=1 p50 ms | 0.01 | – |
| avatar c=1 p99 ms | 0.02 | – |
| avatar c=16 req/s | 366,038 | – |
| avatar c=16 p50 ms | 0.03 | – |
| avatar c=16 p99 ms | 0.12 | – |
| avatar c=64 req/s | 373,621 | – |
| avatar c=64 p50 ms | 0.13 | – |
| avatar c=64 p99 ms | 1.29 | – |
| static_css c=1 req/s | 83,186 | – |
| static_css c=1 p50 ms | 0.01 | – |
| static_css c=1 p99 ms | 0.02 | – |
| static_css c=16 req/s | 386,337 | – |
| static_css c=16 p50 ms | 0.03 | – |
| static_css c=16 p99 ms | 0.12 | – |
| static_css c=64 req/s | 396,059 | – |
| static_css c=64 p50 ms | 0.13 | – |
| static_css c=64 p99 ms | 1.25 | – |
| up c=1 req/s | 31,445 | – |
| up c=1 p50 ms | 0.03 | – |
| up c=1 p99 ms | 0.06 | – |
| up c=16 req/s | 148,866 | – |
| up c=16 p50 ms | 0.11 | – |
| up c=16 p99 ms | 0.19 | – |
| up c=64 req/s | 151,228 | – |
| up c=64 p50 ms | 0.41 | – |
| up c=64 p99 ms | 0.88 | – |
| post_message c=1 req/s | 2,113 | – |
| post_message c=1 p50 ms | 0.38 | – |
| post_message c=1 p99 ms | 1.50 | – |
| post_message c=16 req/s | 4,797 | – |
| post_message c=16 p50 ms | 2.29 | – |
| post_message c=16 p99 ms | 43.9 | – |
| post_message c=64 req/s | 4,851 | – |
| post_message c=64 p50 ms | 9.81 | – |
| post_message c=64 p99 ms | 55.3 | – |

### HTTP errors / non-2xx-3xx (first rep, per app)

| Metric | Rust | Rust adv. |
|---|---|---|
- rust: none

### Action Cable fan-out (one room; chatter.js subscriptions per client)

| Metric | Rust | Rust adv. |
|---|---|---|
| 100 clients: subscribed | 100 | – |
| 100 clients: connect+subscribe all (s) | 0.06 | – |
| 100 clients: paced post→one client p50 ms | 1.16 | – |
| 100 clients: paced post→all clients p50 ms | 1.43 | – |
| 100 clients: paced post→all clients p99 ms | 9.01 | – |
| 100 clients: max sustained msgs/s (delivered to all) | 2,687 | – |
| 100 clients: deliveries/s (client×message) | 268,715 | – |
| 100 clients: saturated post→all p50 ms | 1.15 | – |
| 100 clients: saturated POST p50 ms | 1.11 | – |
| 1000 clients: subscribed | 1,000 | – |
| 1000 clients: connect+subscribe all (s) | 0.11 | – |
| 1000 clients: paced post→one client p50 ms | 3.72 | – |
| 1000 clients: paced post→all clients p50 ms | 6.40 | – |
| 1000 clients: paced post→all clients p99 ms | 9.48 | – |
| 1000 clients: max sustained msgs/s (delivered to all) | 396 | – |
| 1000 clients: deliveries/s (client×message) | 396,244 | – |
| 1000 clients: saturated post→all p50 ms | 19.5 | – |
| 1000 clients: saturated POST p50 ms | 9.67 | – |
| 5000 clients: subscribed | 5,000 | – |
| 5000 clients: connect+subscribe all (s) | 1.45 | – |
| 5000 clients: paced post→one client p50 ms | 12.5 | – |
| 5000 clients: paced post→all clients p50 ms | 26.6 | – |
| 5000 clients: paced post→all clients p99 ms | 42.7 | – |
| 5000 clients: max sustained msgs/s (delivered to all) | 87.6 | – |
| 5000 clients: deliveries/s (client×message) | 437,913 | – |
| 5000 clients: saturated post→all p50 ms | 143 | – |
| 5000 clients: saturated POST p50 ms | 29.8 | – |
| 10000 clients: subscribed | 10,000 | – |
| 10000 clients: connect+subscribe all (s) | 1.46 | – |
| 10000 clients: paced post→one client p50 ms | 24.7 | – |
| 10000 clients: paced post→all clients p50 ms | 53.1 | – |
| 10000 clients: paced post→all clients p99 ms | 87.2 | – |
| 10000 clients: max sustained msgs/s (delivered to all) | 49.5 | – |
| 10000 clients: deliveries/s (client×message) | 494,675 | – |
| 10000 clients: saturated post→all p50 ms | 317 | – |
| 10000 clients: saturated POST p50 ms | 37.6 | – |

### Upload + thumbnail (black_hole.jpg, 505 KB)

| Metric | Rust | Rust adv. |
|---|---|---|
| POST with attachment (ms) | 24.5 | – |
| then GET thumb → 200 (ms) | 0.20 | – |
| POST → thumbnail served (ms) | 24.7 | – |

### Memory during cable fan-out, by process (MB, peak within the phase)

App process: Rails' Puma master and workers (Action Cable runs in them), or Rust's one campfire
process (its front server included). Pss counts pages shared between forked workers once;
RssAnon counts them in every process.

| Metric | Rust | Rust adv. |
|---|---|---|
| 100 clients, all subscribed, idle: app process Pss | 157 | – |
| 100 clients, all subscribed, idle: app process RssAnon | 133 | – |
| 100 clients, all subscribed, idle: app + Redis + Thruster Pss | 157 | – |
| 100 clients, all subscribed, idle: whole container Pss | 157 | – |
| 100 clients, saturated fan-out: app process Pss | 170 | – |
| 100 clients, saturated fan-out: app process RssAnon | 145 | – |
| 100 clients, saturated fan-out: app + Redis + Thruster Pss | 170 | – |
| 100 clients, saturated fan-out: whole container Pss | 170 | – |
| 1000 clients, all subscribed, idle: app process Pss | 182 | – |
| 1000 clients, all subscribed, idle: app process RssAnon | 157 | – |
| 1000 clients, all subscribed, idle: app + Redis + Thruster Pss | 182 | – |
| 1000 clients, all subscribed, idle: whole container Pss | 182 | – |
| 1000 clients, saturated fan-out: app process Pss | 182 | – |
| 1000 clients, saturated fan-out: app process RssAnon | 157 | – |
| 1000 clients, saturated fan-out: app + Redis + Thruster Pss | 182 | – |
| 1000 clients, saturated fan-out: whole container Pss | 182 | – |
| 5000 clients, all subscribed, idle: app process Pss | 244 | – |
| 5000 clients, all subscribed, idle: app process RssAnon | 218 | – |
| 5000 clients, all subscribed, idle: app + Redis + Thruster Pss | 244 | – |
| 5000 clients, all subscribed, idle: whole container Pss | 244 | – |
| 5000 clients, saturated fan-out: app process Pss | 244 | – |
| 5000 clients, saturated fan-out: app process RssAnon | 218 | – |
| 5000 clients, saturated fan-out: app + Redis + Thruster Pss | 244 | – |
| 5000 clients, saturated fan-out: whole container Pss | 244 | – |
| 10000 clients, all subscribed, idle: app process Pss | 329 | – |
| 10000 clients, all subscribed, idle: app process RssAnon | 304 | – |
| 10000 clients, all subscribed, idle: app + Redis + Thruster Pss | 329 | – |
| 10000 clients, all subscribed, idle: whole container Pss | 329 | – |
| 10000 clients, saturated fan-out: app process Pss | 325 | – |
| 10000 clients, saturated fan-out: app process RssAnon | 299 | – |
| 10000 clients, saturated fan-out: app + Redis + Thruster Pss | 325 | – |
| 10000 clients, saturated fan-out: whole container Pss | 325 | – |
