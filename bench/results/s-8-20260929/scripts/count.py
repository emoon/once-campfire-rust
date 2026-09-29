#!/usr/bin/env python3
"""Per-request call counts: runs the app under gdb with ignore-counted breakpoints, for 0 and N
requests of a route, and prints (hits(N) - hits(0)) / N per breakpoint.
  SERVER_CPUS=0-3 LOADGEN_CPUS=4-7 PORT=4490 count.py BINARY ROUTE N"""
import os, re, signal, subprocess, sys, time, json
W = "/home/emoon/campfire-wt/s-8"
sys.path.insert(0, W + "/bench/lib")
import benchlib as b

SC = os.path.dirname(os.path.abspath(__file__))
binary, route, n = sys.argv[1], sys.argv[2], int(sys.argv[3])
POINTS = [
    ("hmac sha1", "message_verifier.rs:143"), ("hmac sha256", "message_verifier.rs:148"),
    ("generate_key (cache lookup)", "key_generator.rs:25"), ("pbkdf2", "key_generator.rs:30"),
    ("aes-gcm decrypt", "message_encryptor.rs:55"), ("cookie verify_signed_value", "cookies.rs:51"),
    ("signed_id::generate", "signed_id.rs:18"), ("turbo signed_stream_name", "turbo.rs:17"), ("GlobalId::to_param", "global_id.rs:61"),
    ("splice PageParts::new", "splice.rs:97"), ("splice text_part sha256", "splice.rs:253"), ("splice etag", "splice.rs:137"),
    ("splice gzip", "splice.rs:165"), ("splice compress (piece miss)", "splice.rs:290"),
    ("Database::read (spawn_blocking)", "database.rs:310"), ("Database::write", "database.rs:279"),
    ("sqlite3_step", "-qualified sqlite3_step"), ("sqlite3 prepare", "-qualified sqlite3LockAndPrepare"),
    ("sqlite3 blocking read (pread64)", "-qualified pread64"), ("sqlite3 write (pwrite64)", "-qualified pwrite64"), ("fsync/fdatasync", "-qualified fdatasync"),
    ("libc writev", "-qualified writev"), ("libc write", "-qualified write"), ("libc syscall (futex etc.)", "-qualified syscall"),
    ("libc epoll_wait", "-qualified epoll_wait"), ("libc clone3", "-qualified clone3"), ("libc recv", "-qualified recv"), ("libc read", "-qualified read"),
    ("libc sendto", "-qualified sendto"), ("libc sched_yield", "-qualified sched_yield"), ("libc clock_gettime", "-qualified clock_gettime"),
]
extra = os.environ.get("COUNT_EXTRA", "")
for spec in filter(None, extra.split(";")):
    name, _, loc = spec.partition("=")
    POINTS.append((name, loc))

cmds = os.path.join(SC, f"gdb-{os.getpid()}.cmds")
with open(cmds, "w") as f:
    f.write("set pagination off\nset confirm off\nset breakpoint pending on\nset print thread-events off\n"
            "handle SIGTERM nostop noprint pass\nhandle SIGPIPE nostop noprint pass\nhandle SIGUSR1 nostop noprint pass\n"
            "handle SIGUSR2 nostop noprint pass\nhandle SIGINT nostop noprint pass\n")
    for i, (_, loc) in enumerate(POINTS, 1):
        f.write(f"break {loc}\nignore {i} 2000000000\n")
    f.write("run\necho ==COUNTS==\\n\ninfo breakpoints\n")
wrapper = os.path.join(SC, f"gdb-wrap-{os.getpid()}.sh")
open(wrapper, "w").write(f"#!/bin/sh\nexec gdb -q -nx -batch -x {cmds} --args {binary} \"$@\"\n")
os.chmod(wrapper, 0o755)
os.environ["NATIVE_CNT_BIN"] = wrapper

seed = b.WORK + "-seed"
labels = b.snapshot_seed(seed)


def run(requests):
    import benchlib
    benchlib_wait = benchlib.wait_up
    benchlib.wait_up = lambda base, timeout=60: benchlib_wait(base, 600)
    app = b.start("native-cnt", seed)
    try:
        sess = b.session(app, labels)
        rts = b.routes(sess)
        if requests:
            r, _ = b.lg("http", "--base", app.base, "--cookie", sess["cookie"], *rts[route], "--conc", 1, "--duration", 900, "--requests", requests)
            assert r["latency"]["n"] == requests, r["latency"]
    finally:
        gdb = app.proc.pid
        inferior = [c for c in b.children(gdb) if b.comm(c) == "campfire"]
        for p in inferior:
            os.kill(p, signal.SIGTERM)
        app.proc.wait(300)
    log = open(os.path.join(b.WORK, "app.log")).read()
    table = log[log.index("==COUNTS=="):]
    hits = {}
    num = None
    for line in table.splitlines():
        m = re.match(r"^(\d+)\s+breakpoint", line)
        if m:
            num = int(m.group(1))
            hits[num] = 0
        m = re.search(r"breakpoint already hit (\d+) time", line)
        if m and num:
            hits[num] = int(m.group(1))
    return hits


h0 = run(0)
hn = run(n)
out = {}
for i, (name, loc) in enumerate(POINTS, 1):
    out[name] = round((hn.get(i, 0) - h0.get(i, 0)) / n, 2)
    print(f"{out[name]:10.2f}  {name}  ({loc})")
json.dump(out, open(os.path.join(SC, f"counts-{route}-{os.environ.get('COUNT_TAG', '')}.json"), "w"), indent=1)
