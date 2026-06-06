# Loop state — idd-merge-loop
session_started: 2026-06-05T00:00:00Z
loop: idd-merge-loop
epic: Complete Delivery + Upgrade (upgrade-only / no-downgrade)
branch: syntect-unmaintained (A3 PR branch off origin/develop @2bfcb4f, carries A1+A2)
worktree: ../idd-syntect-unmaintained  (prior merged: cargo-audit-gate #26, time-a2 #27)
base_branch: develop          # dev work targets develop; main only via gated promotion PR
promote_target: main          # develop->main on DONE, gated by rust + promote-verify
pr_policy: ONE PR PER CYCLE (see race note). per-cycle PR --base develop + auto-merge squash (mandatory). develop protected (required check 'rust') => fail-closed. NEVER push/admin-merge main directly.
open_pr: A1 -> #26 MERGED (4b5cba2). A2 -> #27 MERGED (2bfcb4f). A3 -> new PR (opening now off develop, branch syntect-unmaintained, bundling HANDOFF.md).
cycle_budget: 3
cycles_this_session: 3
cycles_total: 3
last_item: A3 [x] bincode/yaml-rust accepted-risk (no upgrade path — syntect 5.3.0 latest still pulls them; dropping = losing TUI highlight-code capability). Recorded docs/rusty-idd/security-advisories.md + .cargo/audit.toml rationale. No compiled-input change → suite unchanged (429). Gate still fail-closed on new advisories/vulns.
status: HAND OFF — cycle budget (3) reached. Epic A 50% done (A1-A3 [x]; A4-A6 remain). A1+A2 merged to develop; A3 shipping its own PR (bundles HANDOFF.md). Next session: A4 (pin CI toolchain + MSRV/edition floor).
last_update: 2026-06-06T01:45:00Z

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
