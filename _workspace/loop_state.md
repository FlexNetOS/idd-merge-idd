# Loop state — idd-merge-loop
session_started: 2026-06-05T00:00:00Z
loop: idd-merge-loop
epic: Complete Delivery + Upgrade (upgrade-only / no-downgrade)
branch: dedup-deps (A6 branch off origin/develop @b07f7fb, carries A1-A5)
worktree: ../idd-dedup-deps  (prior merged: #26 #27 #28 #29 #30)
base_branch: develop          # dev work targets develop; main only via gated promotion PR
promote_target: main          # develop->main on DONE, gated by rust + promote-verify
pr_policy: ONE PR PER CYCLE (see race note). per-cycle PR --base develop + auto-merge squash (mandatory). develop protected (required check 'rust') => fail-closed. NEVER push/admin-merge main directly.
open_pr: A1 #26, A2 #27, A3 #28, A4 #29, A5 #30 all MERGED. A6 -> opening now (branch dedup-deps, bundles HANDOFF.md).
cycle_budget: 3
cycles_this_session: 3
cycles_total: 6
last_item: A6 [x] dup transitive versions = documented no-op. All multi-version dups (syn/bitflags/nom/phf) trace to ratatui-termwiz (optional, un-enabled backend; we use crossterm) — lock-resident only, never built/shipped, no forward-upgrade unifies them. Host cargo tree -d = same-version build/runtime splits only. Doc: docs/rusty-idd/dependency-duplication.md. No code/lock change → 429. **EPIC A COMPLETE.**
status: HAND OFF — cycle budget (3) reached. **EPIC A (supply-chain, A1-A6) COMPLETE.** Next session: B1 (Epic B — runtime robustness). Epics B/C/D/E remain → NOT full-backlog DONE, so NO develop→main promotion PR yet (promotion only on full clear).
last_update: 2026-06-06T02:40:00Z

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
