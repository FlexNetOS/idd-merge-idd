# Loop state — idd-merge-loop
session_started: 2026-06-07T02:00:00Z
loop: idd-merge-loop
epic: Complete Delivery + Upgrade (upgrade-only / no-downgrade)
branch: lifecycle-parity-D5 (PR #37), next: E1
open_pr: PR #36 merged, PR #37 open/auto-merging.
cycle_budget: 3
cycles_this_session: 3
cycles_total: 21
last_item: D5 [x] Confirm full lifecycle generation parity.
status: RUNNING — D5 complete. Proceeding to E1. 
last_update: 2026-06-07T03:15:00Z

## ⚠️ Race lesson (auto-merge + fast CI) — POLICY UPDATE
A1 and A2 were stacked as two commits on ONE branch (PR #26) with auto-merge enabled
after cycle 1. CI on the A1 commit went green and GitHub auto-squash-merged the PR at
the A1 commit BEFORE the A2 commit was pushed — stranding A2 off-develop. Recovery:
cherry-picked A2 onto a fresh branch off the now-advanced develop and shipped its own PR.
RULE GOING FORWARD: with fast CI + auto-merge, do NOT stack a later cycle's commit onto an
already-auto-merging PR. Ship ONE PR PER CYCLE off the latest develop; if the prior cycle's
PR hasn't merged yet, branch the next cycle off that PR's branch (stack) and base the new PR
on it — never add commits to a PR that already has auto-merge armed and a green-able head.

## Field notes
- Cycle 1: Reconciled PR #35 (merged) + D2 (Spec Validate full surface).
- Cycle 2: D3 (Spec Sync capability) implemented and verified.
- Cycle 3: D4 (Differential oracle harness) re-established. PR #36 merged.
- Cycle 4 (logical): D5 (Lifecycle generation parity) implemented. PR #37 open with auto-merge.
- Next: E1 · Fix stale READMEs.
