# Loop state — idd-merge-loop
session_started: 2026-06-05T00:00:00Z
loop: idd-merge-loop
epic: Complete Delivery + Upgrade (upgrade-only / no-downgrade)
branch: flake-floor (A5 branch off origin/develop @1ea5e9e, carries A1-A4)
worktree: ../idd-flake-floor  (prior merged: #26 #27 #28 #29)
base_branch: develop          # dev work targets develop; main only via gated promotion PR
promote_target: main          # develop->main on DONE, gated by rust + promote-verify
pr_policy: ONE PR PER CYCLE (see race note). per-cycle PR --base develop + auto-merge squash (mandatory). develop protected (required check 'rust') => fail-closed. NEVER push/admin-merge main directly.
open_pr: A1 #26, A2 #27, A3 #28, A4 #29 all MERGED to develop (@1ea5e9e). A5 -> opening now (branch flake-floor).
cycle_budget: 3
cycles_this_session: 2
cycles_total: 5
last_item: A5 [x] flake.nix MSRV floor enforcement. flake.lock already pinned (rustc 1.93≥floor); added a hard assert (rustc.version >= 1.88) failing `nix develop` eval below floor. Tested fail-closed both ways via nix eval. drift 0, validate 0-crit, suite unchanged (429). Next: A6 (collapse dup transitive versions — last of Epic A).
status: CYCLING — A4 merged (#29). A5 done+verified. Opening A5 PR --base develop w/ auto-merge. Next cycle: A6 (completes Epic A).
last_update: 2026-06-06T02:25:00Z

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
