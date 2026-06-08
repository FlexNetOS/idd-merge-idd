#!/usr/bin/env bash
# ralph-idd.sh — external Ralph runner for the idd-merge-loop.
# Self-restarts the loop with a FRESH context each iteration (each `claude -p`
# process is a clean session = the /new effect) until a terminal sentinel.
#
# Safe by default: destructive applies are refused unless IDD_APPLY=1.
# Kill switch:  touch _workspace/STOP   (checked every iteration)
#
#   bash .claude/skills/idd-merge-loop/scripts/ralph-idd.sh                 # SAFE: plan/dry-run, commit non-destructive progress
#   IDD_APPLY=1 bash .claude/skills/idd-merge-loop/scripts/ralph-idd.sh     # UNATTENDED APPLY: opt in deliberately
#   IDD_BUDGET=2 IDD_MAX_ITERS=20 bash .../ralph-idd.sh                     # tune budget / backstop
set -euo pipefail

WORKTREE="${IDD_WORKTREE:-$(pwd)}"
BUDGET="${IDD_BUDGET:-3}"
MAX_ITERS="${IDD_MAX_ITERS:-50}"
SLEEP_BETWEEN="${IDD_SLEEP:-5}"
MODEL="${IDD_MODEL:-opus}"
WS="$WORKTREE/_workspace"; mkdir -p "$WS"

log(){ printf '[ralph-idd %s] %s\n' "$(date -u +%H:%M:%S)" "$*" >&2; }
command -v claude >/dev/null || { log "FATAL: claude not on PATH"; exit 1; }

APPLY_ARGS=()
if [ "${IDD_APPLY:-0}" = "1" ]; then
  APPLY_ARGS=(--dangerously-skip-permissions)
  log "APPLY MODE — will modify the live system unattended (IDD_APPLY=1)."
else
  log "SAFE mode (default): destructive applies refused. Set IDD_APPLY=1 to act."
fi

read -r -d '' PROMPT <<EOF || true
/idd-merge-loop resume (external Ralph runner, fresh context). Worktree: $WORKTREE.
1. If _workspace/HANDOFF.md exists, follow the session-relay RESUME entry from it (the authoritative
   signal): RECONCILE the prior run's PR before choosing a base (merged -> branch off advanced origin/develop;
   open -> branch off the PR's branch, never off stale develop; dev work NEVER branches off main), run the
   verify-on-resume baseline FIRST, reset cycles_this_session=0, then continue at the backlog's current item.
   Else DISCOVER (rusty-idd scan/plan + slice-sequence.md) and build _workspace/backlog.md.
2. Run up to $BUDGET cycles on ONE run branch (off origin/develop): ONE vertical slice each, driving
   vertical-slice-planning -> rust-native-implementation -> merge-verification -> pr-evidence-bundle.
   Dry-run -> apply only for destructive steps. VERIFY across the boundary in a FRESH shell (rusty-idd
   validate [fail-closed], drift-check.sh, rtk cargo fmt/clippy/test --workspace --locked). Commit AND push
   per cycle; on the first cycle open a PR --base develop and enable auto-merge (gh pr merge --auto --squash)
   -- develop is branch-protected (required check 'rust') so this is fail-closed. NEVER push/admin-merge main
   directly; crates/core stays zero-dep.
3. END OF RUN — FIRST ensure the run's work is on a PR to develop with auto-merge enabled and the PR# recorded
   (MANDATORY: never leave unmerged work on a local branch). On backlog-clear, also open a develop->main
   promotion PR with auto-merge (gated by 'rust'+'promote-verify'). THEN write EXACTLY ONE sentinel under
   _workspace/ and stop (do NOT ScheduleWakeup): DONE (evidence + PR#s) | NEEDS-HUMAN (reason; incl. a PR
   whose required check went RED) | else HANDOFF.md (spawn continuity-steward via session-relay; records PR#).
EOF

cd "$WORKTREE"; i=0
while :; do
  i=$((i+1)); [ "$i" -gt "$MAX_ITERS" ] && { log "MAX_ITERS ($MAX_ITERS) hit — halting."; exit 3; }
  [ -f "$WS/STOP" ]        && { log "STOP — halting."; exit 2; }
  [ -f "$WS/DONE" ]        && { log "DONE."; exit 0; }
  [ -f "$WS/NEEDS-HUMAN" ] && { log "NEEDS-HUMAN: $(cat "$WS/NEEDS-HUMAN")"; exit 2; }
  log "iter $i/$MAX_ITERS — spawning fresh agent (budget=$BUDGET, model=$MODEL)"
  claude -p "$PROMPT" --model "$MODEL" --add-dir "$WORKTREE" "${APPLY_ARGS[@]}" \
    >>"$WS/ralph-run-$i.log" 2>&1 || log "iter $i exited nonzero (continuing from durable state)"
  [ -f "$WS/DONE" ]        && { log "DONE."; exit 0; }
  [ -f "$WS/NEEDS-HUMAN" ] && { log "NEEDS-HUMAN: $(cat "$WS/NEEDS-HUMAN")"; exit 2; }
  [ -f "$WS/STOP" ]        && { log "STOP — halting."; exit 2; }
  sleep "$SLEEP_BETWEEN"
done
