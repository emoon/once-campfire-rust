```
date: 2026-09-29T14:30:55+02:00
host: 7.2.3-arch1-3, AMD Ryzen 9 9950X3D 16-Core Processor, 32 threads, 60GB
server cpus: 8-11 (nproc 4); loadgen cpus: 12-15; network: host
env: WEB_CONCURRENCY=3 JOB_CONCURRENCY=3 RAILS_MAX_THREADS=5 
rust extra env: 
rust image: campfire-rust:perf-1 sha256:645c0efeaa03ac38ed0b12718b20f5f79a38fa50edb37d557d470e7c307625f4 2026-09-29T13:46:57.797791654+02:00
rust HEAD: a49dc9b (dirty: 0 files)
```

Reps: rust 3. Cells: median [min–max].

### Startup and memory

| Metric | Rust | Rust adv. |
|---|---|---|
| cold start: docker run → /up 200 (ms) | 416 [393–498] | – |
| idle memory.current (MB) | 14.0 [13.0–72.0] | – |
| idle anon (MB) | 11.0 [11.0–11.0] | – |
| peak memory.current under load (MB) | 241 [226–286] | – |
| peak anon under load (MB) | 130 [128–138] | – |

### HTTP (signed in as david; keep-alive; c = concurrent connections)

| Metric | Rust | Rust adv. |
|---|---|---|
| room_show c=16 req/s | 26,415 [26,232–26,704] | – |
| room_show c=16 p50 ms | 0.61 [0.60–0.62] | – |
| room_show c=16 p99 ms | 1.07 [1.06–1.07] | – |
| room_show c=64 req/s | 26,373 [26,230–26,773] | – |
| room_show c=64 p50 ms | 2.36 [2.32–2.36] | – |
| room_show c=64 p99 ms | 4.96 [4.89–5.19] | – |
| messages_page c=16 req/s | 32,282 [32,118–32,840] | – |
| messages_page c=16 p50 ms | 0.50 [0.49–0.50] | – |
| messages_page c=16 p99 ms | 0.88 [0.86–0.88] | – |
| messages_page c=64 req/s | 32,328 [32,296–32,798] | – |
| messages_page c=64 p50 ms | 1.93 [1.90–1.93] | – |
| messages_page c=64 p99 ms | 4.02 [4.00–4.05] | – |
| sidebar c=16 req/s | 15,606 [15,565–15,693] | – |
| sidebar c=16 p50 ms | 1.03 [1.02–1.03] | – |
| sidebar c=16 p99 ms | 1.79 [1.79–1.80] | – |
| sidebar c=64 req/s | 15,588 [15,568–15,748] | – |
| sidebar c=64 p50 ms | 4.00 [3.95–4.00] | – |
| sidebar c=64 p99 ms | 8.33 [8.32–8.47] | – |
| search c=16 req/s | 30,411 [30,114–30,452] | – |
| search c=16 p50 ms | 0.51 [0.50–0.51] | – |
| search c=16 p99 ms | 1.21 [1.17–1.22] | – |
| search c=64 req/s | 29,627 [29,288–29,916] | – |
| search c=64 p50 ms | 2.08 [2.06–2.10] | – |
| search c=64 p99 ms | 4.78 [4.76–4.90] | – |
| avatar c=16 req/s | 344,088 [340,547–346,797] | – |
| avatar c=16 p50 ms | 0.04 [0.03–0.04] | – |
| avatar c=16 p99 ms | 0.14 [0.13–0.14] | – |
| avatar c=64 req/s | 357,497 [354,764–360,344] | – |
| avatar c=64 p50 ms | 0.14 [0.14–0.14] | – |
| avatar c=64 p99 ms | 1.15 [0.97–1.22] | – |
| static_css c=16 req/s | 365,068 [357,029–368,237] | – |
| static_css c=16 p50 ms | 0.03 [0.03–0.03] | – |
| static_css c=16 p99 ms | 0.13 [0.13–0.13] | – |
| static_css c=64 req/s | 382,763 [346,802–387,490] | – |
| static_css c=64 p50 ms | 0.13 [0.13–0.15] | – |
| static_css c=64 p99 ms | 0.79 [0.62–0.97] | – |
| up c=16 req/s | 145,974 [143,735–146,792] | – |
| up c=16 p50 ms | 0.11 [0.11–0.11] | – |
| up c=16 p99 ms | 0.21 [0.21–0.21] | – |
| up c=64 req/s | 148,385 [147,681–150,052] | – |
| up c=64 p50 ms | 0.42 [0.41–0.42] | – |
| up c=64 p99 ms | 0.90 [0.89–0.90] | – |
| post_message c=16 req/s | 5,059 [4,189–5,290] | – |
| post_message c=16 p50 ms | 2.08 [2.06–2.10] | – |
| post_message c=16 p99 ms | 44.1 [40.8–58.0] | – |
| post_message c=64 req/s | 5,106 [3,994–5,121] | – |
| post_message c=64 p50 ms | 9.30 [9.23–9.65] | – |
| post_message c=64 p99 ms | 55.6 [55.2–81.3] | – |

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
