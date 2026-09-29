#!/usr/bin/env python3
"""Query folded stacks (cpuprof.py / perfprof.py output), for explaining a rollup's buckets:

  stacks.py FOLDED incl REGEX...                share of samples with a frame matching each REGEX
  stacks.py FOLDED callers REGEX [--depth N]    who reaches the frames matching REGEX (N frames up)
  stacks.py FOLDED callees REGEX [--depth N]    where the time under REGEX goes (N frames down)
  stacks.py FOLDED self [--under REGEX]         top self functions, optionally only below REGEX

Frames matching --skip (default: std/core/alloc plumbing, allocator and lock internals) are left
out of caller/callee paths so the path names the code that decides something.
"""
import argparse, collections, re

SKIP = (r"^(std::|core::|alloc::|<alloc::|<core::|<std::|drop_glue|drop<|call_once|{closure}|__rust|_rjem|je_|imalloc|"
        r"lock_api|parking_lot|<parking_lot|tokio::loom|with_thread_data|internal_syscall|<&|<\[|\[)")


def short(frame):
    """Drops generic arguments: `update<CoreWrapper<..>>` → `update`, `<Foo<T> as Bar>::f` → `<Foo as Bar>::f`."""
    prev = None
    while prev != frame:
        prev, frame = frame, re.sub(r"([\w}])<[^<>]*>", r"\1", frame)
    return frame


def load(path):
    stacks = []
    for line in open(path):
        stack, n = line.rstrip("\n").rsplit(" ", 1)
        stacks.append(([short(f) for f in stack.split(";")], int(n)))
    return stacks


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("folded")
    ap.add_argument("cmd", choices=["incl", "callers", "callees", "self"])
    ap.add_argument("regex", nargs="*")
    ap.add_argument("--depth", type=int, default=3)
    ap.add_argument("--top", type=int, default=25)
    ap.add_argument("--under")
    ap.add_argument("--skip", default=SKIP)
    a = ap.parse_args()
    stacks = load(a.folded)
    total = sum(n for _, n in stacks)
    skip = re.compile(a.skip) if a.skip else None
    pct = lambda n: f"{100 * n / max(total, 1):5.2f}%"
    keep = lambda f: not (skip and skip.search(f))
    if a.cmd == "incl":
        for r in a.regex:
            rx = re.compile(r)
            print(pct(sum(n for fr, n in stacks if any(rx.search(f) for f in fr))), r)
        return
    if a.cmd == "self":
        under = re.compile(a.under) if a.under else None
        c = collections.Counter()
        for fr, n in stacks:
            if under is None or any(under.search(f) for f in fr):
                c[fr[-1]] += n
        for f, n in c.most_common(a.top):
            print(pct(n), f[:200])
        return
    rx = re.compile(a.regex[0])
    c = collections.Counter()
    for fr, n in stacks:
        idx = [i for i, f in enumerate(fr) if rx.search(f)]
        if not idx:
            continue
        if a.cmd == "callers":
            i = idx[0]  # outermost match, so recursion counts once
            path = [f for f in fr[:i] if keep(f)][-a.depth:]
            c[" <- ".join([fr[i]] + list(reversed(path)))] += n
        else:
            i = idx[-1]
            path = [f for f in fr[i + 1:] if keep(f)][:a.depth]
            c[" -> ".join([fr[i]] + path)] += n
    for k, n in c.most_common(a.top):
        print(pct(n), k[:400])


if __name__ == "__main__":
    main()
