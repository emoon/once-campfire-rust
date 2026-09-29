# Cleanup plan: faster, smaller, easier to read

A working plan for several agents working in parallel. The live checklist is `TASKS.md`; this file
explains each work package (WP) and the rules they all follow. It was written from a read-only
review of every crate on 2026-09-28 (six reviewers, one per crate group, with the main claims
re-checked by hand). Line numbers below are from commit `9e2a110` and will drift.

## Priorities

In this order, every time they conflict:

1. **Performance.** No change may make a measured path slower or allocate more per request. Where a
   cleanup also removes work from a hot path, measure and record it.
2. **Readability and cleanliness.** Code should say what it means: named types instead of
   `Option<Option<T>>`, bools and strings; one obvious place for each piece of logic; small
   functions.
3. **Reuse and size.** Delete duplicates and boilerplate. Fewer lines is a goal, but never at the
   cost of 1 or 2: no clever macros that hide control flow, and no generic layers with one user.

The compatibility constraints in `CLAUDE.md` still hold. Keep the SQLite schema, the storage layout,
the cookie and message formats (golden vectors) and the observable HTML/HTTP/cable output unchanged,
unless a WP says otherwise. Any deliberate divergence goes under "Known differences" in `README.md`.

## Where we start

### Size

The Rails app is small because Rails and its gems do most of the work. The Rust port carries its
own versions of those, so compare like with like. The counts below are non-blank, non-comment
lines. Tests are excluded, and so is the generated MIME table.

| What | Lines |
|---|---|
| Rails app: `app/`, `lib/`, `config/` (Ruby 3.5k + ERB 1.9k) | ~5.5k |
| **Rust production code, total** | **35.0k** |
| Data: `storage/src/tables.rs` (Marcel MIME tables) | 3.2k |
| Framework replacements: kit 5.8k, storage 2.1k, richtext 2.5k, cable 1.4k, assets 1.3k, rails_compat 1.0k | 14.1k |
| Gem replacements inside `campfire`: user agent 1.0k, search highlighting 0.8k, net + SSRF guard 0.7k, rqrcode 0.6k, Active Storage controllers 0.5k, web push 0.5k, jobs 0.2k | 4.3k |
| App-equivalent Rust: campfire 6.3k, db 4.1k, views 2.9k, routes 0.1k, plus Askama templates 1.8k | ~15.2k |
| Rust tests (vs Rails tests: 3.9k) | 15.0k |

These are review-time estimates. The measured numbers, which WPs diff against, are the `bench/loc`
tables in `bench/results/cleanup-baseline-20260928/README.md` (36.4k production lines after S-4's
`cargo fmt`, templates included).

So the part that corresponds to the Rails app is about 2.8× its size, not 6×. Some of that is the
language: types, explicit errors and borrowing. Some of it is duplication this plan removes. Rough
estimate, unconfirmed until done: 2.5–3k production lines can go without touching behavior or speed.

### Health

- Workspace tests pass (`cargo test --workspace --exclude html5ever`, ~70 s). At review time the
  seed wasn't built, so the integration tests skipped silently; S-2 built it (53 tests now run).
- Default clippy is clean on 1.98.1 (S-1; the richtext `nonminimal_bool` warning seen with cargo
  1.93 at review time no longer fires). Pedantic clippy is almost all doc/attribute noise. The
  useful parts: 44 `push_str(&format!(..))`, 6 redundant clones, 25 by-value arguments that are
  never consumed, 11 lock guards held too long, and ~200 unchecked `as` casts.
- No `rustfmt.toml`, and the code isn't rustfmt-formatted at any width (140 is closest: 1,872 hunks).
- `Box` is not a real problem: 25 `Box::new` calls in 35k lines, nearly all justified (type-erased
  hooks and jobs, futures axum needs boxed, a large enum variant). Only three are worth changing:
  the two dynamic UPDATE builders in db and the web-push handler.

### Keep these patterns

New code should look like these:

- **Route table as data** (`campfire/src/controllers.rs`), the `Before` builder, and `halt` as `Err`,
  with the `Error` enum mapping Rails exceptions to statuses in one place.
- **Literal SQL next to each Rails citation**, through `sql::query_one`/`query_all` and
  `prepare_cached`, with `from_row` by column name. Don't macro-generate finders.
- **`Tx` with the after-commit queue and `EventSink`.** `NewUser`/`NewMessage` attribute structs,
  `PasswordDigest`, and the `ToSql`/`FromSql` enums (`RoomType`, `Involvement`).
- **The cable fan-out.** Encode once per broadcast and wrap once per identifier group. `Arc`-shared
  frames, deflated at most once.
- **The fragment cache** (byte-bounded LRU), checked before any presenter work.
- **Pure, borrow-only seams** (`format::formats`, `is_fresh`, `rack_etag`, `parse_range`) that can be
  tested without a `Ctx`.
- **Errors named after the Ruby exception classes** (`DeliveryError`, `HttpError`, `GuardError`).
- Every module cites the Rails/Rack/gem source it mirrors. Keep those citations when moving code.

## How agents work

The whole effort runs from Phase 0 to P-5 without the human. One **coordinator** session drives it,
starting WP agents, reviewing and merging their work. The human's decisions are recorded in
`TASKS.md` under "Decisions"; everything else is settled by the rules below. The run ends before
anything is pushed.

### Starting the coordinator

Start (or, after a restart, resume) the coordinator with this prompt, in `$MAIN`:

> Coordinate the Campfire cleanup. Read `/home/emoon/once-campfire-rust/plans/cleanup.md` (all of
> it) and `/home/emoon/once-campfire-rust/TASKS.md`, then run "The coordinator loop" until P-5 is
> done. All state is in `TASKS.md` and git: pick up wherever they say the run is.

### Starting a WP agent

The coordinator starts each WP agent with this prompt, filling in the WP:

> Work on the Campfire cleanup. Read `/home/emoon/once-campfire-rust/plans/cleanup.md` (all of
> "How agents work", then your WP) and `/home/emoon/once-campfire-rust/TASKS.md`. Take WP `<id>`.
> Follow the setup, "When you'd otherwise stop" and the definition of done exactly, and finish when
> the WP is `[r]`, `[-]` or `[!]`. Report the branch, the gate results and anything left over.

To continue a WP whose agent ran out of context or was stopped, start a fresh agent with the same
prompt plus "The branch and worktree already exist; read `git log` and your line in `TASKS.md`,
then continue."

### The coordinator loop

The coordinator doesn't write WP code. It repeats:

1. **Pick.** Re-read `TASKS.md`. A WP is runnable when it's `[ ]`, its lane has no other WP in
   flight, and what it waits for is finished (`[x]`, or `[-]` for a WP that was optional to it):
   - Phase 0 runs first, as one agent in `$MAIN`.
   - Phase 1 waits for Phase 0.
   - A Phase 2 WP waits for the F-WPs in the lane table and the WPs its own text names ("after
     DB-2").
   - Phase 3 waits for all of Phase 2, and runs one WP at a time.
   - Phase 4 waits for Phase 3. P-2 is one WP per crate; P-5 is last.
2. **Start** runnable WPs as background agents, at most four at a time (this machine's limit).
3. **Review** each WP that comes back `[r]`. Start a fresh reviewer agent (not the author) with the
   WP text, the diff against `refactor/cleanup`, and this plan's rules. It checks that the diff does
   what the WP says and nothing else, that behavior is unchanged (reading the Rails source where
   code mirrors it), that "Keep these patterns" and the citations survived, that new code inside a
   fn carrying an S-5 `#[expect]` doesn't add hits of that lint (the attribute hides them), and
   that the recorded gate numbers pass. It answers approve, or a list of findings. Send findings
   back to the author (or a fresh agent on the branch); after two rounds, the coordinator decides
   each open finding itself and records its reasoning in the merge commit.
4. **Verify** before merging: rebase the branch onto `refactor/cleanup` in its worktree, re-run
   tests (seed built) and clippy there (`taskset -c 0-7,16-23 cargo test …` / `cargo clippy …`,
   as in the definition of done), and check the recorded perf and parity results against the
   tolerance. If the rebase changed code the WP touched, re-run the perf gate too. If the WP
   changed a crate's dependencies or features, also `cargo check -p <crate>` for each workspace
   crate on its own: the workspace build unifies features (kit, cable and campfire turn on
   `rails_compat`'s `crypto`), so a crate that misses a feature it needs still passes there.
5. **Merge** in `$MAIN`: `git merge --no-ff refactor/cleanup-<id>`. Set the WP to `[x]`, then
   commit `TASKS.md` on `refactor/cleanup` ("Tasks: <id> merged"). This is the only place
   `TASKS.md` is committed. Remove the worktree (`git worktree remove --force`: git refuses
   without it because the worktree has the `reference` submodule, so first check that
   `git -C <worktree> status --short` lists nothing the branch still needs) and the WP's two images
   (`docker rmi campfire-candidate-<id> campfire-rust:<id>`). Keep the branch.
6. **Triage** "Found while working". An item that's a pure refactor or perf fix in scope becomes a
   new WP in its lane (next free number, for example `DB-10`) with a line in this plan. An item that
   would change behavior, or that needs a human call, moves to "For the human" in `TASKS.md` and
   into the final report. It never blocks the run.
7. **Check the disk.** If less than 60 GB is free after step 5's cleanup, stop and ask; freeing
   anything else isn't approved.

The coordinator stops only for the disk check, or for Docker or the toolchain failing in a way no
agent can fix. A `[!]` WP never stalls the run: after its retry, the WPs that depend on it become
`[-]`. At the end it runs P-5 and stops before pushing (see P-5).

### Branches and checkouts

- `main` is untouched until the whole effort (or a phase of it) is merged by PR.
- `refactor/cleanup` is the branch that collects finished work. It's checked out in the main
  checkout, `/home/emoon/once-campfire-rust` (called `$MAIN` below). Only the coordinator merges
  into it.
- Every WP gets its own branch, `refactor/cleanup-<id>` (for example `refactor/cleanup-db-2`),
  branched from the current tip of `refactor/cleanup`, in its own worktree:

```sh
MAIN=/home/emoon/once-campfire-rust
ID=db-2                                   # the WP id, lower case
git -C "$MAIN" worktree add "$MAIN/../campfire-wt/$ID" -b "refactor/cleanup-$ID" refactor/cleanup
cd "$MAIN/../campfire-wt/$ID"
git submodule update --init --reference "$MAIN/reference" reference   # worktrees don't get submodules; crates/assets needs it
mkdir parity/.seed && cp -r "$MAIN"/parity/.seed/* parity/.seed/   # gitignored; without it the integration tests skip silently
```

The seed is copied, not symlinked: the parity harness runs its browsers in a container that mounts
only the worktree, so a link out of it breaks every cell that reads the seed's labels (S-2 dry
run). The glob leaves out `$MAIN`'s hidden `.instances`/`.candidates` state. Build the seed in
`$MAIN` (`flock /tmp/campfire-bench.lock parity/bin/seed build`) if it's missing there.

- Before marking a WP ready, rebase onto the current `refactor/cleanup` and re-run the gates.
- Don't push, and don't merge anything yourself.

### The checklist is shared, not per branch

`TASKS.md` exists once: in `$MAIN`, on `refactor/cleanup`. Always edit **`$MAIN/TASKS.md`**, never
the copy inside your worktree. WP branches must not change `TASKS.md`. Re-read the file right
before each edit, because other agents edit it too, and keep edits to your own lines.

- Claim: `[ ]` → `[~]`, adding your branch.
- Ready for review: `[r]`, adding the branch, the line delta and the allocations/request delta.
- Done: `[x]`, set by the coordinator when it merges.
- Dropped: `[-]`, with the reason and where the numbers are.
- Blocked: `[!]`, saying on what.

### When you'd otherwise stop

Don't ask the human; nobody is watching. Decide by these rules, and write the decision into your
commit message or your `TASKS.md` line.

- **A Rails behavior question** (for example DB-9's clock reads, WEB-5's flash sweep): read the
  Ruby, and the gem source inside the reference image, and match it.
- **A gate fails:** fix it inside the WP. After three different attempts, mark `[!]` with what you
  tried and the evidence, and finish. The coordinator retries a `[!]` WP once, with a fresh agent,
  after the rest of its lane; if it fails again it stays `[!]` in the final report, and the WPs that
  depend on it are skipped as `[-]`.
- **A perf WP gains nothing** (DB-6, WEB-9 and every WP marked **perf** or "measure first"): if no
  target improves by more than the tolerance, keep the measurements under `bench/results/`, reset
  the branch, and mark `[-]` "no measurable gain". If a perf WP also cleans up code, keep the
  cleanup only when it passes the gates on its own.
- **A cleanup costs performance:** find the cost and remove it. If that isn't possible, drop that
  part of the WP (performance comes first) and say so.
- **Something the WP asks for would change behavior** in a way the WP doesn't mention: don't do it.
  Keep the behavior, and add it under "Found while working".
- **A parity cell fails** that also fails in the Phase 0 baseline: not your problem; mention it.
- **A rebase conflicts:** resolve it in your branch, keeping the other side's intent, and re-run
  the gates.
- **The plan is wrong** (a file moved, a claim doesn't hold, a site count differs): do what the WP
  means, and note the correction on your `TASKS.md` line. If the whole WP rests on a false
  premise, mark it `[-]` with the evidence.
- **Choices the WP leaves open** have these defaults:
  - F-1 to F-4, crypto behind a feature: only if a clean build of a crate that gains the dependency
    gets more than 10% slower.
  - LIVE-5, tokio worker: do it only if it deletes code and the web-push tests and alloc gate pass.
    Otherwise skip it and say so.
  - VIEW-7: render the template if the parity and alloc gates pass. Otherwise add the test.
  - X-5, `Ctx<C>`: skip it; list it in the final report.
  - P-3, cast lints: turn them on (`deny`) in the params, headers and range modules only.
  - P-4: do it only if both conditions in P-4 hold; otherwise `[-]`.

### Scope

- Stay inside the files your WP lists. If you must touch another lane's file, keep the edit minimal
  and note it next to your WP in `TASKS.md`, so the other lane rebases knowingly.
- One WP, one branch, one purpose. Don't mix a refactor with a behavior change. If you find
  something outside your WP, add it to `TASKS.md` under "Found while working" instead of doing it.
- Line numbers in this plan are from commit `9e2a110` and drift; find code by name.

### Shared machine resources

Benchmarks pin CPUs 8–15, and the parity harness starts containers on fixed ports. Two at once ruin
both. Wrap every `bench/*` and `parity/bin/*` run in the shared lock (`flock /tmp/campfire-bench.lock
…`).

Agents share one scratchpad directory. Keep your logs and temp files in your own subdirectory of it
(`<scratchpad>/<id>/`), never under a shared name like `parity.log`: two agents once truncated each
other's parity log that way.

`cargo test` and `cargo clippy` can run in parallel, one per worktree. Each worktree has its own
`target/`, and a first build takes a few minutes. The lock doesn't cover builds, so keep them off
the benchmark's cores: CPUs 8–15 and their SMT siblings 24–31 (CPU 8 pairs with 24; the other CCD,
with its own L3, is 0–7 and 16–23). Run cargo as `taskset -c 0-7,16-23 cargo …`.

### Commands for the gates

Perf gate: compare your branch against the tip of `refactor/cleanup`. Build both with symbols; the
base binary comes from `$MAIN`, which has `refactor/cleanup` checked out. Copy it into your
worktree first: another agent may rebuild `$MAIN` while you measure.

```sh
export CARGO_PROFILE_RELEASE_DEBUG=line-tables-only
(cd "$MAIN" && taskset -c 0-7,16-23 cargo build --release -p campfire)   # base
mkdir -p target && cp "$MAIN/target/release/campfire" target/base-campfire
taskset -c 0-7,16-23 cargo build --release -p campfire                   # this WP
OUT=bench/results/$ID-$(date +%Y%m%d)
LABEL=$(echo "$ID" | tr 'a-z-' 'A-Z_')   # bench/profile reads NATIVE_<LABEL>_BIN; db-2 → DB_2
export NATIVE_BASE_BIN=$PWD/target/base-campfire "NATIVE_${LABEL}_BIN=$PWD/target/release/campfire"
# post_message runs twice per side: it's bimodal (78.7 or 77.2 on the same binary), so compare the
# higher of the two runs on each side.
for route in room_show messages_page sidebar post_message post_message-2; do
  r=${route%-2}
  flock /tmp/campfire-bench.lock bench/profile alloc --label base --route $r --out $OUT/base-$route
  flock /tmp/campfire-bench.lock bench/profile alloc --label $ID --route $r --out $OUT/$ID-$route
done
# CPU ms/req and req/s per target, in ABBA order (base, WP, WP, base) so drift over the session
# cancels out; each run writes $OUT/cpu-<label>-<n>/cpu.json.
for run in base:1 $ID:1 $ID:2 base:2; do
  label=${run%:*} n=${run#*:}
  flock /tmp/campfire-bench.lock bench/profile cpu --label $label \
    --targets room_show,messages_page,sidebar,post_message --out $OUT/cpu-$label-$n
done
```

Compare each target's mean of the two base runs with the mean of the two WP runs. If a target
lands within half of T of the pass/fail line (worse by between T/2 and 1.5 T, or, for a perf WP's
gain, better by between T/2 and 1.5 T), run the four again and decide on the mean of all eight.
Each alloc run takes about 12 s and each cpu run about a minute; the first run in a worktree also
builds the load generator. `cpu` doesn't render flamegraph SVGs (no `inferno`); its `.top.md`
rollups and `cpu.json` are what the gate uses (`cpu.json` is written only when every target ran).
Write a short `README.md` in `$OUT` with the before/after table and T from
`bench/results/cleanup-baseline-20260928/README.md`.

For a `bench/run` suite (WPs marked **perf**), build the base image under your WP's own tags from
`$MAIN`, measure it, remove it, then build and measure yours. `SUITES` picks the path: `http`,
`cable` or `upload` (about 4 minutes a rep for `http`). Before each rep `bench/run` waits up to
`LOAD_WAIT_SECS` (default 900) for the load average to drop below 1.5. The load average is
system-wide, so other agents' builds (and the previous rep) keep it above that whether or not they
are pinned; `LOAD_WAIT_SECS=60` means wait up to a minute, then run anyway (the log records the
load). Pinning builds away from the benchmark's cores is what keeps the measurement clean.
`bench/run` labels both runs with the worktree's HEAD; the image line (id and build time) tells
them apart.

```sh
export PARITY_CANDIDATE_APP_IMAGE=campfire-rust:$ID PARITY_CANDIDATE_IMAGE=campfire-candidate-$ID
export RUST_IMAGE=campfire-rust:$ID SUITES=http LOAD_WAIT_SECS=60
flock /tmp/campfire-bench.lock sh -c "
  '$MAIN/parity/bin/candidate' build && bench/run --apps rust --reps 3 --out $OUT/run-base &&
  docker rmi campfire-candidate-$ID campfire-rust:$ID &&
  parity/bin/candidate build && bench/run --apps rust --reps 3 --out $OUT/run-$ID"
bench/report $OUT/run-base; bench/report $OUT/run-$ID
```

Parity gate: build images tagged with your WP id so agents don't overwrite each other's images.
Building takes about 4 minutes and comparing about 33, all under the lock.

```sh
export PARITY_CANDIDATE_APP_IMAGE=campfire-rust:$ID PARITY_CANDIDATE_IMAGE=campfire-candidate-$ID
flock /tmp/campfire-bench.lock sh -c 'parity/bin/candidate build && parity/bin/candidate compare'
```

It ends with a line like `874 cells: 873 pass (0 flaky), 0 fail, 1 allowed, 0 error` and the
path of the report (`parity/out/compare-<stamp>/report.html`). Compare it with the Phase 0
baseline in `bench/results/cleanup-baseline-20260928/README.md`. If you interrupt a compare, stop
what it left running with `parity/bin/candidate down --all` and `parity/bin/reference down --all`.
Each WP's images take disk space (about 165 MB each; 346 GB was free on 2026-09-28), so the
coordinator removes them when it merges the WP. Never remove any other image.

### Definition of done (every WP)

1. `taskset -c 0-7,16-23 cargo test --workspace --exclude html5ever` passes **with the seed
   built**. Say so in the report. If any test skipped, say which.
2. `taskset -c 0-7,16-23 cargo clippy --workspace --exclude html5ever --all-targets` is clean.
3. If the WP can change output (anything in `views`, `richtext`, `kit`, `cable`, `storage`
   serving, or the controllers), the parity lean gate (`parity/bin/candidate compare`) matches the
   Phase 0 baseline: no new failing cells.
4. **Perf gate.** If the WP touches code that runs per request or per broadcast, run
   `bench/profile alloc` on `room_show`, `messages_page`, `sidebar` and `post_message`.
   Allocations per request must not go up. For WPs marked **perf**, also run `bench/profile cpu` on
   the same targets and a `bench/run --apps rust` suite that covers the path (http, cable or
   upload). Record before/after under `bench/results/<wp-id>-<date>/`, with a `README.md` holding
   the before/after table (see "Commands for the gates").
   **Tolerance** (the human's decision, 2026-09-28): for each target, T is the larger of 3% and the
   run-to-run spread S-2 measured for it. S-2's numbers are in
   `bench/results/cleanup-baseline-20260928/README.md`: T = 3% for room_show, messages_page and
   sidebar and 3.2% for post_message, the `bench/run` metrics whose spread is over 3%, and the
   allocation noise (0.1 per request; post_message is bimodal). CPU per request or throughput
   worse than base by more than T fails the gate. A WP marked **perf** must also improve at least
   one target by more than T, or it's dropped (see "When you'd otherwise stop").
5. **Size.** Run `bench/loc --against refactor/cleanup` (WP S-3) and put the production-lines
   delta per crate in the commit message.
6. Match the surrounding code: small named functions, a Rails citation where behavior mirrors
   Rails, and comments only for the non-obvious. Don't delete a "why" comment or a citation. Do
   delete comments that restate the code.
7. Mark the WP `[r]` in `$MAIN/TASKS.md` with the branch, the delta (lines, allocations/request) and
   anything left over. Then finish; the coordinator reviews and merges.

## Phase 0: safety net (one agent, serial; blocks everything)

The Phase 0 agent is the exception to the worktree rule. It works directly in `$MAIN` on
`refactor/cleanup` and commits there, because every later branch starts from its result. That
includes committing its own S-* lines in `TASKS.md`, the one exception to the coordinator-only
rule. The coordinator reviews those commits (step 3 of the loop) before starting Phase 1, and has
them fixed in place rather than reverted.

- **S-1 Toolchain.** Rust 1.98.1 (the `Dockerfile` version) is installed (the human did this on
  2026-09-28; `cargo --version` and `mise exec rust@1.98.1 -- cargo --version` both report 1.98.1).
  Re-run clippy on it, and fix the one richtext warning (`nonminimal_bool`) if it still shows.
- **S-2 Seed, images, baselines.** `parity/bin/seed build`, `parity/bin/reference build`,
  `parity/bin/candidate build`. Then:
  - Confirm the integration tests now run (count them before and after).
  - Record the parity lean gate result on `main` (expect 873/874, with the manifest allowlisted).
  - Record `bench/profile alloc` and `cpu` on the four standard targets, plus one `bench/run --apps
    rust --reps 3`. Run `alloc` and `cpu` twice on the same binary and record the run-to-run
    spread, so the perf gate's tolerance rests on a measured number.
  - Save everything in `bench/results/cleanup-baseline-<date>/`. Every later WP compares against
    this.
  - Dry-run the gate commands from "Commands for the gates" in a throwaway worktree, and fix this
    plan wherever they're wrong. `bench/profile cpu` renders flamegraphs with `inferno`, which isn't
    installed, and installing it isn't approved. Without it `cpuprof.py` still writes the folded
    stacks and the rollup, which is all the gate uses; skip the SVGs.
- **S-3 `bench/loc`.** Add a small script that prints production, test and comment lines per crate,
  excluding `html5ever` and `storage/src/tables.rs` (same rules as the table above). Record the
  baseline.
- **S-4 Formatting (approved by the human, 2026-09-28).** Add `rustfmt.toml` with `max_width = 140`. Run
  `cargo fmt --all` as one mechanical commit, and add that commit to `.git-blame-ignore-revs`. It
  must land before any lane branches, or every branch conflicts. After this, agents run
  `cargo fmt` freely.
- **S-5 Lint floor.** Add a workspace `[lints.clippy]` table with the pedantic lints that match the
  priorities: `format_push_string`, `redundant_clone`, `needless_pass_by_value`,
  `significant_drop_tightening`, `large_enum_variant`, `trivially_copy_pass_by_ref`,
  `needless_collect`, `inefficient_to_string`. Start them at `warn` and allow the existing hits. Each
  lane burns down its own and flips the lint to `deny` at the end (P-3). Done with
  `#[expect(clippy::…, reason = "existing hit under the S-5 lint floor")]` on each hit's enclosing
  fn. An `expect` there also absorbs new hits of the same lint in that fn until the old one is
  fixed, so reviewers should check new code inside such fns for those lints by eye.

## Phase 1: shared foundations (small, early; lanes depend on them)

These give the lanes one shared helper to switch to, instead of each lane adding its own. F-1 to F-4
each add a new module to `rails_compat`, a leaf crate that kit, cable and campfire already depend
on. They can run in parallel; only the `lib.rs` lines conflict. The crates that don't depend on it
yet (views, richtext, storage, assets) add the dependency. If compile time suffers because of the
crypto crates, put the crypto parts behind a feature.

- **F-1 One ERB escaper.** `rails_compat::erb::{escape, escape_into}`: the byte-loop version from
  `views/src/helpers/html.rs` (`ErbEscaper`). Replace the copies in `views/helpers/html.rs`
  (`escape`, `push_escaped`), `richtext/src/ruby.rs`, `cable/src/turbo.rs`, `assets/src/tags.rs` and
  `kit/src/front/tls.rs`. Output is byte-identical; the vectors and view tests cover it.
- **F-2 One ActiveSupport JSON encoder.** `rails_compat::json` already exists and is right for Rails
  8.2 (U+2028/2029 not escaped). Add Ruby `Float#to_s` formatting (moved from `storage/src/json.rs`)
  and an `EncodedJson` newtype that only the encoder can construct. Replace `cable/src/json.rs`,
  `integrations/opengraph/metadata.rs` `json_string` and `db/src/models/webhook.rs` `json_string`.
  `Server::broadcast_json` then takes `EncodedJson` and stops re-escaping. The storage `Json` type
  moves in STORE-6.
- **F-3 One Rack byte-range parser.** `rails_compat::rack::byte_ranges` plus a multipart/byteranges
  body builder, following the Ruby-exact `storage/src/file_server.rs` version. Replace
  `assets/src/serve.rs:225-280` and the multipart assembly in `campfire/src/active_storage.rs:303`.
  This fixes `bytes=0-99999999999999999999` on assets, which currently returns a 1-byte 206 where
  Rack serves the whole file. Add a test for it.
- **F-4 Ruby integer parsing.** `rails_compat::ruby::{to_i, cast_integer}`, kept as two named
  functions because they differ on underscores and overflow. Replace `concerns.rs:498`,
  `concerns.rs:510` and `channels/room.rs:52`.
- **F-5 `Patch<T>` and `Assignments` (db).** In `campfire_db`:
  - `enum Patch<T> { Keep, Set(T), Clear }` for nullable columns.
  - `Assignments`, a `Vec<(&'static str, rusqlite::types::Value)>` builder with `change`, `set` and
    `write(conn, table, id, now)`.
  - Rewrite `User::update` and `Account::update` on it (drops the `Box<dyn ToSql>` vectors and the
    double clones), and `Room::update`'s `name`.
  - Change `UserChanges` and the `Account`/`Room` update signatures, and their 7 campfire callers.
    `string_attribute` (`presenters/accounts.rs:299`) and `room_name_param` (`rooms.rs:146`) return
    `Patch`.
  - `presenters::attachments::Assignment` keeps its extra `Invalid` case but uses the same variant
    names.

## Phase 2: lanes (parallel; one agent per lane)

Each lane owns a set of files. Within a lane, the WPs are in suggested order: cheap measurable wins
first. A lane starts once Phase 0 is done and the F-WPs whose files it owns have merged:

| Lane | Waits for |
|---|---|
| DB | F-2 (`webhook.rs`), F-5 |
| KIT | F-1 (`front/tls.rs`) |
| WEB | F-4 (`concerns.rs`), F-5 (its callers) |
| LIVE | F-1 (`cable/turbo.rs`), F-2 (`cable/json.rs`, opengraph), F-4 (`channels/room.rs`) |
| VIEW | F-1 (`helpers/html.rs`, `richtext/ruby.rs`) |
| STORE | F-1 (`assets/tags.rs`), F-2, F-3 (`assets/serve.rs`, `active_storage.rs`) |

Known cross-lane edits, besides those noted in the WPs: WEB-7 also changes `integrations/jobs.rs`
(LIVE), and VIEW-1 changes `campfire/src/controllers/presenters/page.rs` (WEB). Files in no lane
(`campfire/src/rich_text.rs`) follow the same rule as another lane's files.

### Lane DB: `crates/db`

- **DB-1 perf: N+1 in `Room::find_direct_for`** (`models/room.rs:227`). It runs about 2×(direct
  rooms) queries under the write lock on every new-DM POST. Load `(room_id, user_id)` for all direct
  rooms in one query and group in Rust, keeping candidate order so the first match is unchanged.
- **DB-2 Use the enums that already exist.** `RoomType::default_involvement` returns `Involvement`,
  not `&str`. `insert_memberships` takes `Involvement`. Bind `Involvement::Invisible`,
  `RoomType::Direct`/`Open` and `Status::Active` instead of string and integer literals
  (`room.rs:399,428,461`, `user.rs:440,696`).
- **DB-3 `sql_enum!`.** One small declarative macro that generates `name`, `from_name`, `ToSql` and
  `FromSql` from a single table for `Role`, `Status`, `Involvement` and `RoomType`. This removes the
  drift-prone duplicate discriminants in `integer_enum_sql!` (`user.rs:70-100`), about −100 lines.
  Add tests pinning the integer values that literal SQL relies on (`Status::Active == 0`,
  `Role::Bot == 2`).
- **DB-4 perf: static SQL on hot paths.** `message.rs` paging (`last_page` runs on every room show)
  and `room.rs` `find_for_user`/`SELECT_FOR_USER` build SQL with `format!` on every call. Use
  `concat!`-based prefix macros so the SQL is a `&'static str`.
- **DB-5 perf: one statement per bulk membership insert.** Replace the chunked `VALUES` builders in
  `grant_membership_to_open_rooms` (`user.rs:692`) and `insert_memberships`/`grant_to_active_users`
  (`room.rs:428,461`) with `INSERT … SELECT … ON CONFLICT DO NOTHING`. Explicit id lists use
  `json_each(?)`. Check that row order stays the same (`differential_test`, which is `#[ignore]`d
  in a plain `cargo test`; `reference-tools/db/differential.sh` runs it against the reference
  image).
- **DB-6 perf (measure first): variable-length `IN (?, …)`.** Six queries (`where_ids`,
  `mentionees_in_room`, `for_mentioned_users`, `page_updated_since`, `revoke_from`,
  `trim_recent_searches`) create one `prepare_cached` entry per list length. Try
  `IN (SELECT value FROM json_each(?))`, and keep it only if `EXPLAIN QUERY PLAN` and the bench agree.
  Make `trim_recent_searches` a single `DELETE … NOT IN (… LIMIT 10)`.
- **DB-7 Named parameter structs.**
  - `NewRoom` (3 sites).
  - `NewPushSubscription`, replacing `PushSubscription::new`'s 4 adjacent `Option<&str>` and its
    `id: 0` sentinel.
  - `ClientInfo { user_agent, ip_address }` for `Session::start`/`resume`.
  - `Pushes { payload, everything, mentions }` instead of the tuple from `pushes_for`.
  - Make argument order consistent: `(room_id, user_id)` everywhere (`Room::find_for_user`,
    `create_for`).
- **DB-8 perf: timestamp encoding without allocations.** `Timestamp::to_db` (`time.rs:62`) allocates
  twice per bind, and `parse_db` builds a padded `String` per read. Write into a stack buffer.
  `Membership::update_counter` binds a signed delta instead of keeping two SQL strings.
- **DB-9 Small cleanups.**
  - `Session::start`/`resume` read the clock twice. Reuse `now`, unless Rails does the same; check
    first.
  - Rename predicates `Room::open/closed/direct` → `is_open/…` (16 sites).
  - Delete dead code: `Blob::create`, `Message::content_type`/`sound`/`ContentType` (test-only; move
    them to tests if still needed), and `Webhook::create`'s always-`Some` `url`.
- **DB-10 `differential.sh` knows the app's extra index** (found in S-2; do it before DB-5). The
  script stops at its schema-identity diff because the Rust side has
  `index_messages_on_room_id_and_created_at`, which the app adds on boot on purpose (README, "One
  more index"). Filter that index out of the Rust side of the diff, so the last step (Rails on the
  Rust-written database) runs again. Tooling only; no production code changes.

### Lane KIT: `crates/kit`, `crates/routes`

- **KIT-1 perf: params merged once.** `Ctx::new` (`ctx.rs:83`) clones and merges body, query and
  path params. Campfire's `install_path_params` (`campfire/src/controllers.rs:351`) then does the
  whole thing again. Add `Ctx::set_path_params` and build `params` once. Touches that one campfire
  function; land it before WEB-9.
- **KIT-2 perf: per-request allocations.**
  - `deflater.rs:34`: pick the encoding before `next.run` and format the path only on a 406.
  - `front/cache.rs:197`: `Variant::new` clones the whole `HeaderMap`.
  - `compression.rs:215,232`: `eq_ignore_ascii_case` instead of `to_ascii_lowercase`.
  - `format.rs:360` `browser_like` and `request.rs:131` `is_xhr`: no allocation.
  - `request.rs:109`: compute `remote_ip` lazily and store it as `IpAddr`.
  - `request.rs:159-196`: parse the host once.
  - `ctx.rs:244`: `formats()` returns `&[Format]`.
  - `front/handler.rs:358`: `RequestLog` keeps refcounted `Uri`/`Method`/`HeaderValue`.
  - `front/conn.rs:520`: pin the timeouts with `pin-project-lite` instead of two `Box<Sleep>`.
- **KIT-3 Session dead state.** `cookie_data` is never read after `load`. Delete it and its clones
  (`session.rs:42,57,67-73,119`). The cookie format doesn't change; the vectors cover it.
- **KIT-4 `dispatch` error flow** (`adapter.rs:249-316`). Keep the three extractor results owned and
  take the first error, then delete `clone_error`. This also fixes the doubled "bad request: bad
  request:" log prefix.
- **KIT-5 Concrete types instead of `dyn` where there's one implementation.**
  - `front/handler.rs:187` `Box<dyn FnOnce(Bytes)>` → `PendingEntry { … }` with `fn store(self,
    body)`.
  - `front/acme.rs:36` `Box<dyn Error>` → `anyhow` with `context` (already a dependency).
- **KIT-6 Typed config and options.**
  - `FrontConfig` sizes: `Option<NonZeroU64>`/`u64`, parsed once, instead of `i64` with 0 meaning
    none.
  - `SendOptions.disposition` → `Disposition { Inline, Attachment }`, shared with storage (STORE-4).
  - `Redirect.status` → `StatusCode` with a default.
  - One `status_has_no_body(StatusCode)` for `ctx.rs:341,569`.
  - Replace the struct-with-4+-bools at `cookies.rs:35`, `response.rs:303` and `front/config.rs:15`
    with enums where the bools are really one choice.
- **KIT-7 Routes return `&'static str` for constant paths** (69 uses) and keep `String` for paths
  with parameters.
- **KIT-8 Readability.**
  - Split `compression.rs:77` `apply` into `buffer_head`/`finish_*`.
  - Delete the dead `params.rs:451` `Pair`.
  - `ParamMap::to_json` borrows instead of cloning.
  - Merge the three form media-type lists (`adapter.rs:434`, `body.rs:67,87`) into one `MediaType`.
  - Make `is_xhr` consistent between `request.rs:131` and `adapter.rs:524`: `contains`, as Rails
    does.

### Lane WEB: `crates/campfire/src/{controllers*,concerns*,app*,config.rs,main.rs}`

- **WEB-1 perf: static assets without a copy** (`app.rs:175`). Use `Bytes::from_static` for
  `Cow::Borrowed` instead of `into_owned()`.
- **WEB-2 perf: user agent parsed once per distinct string.**
  - Memoize `(browser_blocked, Platform)` by UA string in a bounded map. Both `allow_browser`
    (`concerns.rs:392`) and `ApplicationPlatform::new` (`view_context.rs:217`) read from it.
  - Make `user_agent::parse` work on `&str` byte offsets instead of rebuilding `Vec<char>` per
    product.
  - `to_view` stops allocating in each `try_browser`.
- **WEB-3 Hand-written matchers → `LazyLock<Regex>`** (`user_agent.rs:1043-1170`). Use ASCII classes
  (`[0-9]`, or `(?-u:…)`) to match Ruby. The vector tests at `user_agent.rs:1226` and
  `platform.rs:197` must still pass. `mac_os_x_version`'s `Option<Option<&str>>` goes away with it.
  `platform.rs:174` gets `enum BrowserRule { Unguarded, AlwaysBlocked, Minimum(&str) }`.
- **WEB-4 `AppCtx::read`/`write` helpers.** 118 hand-written five-line
  `c.app().db.read(move …).await.map_err(…)` chains. Add `c.read(|conn| …)` and `c.write(|tx| …)`.
  - **Parity trap:** `db_error` maps `RecordNotFound` to 404, while `Error::internal` gives 500. Keep
    both flavors (for example `read` and `read_internal`), and move each site keeping its current
    mapping.
  - Estimated −400 lines.
- **WEB-5 perf: fewer DB round trips per page.** Room show does about 6 `spawn_blocking` hops:
  session lookup, `User::find_by_id`, `set_room`, `render_show`, `Account::first` again, and layout
  logo plus `last_room_visited`. Merge them into about 3 reads with the same queries.
  `page::bare` (`page.rs:68`) skips `Layout::load` when the caller ignores it (`messages#destroy`).
  Before also skipping the flash sweep, check what Rails does with `layout false`.
- **WEB-6 Authorization.** Split `User::can_administer(creator_id, record_is_new)` (the bool is always
  `false` in app code) into `is_administrator()` and `can_administer(creator_id)`. Replace the three
  `ensure_can_administer` copies and 7 `halt(head(FORBIDDEN))` with one `forbid_unless(bool)`. Owns
  that one function in `db/src/models/user.rs`.
- **WEB-7 One detached-partial renderer.** 12 `render_detached` sites each clone the app, take the
  base URL, build a `Presenter` and call `Account::first`. Add
  `page::render_partial(c, |presenter, ctx| …)`. This also makes error handling consistent:
  `broadcast_create` ignores askama errors, while `broadcast_replace` maps them.
- **WEB-8 Presenters.**
  - Move the raw SQL in `presenters/accounts.rs` (`help_contact`, `touch`,
    `direct_placeholder_users`, `account_users`, `query_users`, `is_record_not_unique`, `no_rows`)
    into `campfire_db`, after DB-2.
  - Delete the duplicates of `Presenter::room_display_name`, `sidebar_direct`, `RoomKind::param_key`
    and `sql::placeholders`.
  - perf: on a fragment-cache miss, fetch `body_html` once and pass it through (it is currently
    fetched twice, with `to_plain_text` run twice).
  - perf: cache `UserView` (with its signed avatar id) per user instead of cloning `User` and
    re-signing per message.
  - perf: `room_and_name` returns only the name.
  - perf: `accounts::Edit` borrows `&[UserSummary]` so `framed_page!` stops cloning up to 500 users.
- **WEB-9 perf (measure): router.** `recognize` (`controllers.rs:327`) tries 184 regexes in order. Build
  a `RegexSet` once, take the lowest matching index whose verb fits, and capture only that route.
  First-match semantics stay the same, and the route vector tests keep them honest. Add a fast path
  to `normalize_path` when there's nothing to normalize.
- **WEB-10 Small items.**
  - One `c.redirect_to_path(&path)` for the "`url_for` then `redirect_to`" pairs.
  - Replace bool params: `broadcast_to_members(.., update: bool)` → two functions;
    `authenticated_as(.., set_cookie)` → an enum.
  - Precompute the version headers.
  - `cast_integer` slices instead of collecting.

### Lane LIVE: `crates/cable`, `crates/campfire/src/{channels*,integrations*,jobs*}`

- **LIVE-1 perf: don't hold the hub lock while waking subscribers** (`cable/src/pubsub.rs:58`). Clone
  the group senders under the lock, then send after releasing it. The `retain` cleanup is redundant
  with `release`. Measure with the cable suite at 1,000 clients.
- **LIVE-2 perf: encode per-member broadcasts once.** `channels/broadcasts.rs:101-111,164-181`
  re-render, re-encode and re-escape the same payload for every member. Add
  `Server::encode_action` and `broadcast_encoded` (taking F-2's `EncodedJson`).
- **LIVE-3 `Broadcasting` newtype.** Constructors `room_messages`, `user_rooms`, `user_reads` and
  `user_unreads` replace the `format!`/`[String; 2]` stream names (`broadcasts.rs:76-87`,
  `read_rooms.rs:9`, `unread_rooms.rs:10`, `room.rs:24`, `typing_notifications.rs:24`). Add
  `enum Typing { Start, Stop }`.
- **LIVE-4 Split `connection::run`** (`cable/src/connection.rs:67-178`, 114 lines) into `open`,
  `collect_ready` and the select loop. Replace the string match on `"command"` with a serde-tagged
  `enum Command`, keeping the malformed-input logging. The protocol and golden tests guard frame
  order.
- **LIVE-5 Web push.**
  - Remove the `Box<dyn Fn>` `Handler` (`web_push/pool.rs:21`); move the closure straight into the
    worker.
  - Consider one tokio task plus a bounded tokio mpsc instead of `std::thread`, unbounded
    `std::sync::mpsc` and two `Mutex<Option<…>>`.
  - `web_push.rs:80` stops passing an identity resolver callback: add
    `PushSubscription::permitted_endpoint_host() -> Option<&str>` and keep IPs as `IpAddr`.
- **LIVE-6 One resolved target.**
  - `opengraph/location.rs`: replace the `Option<Option<IpAddr>>` memo and the repeated
    `is_valid`/`parsed_url.clone()`/`resolved_ip()` with `enum Target { Invalid, Valid { url, ip } }`,
    resolved once.
  - `net/http.rs`: add `Endpoint::from_uri` and `uri_host`, used by webhook, opengraph and web push.
    Keep header order exactly (Net::HTTP oracle tests).
  - `Request::transport(unstarted: bool, …)` → two methods.
- **LIVE-7 Small items.**
  - `opengraph/metadata.rs` gets `enum Attr` over the ordered `Vec`.
  - One `BoxFuture` (from `futures_util`).
  - `ChannelFactory` and authenticator: `Box` instead of `Arc`.
  - `socket.rs` gets a `FrameHead` struct, and `Writer::send` stops allocating three `Vec`s per flush.
  - Delete the uncalled `turbo.rs` `broadcast_{prepend,replace,update,refresh}_to`.
  - One `unix_now`.
  - `ChannelError` composes with `campfire_db::Error`, so `room_messages.rs:87` loses its `??`.

### Lane VIEW: `crates/views`, `crates/richtext`

- **VIEW-1 perf: stop copying whole pages and fragments.**
  - `h::raw` takes `impl Into<String>` (`helpers/html.rs:36`; pages at `layouts.rs:84` and
    `page.rs:44,62`).
  - `FragmentCache::fetch` returns the stored `Arc` instead of `String::clone` (`fragment_cache.rs:127`).
  - `SidebarDirectPartial` borrows its view (`users.rs:193`).
- **VIEW-2 perf: `Attrs` and string building.**
  - `Attrs` keys become `Cow<'static, str>`, and it renders with `write!`/`push_str` into the output
    (`tag.rs:62,197-207`). Block filters stop cloning `Attrs`.
  - Fix the 44 `push_str(&format!(..))` sites across the workspace, at least those in views and
    richtext.
- **VIEW-3 perf: richtext statics.**
  - `SafeList::defaults/action_text/auto_link/content_filter` become `LazyLock` statics
    (`sanitizer.rs:56-103`).
  - Delete the second `sanitize` in `autolink.rs:245`.
  - `dom.rs`: compare attribute names without building `qualified_name()` strings.
  - `NodeId(u32)` newtype.
- **VIEW-4 One cached-partial mechanism.** `messages.rs:155-368` and `users.rs:172-223` (and
  boosts, partly) each hand-roll the same enum, fetch/key/digest functions and `LazyLock`. Replace
  them with `Cached<V> { Fragment(Fragment), View(V) }` plus a `CachedPartial` trait (`NAME`,
  `digest`, `key`, `render`). Keep the `Box` for the large view variant. Key strings must stay
  byte-identical.
- **VIEW-5 One `dom_id` and one `param_key`.** Six `dom_id` and three `param_key` implementations
  disagree on whether "no prefix" is `""` or `None`. Add `DomId<'a>: Display` (no allocation) and
  `RoomKind::param_key`, and delete the rest (including `broadcasts.rs:30,38` in LIVE's files:
  coordinate with LIVE-3).
- **VIEW-6 Borrowed view context.**
  - `ViewContext` holds `&'a str` instead of per-request `String` clones (`lib.rs:22`).
  - `cable_url` becomes a constant.
  - `asset_path` returns `&'static str` from the manifest.
  - `Platform`'s 11 bools (`lib.rs:98`) become `Os` and `Browser` enums.
- **VIEW-7 `render_mention` single source.** `richtext/attachables.rs:390` rebuilds
  `users/_mention.html`. At minimum add a test that the two agree; better, render the template.
- **VIEW-8 perf (last, careful): parse each body once.** On a cache miss a message body goes through
  `Content::load` about 4 times, and SGID verification runs for every mention each time. Add a
  `ParsedBody` that yields plain text and presentation, keeping the current error order (a
  `to_plain_text` error before the URI error). Don't merge the serialize/re-parse steps inside the
  pipeline; they shape the output.

### Lane STORE: `crates/storage`, `crates/assets`, `crates/rails_compat` (crypto), `crates/campfire/src/active_storage.rs`

- **STORE-1 One Active Storage verifier.** Delete `storage/src/verifier.rs`'s `Verifier` trait,
  `AppMessageVerifier` and the `ActiveStorageVerifier` adapter (`campfire/src/app.rs:198`). Storage
  takes `&rails_compat::MessageVerifier`. Point `storage/tests/vectors.rs` at
  `rails_compat::app_verifier(.., "ActiveStorage")`, so the golden vectors finally test the verifier
  production uses.
- **STORE-2 perf: build verifiers once.**
  - `Secrets` holds ready `MessageVerifier`/`MessageEncryptor` values for signed cookies, encrypted
    cookies, turbo, signed ids and global ids, instead of rebuilding them (lock, salt `String`, key
    clone, AES key schedule) on every call (`cookies.rs:83-92`, `turbo.rs:30`, `signed_id.rs:31`,
    `global_id.rs:105`).
  - `cookies::decrypt` decrypts once and checks the purpose twice (`:65-69`).
  - `KeyGenerator::generate_key` stops allocating the salt on a cache hit.
- **STORE-3 perf: hash while copying.** Uploads read each file 2–3 times: checksum, then copy, then
  `ensure_integrity_of`. `Storage::open` copies and then checksums. Hash inside the copy
  (`blob.rs:54-59`, `disk.rs:60-70,153`, `storage.rs:97-111`). Measure with the upload suite.
- **STORE-4 Storage newtypes and enums.**
  - Newtypes: `BlobKey`, `Checksum`.
  - `Purpose { BlobId, Variation, BlobKey, BlobToken }`.
  - `Disposition` (shared with KIT-6).
  - `MediaKind` (merges `Analyzer::for_content_type` with `is_image`/`is_video`/`is_audio`).
  - `ContentTypeSource` instead of `unfurl(.., identify: bool)`.
  - Constants for the `"ActiveStorage::…"` record types.
  - `as_str()` values stay byte-identical.
- **STORE-5 `Variation` operations.** Keep the raw Marshal values for the digest. Add
  `enum Operation { ResizeToLimit { width, height } }` built with `i32::try_from` (today `as i32`
  truncates silently). `marshal()`/`to_json()` borrow instead of cloning.
- **STORE-6 perf: storage JSON on the shared encoder.** Replace `storage/src/json.rs`'s own `Json`
  type with `serde_json::Value` (order-preserving here) plus F-2's encoder. Its premise ("serde_json
  sorts keys") is false in this workspace, and it escapes U+2028 where Rails 8.2 doesn't. This also
  removes a parse → encode → parse round trip in `verify_raw`. **First** add golden vectors for float
  metadata and a U+2028 value, generated in the reference container.
- **STORE-7 Duplication in `active_storage.rs`.**
  - `processed_variant_with` and `preview_image` become one `find_or_process`.
  - `purge`'s raw SQL moves into `campfire_storage`.
  - `serving` returns `Error::FileNotFound` like everything else, so `active_storage.rs:419` stops
    matching on io error kinds.
  - Delete `query_escape` (a copy of `rails_compat::cookies::escape`) and inline
    `storage_error_to_kit`.
- **STORE-8 Hygiene.**
  - Add `// SAFETY:` comments to the 15 unsafe blocks in `vips.rs`, plus
    `#![deny(clippy::undocumented_unsafe_blocks)]` there. Soften the error-buffer comment at
    `:218`.
  - Warm `ffmpeg_exists()` at boot, so the first render doesn't spawn a process on the runtime.
  - Split `assets/build.rs` `main` (155 lines), `analyze.rs` `video_metadata` and
    `assets/src/serve.rs:142-222`.

## Phase 3: cross-cutting migrations (serial; one at a time)

Each of these touches many files across lanes, so Phase 3 starts once every Phase 2 WP is
finished, and runs one migration at a time in the order listed. Use one agent per migration, and
let the compiler drive the call sites.

- **X-1 `Format` enum.** `enum Format { Html, Json, TurboStream, … }` with `mime()`, plus
  `enum Accepted { Any, Format(Format) }` in place of `format::ALL`. `respond_to` returns the
  enum, so controllers `match` instead of comparing guards (about 50 uses). The `REGISTERED` order
  must stay the same, because `text/*` expansion depends on it.
- **X-2 Static `Permit` lists and typed param access.**
  - `enum Permit { Key(&'static str), ScalarArray(..), AnyHash(..), Nested(&'static str,
    &'static [Permit]) }` as `const`s, so `permit_keys` stops allocating per request (13 sites).
  - Add `ParamMap::id(key) -> Result<i64>`, returning `NotFound`, for the ~15 copies of
    `param_str("id").and_then(cast_integer).ok_or(NotFound)`.
  - `Param::to_s` returns `Cow`.
- **X-3 Safe HTML as a real type.** `Html` becomes a newtype with a private field and three
  constructors: `Html::escape(&str)`, `Html::trusted(&'static str)` and `Html::from_sanitized`
  (richtext's output type).
  - `content_tag`, `link_to`, `button_to` and friends take `&Html`, not `&str`, so raw user text can
    no longer be passed as markup.
  - `Html::concat` replaces the ~38 `format!("{}{}", a.0, b.0)`.
  - `MessageContent::Text` and `EditView::editable_body_html` become `Html`.
  - Parity tests catch any slip.
- **X-4 Stringly view fields → enums.** Use `Involvement` (with `next(kind)` and `humanized()`),
  `RoomKind` and `Role` in `ProfileMembership`, `SidebarRoom`, `InvolvementRoom`, `InvolvementView` and
  `UserJson`. `humanize_involvement`/`next_involvement` stop returning `""` for unknown values.
  Needs VIEW-5.
- **X-5 `Ctx` split, incrementally.**
  - Extract `Negotiation { formats, rendered_format }`, `ResponseDraft { headers, cache_control,
    live, finish/etag/conditional_get }` and `SessionState { session, flash }`, keeping the facade
    methods.
  - Optionally `Ctx<C>` with `type Ctx = kit::Ctx<Current>` in campfire, for typed
    `c.current.user` instead of the `Extensions` typemap.
- **X-6 Typed ids (approved by the human, 2026-09-28: "where it makes sense").** Generate the id
  types with one macro (`#[repr(transparent)]`, `Copy`, `ToSql`, `FromSql`, `Display`, `FromStr`).
  The schema and cookies don't change: ids stay `i64` in SQLite and in signed payloads.

  **Where it makes sense.** Give a record a typed id when its id crosses a function or crate
  boundary next to another kind of id, or appears in a signature where it could be swapped. Start
  with `UserId`, `RoomId` and `MessageId`, which are passed together everywhere
  (`find_for_user`, `create_for`, `room_message(room_id, id)`). Then add others only where the same
  test holds, for example `MembershipId`, `BoostId`, `BlobId`, `SessionId` and
  `PushSubscriptionId`. Keep a bare `i64` for:
  - ids used only inside one module's SQL;
  - counts, positions and other numbers that aren't ids;
  - the wire formats themselves: raw params and GlobalID/signed-id payloads. Convert at those
    edges, in one place each.

  If a type would only add `.0` noise without ever preventing a mix-up, leave it out, and say
  which ones you skipped and why in the WP notes.

  **Stages,** each its own WP branch (`x-6a` … `x-6d`), so each diff stays reviewable:
  1. db struct fields and finders.
  2. Campfire controllers, presenters and channels (let the compiler drive).
  3. Views, and routes helpers, which then take typed ids instead of `impl Display`.
  4. Param parsing and the GlobalID/signed-id edges.

  DB-7 fixes the concrete argument-order hazards first, cheaply. X-6 comes after it.

  **Perf:** the id types are `#[repr(transparent)]` over `i64` and `Copy`, so they cost nothing at
  runtime. The allocation gate must show no change.
- **X-7 One blob model.** `campfire_db::Blob` and `campfire_storage::Blob` model the same table; keep
  one. Similarly, the attachment record triple (`record_type`, `record_id`, `name`) becomes one
  `Record` type, shared by db and campfire.

## Phase 4: size and polish

- **P-1 Test code.** 15k lines of tests against 35k of code. Audit the test support modules
  (`*/tests/support`, `testing.rs`, `fixtures.rs`, `channels/tests/support.rs`) for helpers written
  more than once, and move the shared ones to a `dev-dependency` test-support module. Don't reduce
  coverage.
- **P-2 Comment pass.** Remove comments that restate the code, and doc comments on trivial accessors.
  Keep every Rails citation and every "why". One WP per crate (`P-2-db`, `P-2-kit`, …), after Phase 3,
  not as one big diff.
- **P-3 Lints to `deny`.** Flip each lint from S-5 to `deny` once its count reaches zero. Consider
  `cast_possible_truncation`/`cast_sign_loss` in the places that handle untrusted numbers (params,
  headers, ranges).
- **P-4 (Optional) MIME tables as data.** Consider generating `storage/src/tables.rs` (3.2k lines) at
  build time from a compact data file, only if the build stays hermetic and lookups stay binary
  searches over static slices.
- **P-5 Final report.** Size by crate against the S-3 baseline, allocations/request and throughput
  against S-2, and the parity gate result. Update `README.md` (performance table, Known differences
  if anything changed on purpose).
  - Write the report to `bench/results/cleanup-final-<date>/README.md`. It lists every `[-]` and
    `[!]` WP with its reason, and everything under "For the human" in `TASKS.md`.
  - Write the PR description (`refactor/cleanup` → `main`) to `plans/cleanup-pr.md`.
  - Commit everything on `refactor/cleanup`, run the full gates once more on the tip, and stop.
    **Don't push or open the PR**; that's the human's (decision of 2026-09-28).

## Parallelism at a glance

```
Phase 0  S-1 → S-2 → S-3 → S-4 → S-5                               (one agent)
Phase 1  F-1  F-2  F-3  F-4  F-5                                   (up to 5 agents; F-5 before DB/WEB)
Phase 2  DB   KIT   WEB   LIVE   VIEW   STORE                      (6 lanes, ~4 running at once; benches take turns)
           └─ WEB-8 SQL move after DB-2;  WEB-9 after KIT-1;  VIEW-5 with LIVE-3;  STORE-6 after F-2
Phase 3  X-1 → X-2 → X-3 → X-4 → X-5 → X-6a…d → X-7                (one at a time, after all of Phase 2)
Phase 4  P-1, P-2 (per crate), P-3, P-4 after Phase 3; P-5 last, then stop
```

Four agents at a time is the practical limit on this machine (32 threads). More than that and the
builds starve the benchmarks, even with the lock.
