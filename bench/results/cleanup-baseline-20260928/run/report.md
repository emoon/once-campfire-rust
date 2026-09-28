```
date: 2026-09-28T22:14:58+02:00
host: 7.2.3-arch1-3, AMD Ryzen 9 9950X3D 16-Core Processor, 32 threads, 60GB
server cpus: 8-11 (nproc 4); loadgen cpus: 12-15; network: host
env: WEB_CONCURRENCY=3 JOB_CONCURRENCY=3 RAILS_MAX_THREADS=5 
rust extra env: 
rust image: campfire-rust:app sha256:776069d599316097c45bdd60306bfee9538e5a0c869e623914eccb7f56a8c282 2026-09-28T21:35:23.074165457+02:00
rust HEAD: 7e8c13f (dirty: 1 files)
```

Reps: rust 3. Cells: median [min–max].

### Startup and memory

| Metric | Rust | Rust adv. |
|---|---|---|
| cold start: docker run → /up 200 (ms) | 419 [408–422] | – |
| idle memory.current (MB) | 14.0 [13.0–14.0] | – |
| idle anon (MB) | 11.0 [11.0–11.0] | – |
| peak memory.current under load (MB) | 452 [442–455] | – |
| peak anon under load (MB) | 319 [318–323] | – |

### HTTP (signed in as david; keep-alive; c = concurrent connections)

| Metric | Rust | Rust adv. |
|---|---|---|
| room_show c=1 req/s | 5,432 [5,389–5,538] | – |
| room_show c=1 p50 ms | 0.17 [0.17–0.17] | – |
| room_show c=1 p99 ms | 0.31 [0.30–0.32] | – |
| room_show c=16 req/s | 22,426 [22,274–22,632] | – |
| room_show c=16 p50 ms | 0.68 [0.68–0.69] | – |
| room_show c=16 p99 ms | 1.52 [1.49–1.53] | – |
| room_show c=64 req/s | 22,516 [22,259–22,629] | – |
| room_show c=64 p50 ms | 2.82 [2.80–2.85] | – |
| room_show c=64 p99 ms | 4.86 [4.73–4.89] | – |
| messages_page c=1 req/s | 5,983 [5,971–6,314] | – |
| messages_page c=1 p50 ms | 0.16 [0.15–0.16] | – |
| messages_page c=1 p99 ms | 0.30 [0.28–0.31] | – |
| messages_page c=16 req/s | 25,807 [25,658–25,926] | – |
| messages_page c=16 p50 ms | 0.58 [0.57–0.58] | – |
| messages_page c=16 p99 ms | 1.46 [1.44–1.46] | – |
| messages_page c=64 req/s | 26,304 [26,281–26,320] | – |
| messages_page c=64 p50 ms | 2.38 [2.37–2.38] | – |
| messages_page c=64 p99 ms | 4.27 [4.21–4.27] | – |
| sidebar c=1 req/s | 3,091 [3,028–3,118] | – |
| sidebar c=1 p50 ms | 0.31 [0.31–0.32] | – |
| sidebar c=1 p99 ms | 0.50 [0.50–0.51] | – |
| sidebar c=16 req/s | 13,463 [13,421–13,584] | – |
| sidebar c=16 p50 ms | 1.14 [1.13–1.15] | – |
| sidebar c=16 p99 ms | 2.42 [2.40–2.42] | – |
| sidebar c=64 req/s | 14,000 [13,812–14,072] | – |
| sidebar c=64 p50 ms | 4.54 [4.52–4.60] | – |
| sidebar c=64 p99 ms | 7.78 [7.70–7.91] | – |
| search c=1 req/s | 6,174 [6,146–6,349] | – |
| search c=1 p50 ms | 0.15 [0.14–0.15] | – |
| search c=1 p99 ms | 0.31 [0.29–0.31] | – |
| search c=16 req/s | 25,365 [24,662–25,743] | – |
| search c=16 p50 ms | 0.58 [0.57–0.60] | – |
| search c=16 p99 ms | 1.51 [1.49–1.60] | – |
| search c=64 req/s | 26,796 [26,655–27,119] | – |
| search c=64 p50 ms | 2.33 [2.30–2.33] | – |
| search c=64 p99 ms | 4.16 [4.07–4.24] | – |
| avatar c=1 req/s | 77,105 [76,943–77,486] | – |
| avatar c=1 p50 ms | 0.01 [0.01–0.01] | – |
| avatar c=1 p99 ms | 0.02 [0.02–0.02] | – |
| avatar c=16 req/s | 349,458 [348,698–349,969] | – |
| avatar c=16 p50 ms | 0.03 [0.03–0.04] | – |
| avatar c=16 p99 ms | 0.13 [0.13–0.13] | – |
| avatar c=64 req/s | 361,650 [357,722–361,766] | – |
| avatar c=64 p50 ms | 0.14 [0.14–0.14] | – |
| avatar c=64 p99 ms | 1.09 [1.04–1.15] | – |
| static_css c=1 req/s | 83,155 [82,632–83,548] | – |
| static_css c=1 p50 ms | 0.01 [0.01–0.01] | – |
| static_css c=1 p99 ms | 0.02 [0.02–0.02] | – |
| static_css c=16 req/s | 373,529 [371,841–376,891] | – |
| static_css c=16 p50 ms | 0.03 [0.03–0.03] | – |
| static_css c=16 p99 ms | 0.12 [0.12–0.13] | – |
| static_css c=64 req/s | 383,551 [381,566–383,885] | – |
| static_css c=64 p50 ms | 0.13 [0.13–0.13] | – |
| static_css c=64 p99 ms | 0.84 [0.75–0.88] | – |
| up c=1 req/s | 31,133 [30,954–31,243] | – |
| up c=1 p50 ms | 0.03 [0.03–0.03] | – |
| up c=1 p99 ms | 0.07 [0.06–0.07] | – |
| up c=16 req/s | 147,763 [147,088–148,695] | – |
| up c=16 p50 ms | 0.11 [0.11–0.11] | – |
| up c=16 p99 ms | 0.20 [0.20–0.20] | – |
| up c=64 req/s | 149,470 [148,749–150,382] | – |
| up c=64 p50 ms | 0.42 [0.41–0.42] | – |
| up c=64 p99 ms | 0.90 [0.89–0.90] | – |
| post_message c=1 req/s | 2,096 [2,084–2,124] | – |
| post_message c=1 p50 ms | 0.38 [0.38–0.38] | – |
| post_message c=1 p99 ms | 1.53 [1.52–1.53] | – |
| post_message c=16 req/s | 4,813 [4,755–4,877] | – |
| post_message c=16 p50 ms | 2.31 [2.31–2.37] | – |
| post_message c=16 p99 ms | 40.4 [40.3–41.2] | – |
| post_message c=64 req/s | 4,828 [4,772–4,912] | – |
| post_message c=64 p50 ms | 9.86 [9.86–9.97] | – |
| post_message c=64 p99 ms | 60.4 [52.3–62.0] | – |

### HTTP errors / non-2xx-3xx (first rep, per app)

| Metric | Rust | Rust adv. |
|---|---|---|
- rust: none

### Action Cable fan-out (one room; chatter.js subscriptions per client)

| Metric | Rust | Rust adv. |
|---|---|---|
| 100 clients: subscribed | 100 [100–100] | – |
| 100 clients: connect+subscribe all (s) | 0.06 [0.06–0.06] | – |
| 100 clients: paced post→one client p50 ms | 1.14 [1.14–1.15] | – |
| 100 clients: paced post→all clients p50 ms | 1.39 [1.37–1.41] | – |
| 100 clients: paced post→all clients p99 ms | 1.87 [1.74–2.14] | – |
| 100 clients: max sustained msgs/s (delivered to all) | 2,632 [2,530–2,652] | – |
| 100 clients: deliveries/s (client×message) | 263,246 [253,040–265,236] | – |
| 100 clients: saturated post→all p50 ms | 1.17 [1.16–1.19] | – |
| 100 clients: saturated POST p50 ms | 1.13 [1.12–1.15] | – |
| 500 clients: subscribed | 500 [500–500] | – |
| 500 clients: connect+subscribe all (s) | 0.28 [0.11–1.11] | – |
| 500 clients: paced post→one client p50 ms | 2.38 [2.33–2.41] | – |
| 500 clients: paced post→all clients p50 ms | 3.94 [3.60–4.03] | – |
| 500 clients: paced post→all clients p99 ms | 5.65 [5.53–11.18] | – |
| 500 clients: max sustained msgs/s (delivered to all) | 761 [753–763] | – |
| 500 clients: deliveries/s (client×message) | 380,320 [376,315–381,372] | – |
| 500 clients: saturated post→all p50 ms | 11.0 [10.7–11.0] | – |
| 500 clients: saturated POST p50 ms | 4.58 [4.53–4.61] | – |
| 1000 clients: subscribed | 1,000 [1,000–1,000] | – |
| 1000 clients: connect+subscribe all (s) | 0.19 [0.12–1.15] | – |
| 1000 clients: paced post→one client p50 ms | 3.73 [3.69–3.79] | – |
| 1000 clients: paced post→all clients p50 ms | 6.87 [6.47–7.12] | – |
| 1000 clients: paced post→all clients p99 ms | 15.1 [10.0–19.9] | – |
| 1000 clients: max sustained msgs/s (delivered to all) | 400 [395–403] | – |
| 1000 clients: deliveries/s (client×message) | 399,820 [395,417–402,558] | – |
| 1000 clients: saturated post→all p50 ms | 20.2 [20.1–20.5] | – |
| 1000 clients: saturated POST p50 ms | 9.54 [9.45–9.73] | – |

### Upload + thumbnail (black_hole.jpg, 505 KB)

| Metric | Rust | Rust adv. |
|---|---|---|
| POST with attachment (ms) | 25.8 [25.7–25.8] | – |
| then GET thumb → 200 (ms) | 0.20 [0.20–0.20] | – |
| POST → thumbnail served (ms) | 26.0 [25.9–26.0] | – |

### Memory during cable fan-out, by process (MB, peak within the phase)

App process: Rails' Puma master and workers (Action Cable runs in them), or Rust's one campfire
process (its front server included). Pss counts pages shared between forked workers once;
RssAnon counts them in every process.

| Metric | Rust | Rust adv. |
|---|---|---|
| 100 clients, all subscribed, idle: app process Pss | 162 [158–165] | – |
| 100 clients, all subscribed, idle: app process RssAnon | 136 [133–140] | – |
| 100 clients, all subscribed, idle: app + Redis + Thruster Pss | 162 [158–165] | – |
| 100 clients, all subscribed, idle: whole container Pss | 162 [158–165] | – |
| 100 clients, saturated fan-out: app process Pss | 172 [169–178] | – |
| 100 clients, saturated fan-out: app process RssAnon | 146 [144–152] | – |
| 100 clients, saturated fan-out: app + Redis + Thruster Pss | 172 [169–178] | – |
| 100 clients, saturated fan-out: whole container Pss | 172 [169–178] | – |
| 500 clients, all subscribed, idle: app process Pss | 177 [170–179] | – |
| 500 clients, all subscribed, idle: app process RssAnon | 150 [143–152] | – |
| 500 clients, all subscribed, idle: app + Redis + Thruster Pss | 177 [170–179] | – |
| 500 clients, all subscribed, idle: whole container Pss | 177 [170–179] | – |
| 500 clients, saturated fan-out: app process Pss | 177 [169–179] | – |
| 500 clients, saturated fan-out: app process RssAnon | 150 [142–152] | – |
| 500 clients, saturated fan-out: app + Redis + Thruster Pss | 177 [169–179] | – |
| 500 clients, saturated fan-out: whole container Pss | 177 [169–179] | – |
| 1000 clients, all subscribed, idle: app process Pss | 181 [179–181] | – |
| 1000 clients, all subscribed, idle: app process RssAnon | 154 [153–154] | – |
| 1000 clients, all subscribed, idle: app + Redis + Thruster Pss | 181 [179–181] | – |
| 1000 clients, all subscribed, idle: whole container Pss | 181 [179–181] | – |
| 1000 clients, saturated fan-out: app process Pss | 180 [179–181] | – |
| 1000 clients, saturated fan-out: app process RssAnon | 153 [152–154] | – |
| 1000 clients, saturated fan-out: app + Redis + Thruster Pss | 180 [179–181] | – |
| 1000 clients, saturated fan-out: whole container Pss | 180 [179–181] | – |
