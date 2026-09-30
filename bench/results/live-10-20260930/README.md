# LIVE-10: the message broadcast off the runtime workers (2026-09-30), dropped

The tables run (`../tables-20260930/`) had the tip behind v0.1.1 on post to all 10,000 clients received: p50 −2.3%
and p99 −11%, from 30 paced posts a rep. The cable path hasn't changed since v0.1.1, but DB-11 did change where
it runs. `broadcast_create` renders the message and calls `Hub::broadcast` inside `db.read`, and since DB-11 that
read runs inline on a runtime worker instead of on the blocking pool. So the `broadcast::Sender::send` that wakes
every subscriber now runs on a worker. LIVE-10 (`refactor/cleanup-live-10`) moves that read back to the blocking
pool with `read_offloaded`.

One `bench/run` session, three Rust images interleaved, 4 reps each: v0.1.1 (`rust-base`, 080f903), the tip
(`rust`, 335f46f; the code is the same at 2e0dc37) and LIVE-10 (`rust-alt`). HTTP at c=16 only; cable at 1,000 and
10,000 clients with `CABLE_LATENCY_MSGS=150`. `LOAD_MAX=10`, because another session's hung `bfs` kept 8 threads in
uninterruptible sleep, which adds 8 to the load average but uses no CPU. `bench/quiet` passed: other CCD 2.6%,
siblings 5.9%. The full tables are in `report.md`.

Means of 4 reps:

| 10,000 clients | v0.1.1 | tip | LIVE-10 |
|---|---|---|---|
| post → all p50 (ms) | 53.7 | 54.4 (+1.3%) | 54.4 (+1.3%) |
| post → all p90 (ms) | 72.9 | 73.1 (+0.4%) | 72.4 (−0.6%) |
| post → all p99 (ms) | 83.1 | 85.6 (+3.0%) | 83.9 (+1.0%) |
| deliveries/s | 479k | 493k (+3.0%) | 483k (+0.8%) |
| saturated post → all p50 (ms) | 297 | 321 (+8%) | 293 |

| 1,000 clients | v0.1.1 | tip | LIVE-10 |
|---|---|---|---|
| post → all p50 (ms) | 6.35 | 6.38 | 6.22 |
| post → all p99 (ms) | 13.7 | 13.7 | 17.1 (reps: 19.9, 14.8, 19.2, 14.5) |
| post → all max (ms) | 16.1 | 15.2 | 68.9 (one rep 196) |
| deliveries/s | 409k | 417k | 415k |

| c=16 req/s | v0.1.1 | tip | LIVE-10 |
|---|---|---|---|
| room_show | 22,536 | 36,696 | 36,672 |
| post_message | 4,795 | 5,319 | 5,423 |
| sidebar | 13,901 | 35,765 | 35,939 |

- **The regression is smaller than the tables run showed.** With 150 posts a rep, the tip trails v0.1.1 at 10,000
  clients by about 1% at p50 and 3% at p99, and leads by 3% on deliveries/s. The tip's p99 was ≥ 85.1 ms in every rep;
  v0.1.1's was below that in three of four. The 11% from 30 samples was mostly noise.
- **LIVE-10 trades that throughput back for the 10k tail**, bringing p99 and saturated latency back to v0.1.1's level.
  But it makes the 1,000-client tail worse, from the hop to the blocking pool: two reps had p99 near 20 ms, and one post
  took 196 ms to reach everyone. Dropped. Beating v0.1.1 on every cable row needs a faster fan-out (LIVE-9), not
  moving work between threads.
