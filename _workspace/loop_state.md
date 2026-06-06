# Loop state — idd-merge-loop
session_started: 2026-06-05T00:00:00Z
loop: idd-merge-loop
epic: Complete Delivery + Upgrade (upgrade-only / no-downgrade)
branch: time-a2 (A2 PR branch off origin/develop @4b5cba2, which already carries A1)
worktree: ../idd-time-a2  (prior: ../idd-cargo-audit-gate for A1; cargo-audit-gate merged as #26)
base_branch: develop          # dev work targets develop; main only via gated promotion PR
promote_target: main          # develop->main on DONE, gated by rust + promote-verify
pr_policy: ONE PR PER CYCLE (see race note). per-cycle PR --base develop + auto-merge squash (mandatory). develop protected (required check 'rust') => fail-closed. NEVER push/admin-merge main directly.
open_pr: A1 -> PR #26 MERGED into develop (4b5cba2). A2 -> new PR (opening now off develop, branch time-a2).
cycle_budget: 3
cycles_this_session: 2
cycles_total: 2
last_item: A2 [x] time 0.3.41->0.3.47 (forward, --precise; resolver held at .41). audit clears time; removed the develop-gate time-ignore; 429 tests --locked; build/clippy/fmt/drift/validate clean. promote-verify time wall cleared. Next: A3 (retire unmaintained bincode/yaml-rust via syntect).
status: CYCLING — A1 merged (#26). A2 verified, shipping its own PR off develop. Next cycle: A3.
last_update: 2026-06-06T01:30:00Z

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
