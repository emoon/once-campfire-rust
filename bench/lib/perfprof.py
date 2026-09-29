#!/usr/bin/env python3
"""Turn a `perf record --call-graph` profile into the same folded stacks and rollup as cpuprof.py,
symbolized the same way (llvm-symbolizer, inlined frames expanded), so the two profilers' reports
compare line for line.

  perfprof.py PERF_DATA --out PREFIX [--title T] [--comm REGEX]

Samples are weighted by their period (cpu-clock: nanoseconds), reported in µs-sized units.
--comm keeps only the threads whose name matches (e.g. "tokio-rt-worker" or "campfire-db").

--per-thread runs `perf script` once per thread, for `--call-graph dwarf` recordings: perf 7.2's
libdw unwinder keeps one Dwfl per process, attached to the first thread it unwinds, and fails every
other thread's samples with "No such process". Run it pinned off the benchmark cores (taskset).
"""
import argparse, collections, concurrent.futures, os, re, subprocess, sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import cpuprof

ENTRY = re.compile(r"^\s*([0-9a-f]+) \((.*)\+0x([0-9a-f]+)\)$")


def perf_script(data, *extra):
    return subprocess.run(["perf", "script", "-i", data, *extra], capture_output=True, text=True, check=True).stdout


def read_samples(data, comm_re, per_thread=False, jobs=8):
    """[(period ns, comm, [(dso, file offset), ...] leaf first)] from `perf script`."""
    fields = ["-F", "comm,tid,period,ip,dsoff", "--no-inline"]
    if per_thread:
        tids = sorted({line.strip() for line in perf_script(data, "-F", "tid").splitlines() if line.strip()}, key=int)
        with concurrent.futures.ThreadPoolExecutor(jobs) as pool:
            outputs = pool.map(lambda tid: perf_script(data, "--tid", tid, *fields), tids)
            return [sample for out in outputs for sample in parse(out, comm_re)]
    return parse(perf_script(data, *fields), comm_re)


def parse(out, comm_re):
    samples = []
    head, chain = None, []
    for line in out.splitlines() + [""]:
        if not line.strip():
            if head and (comm_re is None or comm_re.search(head[1])):
                samples.append((head[0], head[1], chain))
            head, chain = None, []
        elif head is None:
            # "<comm> <tid> <period>": comm may contain spaces.
            parts = line.split()
            head = (int(parts[-1]), " ".join(parts[:-2]))
        else:
            m = ENTRY.match(line)
            chain.append((m.group(2), int(m.group(3), 16)) if m else ("?", int(line.split()[0], 16)))
    return samples


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("data")
    ap.add_argument("--out", required=True)
    ap.add_argument("--title", default="")
    ap.add_argument("--comm")
    ap.add_argument("--per-thread", action="store_true")
    a = ap.parse_args()
    samples = read_samples(a.data, re.compile(a.comm) if a.comm else None, a.per_thread)
    by_obj = collections.defaultdict(dict)
    for _, _, chain in samples:
        for j, (dso, off) in enumerate(chain):
            # Non-leaf entries are return addresses: look up the call instruction's line.
            key = (dso, off if j == 0 else off - 1)
            by_obj[dso][key] = key[1]
    names = cpuprof.symbolize_offsets(by_obj)
    stacks = []
    for period, comm, chain in samples:
        frames = []
        for j, (dso, off) in enumerate(chain):
            key = (dso, off if j == 0 else off - 1)
            frames.extend(reversed(names.get(key, [f"{os.path.basename(dso)}+{hex(off)}"])))
        stacks.append((period // 1000, frames))
    cpuprof.write_reports(stacks, 1, a.out, a.title)


if __name__ == "__main__":
    main()
