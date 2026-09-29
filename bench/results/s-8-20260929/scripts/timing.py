#!/usr/bin/env python3
"""Table of CPU ms/req, req/s, user/sys split and context switches per request from perf.json/cpu.json runs."""
import json, os, sys
root = sys.argv[1]
runs = sys.argv[2:]
T = ["room_show", "messages_page", "sidebar", "post_message", "cable1000"]
def load(r):
    for f in ("perf.json", "cpu.json"):
        p = os.path.join(root, r, f)
        if os.path.exists(p):
            return json.load(open(p))
    return {}
print("| run | target | CPU ms/req | req/s | user ms | sys ms | sys share | vol cs | invol cs |")
print("|---|---|---|---|---|---|---|---|---|")
for r in runs:
    d = load(r)
    for t in T:
        if t not in d: continue
        x = d[t]
        if t.startswith("cable"):
            pm = x.get("per_message", {})
            u = sum(v.get("user_ms", 0) for k, v in pm.items() if k != "process"); s = sum(v.get("sys_ms", 0) for k, v in pm.items() if k != "process")
            vc = sum(v.get("voluntary", 0) for k, v in pm.items() if k != "process"); ic = sum(v.get("nonvoluntary", 0) for k, v in pm.items() if k != "process")
            print(f"| {r} | {t} (per message, {x['delivered_msgs_per_sec']} msg/s) | {u+s:.3f} | {x['delivered_msgs_per_sec']} | {u:.3f} | {s:.3f} | {100*s/max(u+s,1e-9):.0f}% | {vc:.0f} | {ic:.0f} |" if pm else f"| {r} | {t} | | {x['delivered_msgs_per_sec']} msg/s | | | | | |")
            continue
        pr = x.get("per_request", {})
        u = sum(v.get("user_ms", 0) for k, v in pr.items() if k != "process"); s = sum(v.get("sys_ms", 0) for k, v in pr.items() if k != "process")
        vc = sum(v.get("voluntary", 0) for k, v in pr.items() if k != "process"); ic = sum(v.get("nonvoluntary", 0) for k, v in pr.items() if k != "process")
        extra = f"{u:.4f} | {s:.4f} | {100*s/max(u+s,1e-9):.0f}% | {vc:.1f} | {ic:.1f}" if pr else " | | | | "
        print(f"| {r} | {t} | {x['cpu_ms_per_req']:.4f} | {x['rps']:,.0f} | {extra} |")
