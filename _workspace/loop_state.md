# Loop state — idd-merge-loop
session_started: 2026-06-07T02:00:00Z
loop: idd-merge-loop
epic: Complete Delivery + Upgrade (upgrade-only / no-downgrade)
branch: develop (synced after PR #35 merged)
open_pr: PR #35 merged.
cycle_budget: 3
cycles_this_session: 1
cycles_total: 18
last_item: D2 [x] spec validate full surface.
status: RUNNING — D2 complete. Proceeding to D3. 
last_update: 2026-06-07T02:45:00Z

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
- Cycle 1 (this session): Reconciled CI failure in PR #35 (fmt issues) and completed slice D2. PR #35 merged.
- Next: D3 · Implement the `sync` capability.
