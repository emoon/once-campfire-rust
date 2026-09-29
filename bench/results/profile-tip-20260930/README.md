# Profile of the tip after VIEW-2 (2026-09-30)

`bench/profile perf --call-graph fp` on a frame-pointer build of efea14c
(`RUSTFLAGS=-Cforce-frame-pointers=yes`, `CFLAGS=-fno-omit-frame-pointer -mno-omit-leaf-frame-pointer`),
user-space samples (`cpu-clock:u`), c=16, one run in `bench/quiet`. `*.top.md` are the rollups;
the folded stacks and raw data aren't kept.

What it pointed at (share of each target's user CPU):

| where | room_show | messages_page | sidebar | post_message | follow-up |
|---|---|---|---|---|---|
| `Row::get(&str)` name lookups (`columnName`, `strlen`, case folding) | ~3% | ~3% | ~8% (6.5% in `room_from_prefixed_row`) | | DB-13 re-timed |
| BLAKE3 over page text in `PageParts::new` | 7.8% | | | | KIT-9c |
| fragment search (`memchr`) in `PageParts::new` | ~3% | | | | KIT-9b |
| copying fragments into the page (`render_into` memcpy) | 5.0% | 6.1% | | | page assembly from parts |
| SQLite page-cache mutexes | 3.1% | 4.0% | 3.8% | 4.2% | (config; not pursued) |
| gzip of a new body (`gzip_member`) | | | | 17% | |
| JSON escaping of broadcasts | | | | 5.1% | |
