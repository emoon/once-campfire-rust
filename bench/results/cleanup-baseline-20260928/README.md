# Cleanup baseline (Phase 0, S-2)

The numbers every cleanup WP compares against (`plans/cleanup.md`, "Definition of done"). Code:
`refactor/cleanup` at `7e8c13f`, which is `main` (`9e2a110`) plus plan commits only. Rust 1.98.1.
Host: AMD Ryzen 9 9950X3D, 32 threads; server pinned to CPUs 8-11, load generator to 12-15.

## Tests

`cargo test --workspace --exclude html5ever`:

| | passed | failed | ignored | skipped at runtime |
|---|---|---|---|---|
| before the seed was built | 652 | 0 | 7 | 53 seed-backed tests (52 on `default`, 1 on `first_run`) returned early and "passed" |
| after `parity/bin/seed build` | 652 | 0 | 7 | none on the seed |

Left over after the seed: the storage vectors skip their libvips/ffmpeg byte comparisons because the
host's versions (libvips 8.18.6, ffmpeg n9.0.1) differ from the image's the vectors were made with
(8.16.1, 7.1.5). The 7 ignored tests need a running reference app or a database the reference made;
`reference-tools/db/differential.sh` runs the three db ones against the reference image: all
three pass (`scenario_matches_ruby`, `fixtures_match_ruby_row_for_row`, `export_database_for_rails`).
The script then stops at its schema-identity diff, which doesn't know about the index the app adds
on boot on purpose (README, "One more index"), so its last step (Rails on the Rust-written
database) doesn't run. Noted in `TASKS.md` under "Found while working".

## Parity lean gate

`parity/bin/candidate compare` (seed `default`, clock frozen at 2026-03-02T16:00:00Z):
**874 cells: 873 pass (0 flaky), 0 fail, 1 allowed, 0 error** in 1,948 s. The allowed cell is
`pwa/manifest @ chromium-desktop-light`, the manifest listed in `parity/allowlist.yml`. A WP's
gate passes with the same result: no failing or erroring cell.

## Allocations and CPU per request

`bench/profile alloc` (1,000 requests at c=1) and `bench/profile cpu` (10 s at c=16, under the
1 kHz profiler) on the same binary (release, `CARGO_PROFILE_RELEASE_DEBUG=line-tables-only`), each
run twice. Spread is |run 1 − run 2| / mean. T is the gate's tolerance for the target:
max(3%, the larger spread).

| target | allocs/req run 1 | run 2 | CPU ms/req run 1 | run 2 | spread | req/s run 1 | run 2 | spread | T |
|---|---|---|---|---|---|---|---|---|---|
| room_show | 18.0 | 18.1 | 0.1708 | 0.1709 | 0.06% | 21,641 | 21,627 | 0.07% | 3% |
| messages_page | 19.0 | 19.0 | 0.1447 | 0.1448 | 0.07% | 25,410 | 25,377 | 0.13% | 3% |
| sidebar | 53.1 | 53.0 | 0.2748 | 0.2766 | 0.65% | 13,485 | 13,438 | 0.35% | 3% |
| post_message | 78.7 | 78.7 | 0.4608 | 0.4586 | 0.48% | 4,803 | 4,729 | 1.55% | 3.2% (see below) |

The S-2 dry runs of the gate commands (`plans/cleanup.md`, "Commands for the gates") measured the
same code again, as a `base` and a `dryrun` binary built in two checkouts, twice over about an hour
(the second `base` was after S-4 and S-5, which change formatting and lint attributes only):

| target | allocs/req (all runs) | CPU ms/req (6 runs) | req/s (6 runs) | spread over all runs |
|---|---|---|---|---|
| room_show | 18.0 ×4, 18.1 ×2 | 0.1706–0.1730 | 21,236–21,642 | 1.4% CPU, 1.9% req/s |
| messages_page | 19.0 ×4 | 0.1438–0.1448 | 25,200–25,467 | 0.7% CPU, 1.1% req/s |
| sidebar | 53.1, 53.0, 53.1, 53.0 | 0.2748–0.2824 | 13,114–13,517 | 2.7% CPU, 3.0% req/s |
| post_message | 78.7 ×5, 78.8, 77.2 ×2, 77.3 ×2 | 0.4586–0.4668 | 4,651–4,803 | 1.8% CPU, 3.2% req/s |

Runs back to back agree much more closely than runs an hour apart, which is why the gate measures
base and WP in the same session.

**Tolerances.** T = 3% for room_show, messages_page and sidebar, and 3.2% for post_message (its
req/s spread over all runs; its CPU ms/req spread is 1.8%).

**Allocations.** room_show, messages_page and sidebar move by at most 0.1 per request between
runs of the same code, so 0.1 is noise and 0.2 or more is a real change. post_message is bimodal:
the same binary measures either 78.7 (±0.1) or 77.2 (±0.1); 4 of 10 runs landed low. Run it twice
on each side and compare the higher run of each; a gap of about 1.5 is the mode flipping, not the
change, so re-run before deciding.

Files: `alloc-{1,2}/<route>/alloc-<route>.json` (and the `jeprof` top sites, `*.alloc_objects.txt`,
`*.alloc_space.txt`), `cpu-{1,2}/cpu.json` and `cpu-{1,2}/cpu-<target>.top.md` (rollups; the folded
stacks are gitignored, and there are no flamegraph SVGs because `inferno` isn't installed).

## bench/run --apps rust --reps 3

The production image (`campfire-rust:app`, built by `parity/bin/candidate build` from this commit),
full suites (http, cable, upload). Medians and [min–max] are in [`run/report.md`](run/report.md).
Headlines (median req/s): room_show c=16 22,426; messages_page c=16 25,807; sidebar c=16 13,463;
post_message c=16 4,813; cable 1,000 clients 400 msgs/s delivered to all; upload → thumbnail 26.0 ms.

Most throughput metrics vary by less than 3% (max − min over the median) across the three reps.
These vary more, so their T is the spread:

| metric | spread (T) |
|---|---|
| messages_page c=1 req/s | 5.7% |
| cable 100 clients: max sustained msgs/s, deliveries/s | 4.6% |
| search c=16 req/s | 4.3% |
| search c=1 req/s | 3.3% |
| cold start (ms) | 3.3% |

## How this was run

```sh
export CARGO_PROFILE_RELEASE_DEBUG=line-tables-only
cargo build --release -p campfire     # copied aside so later builds couldn't change it mid-run
export NATIVE_BASE_BIN=<that copy>
for rep in 1 2; do for route in room_show messages_page sidebar post_message; do
  flock /tmp/campfire-bench.lock bench/profile alloc --label base --route $route --out $OUT/alloc-$rep/$route
done; done
for rep in 1 2; do
  flock /tmp/campfire-bench.lock bench/profile cpu --label base \
    --targets room_show,messages_page,sidebar,post_message --out $OUT/cpu-$rep
done
flock /tmp/campfire-bench.lock bench/run --apps rust --reps 3 --out $OUT/run
```

`cpu.json` comes from a small change to `bench/profile` in this commit: before it, `cpu` printed
each target's req/s and CPU ms/req only to the terminal.

## Size (bench/loc, S-3)

Non-blank lines per crate at this commit; the rules are in `bench/loc`'s header. WPs put the
production delta from `bench/loc --against refactor/cleanup` in their commit message.

| crate | production | comments | tests |
|---|---|---|---|
| assets | 1,341 | 126 | 318 |
| cable | 1,384 | 268 | 1,146 |
| campfire | 10,887 | 1,552 | 4,844 |
| db | 4,505 | 476 | 2,562 |
| kit | 6,002 | 794 | 2,612 |
| rails_compat | 739 | 182 | 389 |
| richtext | 2,490 | 313 | 1,026 |
| routes | 89 | 4 | 0 |
| storage | 2,154 | 278 | 457 |
| views | 2,872 | 562 | 1,634 |
| views (templates) | 1,771 | 36 | 0 |
| **total** | 34,234 | 4,591 | 14,988 |

After S-4's `cargo fmt --all` (63b72b1), the baseline later WPs actually diff against:

| crate | production | comments | tests |
|---|---|---|---|
| assets | 1,163 | 126 | 273 |
| cable | 1,473 | 268 | 1,393 |
| campfire | 12,252 | 1,552 | 6,007 |
| db | 4,073 | 476 | 2,094 |
| kit | 6,683 | 794 | 3,258 |
| rails_compat | 828 | 182 | 440 |
| richtext | 2,823 | 313 | 1,239 |
| routes | 106 | 4 | 0 |
| storage | 2,319 | 278 | 561 |
| views | 3,285 | 562 | 1,869 |
| views (templates) | 1,771 | 36 | 0 |
| **total** | 36,776 | 4,591 | 17,134 |
