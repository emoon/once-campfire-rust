```
date: 2026-09-29T18:22:38+02:00
host: 7.2.3-arch1-3, AMD Ryzen 9 9950X3D 16-Core Processor, 32 threads, 60GB
server cpus: 8-11 (nproc 4); loadgen cpus: 12-15; network: host
env: WEB_CONCURRENCY=3 JOB_CONCURRENCY=3 RAILS_MAX_THREADS=5 
rust extra env: 
rust image: campfire-rust:perf-1 sha256:2f8906ede435c36f438a2039c69e898ed67b84868b40e30c22eb1f7eaff6198e 2026-09-29T13:26:50.200167137+02:00
rust HEAD: d4a0e74 (dirty: 0 files)
```

Reps: rust 1. Cells: median [min–max].

### Startup and memory

| Metric | Rust | Rust adv. |
|---|---|---|
| cold start: docker run → /up 200 (ms) | 554 | – |
| idle memory.current (MB) | 71.0 | – |
| idle anon (MB) | 11.0 | – |
| peak memory.current under load (MB) | 339 | – |
| peak anon under load (MB) | 163 | – |

### HTTP (signed in as david; keep-alive; c = concurrent connections)

| Metric | Rust | Rust adv. |
|---|---|---|
| room_show c=16 req/s | 22,061 | – |
| room_show c=16 p50 ms | 0.69 | – |
| room_show c=16 p99 ms | 1.60 | – |
| room_show c=64 req/s | 22,003 | – |
| room_show c=64 p50 ms | 2.87 | – |
| room_show c=64 p99 ms | 5.05 | – |
| room_show c=256 req/s | 20,887 | – |
| room_show c=256 p50 ms | 12.1 | – |
| room_show c=256 p99 ms | 19.0 | – |
| messages_page c=16 req/s | 23,029 | – |
| messages_page c=16 p50 ms | 0.64 | – |
| messages_page c=16 p99 ms | 1.71 | – |
| messages_page c=64 req/s | 24,986 | – |
| messages_page c=64 p50 ms | 2.49 | – |
| messages_page c=64 p99 ms | 4.51 | – |
| messages_page c=256 req/s | 23,774 | – |
| messages_page c=256 p50 ms | 10.8 | – |
| messages_page c=256 p99 ms | 15.7 | – |
| sidebar c=16 req/s | 13,309 | – |
| sidebar c=16 p50 ms | 1.15 | – |
| sidebar c=16 p99 ms | 2.46 | – |
| sidebar c=64 req/s | 13,699 | – |
| sidebar c=64 p50 ms | 4.63 | – |
| sidebar c=64 p99 ms | 7.97 | – |
| sidebar c=256 req/s | 13,129 | – |
| sidebar c=256 p50 ms | 19.6 | – |
| sidebar c=256 p99 ms | 32.1 | – |
| search c=16 req/s | 24,416 | – |
| search c=16 p50 ms | 0.61 | – |
| search c=16 p99 ms | 1.52 | – |
| search c=64 req/s | 26,523 | – |
| search c=64 p50 ms | 2.35 | – |
| search c=64 p99 ms | 4.21 | – |
| search c=256 req/s | 24,537 | – |
| search c=256 p50 ms | 10.4 | – |
| search c=256 p99 ms | 14.8 | – |
| avatar c=16 req/s | 347,555 | – |
| avatar c=16 p50 ms | 0.03 | – |
| avatar c=16 p99 ms | 0.13 | – |
| avatar c=64 req/s | 360,196 | – |
| avatar c=64 p50 ms | 0.14 | – |
| avatar c=64 p99 ms | 1.23 | – |
| avatar c=256 req/s | 380,221 | – |
| avatar c=256 p50 ms | 0.50 | – |
| avatar c=256 p99 ms | 2.60 | – |
| static_css c=16 req/s | 371,958 | – |
| static_css c=16 p50 ms | 0.03 | – |
| static_css c=16 p99 ms | 0.12 | – |
| static_css c=64 req/s | 386,426 | – |
| static_css c=64 p50 ms | 0.13 | – |
| static_css c=64 p99 ms | 1.20 | – |
| static_css c=256 req/s | 399,156 | – |
| static_css c=256 p50 ms | 0.48 | – |
| static_css c=256 p99 ms | 2.53 | – |
| up c=16 req/s | 147,700 | – |
| up c=16 p50 ms | 0.11 | – |
| up c=16 p99 ms | 0.20 | – |
| up c=64 req/s | 150,982 | – |
| up c=64 p50 ms | 0.41 | – |
| up c=64 p99 ms | 0.88 | – |
| up c=256 req/s | 152,679 | – |
| up c=256 p50 ms | 1.56 | – |
| up c=256 p99 ms | 3.76 | – |
| post_message c=16 req/s | 4,819 | – |
| post_message c=16 p50 ms | 2.29 | – |
| post_message c=16 p99 ms | 43.5 | – |
| post_message c=64 req/s | 4,784 | – |
| post_message c=64 p50 ms | 10.00 | – |
| post_message c=64 p99 ms | 58.0 | – |
| post_message c=256 req/s | 4,712 | – |
| post_message c=256 p50 ms | 41.4 | – |
| post_message c=256 p99 ms | 102 | – |

### HTTP errors / non-2xx-3xx (first rep, per app)

| Metric | Rust | Rust adv. |
|---|---|---|
- rust: none

### Action Cable fan-out (one room; chatter.js subscriptions per client)

| Metric | Rust | Rust adv. |
|---|---|---|

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
