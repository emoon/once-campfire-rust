# LIVE-9: the connection loop polls only what woke (2026-09-30), not merged yet

Branch `refactor/cleanup-live-9` (measured at 7efd214; 8abf7bb adds the review fixes). Each connection task used
`tokio::select!` over the reader's mpsc, a `SelectAll` of subscription streams, a `SelectAll` of the internal channel,
the shared heartbeat `watch` and the shared restart `broadcast`. Every delivery woke it and re-polled every branch,
and polling the shared channels locks them; `SelectAll` also re-pushes a `FuturesUnordered` task per item. LIVE-9
adds `crates/cable/src/merge.rs`, which polls only the streams whose wakers fired (an atomic bitmask of up to 128
slots), and uses it for subscriptions, for the signals (internal channel, heartbeat, restarts) and for the reader's
messages. The write-batching loop takes ready frames with `Merge::try_next`, not `now_or_never`.

## CPU per message (profile)

`bench/profile perf --targets cable10000 --call-graph fp`, saturated fan-out to 10,000 clients, one run each. Profiles
in `../profile-cable-20260930/` (the tip) and `../profile-cable-live-9{,b,c}-20260930/` (the three steps):

| build | cable user ms/msg | cable sys ms/msg | process user ms/msg | writes/msg | msg/s |
|---|---|---|---|---|---|
| tip | 12.23 | 33.49 | 16.27 | 3,821 | 46.2 |
| + signals polled only when they fire, mutex-queue merge | 10.98 | 32.56 | 14.51 | 3,643 | 48.6 |
| + lock-free merge (profiled before the last two changes) | 9.94 | 31.65 | 13.01 | 3,381 | 49.4 |
| + merges for signals and messages, `try_next` (measured below) | 10.28 | 33.10 | 13.51 | 3,631 | 46.5 |

User CPU per message falls 16–19%. But three quarters of the cable runtime's CPU is the kernel sending on loopback,
which this doesn't touch. (`bench/lib/cpuprof.py`'s "malloc/free/realloc" bucket matches `alloc::alloc` anywhere in
a frame, so it counts every `Arc`/`Vec`/`Box` generic; the real jemalloc frames are under 1% here.)

## End to end

One `bench/run` session, three images interleaved: v0.1.1 (`rust-base`, 080f903), the tip (`rust-alt`, 335f46f) and
LIVE-9 (`rust`, 7efd214). HTTP at c=16; cable at 100, 1,000, 5,000 and 10,000 clients with `CABLE_LATENCY_MSGS=150`.
Four reps were planned. Another session's test suite drove the load average to 53 during LIVE-9's rep 4, which ran
after the 900 s wait for quiet expired and is degraded throughout (post_message 4,457 req/s against ~5,500); the run
was stopped before v0.1.1's rep 4. Reps 1–3 of each are below; rep 4 of LIVE-9 and of the tip are in `excluded/`.
Medians of 3; ratios > 1 mean better.

| Row | v0.1.1 | tip | LIVE-9 | tip vs v0.1.1 | LIVE-9 vs v0.1.1 | LIVE-9 vs tip |
|---|---|---|---|---|---|---|
| Room page req/s | 22,662 | 36,554 | 36,927 | 1.613 | 1.629 | 1.010 |
| Messages page req/s | 26,624 | 42,352 | 42,571 | 1.591 | 1.599 | 1.005 |
| Sidebar req/s | 14,045 | 35,995 | 35,958 | 2.563 | 2.560 | 0.999 |
| Search req/s | 26,842 | 32,739 | 33,167 | 1.220 | 1.236 | 1.013 |
| Post a message req/s | 4,892 | 5,424 | 5,491 | 1.109 | 1.122 | 1.012 |
| `/up` req/s | 150,323 | 230,215 | 231,802 | 1.531 | 1.542 | 1.007 |
| Deliveries/s, 100 clients | 271,748 | 337,467 | 340,245 | 1.242 | 1.252 | 1.008 |
| Deliveries/s, 1,000 clients | 412,124 | 415,323 | 422,380 | 1.008 | 1.025 | 1.017 |
| Deliveries/s, 5,000 clients | 439,354 | 447,512 | 450,421 | 1.019 | 1.025 | 1.007 |
| Deliveries/s, 10,000 clients | 474,565 | 497,371 | 485,108 | 1.048 | 1.022 | 0.975 |
| Post to all 1,000, p50 (ms) | 6.31 | 6.28 | 6.19 | 1.004 | 1.020 | 1.016 |
| Post to all 10,000, p50 (ms) | 55.9 | 55.6 | 54.3 | 1.005 | 1.029 | 1.024 |
| Post to all 10,000, p99 (ms) | 83.6 | 81.3 | 83.6 | 1.028 | 1.000 | 0.972 |
| Connect 10,000 (s) | 1.43 | 1.37 | 1.51 | 1.044 | 0.947 | 0.907 |

Per rep, 10,000 clients (v0.1.1 / tip / LIVE-9): p50 55.9, 56.3, 55.2 / 55.6, 53.4, 55.8 / 53.6, 54.3, 54.4; p99 83.6,
87.6, 79.4 / 79.6, 81.3, 83.2 / 80.4, 83.6, 87.4; connect 1.25, 1.43, 1.84 / 1.37, 0.42, 1.41 / 1.39, 1.51, 1.71 s.

- **The tip's cable "regression" against v0.1.1 isn't there.** Here the tip leads v0.1.1 on every row. The 10,000-client
  latency rows have moved between runs today: tip −11% p99 (`../tables-20260930/`, 30 posts a rep), −1% to −3%
  (`../live-10-20260930/`, 150 posts), +0.5% to +3% here. Run-to-run spread (±3%) is larger than any difference between
  the builds, so on those rows the two are tied.
- **LIVE-9 isn't a measurable end-to-end win.** It leads the tip on 9 of 14 rows by 0.5–2.4% and trails on 10k
  deliveries/s (−2.5%), 10k p99 (−2.8%) and connect (noisy: the tip's 0.42 s rep), all inside the same spread. Its
  saving is CPU the kernel-bound fan-out doesn't wait on.
