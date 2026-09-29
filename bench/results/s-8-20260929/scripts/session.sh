#!/bin/bash
# S-8 recording session, run under the bench lock: flock /tmp/campfire-bench.lock session.sh OUT
# A step counts ($OUT/<step>.ok) only if the screensaver was off right before and right after it;
# steps already .ok are skipped, and the session gives the lock back (exit 3) as soon as it's on.
set -u
W=/home/emoon/campfire-wt/s-8; cd $W
OUT=$1; mkdir -p $OUT
quiet() { ! pgrep -f omarchy-screensaver >/dev/null; }
export NATIVE_BASE_BIN=$W/target/s-8/campfire NATIVE_FP_BIN=$W/target/s-8/campfire-fp NATIVE_FPBFD_BIN=$W/target/s-8/campfire-fpbfd NATIVE_EXP_BIN=$W/target/s-8/campfire-exp
T=room_show,messages_page,sidebar,post_message
step() {
  name=$1; shift
  [ -e $OUT/$name.ok ] && return 0
  if ! quiet; then echo "$(date +%T) screensaver RUNNING before $name"; exit 3; fi
  echo "$(date +%T) step $name: $*"
  "$@" > $OUT/$name.log 2>&1; rc=$?
  if ! quiet; then echo "$(date +%T) step $name exit $rc, screensaver RUNNING after: discarded"; exit 3; fi
  echo "$(date +%T) step $name exit $rc"
  [ $rc = 0 ] && touch $OUT/$name.ok
}
step time-base-1 bench/profile perf --label base --freq 0 --targets $T,cable1000 --out $OUT/time-base-1
step time-fp-1 bench/profile perf --label fp --freq 0 --targets $T,cable1000 --out $OUT/time-fp-1
step perf-fp bench/profile perf --label fp --call-graph fp --targets $T,cable1000 --no-report --out $OUT/perf-fp
step gperf bench/profile cpu --label base --targets $T,cable1000 --out $OUT/gperf
step perf-dwarf bench/profile perf --label fpbfd --call-graph dwarf --targets $T --no-report --out $OUT/perf-dwarf
step time-fp-2 bench/profile perf --label fp --freq 0 --targets $T,cable1000 --out $OUT/time-fp-2
step time-base-2 bench/profile perf --label base --freq 0 --targets $T,cable1000 --out $OUT/time-base-2
step exp-base-1 bench/profile perf --label exp --freq 0 --targets $T --out $OUT/exp-base-1
step exp-inline-1 bench/profile perf --label exp --freq 0 --app-env S8_INLINE_READS=1 --targets $T --out $OUT/exp-inline-1
step exp-mmap0-1 bench/profile perf --label exp --freq 0 --app-env S8_MMAP_SIZE=0 --targets $T --out $OUT/exp-mmap0-1
step exp-mmap0-2 bench/profile perf --label exp --freq 0 --app-env S8_MMAP_SIZE=0 --targets $T --out $OUT/exp-mmap0-2
step exp-inline-2 bench/profile perf --label exp --freq 0 --app-env S8_INLINE_READS=1 --targets $T --out $OUT/exp-inline-2
step exp-base-2 bench/profile perf --label exp --freq 0 --targets $T --out $OUT/exp-base-2
echo "$(date +%T) DONE"
