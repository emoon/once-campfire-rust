# S-9: is the write path and cable slower than published v0.1.1? (2026-09-29)

Answer: no regression. The README's v0.1.1 table was measured on a Ryzen AI Max+ 395; this host is
a 9950X3D, slower on loopback and kernel paths and faster on compute, so the published numbers
don't transfer. Re-measured here, `a` (080f903, the v0.1.1 build) and `b` (9e2a110, just before
the cleanup started) match within noise. `bench/run` (http + cable, 3 reps each, `run/a`, `run/b`):

| metric | a (080f903) | b (9e2a110) |
|---|---|---|
| post_message c=16 req/s | 4,776 [4,626–4,797] | 4,702 [2,604–4,866] |
| post_message c=16 p50 ms | 2.29 | 2.35 |
| post_message c=16 p99 ms | 43.9 | 41.5 |

`time/` holds the ABBA CPU runs and `cpu/` the profiles. The 080f903 numbers here are the
reference for comparing the cleanup's end state with v0.1.1 on this host (P-5).

Written from the S-9 agent's results and the coordinator's handoff after the agent was stopped;
the agent left no README of its own, and the label "v0.1.1" for 080f903 is its, not re-checked.
