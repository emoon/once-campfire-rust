#!/usr/bin/env bash
# S-6 proof, run under the bench lock: images; (a) alloc + 2 cpu; pinned compare under the parity
# lock; (b) alloc + cpu runs while it runs; then (a) again after it ends.
set -u
S=/tmp/claude-1000/-home-emoon-once-campfire-rust/323fea70-dac5-4bd4-9de5-6fbfbd69ed69/scratchpad/s-6
W=/home/emoon/campfire-wt/s-6
cd "$W"
OUT=bench/results/s-6-20260929
mkdir -p "$OUT"
export NATIVE_A_BIN=$W/target/s6-campfire NATIVE_B_BIN=$W/target/s6-campfire
mark() { echo "$(date +%s) $(date -Is) $*" | tee -a "$S/phases.log"; }
alloc_all() { local label=$1 set=$2 route; for route in room_show messages_page sidebar post_message post_message-2; do
  bench/profile alloc --label "$label" --route "${route%-2}" --out "$OUT/$set-$route" >>"$S/profile.log" 2>&1 || mark "alloc $set $route FAILED"; done; }
cpu() { bench/profile cpu --label "$1" --targets room_show,messages_page,sidebar,post_message --out "$OUT/cpu-$2" >>"$S/profile.log" 2>&1 || mark "cpu $2 FAILED"; }

python3 "$S/sampler.py" "$S/cpu-sample.csv" & SAMPLER=$!
trap 'kill $SAMPLER 2>/dev/null' EXIT

mark "lock acquired; building images"
PARITY_CANDIDATE_APP_IMAGE=campfire-rust:s-6 PARITY_CANDIDATE_IMAGE=campfire-candidate-s-6 \
  parity/bin/candidate build >"$S/image-build.log" 2>&1 || { mark "image build FAILED"; exit 1; }

mark "a1 start"
alloc_all a a1
cpu a a-1; cpu a a-2
mark "a1 end"

mark "compare start"
( export PARITY_CANDIDATE_APP_IMAGE=campfire-rust:s-6 PARITY_CANDIDATE_IMAGE=campfire-candidate-s-6 PARITY_WORKERS=8
  flock /tmp/campfire-parity.lock taskset -c 0-7,16-23 parity/bin/candidate compare --ports 4121,4122 ) >"$S/compare.log" 2>&1 &
CPID=$!
# Wait until the capture is scheduling cells, then a minute more so every worker is busy.
for _ in $(seq 1 60); do grep -q " cells (" "$S/compare.log" 2>/dev/null && break; kill -0 $CPID 2>/dev/null || break; sleep 5; done
sleep 60
mark "b start (compare alive: $(kill -0 $CPID 2>/dev/null && echo yes || echo no))"
alloc_all b b
for n in 1 2 3 4; do cpu b b-$n; mark "cpu b-$n done (compare alive: $(kill -0 $CPID 2>/dev/null && echo yes || echo no))"; done
# Extra (b) runs for the rest of the compare, to see every phase of it.
n=5
while kill -0 $CPID 2>/dev/null; do cpu b b-$n; mark "cpu b-$n done (compare alive: $(kill -0 $CPID 2>/dev/null && echo yes || echo no))"; n=$((n + 1)); done
wait $CPID; status=$?
mark "compare end (exit $status)"

mark "a2 start"
cpu a a-3; cpu a a-4
alloc_all a a2
mark "a2 end"
