# Loop state — idd-merge-loop
session_started: 2026-06-05T00:00:00Z
loop: idd-merge-loop
epic: Complete Delivery + Upgrade (upgrade-only / no-downgrade)
branch: dedup-deps (A6 branch off origin/develop @b07f7fb, carries A1-A5)
worktree: ../idd-dedup-deps  (prior merged: #26 #27 #28 #29 #30)
base_branch: develop          # dev work targets develop; main only via gated promotion PR
promote_target: main          # develop->main on DONE, gated by rust + promote-verify
pr_policy: ONE PR PER CYCLE (see race note). per-cycle PR --base develop + auto-merge squash (mandatory). develop protected (required check 'rust') => fail-closed. NEVER push/admin-merge main directly.
open_pr: B1 #33 (armed auto-merge squash).
cycle_budget: 3
cycles_this_session: 1
cycles_total: 7
last_item: B1 [x] lock().unwrap() -> poison-tolerant. Added test_poisoned_mutex_recovery. Refactored 19 flaky runner tests. Verified: 430 tests. PR #33.
status: RUNNING — B1 complete. Proceeding to B2.
last_update: 2026-06-06T18:25:00Z

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
