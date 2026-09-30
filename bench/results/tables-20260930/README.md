# The published tables, regenerated: Rails, v0.1.1 and the tip (2026-09-30)

One `bench/run` session on this machine (Ryzen 9 9950X3D), three apps interleaved, 3 reps each:

- **Rails**: `campfire-reference:app` (reference at `90b3300`).
- **v0.1.1**: `campfire-rust:v0.1.1`, built from `080f903` (the `rust-base` app).
- **Tip**: `campfire-rust:tip`, built from `335f46f`.

```
SUITES="http cable" CABLE_CLIENTS="100 1000 5000 10000" REFERENCE_IMAGE=campfire-reference:app \
  RUST_IMAGE=campfire-rust:tip BASE_RUST_IMAGE=campfire-rust:v0.1.1 \
  flock /tmp/campfire-bench.lock bench/quiet -- bench/run --apps reference,rust-base,rust --reps 3
```

`bench/quiet` passed: the other CCD was 3.1% busy and the bench cores' SMT siblings 6.3%. Two caveats. The tip's rep 3
started after `bench/run`'s 900 s wait for the load to settle expired (load 2.74), beside another project's test
suite, and it is the low minimum in several of the tip's cells. The medians below come from the other two reps. In
Rails' rep 2, 9,927 of 10,000 cable clients subscribed; every other run subscribed every client. The full tables
with spreads are in `report.md`.

"Published" is v0.1.1's README table, measured on a Ryzen AI Max+ 395. This machine is faster on compute and slower
on loopback and kernel paths (`../s-9-20260929/`), so Rust's cable numbers come out lower here for both builds.
Compare the columns measured here with each other, not with the published columns.

## Throughput (16 concurrent clients), req/s

| Route | Rails | v0.1.1 | Tip | Tip vs Rails | Tip vs v0.1.1 | Published Rails | Published Rust |
|---|---|---|---|---|---|---|---|
| Room page | 252 | 22,653 | 36,632 | **145×** | **1.62×** | 215 | 20,479 |
| Messages page (`?before=`) | 464 | 26,137 | 42,402 | **91×** | **1.62×** | 406 | 23,365 |
| Sidebar | 611 | 13,847 | 35,506 | **58×** | **2.56×** | 528 | 12,642 |
| Search | 430 | 25,786 | 32,883 | **76×** | **1.28×** | 385 | 23,399 |
| Post a message | 250 | 4,952 | 5,657 | **23×** | **1.14×** | 274 | 5,452 |
| `/up` | 4,617 | 149,936 | 234,295 | **51×** | **1.56×** | 4,069 | 136,228 |

## Real time (Action Cable, up to 10,000 clients in one room)

| Measurement | Rails | v0.1.1 | Tip | Tip vs Rails | Tip vs v0.1.1 | Published Rails | Published Rust |
|---|---|---|---|---|---|---|---|
| Deliveries per second, 100 clients | 8,923 | 265,409 | 325,791 | **37×** | **1.23×** | 7,892 | 312,272 |
| Deliveries per second, 1,000 clients | 13,028 | 407,493 | 416,080 | **32×** | 1.02× | 10,771 | 503,302 |
| Deliveries per second, 5,000 clients | 13,414 | 434,157 | 437,814 | **33×** | 1.01× | 11,930 | 595,886 |
| Deliveries per second, 10,000 clients | 11,816 | 481,619 | 495,572 | **42×** | 1.03× | 9,585 | 638,688 |
| Post to all 1,000 clients received, p50 | 90.6 ms | 6.62 ms | 6.33 ms | **14×** | 1.05× | 107 ms | 6.5 ms |
| Post to all 10,000 clients received, p50 | 1,002 ms | 51.3 ms | 52.5 ms | **19×** | *0.98×* | 1,171 ms | 40 ms |
| Post to all 10,000 clients received, p99 | 1,239 ms | 76.0 ms | 85.1 ms | **15×** | *0.89×* | 1,519 ms | 61 ms |
| Connect and subscribe 10,000 clients | 29.0 s | 1.84 s | 0.59 s | **49×** | **3.1×** | 29.1 s | 2.4 s |

**Follow-up (same day): not a regression.** With 150 posts a rep, over two more sessions, these rows came out
−1..−3% and then +0.5..+3%, inside run-to-run spread: the tip ties v0.1.1 on them (`../live-10-20260930/`,
`../live-9-20260930/`). What this run showed:

**The tip doesn't beat v0.1.1 on two rows:** post to all 10,000 clients received, at p50 (−2.3%) and p99 (−11%). The
ranges overlap (p50: v0.1.1 50.7–56.1 ms, tip 52.0–55.6 ms; p99: 74.9–84.5 against 80.7–88.4). The p99 comes from
only 30 paced posts a rep. The tip was worse in both of the clean reps, though, and the saturated post-to-all p50
also leans worse at 5,000 (0.96×) and 10,000 clients (0.93×). So the gap is probably real but small, and unconfirmed.
The cable crate has changed only in formatting since v0.1.1, apart from JSON escaping moving to
`rails_compat::json`. The next step is a cable-only run of the two Rust images with more reps, then a bisect if it
holds. The 1,000- and 5,000-client deliveries rows are within noise (+2%, +1%).

## Startup and memory

| Measurement | Rails | v0.1.1 | Tip | Tip vs Rails | Tip vs v0.1.1 |
|---|---|---|---|---|---|
| Cold start (`docker run` until `/up` answers) | 2,514 ms | 432 ms | 446 ms | **5.6×** | 0.97× |
| Idle memory (container) | 304 MB | 14 MB | 13 MB | **23×** | 1.08× |
| App process, 1,000 idle cable clients (Pss) | 656 MB | 184 MB | 177 MB | **3.7×** | 1.04× |
| App process, 10,000 idle cable clients (Pss) | 1,526 MB | 322 MB | 281 MB | **5.4×** | **1.15×** |
| App process, 10,000 cable clients under load (Pss) | 2,034 MB | 322 MB | 279 MB | **7.3×** | **1.16×** |
| Whole container, 10,000 cable clients under load (Pss) | 3,375 MB | 322 MB | 279 MB | **12×** | **1.16×** |

Cold start here is about 3× the published 149 ms for both Rust builds, so it's this machine's `docker run`, not
the app. The tip's 446 ms against 432 ms is within noise. This session had no upload suite (`SUITES="http cable"`).
