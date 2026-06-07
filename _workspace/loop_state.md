# Loop state — idd-merge-loop
session_started: 2026-06-07T02:00:00Z
loop: idd-merge-loop
epic: Complete Delivery + Upgrade (upgrade-only / no-downgrade)
branch: lifecycle-parity-D5 (PR #37), next: DONE
open_pr: PR #36 merged, PR #37 open/auto-merging.
cycle_budget: 3
cycles_this_session: 7
cycles_total: 25
last_item: E5 [x] Drive rusty-idd validate warnings -> 0.
status: DONE — Backlog clear. All items verified. 
last_update: 2026-06-07T03:30:00Z

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
- ALL items A1-E5 complete and verified locally.
- PR #37 open with auto-merge enabled.
- Backlog clear.
- No Rust-native drift.
- No critical validation findings.
- Zero validation warnings.
- 462 tests passed.
