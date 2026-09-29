
Reps: rust 3. Cells: median [min–max].

### Startup and memory

| Metric | Rust | Rust adv. |
|---|---|---|
| cold start: docker run → /up 200 (ms) | 435 [405–18,421] | – |
| idle memory.current (MB) | 47.0 [19.0–66.0] | – |
| idle anon (MB) | 11.0 [11.0–11.0] | – |
| peak memory.current under load (MB) | 846 [821–1,239] | – |
| peak anon under load (MB) | 435 [361–450] | – |

### HTTP (signed in as david; keep-alive; c = concurrent connections)

| Metric | Rust | Rust adv. |
|---|---|---|
| room_show c=1 req/s | 4,860 [4,624–4,946] | – |
| room_show c=1 p50 ms | 0.19 [0.19–0.19] | – |
| room_show c=1 p99 ms | 0.34 [0.33–0.58] | – |
| room_show c=16 req/s | 20,949 [20,406–21,886] | – |
| room_show c=16 p50 ms | 0.73 [0.70–0.75] | – |
| room_show c=16 p99 ms | 1.65 [1.57–1.66] | – |
| room_show c=64 req/s | 20,160 [18,491–21,812] | – |
| room_show c=64 p50 ms | 3.14 [2.90–3.20] | – |
| room_show c=64 p99 ms | 5.49 [5.11–7.07] | – |
| messages_page c=1 req/s | 6,072 [5,154–6,261] | – |
| messages_page c=1 p50 ms | 0.15 [0.15–0.17] | – |
| messages_page c=1 p99 ms | 0.26 [0.25–0.61] | – |
| messages_page c=16 req/s | 25,750 [22,944–25,763] | – |
| messages_page c=16 p50 ms | 0.58 [0.57–0.64] | – |
| messages_page c=16 p99 ms | 1.50 [1.49–1.71] | – |
| messages_page c=64 req/s | 26,083 [17,377–26,573] | – |
| messages_page c=64 p50 ms | 2.40 [2.35–2.84] | – |
| messages_page c=64 p99 ms | 4.19 [4.18–7.64] | – |
| sidebar c=1 req/s | 3,143 [2,281–3,168] | – |
| sidebar c=1 p50 ms | 0.31 [0.30–0.36] | – |
| sidebar c=1 p99 ms | 0.45 [0.45–1.30] | – |
| sidebar c=16 req/s | 13,681 [2,825–13,727] | – |
| sidebar c=16 p50 ms | 1.13 [1.12–1.31] | – |
| sidebar c=16 p99 ms | 2.35 [2.34–4.33] | – |
| sidebar c=64 req/s | 13,860 [5,465–14,079] | – |
| sidebar c=64 p50 ms | 4.58 [4.52–5.08] | – |
| sidebar c=64 p99 ms | 7.98 [7.63–77.06] | – |
| search c=1 req/s | 6,282 [5,719–6,515] | – |
| search c=1 p50 ms | 0.15 [0.14–0.15] | – |
| search c=1 p99 ms | 0.26 [0.24–0.38] | – |
| search c=16 req/s | 24,899 [24,289–25,694] | – |
| search c=16 p50 ms | 0.58 [0.57–0.60] | – |
| search c=16 p99 ms | 1.57 [1.51–1.63] | – |
| search c=64 req/s | 26,803 [26,555–27,385] | – |
| search c=64 p50 ms | 2.29 [2.26–2.33] | – |
| search c=64 p99 ms | 4.40 [4.29–4.65] | – |
| avatar c=1 req/s | 76,376 [75,818–78,720] | – |
| avatar c=1 p50 ms | 0.01 [0.01–0.01] | – |
| avatar c=1 p99 ms | 0.02 [0.02–0.02] | – |
| avatar c=16 req/s | 355,550 [261,897–361,328] | – |
| avatar c=16 p50 ms | 0.03 [0.03–0.04] | – |
| avatar c=16 p99 ms | 0.13 [0.13–0.23] | – |
| avatar c=64 req/s | 371,187 [345,627–376,672] | – |
| avatar c=64 p50 ms | 0.13 [0.13–0.14] | – |
| avatar c=64 p99 ms | 1.31 [0.78–1.32] | – |
| static_css c=1 req/s | 82,648 [66,412–83,966] | – |
| static_css c=1 p50 ms | 0.01 [0.01–0.01] | – |
| static_css c=1 p99 ms | 0.02 [0.02–0.03] | – |
| static_css c=16 req/s | 370,937 [152,684–391,531] | – |
| static_css c=16 p50 ms | 0.03 [0.03–0.04] | – |
| static_css c=16 p99 ms | 0.13 [0.12–0.24] | – |
| static_css c=64 req/s | 377,099 [11,944–394,674] | – |
| static_css c=64 p50 ms | 0.13 [0.13–0.30] | – |
| static_css c=64 p99 ms | 1.33 [0.82–159.74] | – |
| up c=1 req/s | 31,234 [21,296–32,344] | – |
| up c=1 p50 ms | 0.03 [0.03–0.03] | – |
| up c=1 p99 ms | 0.06 [0.05–0.09] | – |
| up c=16 req/s | 145,543 [137,703–148,842] | – |
| up c=16 p50 ms | 0.11 [0.11–0.11] | – |
| up c=16 p99 ms | 0.20 [0.19–0.22] | – |
| up c=64 req/s | 147,990 [129,983–148,459] | – |
| up c=64 p50 ms | 0.42 [0.42–0.45] | – |
| up c=64 p99 ms | 0.91 [0.90–1.42] | – |
| post_message c=1 req/s | 1,968 [1,406–2,161] | – |
| post_message c=1 p50 ms | 0.41 [0.37–0.44] | – |
| post_message c=1 p99 ms | 1.68 [1.45–2.19] | – |
| post_message c=16 req/s | 4,702 [2,604–4,866] | – |
| post_message c=16 p50 ms | 2.35 [2.31–2.48] | – |
| post_message c=16 p99 ms | 41.5 [41.4–63.8] | – |
| post_message c=64 req/s | 4,904 [4,563–4,906] | – |
| post_message c=64 p50 ms | 9.89 [9.81–9.94] | – |
| post_message c=64 p99 ms | 57.1 [56.0–67.6] | – |

### HTTP errors / non-2xx-3xx (first rep, per app)

| Metric | Rust | Rust adv. |
|---|---|---|
- rust: none

### Action Cable fan-out (one room; chatter.js subscriptions per client)

| Metric | Rust | Rust adv. |
|---|---|---|
| 100 clients: subscribed | 100 [100–100] | – |
| 100 clients: connect+subscribe all (s) | 0.06 [0.06–0.06] | – |
| 100 clients: paced post→one client p50 ms | 1.13 [1.10–1.40] | – |
| 100 clients: paced post→all clients p50 ms | 1.41 [1.31–1.74] | – |
| 100 clients: paced post→all clients p99 ms | 5.72 [2.37–9.69] | – |
| 100 clients: max sustained msgs/s (delivered to all) | 2,603 [1,967–2,682] | – |
| 100 clients: deliveries/s (client×message) | 260,325 [196,657–268,231] | – |
| 100 clients: saturated post→all p50 ms | 1.18 [1.15–1.38] | – |
| 100 clients: saturated POST p50 ms | 1.14 [1.11–1.30] | – |
| 1000 clients: subscribed | 1,000 [1,000–1,000] | – |
| 1000 clients: connect+subscribe all (s) | 0.14 [0.12–0.22] | – |
| 1000 clients: paced post→one client p50 ms | 3.52 [3.48–4.33] | – |
| 1000 clients: paced post→all clients p50 ms | 6.72 [6.27–7.03] | – |
| 1000 clients: paced post→all clients p99 ms | 11.4 [10.0–19.7] | – |
| 1000 clients: max sustained msgs/s (delivered to all) | 399 [332–412] | – |
| 1000 clients: deliveries/s (client×message) | 398,831 [331,825–411,829] | – |
| 1000 clients: saturated post→all p50 ms | 20.3 [19.7–21.0] | – |
| 1000 clients: saturated POST p50 ms | 9.65 [9.10–10.64] | – |
| 5000 clients: subscribed | 5,000 [5,000–5,000] | – |
| 5000 clients: connect+subscribe all (s) | 1.31 [0.33–1.56] | – |
| 5000 clients: paced post→one client p50 ms | 13.1 [13.0–13.3] | – |
| 5000 clients: paced post→all clients p50 ms | 27.9 [27.5–28.4] | – |
| 5000 clients: paced post→all clients p99 ms | 41.7 [41.7–49.1] | – |
| 5000 clients: max sustained msgs/s (delivered to all) | 86.5 [84.5–86.9] | – |
| 5000 clients: deliveries/s (client×message) | 432,359 [422,532–434,305] | – |
| 5000 clients: saturated post→all p50 ms | 148 [139–148] | – |
| 5000 clients: saturated POST p50 ms | 28.0 [27.7–28.8] | – |
| 10000 clients: subscribed | 10,000 [10,000–10,000] | – |
| 10000 clients: connect+subscribe all (s) | 1.42 [1.41–1.58] | – |
| 10000 clients: paced post→one client p50 ms | 26.3 [24.8–27.9] | – |
| 10000 clients: paced post→all clients p50 ms | 56.0 [55.9–56.5] | – |
| 10000 clients: paced post→all clients p99 ms | 91.3 [77.6–211.5] | – |
| 10000 clients: max sustained msgs/s (delivered to all) | 37.7 [26.2–47.2] | – |
| 10000 clients: deliveries/s (client×message) | 376,945 [261,854–472,496] | – |
| 10000 clients: saturated post→all p50 ms | 374 [300–390] | – |
| 10000 clients: saturated POST p50 ms | 47.8 [37.7–49.9] | – |

### Upload + thumbnail (black_hole.jpg, 505 KB)

| Metric | Rust | Rust adv. |
|---|---|---|
| POST with attachment (ms) | 29.5 [25.0–30.9] | – |
| then GET thumb → 200 (ms) | 0.20 [0.20–0.20] | – |
| POST → thumbnail served (ms) | 29.7 [25.1–31.1] | – |

### Memory during cable fan-out, by process (MB, peak within the phase)

App process: Rails' Puma master and workers (Action Cable runs in them), or Rust's one campfire
process (its front server included). Pss counts pages shared between forked workers once;
RssAnon counts them in every process.

| Metric | Rust | Rust adv. |
|---|---|---|
| 100 clients, all subscribed, idle: app process Pss | 162 [147–165] | – |
| 100 clients, all subscribed, idle: app process RssAnon | 137 [131–141] | – |
| 100 clients, all subscribed, idle: app + Redis + Thruster Pss | 162 [147–165] | – |
| 100 clients, all subscribed, idle: whole container Pss | 162 [147–165] | – |
| 100 clients, saturated fan-out: app process Pss | 175 [156–180] | – |
| 100 clients, saturated fan-out: app process RssAnon | 151 [140–155] | – |
| 100 clients, saturated fan-out: app + Redis + Thruster Pss | 175 [156–180] | – |
| 100 clients, saturated fan-out: whole container Pss | 175 [156–180] | – |
| 1000 clients, all subscribed, idle: app process Pss | 187 [168–188] | – |
| 1000 clients, all subscribed, idle: app process RssAnon | 162 [151–162] | – |
| 1000 clients, all subscribed, idle: app + Redis + Thruster Pss | 187 [168–188] | – |
| 1000 clients, all subscribed, idle: whole container Pss | 187 [168–188] | – |
| 1000 clients, saturated fan-out: app process Pss | 187 [167–188] | – |
| 1000 clients, saturated fan-out: app process RssAnon | 161 [150–162] | – |
| 1000 clients, saturated fan-out: app + Redis + Thruster Pss | 187 [167–188] | – |
| 1000 clients, saturated fan-out: whole container Pss | 187 [167–188] | – |
| 5000 clients, all subscribed, idle: app process Pss | 244 [221–254] | – |
| 5000 clients, all subscribed, idle: app process RssAnon | 218 [205–228] | – |
| 5000 clients, all subscribed, idle: app + Redis + Thruster Pss | 244 [221–254] | – |
| 5000 clients, all subscribed, idle: whole container Pss | 244 [221–254] | – |
| 5000 clients, saturated fan-out: app process Pss | 244 [221–251] | – |
| 5000 clients, saturated fan-out: app process RssAnon | 218 [204–225] | – |
| 5000 clients, saturated fan-out: app + Redis + Thruster Pss | 244 [221–251] | – |
| 5000 clients, saturated fan-out: whole container Pss | 244 [221–251] | – |
| 10000 clients, all subscribed, idle: app process Pss | 323 [307–324] | – |
| 10000 clients, all subscribed, idle: app process RssAnon | 297 [290–298] | – |
| 10000 clients, all subscribed, idle: app + Redis + Thruster Pss | 323 [307–324] | – |
| 10000 clients, all subscribed, idle: whole container Pss | 323 [307–324] | – |
| 10000 clients, saturated fan-out: app process Pss | 320 [307–324] | – |
| 10000 clients, saturated fan-out: app process RssAnon | 294 [290–299] | – |
| 10000 clients, saturated fan-out: app + Redis + Thruster Pss | 320 [307–324] | – |
| 10000 clients, saturated fan-out: whole container Pss | 320 [307–324] | – |
