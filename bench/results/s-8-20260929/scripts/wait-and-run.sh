#!/bin/bash
# Waits for a quiet desktop, then runs session.sh under the bench lock; retries if the screensaver was on.
SC=/tmp/claude-1000/-home-emoon-once-campfire-rust/323fea70-dac5-4bd4-9de5-6fbfbd69ed69/scratchpad/s-8
OUT=$1
while true; do
  if pgrep -f omarchy-screensaver >/dev/null; then echo "$(date +%T) waiting: screensaver"; sleep 120; continue; fi
  flock /tmp/campfire-bench.lock $SC/session.sh $OUT
  rc=$?
  if [ $rc = 3 ]; then echo "$(date +%T) screensaver at start, retrying"; sleep 120; continue; fi
  echo "$(date +%T) session exit $rc"; break
done
