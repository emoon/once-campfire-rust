#!/usr/bin/env python3
"""Before/after table from the ABBA `bench/profile perf --freq 0` runs in this directory.

  compare.py [BASE_GLOB WP_GLOB]   (default: time-base-[12] and time-perf-1-[12])

Per target: mean CPU ms/request and req/s of each side, the change, and per request the system CPU
share, context switches (voluntary + involuntary, all threads) and the blocking pool's share of them.
"""
import glob, json, os, sys

HERE = os.path.dirname(os.path.abspath(__file__))
base_glob, wp_glob = (sys.argv[1:3] if len(sys.argv) > 2 else ("time-base-[12]", "time-perf-1-[12]"))


def runs(pattern):
    paths = sorted(glob.glob(os.path.join(HERE, pattern)))
    return [json.load(open(os.path.join(p, "perf.json") if os.path.isdir(p) else p)) for p in paths]


def mean(xs):
    return sum(xs) / len(xs)


def split(target_run):
    threads = {k: v for k, v in target_run["per_request"].items() if k != "process"}
    user = sum(t["user_ms"] for t in threads.values())
    sys_ms = sum(t["sys_ms"] for t in threads.values())
    switches = sum(t["voluntary"] + t["nonvoluntary"] for t in threads.values())
    pool = threads.get("blocking pool", {})
    pool_switches = pool.get("voluntary", 0) + pool.get("nonvoluntary", 0)
    return sys_ms / (user + sys_ms), switches, pool_switches


base, wp = runs(base_glob), runs(wp_glob)
print(f"{len(base)} base runs ({base_glob}), {len(wp)} WP runs ({wp_glob})\n")
print("| target | CPU ms/req base | WP | Δ | req/s base | WP | Δ | sys share base → WP | ctx switches/req base → WP (blocking pool) |")
print("|---|---|---|---|---|---|---|---|---|")
for target in base[0]:
    if not all(target in r for r in base + wp):
        continue
    cb, cw = mean([r[target]["cpu_ms_per_req"] for r in base]), mean([r[target]["cpu_ms_per_req"] for r in wp])
    rb, rw = mean([r[target]["rps"] for r in base]), mean([r[target]["rps"] for r in wp])
    sb = [split(r[target]) for r in base]
    sw = [split(r[target]) for r in wp]
    print(f"| {target} | {cb:.4f} | {cw:.4f} | {100 * (cw / cb - 1):+.1f}% | {rb:,.0f} | {rw:,.0f} | {100 * (rw / rb - 1):+.1f}% "
          f"| {100 * mean([s[0] for s in sb]):.0f}% → {100 * mean([s[0] for s in sw]):.0f}% "
          f"| {mean([s[1] for s in sb]):.1f} ({mean([s[2] for s in sb]):.1f}) → {mean([s[1] for s in sw]):.1f} ({mean([s[2] for s in sw]):.1f}) |")
