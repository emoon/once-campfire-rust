#!/usr/bin/env python3
"""Tail latency at c=16 and c=64 for native builds, several apps in one session (bench/lib).

  tails.py --labels base,perf-1,perf-1,base [--routes sidebar,search,room_show,messages_page] [--secs 8] --out DIR

Each label (binary from NATIVE_<LABEL>_BIN) gets a fresh app on a fresh seed copy, a 2 s warm-up
per route, then each route at c=16 and c=64. Writes DIR/<n>-<label>.json per app. Run it inside the
bench lock and bench/quiet, like the other timing runs.
"""
import argparse, json, os, sys

sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "..", "lib"))
import benchlib as b

p = argparse.ArgumentParser()
p.add_argument("--labels", required=True)
p.add_argument("--routes", default="sidebar,search,room_show,messages_page")
p.add_argument("--concs", default="16,64")
p.add_argument("--secs", default="8")
p.add_argument("--out", required=True)
a = p.parse_args()
os.makedirs(a.out, exist_ok=True)
b.build_loadgen()
seed = b.WORK + "-seed"
labels = b.snapshot_seed(seed)

for n, label in enumerate(a.labels.split(",")):
    app = b.start(f"native-{label}", seed)
    results = {}
    try:
        sess = b.session(app, labels)
        rts = b.routes(sess)
        for route in a.routes.split(","):
            b.lg("http", "--base", app.base, "--cookie", sess["cookie"], *rts[route], "--conc", 4, "--duration", 2)
            for conc in a.concs.split(","):
                r = b.http(app, sess, rts[route], int(conc), a.secs)
                results[f"{route} c={conc}"] = {"rps": r["rps"], **{k: r["latency"].get(k) for k in ("p50_ms", "p90_ms", "p99_ms", "max_ms")}}
                b.log(f"{label}: {route} c={conc}: {r['rps']} rps, p50 {r['latency'].get('p50_ms')} p99 {r['latency'].get('p99_ms')}")
    finally:
        app.stop()
    json.dump(results, open(os.path.join(a.out, f"{n}-{label}.json"), "w"), indent=1)
