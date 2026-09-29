```
date: 2026-09-29T15:44:40+02:00
host: 7.2.3-arch1-3, AMD Ryzen 9 9950X3D 16-Core Processor, 32 threads, 60GB
server cpus: 8-11 (nproc 4); loadgen cpus: 12-15; network: host
env: WEB_CONCURRENCY=3 JOB_CONCURRENCY=3 RAILS_MAX_THREADS=5 
rust extra env: 
rust image: campfire-rust:perf-1 sha256:2f8906ede435c36f438a2039c69e898ed67b84868b40e30c22eb1f7eaff6198e 2026-09-29T13:26:50.200167137+02:00
rust HEAD: a49dc9b (dirty: 0 files)
```

Reps: rust 3. Cells: median [min–max].

### Startup and memory

| Metric | Rust | Rust adv. |
|---|---|---|
| cold start: docker run → /up 200 (ms) | 422 [396–427] | – |
| idle memory.current (MB) | 14.0 [14.0–14.0] | – |
| idle anon (MB) | 11.0 [11.0–11.0] | – |
| peak memory.current under load (MB) | 238 [226–238] | – |
| peak anon under load (MB) | 141 [129–146] | – |

### HTTP (signed in as david; keep-alive; c = concurrent connections)

| Metric | Rust | Rust adv. |
|---|---|---|
| room_show c=16 req/s | 22,732 [22,666–22,947] | – |
| room_show c=16 p50 ms | 0.67 [0.66–0.67] | – |
| room_show c=16 p99 ms | 1.52 [1.49–1.52] | – |
| room_show c=64 req/s | 22,744 [22,725–22,789] | – |
| room_show c=64 p50 ms | 2.79 [2.78–2.79] | – |
| room_show c=64 p99 ms | 4.78 [4.77–4.83] | – |
| messages_page c=16 req/s | 25,964 [25,830–26,160] | – |
| messages_page c=16 p50 ms | 0.57 [0.57–0.57] | – |
| messages_page c=16 p99 ms | 1.48 [1.45–1.50] | – |
| messages_page c=64 req/s | 26,499 [26,454–26,718] | – |
| messages_page c=64 p50 ms | 2.36 [2.34–2.38] | – |
| messages_page c=64 p99 ms | 4.16 [4.08–4.20] | – |
| sidebar c=16 req/s | 13,854 [13,710–13,961] | – |
| sidebar c=16 p50 ms | 1.11 [1.10–1.12] | – |
| sidebar c=16 p99 ms | 2.34 [2.30–2.37] | – |
| sidebar c=64 req/s | 14,244 [14,059–14,253] | – |
| sidebar c=64 p50 ms | 4.46 [4.46–4.53] | – |
| sidebar c=64 p99 ms | 7.58 [7.58–7.65] | – |
| search c=16 req/s | 25,648 [25,565–26,025] | – |
| search c=16 p50 ms | 0.57 [0.57–0.58] | – |
| search c=16 p99 ms | 1.50 [1.49–1.50] | – |
| search c=64 req/s | 27,304 [27,235–27,595] | – |
| search c=64 p50 ms | 2.28 [2.26–2.29] | – |
| search c=64 p99 ms | 4.05 [3.97–4.11] | – |
| avatar c=16 req/s | 355,404 [353,313–358,060] | – |
| avatar c=16 p50 ms | 0.03 [0.03–0.03] | – |
| avatar c=16 p99 ms | 0.13 [0.13–0.13] | – |
| avatar c=64 req/s | 367,962 [365,582–372,691] | – |
| avatar c=64 p50 ms | 0.13 [0.13–0.13] | – |
| avatar c=64 p99 ms | 1.27 [1.25–1.28] | – |
| static_css c=16 req/s | 379,973 [377,967–381,710] | – |
| static_css c=16 p50 ms | 0.03 [0.03–0.03] | – |
| static_css c=16 p99 ms | 0.12 [0.12–0.12] | – |
| static_css c=64 req/s | 391,564 [387,184–391,987] | – |
| static_css c=64 p50 ms | 0.13 [0.13–0.13] | – |
| static_css c=64 p99 ms | 1.02 [0.99–1.29] | – |
| up c=16 req/s | 148,914 [148,279–150,961] | – |
| up c=16 p50 ms | 0.11 [0.11–0.11] | – |
| up c=16 p99 ms | 0.19 [0.19–0.19] | – |
| up c=64 req/s | 151,401 [150,175–152,774] | – |
| up c=64 p50 ms | 0.41 [0.41–0.41] | – |
| up c=64 p99 ms | 0.87 [0.86–0.88] | – |
| post_message c=16 req/s | 4,932 [4,878–4,949] | – |
| post_message c=16 p50 ms | 2.26 [2.22–2.28] | – |
| post_message c=16 p99 ms | 41.2 [40.3–41.6] | – |
| post_message c=64 req/s | 4,993 [4,926–5,001] | – |
| post_message c=64 p50 ms | 9.84 [9.79–9.88] | – |
| post_message c=64 p99 ms | 52.3 [51.0–57.3] | – |

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
