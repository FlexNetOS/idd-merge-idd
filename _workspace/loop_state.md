# Loop state — idd-merge-loop
session_started: 2026-06-05T00:00:00Z
loop: idd-merge-loop
epic: Complete Delivery + Upgrade (upgrade-only / no-downgrade)
branch: develop (no slice cut yet — next cycle cuts a worktree off origin/develop)
worktree: (none yet — must NOT contain the substring "archive")
base_branch: develop          # dev work targets develop; main only via gated promotion PR
promote_target: main          # develop->main on DONE, gated by rust + promote-verify
pr_policy: per-run PR --base develop + auto-merge squash (mandatory). develop protected (required check 'rust') => fail-closed. Push every cycle; PR opens on cycle 1; resume off the prior PR if still open. NEVER push/admin-merge main directly.
open_pr: (none yet)
cycle_budget: 3
cycles_this_session: 0
cycles_total: 0
last_item: (none — DISCOVER + setup only; cycling not started)
status: HANDOFF — setup session complete (audit + backlog + develop/main branching + auto-merge policy). Next session = cycle 1 at A1.
last_update: 2026-06-06T00:00:00Z

## Field notes
- 25 slices across 5 epics. Dependency order: A (supply-chain, audit RED today) → B (robustness) → C (tests) → D (feature/spec completeness) → E (docs/harness). A1/A2 first.
- Global invariant baked into backlog header: UPGRADE ONLY, NO DOWNGRADES. Suite baseline 429 (only grows).
- Not yet running cycles — backlog built and awaiting go-ahead to start the loop.
