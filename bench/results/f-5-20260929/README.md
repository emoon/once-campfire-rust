# F-5 perf gate: `Patch<T>` and `Assignments`

Base: `refactor/cleanup` at 3c7a173 (the rebase to 0b5288f changed only `plans/`). WP: the F-5
commit. Both built with `CARGO_PROFILE_RELEASE_DEBUG=line-tables-only` and measured on 2026-09-29
with the "Commands for the gates" recipe in `plans/cleanup.md`. Tolerances come from
`bench/results/cleanup-baseline-20260928/README.md`: T = 3% (room_show, messages_page, sidebar),
3.2% (post_message).

F-5 changes the model update paths (`User`/`Account`/`Room::update`) and renames
`Assignment`'s variants. None of the four targets runs an update, so no change is expected.

## Allocations per request (`bench/profile alloc`)

| target | base | F-5 |
|---|---|---|
| room_show | 18.0 | 18.0 |
| messages_page | 19.0 | 19.0 |
| sidebar | 53.1 | 53.1 |
| post_message (run 1, run 2; higher) | 79.1, 78.5; **79.1** | 77.1, 78.4; **78.4** |

post_message is bimodal (see the baseline). F-5 doesn't touch its path, and its higher run is
lower, not higher. Pass.

## CPU and throughput (`bench/profile cpu`, ABBA: base, F-5, F-5, base)

| target | CPU ms/req base (1, 2) | F-5 (1, 2) | Δ mean | req/s base mean | F-5 mean | Δ |
|---|---|---|---|---|---|---|
| room_show | 0.1716, 0.1716 | 0.1695, 0.1697 | −1.2% | 21,486 | 21,769 | +1.3% |
| messages_page | 0.1436, 0.1445 | 0.1434, 0.1438 | −0.3% | 25,557 | 25,538 | −0.1% |
| sidebar | 0.2785, 0.2751 | 0.2729, 0.2749 | −1.0% | 13,406 | 13,568 | +1.2% |
| post_message | 0.4628, 0.4680 | 0.4672, 0.4654 | +0.2% | 4,680 | 4,723 | +0.9% |

Every difference is below T/2, so no re-run was due. Pass.

## Parity lean gate

`parity/bin/candidate compare`, with images built from this branch (the build stage re-run so
the shared cargo cache couldn't supply another tree's crates; see a0df4b6). Before the review
fix: **874 cells: 873 pass (0 flaky), 0 fail, 1 allowed, 0 error** in 2,028 s. On the final
code (`compare-2026-09-29T06-20-05-599Z`): **873 pass, 0 fail, 1 allowed, 0 error** in 2,073 s.
Both match the baseline.

## db differential

`reference-tools/db/differential.sh` on the final code, after DB-10:
`scenario_matches_ruby`, `fixtures_match_ruby_row_for_row` and `export_database_for_rails` pass,
the schemas agree, and the reference reads, edits and deletes through the Rust-written database
("rollback ok").

## Review round

The review found that moving `Room::update` onto `Assignments` made it allocate more per call.
It keeps its literal UPDATE now (via `Patch::apply`), and `Assignments::write` sizes its SQL text
up front. Per-call allocations, counted inside the call with a thread-local counting allocator
once the statement cache is warm (throwaway harness, not committed):

| call | base 3c7a173 | first round | final |
|---|---|---|---|
| `Room::update` rename | 6 | 9 | 4 |
| `Room::update` type change | 6 | 12 | 6 |
| `User::update` name + bio | 20 | 11 | 9 |
| `Account::update` custom_styles | 18 | 9 | 7 |

The alloc gate re-run on the final code (base: `refactor/cleanup` at 6edb805), in `review/`:

| target | base | F-5 |
|---|---|---|
| room_show | 18.1 | 18.1 |
| messages_page | 19.0 | 19.0 |
| sidebar | 53.1 | 53.1 |
| post_message (run 1, run 2; higher) | 77.9, 78.2; **78.2** | 78.2, 78.0; **78.2** |

Unchanged. None of these targets runs a model update, so the first round's CPU numbers still
stand.
