#!/usr/bin/env python3
"""Every 5 s: busy % of the compare's CPUs (0-7,16-23) and the benchmark's (8-15,24-31), and the
mean clock of the benchmark's server/loadgen cores (8-15). Appends CSV lines to argv[1]."""
import sys, time

def cpus(spec):
    out = []
    for part in spec.split(","):
        lo, _, hi = part.partition("-")
        out += range(int(lo), int(hi or lo) + 1)
    return out

GROUPS = {"compare": cpus("0-7,16-23"), "bench": cpus("8-15,24-31")}

def stat():
    t = {}
    for line in open("/proc/stat"):
        if line.startswith("cpu") and line[3].isdigit():
            f = line.split(); v = list(map(int, f[1:9]))
            idle = v[3] + v[4]
            t[int(f[0][3:])] = (sum(v), idle)
    return t

def mhz(cs):
    vals = []
    for c in cs:
        try: vals.append(int(open(f"/sys/devices/system/cpu/cpu{c}/cpufreq/scaling_cur_freq").read()) / 1000)
        except OSError: pass
    return sum(vals) / len(vals) if vals else 0

out = open(sys.argv[1], "a", buffering=1)
out.write("epoch,compare_busy_pct,bench_busy_pct,bench_mhz,load1\n")
prev = stat()
while True:
    time.sleep(5)
    cur = stat()
    row = [f"{time.time():.0f}"]
    for g in ("compare", "bench"):
        tot = sum(cur[c][0] - prev[c][0] for c in GROUPS[g]); idle = sum(cur[c][1] - prev[c][1] for c in GROUPS[g])
        row.append(f"{100 * (tot - idle) / tot:.1f}" if tot else "0")
    row.append(f"{mhz(range(8, 16)):.0f}")
    row.append(open("/proc/loadavg").read().split()[0])
    out.write(",".join(row) + "\n")
    prev = cur
