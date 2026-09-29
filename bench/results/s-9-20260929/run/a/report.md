
Reps: rust 3. Cells: median [min–max].

### Startup and memory

| Metric | Rust | Rust adv. |
|---|---|---|
| cold start: docker run → /up 200 (ms) | 415 [413–455] | – |
| idle memory.current (MB) | 30.0 [14.0–62.0] | – |
| idle anon (MB) | 11.0 [11.0–11.0] | – |
| peak memory.current under load (MB) | 874 [660–900] | – |
| peak anon under load (MB) | 442 [441–453] | – |

### HTTP (signed in as david; keep-alive; c = concurrent connections)

| Metric | Rust | Rust adv. |
|---|---|---|
| room_show c=1 req/s | 5,473 [4,956–5,669] | – |
| room_show c=1 p50 ms | 0.17 [0.17–0.19] | – |
| room_show c=1 p99 ms | 0.27 [0.26–0.33] | – |
| room_show c=16 req/s | 22,522 [19,898–22,549] | – |
| room_show c=16 p50 ms | 0.68 [0.68–0.76] | – |
| room_show c=16 p99 ms | 1.56 [1.53–1.86] | – |
| room_show c=64 req/s | 21,542 [20,371–22,614] | – |
| room_show c=64 p50 ms | 2.87 [2.80–3.08] | – |
| room_show c=64 p99 ms | 5.54 [4.82–5.66] | – |
| messages_page c=1 req/s | 5,962 [5,459–6,377] | – |
| messages_page c=1 p50 ms | 0.16 [0.15–0.17] | – |
| messages_page c=1 p99 ms | 0.26 [0.23–0.30] | – |
| messages_page c=16 req/s | 26,031 [24,266–26,282] | – |
| messages_page c=16 p50 ms | 0.57 [0.57–0.61] | – |
| messages_page c=16 p99 ms | 1.47 [1.44–1.62] | – |
| messages_page c=64 req/s | 26,348 [25,749–26,489] | – |
| messages_page c=64 p50 ms | 2.38 [2.37–2.41] | – |
| messages_page c=64 p99 ms | 4.13 [4.10–4.61] | – |
| sidebar c=1 req/s | 3,048 [2,988–3,202] | – |
| sidebar c=1 p50 ms | 0.32 [0.30–0.32] | – |
| sidebar c=1 p99 ms | 0.47 [0.47–0.48] | – |
| sidebar c=16 req/s | 13,478 [13,361–13,770] | – |
| sidebar c=16 p50 ms | 1.14 [1.12–1.15] | – |
| sidebar c=16 p99 ms | 2.38 [2.36–2.42] | – |
| sidebar c=64 req/s | 14,027 [13,387–14,081] | – |
| sidebar c=64 p50 ms | 4.53 [4.51–4.75] | – |
| sidebar c=64 p99 ms | 7.76 [7.66–8.19] | – |
| search c=1 req/s | 6,322 [6,147–6,421] | – |
| search c=1 p50 ms | 0.15 [0.14–0.15] | – |
| search c=1 p99 ms | 0.26 [0.25–0.27] | – |
| search c=16 req/s | 25,046 [23,444–26,049] | – |
| search c=16 p50 ms | 0.59 [0.57–0.59] | – |
| search c=16 p99 ms | 1.52 [1.48–2.27] | – |
| search c=64 req/s | 26,430 [23,474–27,641] | – |
| search c=64 p50 ms | 2.35 [2.26–2.58] | – |
| search c=64 p99 ms | 4.36 [3.90–5.51] | – |
| avatar c=1 req/s | 77,229 [15,402–77,509] | – |
| avatar c=1 p50 ms | 0.01 [0.01–0.02] | – |
| avatar c=1 p99 ms | 0.02 [0.02–0.52] | – |
| avatar c=16 req/s | 342,806 [172,908–366,038] | – |
| avatar c=16 p50 ms | 0.03 [0.03–0.04] | – |
| avatar c=16 p99 ms | 0.14 [0.12–0.22] | – |
| avatar c=64 req/s | 344,596 [340,073–373,621] | – |
| avatar c=64 p50 ms | 0.14 [0.13–0.14] | – |
| avatar c=64 p99 ms | 1.07 [0.88–1.29] | – |
| static_css c=1 req/s | 83,186 [78,688–84,410] | – |
| static_css c=1 p50 ms | 0.01 [0.01–0.01] | – |
| static_css c=1 p99 ms | 0.02 [0.02–0.03] | – |
| static_css c=16 req/s | 377,377 [363,573–386,337] | – |
| static_css c=16 p50 ms | 0.03 [0.03–0.03] | – |
| static_css c=16 p99 ms | 0.12 [0.12–0.13] | – |
| static_css c=64 req/s | 392,552 [346,918–396,059] | – |
| static_css c=64 p50 ms | 0.13 [0.13–0.14] | – |
| static_css c=64 p99 ms | 1.07 [0.92–1.25] | – |
| up c=1 req/s | 31,397 [29,883–31,445] | – |
| up c=1 p50 ms | 0.03 [0.03–0.03] | – |
| up c=1 p99 ms | 0.06 [0.06–0.09] | – |
| up c=16 req/s | 148,866 [134,321–150,611] | – |
| up c=16 p50 ms | 0.11 [0.11–0.12] | – |
| up c=16 p99 ms | 0.19 [0.19–0.25] | – |
| up c=64 req/s | 151,228 [135,087–153,300] | – |
| up c=64 p50 ms | 0.41 [0.41–0.46] | – |
| up c=64 p99 ms | 0.88 [0.86–1.03] | – |
| post_message c=1 req/s | 2,046 [1,911–2,113] | – |
| post_message c=1 p50 ms | 0.38 [0.38–0.41] | – |
| post_message c=1 p99 ms | 1.72 [1.50–1.73] | – |
| post_message c=16 req/s | 4,776 [4,626–4,797] | – |
| post_message c=16 p50 ms | 2.29 [2.23–2.41] | – |
| post_message c=16 p99 ms | 43.9 [40.3–49.8] | – |
| post_message c=64 req/s | 4,798 [4,751–4,851] | – |
| post_message c=64 p50 ms | 9.81 [9.69–10.10] | – |
| post_message c=64 p99 ms | 56.5 [55.3–61.8] | – |

### HTTP errors / non-2xx-3xx (first rep, per app)

| Metric | Rust | Rust adv. |
|---|---|---|
- rust: none

### Action Cable fan-out (one room; chatter.js subscriptions per client)

| Metric | Rust | Rust adv. |
|---|---|---|
| 100 clients: subscribed | 100 [100–100] | – |
| 100 clients: connect+subscribe all (s) | 0.06 [0.06–0.06] | – |
| 100 clients: paced post→one client p50 ms | 1.16 [1.14–1.19] | – |
| 100 clients: paced post→all clients p50 ms | 1.43 [1.40–1.49] | – |
| 100 clients: paced post→all clients p99 ms | 1.80 [1.74–9.01] | – |
| 100 clients: max sustained msgs/s (delivered to all) | 2,687 [2,596–2,694] | – |
| 100 clients: deliveries/s (client×message) | 268,715 [259,546–269,382] | – |
| 100 clients: saturated post→all p50 ms | 1.16 [1.15–1.19] | – |
| 100 clients: saturated POST p50 ms | 1.12 [1.11–1.14] | – |
| 1000 clients: subscribed | 1,000 [1,000–1,000] | – |
| 1000 clients: connect+subscribe all (s) | 0.13 [0.11–0.13] | – |
| 1000 clients: paced post→one client p50 ms | 3.72 [3.58–3.91] | – |
| 1000 clients: paced post→all clients p50 ms | 6.40 [6.22–6.49] | – |
| 1000 clients: paced post→all clients p99 ms | 11.6 [9.5–22.6] | – |
| 1000 clients: max sustained msgs/s (delivered to all) | 396 [390–407] | – |
| 1000 clients: deliveries/s (client×message) | 396,244 [390,043–407,005] | – |
| 1000 clients: saturated post→all p50 ms | 20.2 [19.5–20.6] | – |
| 1000 clients: saturated POST p50 ms | 9.67 [9.33–9.78] | – |
| 5000 clients: subscribed | 5,000 [5,000–5,000] | – |
| 5000 clients: connect+subscribe all (s) | 1.29 [1.14–1.45] | – |
| 5000 clients: paced post→one client p50 ms | 13.1 [12.5–13.2] | – |
| 5000 clients: paced post→all clients p50 ms | 29.3 [26.6–29.5] | – |
| 5000 clients: paced post→all clients p99 ms | 42.7 [40.5–43.4] | – |
| 5000 clients: max sustained msgs/s (delivered to all) | 87.4 [77.8–87.6] | – |
| 5000 clients: deliveries/s (client×message) | 436,941 [389,147–437,913] | – |
| 5000 clients: saturated post→all p50 ms | 145 [143–151] | – |
| 5000 clients: saturated POST p50 ms | 29.8 [28.2–33.6] | – |
| 10000 clients: subscribed | 10,000 [10,000–10,000] | – |
| 10000 clients: connect+subscribe all (s) | 2.45 [1.46–2.49] | – |
| 10000 clients: paced post→one client p50 ms | 25.4 [24.7–57.8] | – |
| 10000 clients: paced post→all clients p50 ms | 54.7 [53.1–130.8] | – |
| 10000 clients: paced post→all clients p99 ms | 87.2 [86.6–235.6] | – |
| 10000 clients: max sustained msgs/s (delivered to all) | 47.1 [20.7–49.5] | – |
| 10000 clients: deliveries/s (client×message) | 471,165 [206,749–494,675] | – |
| 10000 clients: saturated post→all p50 ms | 338 [317–402] | – |
| 10000 clients: saturated POST p50 ms | 41.3 [37.6–102.6] | – |

### Upload + thumbnail (black_hole.jpg, 505 KB)

| Metric | Rust | Rust adv. |
|---|---|---|
| POST with attachment (ms) | 31.6 [24.5–34.3] | – |
| then GET thumb → 200 (ms) | 0.20 [0.20–0.30] | – |
| POST → thumbnail served (ms) | 31.8 [24.7–34.5] | – |

### Memory during cable fan-out, by process (MB, peak within the phase)

App process: Rails' Puma master and workers (Action Cable runs in them), or Rust's one campfire
process (its front server included). Pss counts pages shared between forked workers once;
RssAnon counts them in every process.

| Metric | Rust | Rust adv. |
|---|---|---|
| 100 clients, all subscribed, idle: app process Pss | 158 [157–172] | – |
| 100 clients, all subscribed, idle: app process RssAnon | 133 [133–145] | – |
| 100 clients, all subscribed, idle: app + Redis + Thruster Pss | 158 [157–172] | – |
| 100 clients, all subscribed, idle: whole container Pss | 158 [157–172] | – |
| 100 clients, saturated fan-out: app process Pss | 170 [168–185] | – |
| 100 clients, saturated fan-out: app process RssAnon | 145 [144–158] | – |
| 100 clients, saturated fan-out: app + Redis + Thruster Pss | 170 [168–185] | – |
| 100 clients, saturated fan-out: whole container Pss | 170 [168–185] | – |
| 1000 clients, all subscribed, idle: app process Pss | 182 [181–197] | – |
| 1000 clients, all subscribed, idle: app process RssAnon | 157 [154–169] | – |
| 1000 clients, all subscribed, idle: app + Redis + Thruster Pss | 182 [181–197] | – |
| 1000 clients, all subscribed, idle: whole container Pss | 182 [181–197] | – |
| 1000 clients, saturated fan-out: app process Pss | 182 [180–197] | – |
| 1000 clients, saturated fan-out: app process RssAnon | 157 [153–169] | – |
| 1000 clients, saturated fan-out: app + Redis + Thruster Pss | 182 [180–197] | – |
| 1000 clients, saturated fan-out: whole container Pss | 182 [180–197] | – |
| 5000 clients, all subscribed, idle: app process Pss | 245 [244–258] | – |
| 5000 clients, all subscribed, idle: app process RssAnon | 219 [218–230] | – |
| 5000 clients, all subscribed, idle: app + Redis + Thruster Pss | 245 [244–258] | – |
| 5000 clients, all subscribed, idle: whole container Pss | 245 [244–258] | – |
| 5000 clients, saturated fan-out: app process Pss | 244 [242–258] | – |
| 5000 clients, saturated fan-out: app process RssAnon | 218 [216–230] | – |
| 5000 clients, saturated fan-out: app + Redis + Thruster Pss | 244 [242–258] | – |
| 5000 clients, saturated fan-out: whole container Pss | 244 [242–258] | – |
| 10000 clients, all subscribed, idle: app process Pss | 329 [322–332] | – |
| 10000 clients, all subscribed, idle: app process RssAnon | 304 [296–304] | – |
| 10000 clients, all subscribed, idle: app + Redis + Thruster Pss | 329 [322–332] | – |
| 10000 clients, all subscribed, idle: whole container Pss | 329 [322–332] | – |
| 10000 clients, saturated fan-out: app process Pss | 325 [320–331] | – |
| 10000 clients, saturated fan-out: app process RssAnon | 299 [294–303] | – |
| 10000 clients, saturated fan-out: app + Redis + Thruster Pss | 325 [320–331] | – |
| 10000 clients, saturated fan-out: whole container Pss | 325 [320–331] | – |
