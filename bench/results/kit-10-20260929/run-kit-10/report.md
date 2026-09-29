```
date: 2026-09-29T17:10:35+02:00
host: 7.2.3-arch1-3, AMD Ryzen 9 9950X3D 16-Core Processor, 32 threads, 60GB
server cpus: 8-11 (nproc 4); loadgen cpus: 12-15; network: host
env: WEB_CONCURRENCY=3 JOB_CONCURRENCY=3 RAILS_MAX_THREADS=5 
rust extra env: 
rust image: campfire-rust:kit-10 sha256:2f5d6ff7838c193cd74bdb6bdd4be51d21417e02d68de21746221da0c42dceba 2026-09-29T16:39:47.531885922+02:00
rust HEAD: 93d3d1f (dirty: 0 files)
```

Reps: rust 3. Cells: median [min–max].

### Startup and memory

| Metric | Rust | Rust adv. |
|---|---|---|
| cold start: docker run → /up 200 (ms) | 422 [421–450] | – |
| idle memory.current (MB) | 14.0 [13.0–20.0] | – |
| idle anon (MB) | 11.0 [11.0–11.0] | – |
| peak memory.current under load (MB) | 270 [258–278] | – |
| peak anon under load (MB) | 160 [151–170] | – |

### HTTP (signed in as david; keep-alive; c = concurrent connections)

| Metric | Rust | Rust adv. |
|---|---|---|
| room_show c=1 req/s | 5,559 [5,515–5,562] | – |
| room_show c=1 p50 ms | 0.17 [0.17–0.17] | – |
| room_show c=1 p99 ms | 0.27 [0.27–0.27] | – |
| room_show c=16 req/s | 22,620 [22,287–22,675] | – |
| room_show c=16 p50 ms | 0.68 [0.67–0.68] | – |
| room_show c=16 p99 ms | 1.51 [1.51–1.55] | – |
| room_show c=64 req/s | 22,564 [22,367–22,729] | – |
| room_show c=64 p50 ms | 2.81 [2.79–2.83] | – |
| room_show c=64 p99 ms | 4.83 [4.79–4.85] | – |
| messages_page c=1 req/s | 6,276 [6,231–6,286] | – |
| messages_page c=1 p50 ms | 0.15 [0.15–0.15] | – |
| messages_page c=1 p99 ms | 0.24 [0.24–0.25] | – |
| messages_page c=16 req/s | 25,997 [25,767–26,241] | – |
| messages_page c=16 p50 ms | 0.57 [0.57–0.58] | – |
| messages_page c=16 p99 ms | 1.48 [1.45–1.50] | – |
| messages_page c=64 req/s | 26,533 [26,266–26,730] | – |
| messages_page c=64 p50 ms | 2.35 [2.35–2.36] | – |
| messages_page c=64 p99 ms | 4.30 [4.05–4.53] | – |
| sidebar c=1 req/s | 5,919 [5,905–6,128] | – |
| sidebar c=1 p50 ms | 0.16 [0.15–0.16] | – |
| sidebar c=1 p99 ms | 0.25 [0.25–0.25] | – |
| sidebar c=16 req/s | 25,220 [25,218–25,563] | – |
| sidebar c=16 p50 ms | 0.58 [0.58–0.59] | – |
| sidebar c=16 p99 ms | 1.53 [1.51–1.53] | – |
| sidebar c=64 req/s | 26,703 [26,670–26,853] | – |
| sidebar c=64 p50 ms | 2.35 [2.33–2.35] | – |
| sidebar c=64 p99 ms | 4.19 [4.13–4.20] | – |
| search c=1 req/s | 6,638 [6,466–6,696] | – |
| search c=1 p50 ms | 0.14 [0.14–0.14] | – |
| search c=1 p99 ms | 0.24 [0.24–0.24] | – |
| search c=16 req/s | 25,969 [25,618–26,120] | – |
| search c=16 p50 ms | 0.57 [0.56–0.58] | – |
| search c=16 p99 ms | 1.51 [1.50–1.51] | – |
| search c=64 req/s | 27,225 [27,185–27,494] | – |
| search c=64 p50 ms | 2.29 [2.25–2.30] | – |
| search c=64 p99 ms | 4.05 [4.03–4.24] | – |
| avatar c=1 req/s | 78,389 [78,047–78,465] | – |
| avatar c=1 p50 ms | 0.01 [0.01–0.01] | – |
| avatar c=1 p99 ms | 0.02 [0.02–0.02] | – |
| avatar c=16 req/s | 358,392 [356,252–358,703] | – |
| avatar c=16 p50 ms | 0.03 [0.03–0.03] | – |
| avatar c=16 p99 ms | 0.13 [0.13–0.13] | – |
| avatar c=64 req/s | 363,559 [362,030–365,236] | – |
| avatar c=64 p50 ms | 0.13 [0.13–0.14] | – |
| avatar c=64 p99 ms | 1.29 [1.26–1.35] | – |
| static_css c=1 req/s | 84,098 [83,910–84,223] | – |
| static_css c=1 p50 ms | 0.01 [0.01–0.01] | – |
| static_css c=1 p99 ms | 0.02 [0.02–0.02] | – |
| static_css c=16 req/s | 380,715 [357,689–383,940] | – |
| static_css c=16 p50 ms | 0.03 [0.03–0.03] | – |
| static_css c=16 p99 ms | 0.12 [0.12–0.13] | – |
| static_css c=64 req/s | 387,403 [386,580–391,655] | – |
| static_css c=64 p50 ms | 0.13 [0.13–0.13] | – |
| static_css c=64 p99 ms | 1.31 [1.07–1.33] | – |
| up c=1 req/s | 46,218 [34,172–46,606] | – |
| up c=1 p50 ms | 0.02 [0.02–0.02] | – |
| up c=1 p99 ms | 0.04 [0.04–0.04] | – |
| up c=16 req/s | 231,468 [222,520–234,591] | – |
| up c=16 p50 ms | 0.07 [0.07–0.07] | – |
| up c=16 p99 ms | 0.12 [0.12–0.20] | – |
| up c=64 req/s | 234,955 [233,109–235,051] | – |
| up c=64 p50 ms | 0.27 [0.27–0.27] | – |
| up c=64 p99 ms | 0.57 [0.56–0.58] | – |
| post_message c=1 req/s | 2,144 [2,136–2,181] | – |
| post_message c=1 p50 ms | 0.38 [0.38–0.38] | – |
| post_message c=1 p99 ms | 1.41 [1.40–1.41] | – |
| post_message c=16 req/s | 4,962 [4,826–5,050] | – |
| post_message c=16 p50 ms | 2.27 [2.26–2.27] | – |
| post_message c=16 p99 ms | 41.2 [36.6–42.2] | – |
| post_message c=64 req/s | 4,910 [4,820–4,925] | – |
| post_message c=64 p50 ms | 9.78 [9.78–9.89] | – |
| post_message c=64 p99 ms | 53.9 [52.9–54.5] | – |

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
