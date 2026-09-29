```
date: 2026-09-29T11:39:52+02:00
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
| cold start: docker run → /up 200 (ms) | 455 | – |
| idle memory.current (MB) | 14.0 | – |
| idle anon (MB) | 11.0 | – |
| peak memory.current under load (MB) | 660 | – |
| peak anon under load (MB) | 453 | – |

### HTTP (signed in as david; keep-alive; c = concurrent connections)

| Metric | Rust | Rust adv. |
|---|---|---|
| room_show c=1 req/s | 5,669 | – |
| room_show c=1 p50 ms | 0.17 | – |
| room_show c=1 p99 ms | 0.26 | – |
| room_show c=16 req/s | 22,549 | – |
| room_show c=16 p50 ms | 0.68 | – |
| room_show c=16 p99 ms | 1.56 | – |
| room_show c=64 req/s | 22,614 | – |
| room_show c=64 p50 ms | 2.80 | – |
| room_show c=64 p99 ms | 4.82 | – |
| messages_page c=1 req/s | 6,377 | – |
| messages_page c=1 p50 ms | 0.15 | – |
| messages_page c=1 p99 ms | 0.23 | – |
| messages_page c=16 req/s | 26,282 | – |
| messages_page c=16 p50 ms | 0.57 | – |
| messages_page c=16 p99 ms | 1.44 | – |
| messages_page c=64 req/s | 26,348 | – |
| messages_page c=64 p50 ms | 2.38 | – |
| messages_page c=64 p99 ms | 4.10 | – |
| sidebar c=1 req/s | 3,202 | – |
| sidebar c=1 p50 ms | 0.30 | – |
| sidebar c=1 p99 ms | 0.47 | – |
| sidebar c=16 req/s | 13,770 | – |
| sidebar c=16 p50 ms | 1.12 | – |
| sidebar c=16 p99 ms | 2.36 | – |
| sidebar c=64 req/s | 14,081 | – |
| sidebar c=64 p50 ms | 4.51 | – |
| sidebar c=64 p99 ms | 7.66 | – |
| search c=1 req/s | 6,322 | – |
| search c=1 p50 ms | 0.15 | – |
| search c=1 p99 ms | 0.26 | – |
| search c=16 req/s | 23,444 | – |
| search c=16 p50 ms | 0.59 | – |
| search c=16 p99 ms | 2.27 | – |
| search c=64 req/s | 23,474 | – |
| search c=64 p50 ms | 2.58 | – |
| search c=64 p99 ms | 5.51 | – |
| avatar c=1 req/s | 15,402 | – |
| avatar c=1 p50 ms | 0.02 | – |
| avatar c=1 p99 ms | 0.52 | – |
| avatar c=16 req/s | 172,908 | – |
| avatar c=16 p50 ms | 0.04 | – |
| avatar c=16 p99 ms | 0.22 | – |
| avatar c=64 req/s | 340,073 | – |
| avatar c=64 p50 ms | 0.14 | – |
| avatar c=64 p99 ms | 1.07 | – |
| static_css c=1 req/s | 84,410 | – |
| static_css c=1 p50 ms | 0.01 | – |
| static_css c=1 p99 ms | 0.02 | – |
| static_css c=16 req/s | 363,573 | – |
| static_css c=16 p50 ms | 0.03 | – |
| static_css c=16 p99 ms | 0.13 | – |
| static_css c=64 req/s | 346,918 | – |
| static_css c=64 p50 ms | 0.14 | – |
| static_css c=64 p99 ms | 0.92 | – |
| up c=1 req/s | 29,883 | – |
| up c=1 p50 ms | 0.03 | – |
| up c=1 p99 ms | 0.09 | – |
| up c=16 req/s | 150,611 | – |
| up c=16 p50 ms | 0.11 | – |
| up c=16 p99 ms | 0.19 | – |
| up c=64 req/s | 153,300 | – |
| up c=64 p50 ms | 0.41 | – |
| up c=64 p99 ms | 0.86 | – |
| post_message c=1 req/s | 2,046 | – |
| post_message c=1 p50 ms | 0.38 | – |
| post_message c=1 p99 ms | 1.72 | – |
| post_message c=16 req/s | 4,776 | – |
| post_message c=16 p50 ms | 2.23 | – |
| post_message c=16 p99 ms | 49.8 | – |
| post_message c=64 req/s | 4,751 | – |
| post_message c=64 p50 ms | 9.69 | – |
| post_message c=64 p99 ms | 61.8 | – |

### HTTP errors / non-2xx-3xx (first rep, per app)

| Metric | Rust | Rust adv. |
|---|---|---|
- rust: none

### Action Cable fan-out (one room; chatter.js subscriptions per client)

| Metric | Rust | Rust adv. |
|---|---|---|
| 100 clients: subscribed | 100 | – |
| 100 clients: connect+subscribe all (s) | 0.06 | – |
| 100 clients: paced post→one client p50 ms | 1.14 | – |
| 100 clients: paced post→all clients p50 ms | 1.40 | – |
| 100 clients: paced post→all clients p99 ms | 1.74 | – |
| 100 clients: max sustained msgs/s (delivered to all) | 2,596 | – |
| 100 clients: deliveries/s (client×message) | 259,546 | – |
| 100 clients: saturated post→all p50 ms | 1.19 | – |
| 100 clients: saturated POST p50 ms | 1.14 | – |
| 1000 clients: subscribed | 1,000 | – |
| 1000 clients: connect+subscribe all (s) | 0.13 | – |
| 1000 clients: paced post→one client p50 ms | 3.58 | – |
| 1000 clients: paced post→all clients p50 ms | 6.22 | – |
| 1000 clients: paced post→all clients p99 ms | 22.6 | – |
| 1000 clients: max sustained msgs/s (delivered to all) | 407 | – |
| 1000 clients: deliveries/s (client×message) | 407,005 | – |
| 1000 clients: saturated post→all p50 ms | 20.2 | – |
| 1000 clients: saturated POST p50 ms | 9.33 | – |
| 5000 clients: subscribed | 5,000 | – |
| 5000 clients: connect+subscribe all (s) | 1.14 | – |
| 5000 clients: paced post→one client p50 ms | 13.1 | – |
| 5000 clients: paced post→all clients p50 ms | 29.5 | – |
| 5000 clients: paced post→all clients p99 ms | 40.5 | – |
| 5000 clients: max sustained msgs/s (delivered to all) | 77.8 | – |
| 5000 clients: deliveries/s (client×message) | 389,147 | – |
| 5000 clients: saturated post→all p50 ms | 145 | – |
| 5000 clients: saturated POST p50 ms | 33.6 | – |
| 10000 clients: subscribed | 10,000 | – |
| 10000 clients: connect+subscribe all (s) | 2.49 | – |
| 10000 clients: paced post→one client p50 ms | 57.8 | – |
| 10000 clients: paced post→all clients p50 ms | 131 | – |
| 10000 clients: paced post→all clients p99 ms | 236 | – |
| 10000 clients: max sustained msgs/s (delivered to all) | 20.7 | – |
| 10000 clients: deliveries/s (client×message) | 206,749 | – |
| 10000 clients: saturated post→all p50 ms | 402 | – |
| 10000 clients: saturated POST p50 ms | 103 | – |

### Upload + thumbnail (black_hole.jpg, 505 KB)

| Metric | Rust | Rust adv. |
|---|---|---|
| POST with attachment (ms) | 31.6 | – |
| then GET thumb → 200 (ms) | 0.30 | – |
| POST → thumbnail served (ms) | 31.8 | – |

### Memory during cable fan-out, by process (MB, peak within the phase)

App process: Rails' Puma master and workers (Action Cable runs in them), or Rust's one campfire
process (its front server included). Pss counts pages shared between forked workers once;
RssAnon counts them in every process.

| Metric | Rust | Rust adv. |
|---|---|---|
| 100 clients, all subscribed, idle: app process Pss | 172 | – |
| 100 clients, all subscribed, idle: app process RssAnon | 145 | – |
| 100 clients, all subscribed, idle: app + Redis + Thruster Pss | 172 | – |
| 100 clients, all subscribed, idle: whole container Pss | 172 | – |
| 100 clients, saturated fan-out: app process Pss | 185 | – |
| 100 clients, saturated fan-out: app process RssAnon | 158 | – |
| 100 clients, saturated fan-out: app + Redis + Thruster Pss | 185 | – |
| 100 clients, saturated fan-out: whole container Pss | 185 | – |
| 1000 clients, all subscribed, idle: app process Pss | 197 | – |
| 1000 clients, all subscribed, idle: app process RssAnon | 169 | – |
| 1000 clients, all subscribed, idle: app + Redis + Thruster Pss | 197 | – |
| 1000 clients, all subscribed, idle: whole container Pss | 197 | – |
| 1000 clients, saturated fan-out: app process Pss | 197 | – |
| 1000 clients, saturated fan-out: app process RssAnon | 169 | – |
| 1000 clients, saturated fan-out: app + Redis + Thruster Pss | 197 | – |
| 1000 clients, saturated fan-out: whole container Pss | 197 | – |
| 5000 clients, all subscribed, idle: app process Pss | 258 | – |
| 5000 clients, all subscribed, idle: app process RssAnon | 230 | – |
| 5000 clients, all subscribed, idle: app + Redis + Thruster Pss | 258 | – |
| 5000 clients, all subscribed, idle: whole container Pss | 258 | – |
| 5000 clients, saturated fan-out: app process Pss | 258 | – |
| 5000 clients, saturated fan-out: app process RssAnon | 230 | – |
| 5000 clients, saturated fan-out: app + Redis + Thruster Pss | 258 | – |
| 5000 clients, saturated fan-out: whole container Pss | 258 | – |
| 10000 clients, all subscribed, idle: app process Pss | 332 | – |
| 10000 clients, all subscribed, idle: app process RssAnon | 304 | – |
| 10000 clients, all subscribed, idle: app + Redis + Thruster Pss | 332 | – |
| 10000 clients, all subscribed, idle: whole container Pss | 332 | – |
| 10000 clients, saturated fan-out: app process Pss | 331 | – |
| 10000 clients, saturated fan-out: app process RssAnon | 303 | – |
| 10000 clients, saturated fan-out: app + Redis + Thruster Pss | 331 | – |
| 10000 clients, saturated fan-out: whole container Pss | 331 | – |
