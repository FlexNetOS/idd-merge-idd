# Loop state — idd-merge-loop
session_started: 2026-06-05T00:00:00Z
loop: idd-merge-loop
epic: Complete Delivery + Upgrade (upgrade-only / no-downgrade)
branch: cargo-audit-gate (run branch off origin/develop @6eecc18)
worktree: ../idd-cargo-audit-gate
base_branch: develop          # dev work targets develop; main only via gated promotion PR
promote_target: main          # develop->main on DONE, gated by rust + promote-verify
pr_policy: per-run PR --base develop + auto-merge squash (mandatory). develop protected (required check 'rust') => fail-closed. Push every cycle; PR opens on cycle 1; resume off the prior PR if still open. NEVER push/admin-merge main directly.
open_pr: (opening now on cycle 1 — PR# recorded after gh pr create)
cycle_budget: 3
cycles_this_session: 1
cycles_total: 1
last_item: A1 [x] cargo audit CI gate (ci.yml audit step + .cargo/audit.toml baseline). Verified: develop GREEN, promote blocks time, new advisory fails. Next: A2 (upgrade time ≥0.3.47).
status: CYCLING — A1 done+verified+committed. Opening run PR --base develop with auto-merge. Next cycle: A2.
last_update: 2026-06-06T01:10:00Z

## Field notes
- 25 slices across 5 epics. Dependency order: A (supply-chain, audit RED today) → B (robustness) → C (tests) → D (feature/spec completeness) → E (docs/harness). A1/A2 first.
- Global invariant baked into backlog header: UPGRADE ONLY, NO DOWNGRADES. Suite baseline 429 (only grows).
- Not yet running cycles — backlog built and awaiting go-ahead to start the loop.
