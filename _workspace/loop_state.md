# Loop state — idd-merge-loop
session_started: 2026-06-05T00:00:00Z
loop: idd-merge-loop
epic: Complete Delivery + Upgrade (upgrade-only / no-downgrade)
branch: spec-tests-C3-C4 (PR #35), next: D1
open_pr: PR #33 (B1/B2 merged), PR #34 (B3 open/merged), PR #35 (C3-C5 open/auto-merging).
cycle_budget: 3
cycles_this_session: 2
cycles_total: 17
last_item: D1 [x] Build the parity matrix.
status: RUNNING — D1 complete. Proceeding to D2. 
last_update: 2026-06-07T02:30:00Z

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
- 25 slices across 5 epics. Dependency order: A (supply-chain, audit RED today) → B (robustness) → C (tests) → D (feature/spec completeness) → E (docs/harness). A1/A2 first.
- Global invariant baked into backlog header: UPGRADE ONLY, NO DOWNGRADES. Suite baseline 429 (only grows).
- Not yet running cycles — backlog built and awaiting go-ahead to start the loop.
