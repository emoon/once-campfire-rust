# S-6: parity compare beside the benchmarks (dropped)

The question (`plans/cleanup.md`, S-6): can a parity compare run on its own lock, with all its
containers pinned to the CCD the benchmarks leave free (CPUs 0-7,16-23), while `bench/profile` runs
on 8-15? The answer is no. Pinned, the compare still costs the benchmark 3.6-6.5% CPU per request and
3.6-17% throughput, which is outside T on every target. So parity stays under
`/tmp/campfire-bench.lock`.

## What was run

One binary: `refactor/cleanup` at `031ba60`, release with `CARGO_PROFILE_RELEASE_DEBUG=line-tables-only`,
copied to `target/s6-campfire`, used as both label `a` and label `b`. Everything ran in one hold of
the bench lock ([`orchestrate.sh`](orchestrate.sh), timeline in [`phases.log`](phases.log)):

1. Images `campfire-rust:s-6` and `campfire-candidate-s-6` (`parity/bin/candidate build`).
2. **(a1)**, with nothing else running: `bench/profile alloc` on the four targets (post_message
   twice), then `bench/profile cpu` twice (`cpu-a-1`, `cpu-a-2`).
3. A pinned compare under `/tmp/campfire-parity.lock`:
   `PARITY_WORKERS=8 taskset -c 0-7,16-23 parity/bin/candidate compare --ports 4121,4122`, with
   the scripts patched so that every container it starts (reference, candidate, Playwright capture
   and its forwarder) gets `--cpuset-cpus 0-7,16-23` ([`parity-cpuset.diff`](parity-cpuset.diff),
   `PARITY_CPUSET`). `docker inspect` confirmed the pinning on all eight containers.
   `PARITY_WORKERS=8` keeps the usual 8 browser workers: Node's `availableParallelism()` sees the
   cpuset's 16 CPUs, so the default would drop to 4.
4. **(b)**, while the compare ran (starting once it was scheduling cells, plus a minute): alloc on
   the four targets, then `cpu` runs back to back until it ended (`cpu-b-1` … `cpu-b-17`).
5. **(a2)**, after the compare: `cpu` twice (`cpu-a-3`, `cpu-a-4`), then alloc.

[`sampler.py`](sampler.py) logged, every 5 s, how busy each CCD was, the mean clock of CPUs 8-15,
and the load average ([`cpu-sample.csv`](cpu-sample.csv)).

## The compare itself passes

`874 cells: 873 pass (0 flaky), 0 fail, 1 allowed, 0 error in 2061.2s`, the same result as the
baseline (1,948 s unpinned), so pinning to 16 threads costs the compare about 6% in wall time.
While it ran it kept its CCD only 20-35% busy.

## Allocations: unchanged

| target | (a1) | (b) | (a2) |
|---|---|---|---|
| room_show | 18.0 | 18.0 | 18.1 |
| messages_page | 19.0 | 19.0 | 19.0 |
| sidebar | 53.1 | 53.1 | 53.1 |
| post_message (two runs) | 78.9, 78.9 | 78.4, 77.8 | 77.7, 78.4 |

post_message's higher run is 0.5 lower with the compare running, but (a2), with no compare, reads
the same 78.4. So the drop is in the load on the box, not the compare, and it points down, not up.
These counts come from the preloaded system jemalloc, so they cover SQLite and C allocations only,
not Rust's (S-7 in `plans/cleanup.md`). The verdict below rests on CPU and throughput, so this
doesn't change it.

## CPU per request and throughput: outside T

This is the clean comparison: (a1) against the first four (b) runs, 09:03-09:12, all measured
before the screensaver (below) started.

| target | (a) CPU ms/req | (b) CPU ms/req | Δ | (a) req/s | (b) req/s | Δ | T |
|---|---|---|---|---|---|---|---|
| room_show | 0.1693, 0.1694 | 0.1743, 0.1765, 0.1771, 0.1770 | **+4.1%** | 21,753, 21,773 | 21,172, 20,804, 20,476, 20,793 | **−4.4%** | 3% |
| messages_page | 0.1448, 0.1428 | 0.1488, 0.1475, 0.1508, 0.1486 | **+3.6%** | 25,353, 25,674 | 24,810, 25,119, 23,963, 24,444 | **−3.6%** | 3% |
| sidebar | 0.2717, 0.2700 | 0.2825, 0.2828, 0.2842, 0.2826 | **+4.5%** | 13,683, 13,707 | 13,149, 13,154, 12,788, 13,072 | **−4.8%** | 3% |
| post_message | 0.4509, 0.4522 | 0.4779, 0.4798, 0.4789, 0.4862 | **+6.5%** | 4,789, 4,802 | 4,152, 4,253, 3,311, 4,212 | **−17.0%** | 3.2% |

Every target fails on both CPU/req and req/s. `b-5` and `b-6` (still before the screensaver) agree:
+2.9 to +5.4% CPU/req. Back-to-back runs of the same binary normally agree to within 0.1-0.7%
(`cleanup-baseline-20260928`), so a 4-6% shift is the compare.

**Why pinning doesn't isolate it.** The sampler shows no extra work on the benchmark's CPUs: when
the benchmark was idle they were 7% busy in (a1) and 9% in (b). But their clock dropped: CPUs 8-15
averaged 5,475 MHz during the (a1) cpu runs and 5,349 MHz (−2.3%) during b-1..b-4 (5,259 MHz in
b-5..b-6), with the other CCD 24-26% busy. Boost is a package-wide power and thermal budget, and the
two CCDs also share the I/O die, memory and the disk. The compare's disk I/O goes through unbound
kernel workers (btrfs endio, kcryptd), which `ps` caught running on the benchmark's CPUs. dockerd
and docker-proxy can't be pinned per container either. post_message, which writes to SQLite, suffers
most (−17% req/s).

## The screensaver: a finding that affects every perf gate

From about 09:16 the desktop screensaver (two `foot --app-id=org.omarchy.screensaver … -e
omarchy-screensaver` windows under Hyprland) used about 5 unpinned CPUs, most of them the
benchmark's. From `b-7` on, and in (a2) with no compare at all, the numbers are 18-47% worse and
noisy:

| run | room_show | messages_page | sidebar | post_message |
|---|---|---|---|---|
| a-1 (clean) | 0.1693 / 21,753 | 0.1448 / 25,353 | 0.2717 / 13,683 | 0.4509 / 4,789 |
| a-2 (clean) | 0.1694 / 21,773 | 0.1428 / 25,674 | 0.2700 / 13,707 | 0.4522 / 4,802 |
| b-8 (compare + screensaver) | 0.2202 / 15,666 | 0.2051 / 16,398 | 0.3759 / 9,277 | 0.6879 / 3,104 |
| a-3 (screensaver only) | 0.2107 / 17,092 | 0.1731 / 20,572 | 0.3374 / 10,824 | 0.6642 / 3,355 |
| a-4 (screensaver only) | 0.1989 / 18,245 | 0.1727 / 20,540 | 0.3335 / 10,978 | 0.6287 / 3,598 |

(CPU ms/req / req/s.) The two screensaver-only runs differ by up to 6%, so ABBA can't cancel it at
a 3% tolerance. These runs aren't used in the S-6 verdict. Every run is in `cpu-*/cpu.json`, and
`b-17` straddled the compare's end. A `mise x gh -- gh auth token` process (up 3.7 days) also spins
at about 22% of a core, unpinned.

## Verdict

Dropped (`[-]`). A compare pinned to the other CCD still moves `bench/profile cpu` by 3.6-6.5%
CPU/req and 3.6-17% req/s. Allocation counts are unaffected. The script change is not kept: pinned
by default, every compare would get 4 browser workers instead of 8 and take longer, for nothing.
The diff is here for reference.

Files: `{a1,b,a2}-<route>/alloc-<route>.json`, `cpu-{a,b}-<n>/cpu.json` and
`cpu-*/cpu-<target>.top.md`, `phases.log`, `cpu-sample.csv`, and the compare report at
`parity/out/compare-2026-09-29T07-02-44-236Z/` in the S-6 worktree (not committed).
