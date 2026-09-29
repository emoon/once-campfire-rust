# VIEW-2: `Attrs` and string building (2026-09-30)

`Attrs` keys are `Cow<'static, str>`; attributes render with `push_str`/`escape_into` into one
sized tag buffer; block helpers borrow their options (`AttrsView`) instead of cloning; the 44
`push_str(&format!)` sites use `write!`. Output byte-identical (292 rendered pages compared by the
WP agent; parity 873/874 on the merged branch). ABBA (`bench/profile perf --freq 0`, each run in
`bench/quiet`) against `refactor/cleanup` at 8559e9c, c=16, mean of two runs a side:

| target | CPU ms/req base | view-2 | change | req/s base | view-2 | change | T |
|---|---|---|---|---|---|---|---|
| room_show | 0.1351 | 0.1297 | -4.0% | 28,603 | 29,659 | +3.7% | 3.0% |
| messages_page | 0.1085 | 0.1083 | -0.2% | 35,286 | 35,414 | +0.4% | 3.0% |
| sidebar | 0.1260 | 0.1107 | -12.1% | 30,618 | 34,543 | +12.8% | 3.0% |
| post_message | 0.3490 | 0.3458 | -0.9% | 5,504 | 5,415 | -1.6% | 3.2% |

sidebar gains well past 1.5 T and room_show past T; no second pass was needed.
