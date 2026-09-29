```
date: 2026-09-29T18:36:10+02:00
host: 7.2.3-arch1-3, AMD Ryzen 9 9950X3D 16-Core Processor, 32 threads, 60GB
server cpus: 8-11 (nproc 4); loadgen cpus: 12-15; network: host
env: WEB_CONCURRENCY=3 JOB_CONCURRENCY=3 RAILS_MAX_THREADS=5 
rust extra env: 
rust image: campfire-rust:perf-1 sha256:4f753eaaeed7373f70ad9e3bc962e292389e9b0136cee6431733a6e68783f23c 2026-09-29T18:29:35.121728627+02:00
rust HEAD: 76030e4 (dirty: 0 files)
```

Reps: rust 1. Cells: median [min–max].

### Startup and memory

| Metric | Rust | Rust adv. |
|---|---|---|
| cold start: docker run → /up 200 (ms) | 438 | – |
| idle memory.current (MB) | 13.0 | – |
| idle anon (MB) | 11.0 | – |
| peak memory.current under load (MB) | 260 | – |
| peak anon under load (MB) | 153 | – |

### HTTP (signed in as david; keep-alive; c = concurrent connections)

| Metric | Rust | Rust adv. |
|---|---|---|
| room_show c=16 req/s | 21,756 | – |
| room_show c=16 p50 ms | 0.67 | – |
| room_show c=16 p99 ms | 2.15 | – |
| room_show c=64 req/s | 22,490 | – |
| room_show c=64 p50 ms | 2.73 | – |
| room_show c=64 p99 ms | 6.07 | – |
| room_show c=256 req/s | 22,042 | – |
| room_show c=256 p50 ms | 11.0 | – |
| room_show c=256 p99 ms | 20.9 | – |
| messages_page c=16 req/s | 23,547 | – |
| messages_page c=16 p50 ms | 0.60 | – |
| messages_page c=16 p99 ms | 2.06 | – |
| messages_page c=64 req/s | 25,826 | – |
| messages_page c=64 p50 ms | 2.34 | – |
| messages_page c=64 p99 ms | 5.47 | – |
| messages_page c=256 req/s | 25,220 | – |
| messages_page c=256 p50 ms | 9.92 | – |
| messages_page c=256 p99 ms | 16.1 | – |
| sidebar c=16 req/s | 13,239 | – |
| sidebar c=16 p50 ms | 1.11 | – |
| sidebar c=16 p99 ms | 3.27 | – |
| sidebar c=64 req/s | 14,114 | – |
| sidebar c=64 p50 ms | 4.32 | – |
| sidebar c=64 p99 ms | 9.71 | – |
| sidebar c=256 req/s | 14,741 | – |
| sidebar c=256 p50 ms | 16.6 | – |
| sidebar c=256 p99 ms | 31.6 | – |
| search c=16 req/s | 24,807 | – |
| search c=16 p50 ms | 0.59 | – |
| search c=16 p99 ms | 1.55 | – |
| search c=64 req/s | 26,715 | – |
| search c=64 p50 ms | 2.28 | – |
| search c=64 p99 ms | 4.95 | – |
| search c=256 req/s | 25,167 | – |
| search c=256 p50 ms | 10.0 | – |
| search c=256 p99 ms | 15.0 | – |
| avatar c=16 req/s | 350,035 | – |
| avatar c=16 p50 ms | 0.03 | – |
| avatar c=16 p99 ms | 0.13 | – |
| avatar c=64 req/s | 360,536 | – |
| avatar c=64 p50 ms | 0.14 | – |
| avatar c=64 p99 ms | 1.27 | – |
| avatar c=256 req/s | 378,375 | – |
| avatar c=256 p50 ms | 0.50 | – |
| avatar c=256 p99 ms | 2.56 | – |
| static_css c=16 req/s | 374,077 | – |
| static_css c=16 p50 ms | 0.03 | – |
| static_css c=16 p99 ms | 0.13 | – |
| static_css c=64 req/s | 383,212 | – |
| static_css c=64 p50 ms | 0.13 | – |
| static_css c=64 p99 ms | 1.09 | – |
| static_css c=256 req/s | 403,334 | – |
| static_css c=256 p50 ms | 0.48 | – |
| static_css c=256 p99 ms | 2.52 | – |
| up c=16 req/s | 145,235 | – |
| up c=16 p50 ms | 0.11 | – |
| up c=16 p99 ms | 0.21 | – |
| up c=64 req/s | 149,493 | – |
| up c=64 p50 ms | 0.42 | – |
| up c=64 p99 ms | 0.89 | – |
| up c=256 req/s | 149,260 | – |
| up c=256 p50 ms | 1.60 | – |
| up c=256 p99 ms | 3.81 | – |
| post_message c=16 req/s | 4,755 | – |
| post_message c=16 p50 ms | 2.35 | – |
| post_message c=16 p99 ms | 40.0 | – |
| post_message c=64 req/s | 4,900 | – |
| post_message c=64 p50 ms | 10.0 | – |
| post_message c=64 p99 ms | 49.5 | – |
| post_message c=256 req/s | 4,938 | – |
| post_message c=256 p50 ms | 41.2 | – |
| post_message c=256 p99 ms | 87.3 | – |

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
