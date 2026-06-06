# Loop state — idd-merge-loop
session_started: 2026-06-05T00:00:00Z
loop: idd-merge-loop
epic: Complete Delivery + Upgrade (upgrade-only / no-downgrade)
branch: ci-msrv-floor (A4 branch off origin/develop @4a6f8bd, carries A1+A2+A3)
worktree: ../idd-ci-msrv-floor  (prior merged: #26 #27 #28)
base_branch: develop          # dev work targets develop; main only via gated promotion PR
promote_target: main          # develop->main on DONE, gated by rust + promote-verify
pr_policy: ONE PR PER CYCLE (see race note). per-cycle PR --base develop + auto-merge squash (mandatory). develop protected (required check 'rust') => fail-closed. NEVER push/admin-merge main directly.
open_pr: A1 #26, A2 #27, A3 #28 all MERGED to develop (@4a6f8bd). A4 -> opening now (branch ci-msrv-floor).
cycle_budget: 3
cycles_this_session: 1
cycles_total: 4
last_item: A4 [x] pin CI toolchain + MSRV floor. Empirically measured floor = 1.88 (NOT guessed 1.85): time@0.3.47⇒1.88, ratatui 0.30⇒1.86, runner let-chains⇒1.88. Declared rust-version core=1.74 / spec,runner,tui,cli=1.88. Pinned rust job @stable→@1.96.0. Added msrv job @1.88.0 (build+test-compile --locked). All gates green; 429 tests. Next: A5 (flake.nix ≥1.88).
status: CYCLING — A4 done+verified. Opening run PR --base develop w/ auto-merge. Next cycle: A5.
last_update: 2026-06-06T02:10:00Z

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
