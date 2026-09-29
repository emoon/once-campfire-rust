```
date: 2026-09-29T15:20:55+02:00
host: 7.2.3-arch1-3, AMD Ryzen 9 9950X3D 16-Core Processor, 32 threads, 60GB
server cpus: 8-11 (nproc 4); loadgen cpus: 12-15; network: host
env: WEB_CONCURRENCY=3 JOB_CONCURRENCY=3 RAILS_MAX_THREADS=5 
rust extra env: 
rust image: campfire-rust:kit-10 sha256:2f8906ede435c36f438a2039c69e898ed67b84868b40e30c22eb1f7eaff6198e 2026-09-29T13:26:50.200167137+02:00
rust HEAD: 93d3d1f (dirty: 0 files)
```

Reps: rust 3. Cells: median [min–max].

### Startup and memory

| Metric | Rust | Rust adv. |
|---|---|---|
| cold start: docker run → /up 200 (ms) | 462 [414–546] | – |
| idle memory.current (MB) | 13.0 [13.0–69.0] | – |
| idle anon (MB) | 11.0 [11.0–11.0] | – |
| peak memory.current under load (MB) | 267 [239–286] | – |
| peak anon under load (MB) | 133 [131–135] | – |

### HTTP (signed in as david; keep-alive; c = concurrent connections)

| Metric | Rust | Rust adv. |
|---|---|---|
| room_show c=1 req/s | 5,382 [4,343–5,443] | – |
| room_show c=1 p50 ms | 0.17 [0.17–0.18] | – |
| room_show c=1 p99 ms | 0.31 [0.28–1.16] | – |
| room_show c=16 req/s | 22,537 [20,659–22,699] | – |
| room_show c=16 p50 ms | 0.68 [0.67–0.71] | – |
| room_show c=16 p99 ms | 1.55 [1.49–2.14] | – |
| room_show c=64 req/s | 21,889 [20,076–22,737] | – |
| room_show c=64 p50 ms | 2.88 [2.78–3.06] | – |
| room_show c=64 p99 ms | 5.04 [4.81–6.65] | – |
| messages_page c=1 req/s | 6,111 [5,021–6,277] | – |
| messages_page c=1 p50 ms | 0.15 [0.15–0.17] | – |
| messages_page c=1 p99 ms | 0.25 [0.24–0.64] | – |
| messages_page c=16 req/s | 25,701 [18,534–25,807] | – |
| messages_page c=16 p50 ms | 0.58 [0.57–0.65] | – |
| messages_page c=16 p99 ms | 1.49 [1.47–3.73] | – |
| messages_page c=64 req/s | 25,176 [6,274–26,837] | – |
| messages_page c=64 p50 ms | 2.49 [2.32–2.60] | – |
| messages_page c=64 p99 ms | 4.31 [4.28–105.66] | – |
| sidebar c=1 req/s | 3,080 [2,760–3,205] | – |
| sidebar c=1 p50 ms | 0.30 [0.30–0.31] | – |
| sidebar c=1 p99 ms | 0.45 [0.44–0.75] | – |
| sidebar c=16 req/s | 13,460 [9,324–13,830] | – |
| sidebar c=16 p50 ms | 1.15 [1.11–1.18] | – |
| sidebar c=16 p99 ms | 2.37 [2.36–3.66] | – |
| sidebar c=64 req/s | 13,917 [12,618–14,183] | – |
| sidebar c=64 p50 ms | 4.57 [4.48–4.88] | – |
| sidebar c=64 p99 ms | 7.77 [7.62–10.46] | – |
| search c=1 req/s | 6,338 [5,075–6,490] | – |
| search c=1 p50 ms | 0.15 [0.14–0.16] | – |
| search c=1 p99 ms | 0.26 [0.25–0.76] | – |
| search c=16 req/s | 25,485 [25,417–26,028] | – |
| search c=16 p50 ms | 0.58 [0.57–0.58] | – |
| search c=16 p99 ms | 1.54 [1.49–1.56] | – |
| search c=64 req/s | 27,267 [25,661–27,337] | – |
| search c=64 p50 ms | 2.29 [2.25–2.32] | – |
| search c=64 p99 ms | 4.52 [4.01–4.82] | – |
| avatar c=1 req/s | 78,113 [68,138–78,160] | – |
| avatar c=1 p50 ms | 0.01 [0.01–0.01] | – |
| avatar c=1 p99 ms | 0.02 [0.02–0.04] | – |
| avatar c=16 req/s | 325,724 [322,840–358,603] | – |
| avatar c=16 p50 ms | 0.04 [0.03–0.04] | – |
| avatar c=16 p99 ms | 0.14 [0.13–0.15] | – |
| avatar c=64 req/s | 364,029 [360,388–365,650] | – |
| avatar c=64 p50 ms | 0.14 [0.13–0.14] | – |
| avatar c=64 p99 ms | 1.27 [1.26–1.29] | – |
| static_css c=1 req/s | 83,611 [78,670–83,740] | – |
| static_css c=1 p50 ms | 0.01 [0.01–0.01] | – |
| static_css c=1 p99 ms | 0.02 [0.02–0.02] | – |
| static_css c=16 req/s | 348,191 [305,905–378,345] | – |
| static_css c=16 p50 ms | 0.03 [0.03–0.04] | – |
| static_css c=16 p99 ms | 0.13 [0.12–0.14] | – |
| static_css c=64 req/s | 382,663 [380,952–388,837] | – |
| static_css c=64 p50 ms | 0.13 [0.13–0.13] | – |
| static_css c=64 p99 ms | 1.14 [1.00–1.27] | – |
| up c=1 req/s | 31,558 [30,041–31,844] | – |
| up c=1 p50 ms | 0.03 [0.03–0.03] | – |
| up c=1 p99 ms | 0.06 [0.06–0.08] | – |
| up c=16 req/s | 147,176 [146,927–148,834] | – |
| up c=16 p50 ms | 0.11 [0.11–0.11] | – |
| up c=16 p99 ms | 0.20 [0.19–0.20] | – |
| up c=64 req/s | 150,136 [148,418–150,602] | – |
| up c=64 p50 ms | 0.41 [0.41–0.42] | – |
| up c=64 p99 ms | 0.89 [0.88–0.90] | – |
| post_message c=1 req/s | 2,081 [2,019–2,136] | – |
| post_message c=1 p50 ms | 0.38 [0.37–0.39] | – |
| post_message c=1 p99 ms | 1.43 [1.42–1.54] | – |
| post_message c=16 req/s | 4,771 [4,657–4,979] | – |
| post_message c=16 p50 ms | 2.29 [2.26–2.30] | – |
| post_message c=16 p99 ms | 43.5 [37.0–51.8] | – |
| post_message c=64 req/s | 4,805 [4,736–5,012] | – |
| post_message c=64 p50 ms | 9.79 [9.75–9.92] | – |
| post_message c=64 p99 ms | 60.7 [50.2–66.0] | – |

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
