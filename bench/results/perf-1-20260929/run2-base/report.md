```
date: 2026-09-29T14:44:22+02:00
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
| cold start: docker run → /up 200 (ms) | 1,005 [482–13,777] | – |
| idle memory.current (MB) | 38.0 [16.0–68.0] | – |
| idle anon (MB) | 11.0 [11.0–11.0] | – |
| peak memory.current under load (MB) | 232 [228–318] | – |
| peak anon under load (MB) | 131 [101–143] | – |

### HTTP (signed in as david; keep-alive; c = concurrent connections)

| Metric | Rust | Rust adv. |
|---|---|---|
| room_show c=16 req/s | 19,282 [11,348–20,454] | – |
| room_show c=16 p50 ms | 0.75 [0.73–0.92] | – |
| room_show c=16 p99 ms | 2.29 [1.82–5.80] | – |
| room_show c=64 req/s | 21,143 [11,659–21,324] | – |
| room_show c=64 p50 ms | 2.93 [2.85–3.50] | – |
| room_show c=64 p99 ms | 6.11 [5.83–15.16] | – |
| messages_page c=16 req/s | 24,234 [3,195–25,336] | – |
| messages_page c=16 p50 ms | 0.59 [0.58–0.67] | – |
| messages_page c=16 p99 ms | 1.77 [1.60–3.96] | – |
| messages_page c=64 req/s | 17,160 [3,570–26,089] | – |
| messages_page c=64 p50 ms | 2.38 [2.36–3.49] | – |
| messages_page c=64 p99 ms | 4.91 [4.73–487.17] | – |
| sidebar c=16 req/s | 12,107 [10,443–12,551] | – |
| sidebar c=16 p50 ms | 1.24 [1.19–1.37] | – |
| sidebar c=16 p99 ms | 2.92 [2.92–4.03] | – |
| sidebar c=64 req/s | 11,125 [5,399–13,293] | – |
| sidebar c=64 p50 ms | 5.59 [4.72–6.10] | – |
| sidebar c=64 p99 ms | 10.7 [8.9–38.0] | – |
| search c=16 req/s | 20,979 [2,703–25,162] | – |
| search c=16 p50 ms | 0.68 [0.58–0.79] | – |
| search c=16 p99 ms | 2.06 [1.56–4.38] | – |
| search c=64 req/s | 21,838 [3,992–26,010] | – |
| search c=64 p50 ms | 2.51 [2.37–3.08] | – |
| search c=64 p99 ms | 6.08 [4.66–487.17] | – |
| avatar c=16 req/s | 290,995 [187,883–306,842] | – |
| avatar c=16 p50 ms | 0.04 [0.04–0.04] | – |
| avatar c=16 p99 ms | 0.17 [0.15–0.38] | – |
| avatar c=64 req/s | 300,504 [292,798–309,460] | – |
| avatar c=64 p50 ms | 0.14 [0.14–0.15] | – |
| avatar c=64 p99 ms | 1.25 [1.17–1.26] | – |
| static_css c=16 req/s | 256,962 [5,412–375,146] | – |
| static_css c=16 p50 ms | 0.03 [0.03–0.04] | – |
| static_css c=16 p99 ms | 0.17 [0.12–0.19] | – |
| static_css c=64 req/s | 288,341 [2,129–388,627] | – |
| static_css c=64 p50 ms | 0.17 [0.13–0.20] | – |
| static_css c=64 p99 ms | 1.09 [0.78–1249.28] | – |
| up c=16 req/s | 133,167 [3,934–138,097] | – |
| up c=16 p50 ms | 0.11 [0.11–0.12] | – |
| up c=16 p99 ms | 0.25 [0.23–0.72] | – |
| up c=64 req/s | 149,227 [3,264–150,906] | – |
| up c=64 p50 ms | 0.41 [0.41–0.42] | – |
| up c=64 p99 ms | 0.92 [0.89–962.05] | – |
| post_message c=16 req/s | 3,852 [1,731–4,560] | – |
| post_message c=16 p50 ms | 2.34 [2.28–3.24] | – |
| post_message c=16 p99 ms | 53.4 [47.7–85.2] | – |
| post_message c=64 req/s | 4,672 [36–4,790] | – |
| post_message c=64 p50 ms | 9.93 [9.78–251.26] | – |
| post_message c=64 p99 ms | 60.6 [59.4–6635.5] | – |

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
