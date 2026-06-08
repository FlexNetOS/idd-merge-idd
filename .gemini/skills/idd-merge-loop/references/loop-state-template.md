# `_workspace/loop_state.md` — ledger schema

The loop's per-session ledger. Seed it at DISCOVER, update it every cycle, commit it
with the cycle. Scripts cannot read the clock — **you** supply each UTC timestamp.

```markdown
# Loop state — idd-merge-loop
session_started: 2026-06-05T00:00:00Z   # you supply it
loop: idd-merge-loop
branch: <task-branch>
worktree: <abs path, NOT containing the substring "archive">
cycle_budget: 3            # completed cycles per session before handoff (override via runner arg)
cycles_this_session: 0     # reset to 0 on RESUME
cycles_total: 0            # carried across sessions
last_item: (none — discovery only)
status: DISCOVER complete — backlog seeded
last_update: 2026-06-05T00:00:00Z
```

## Field notes
- `cycles_this_session` is the budget counter; `session-relay` RESUME resets it to 0.
- `cycles_total` is monotonic across sessions (don't reset on resume).
- `last_item` = the backlog item the last completed cycle worked (verbatim text).
- `status` = a one-line human-readable state (e.g. "cycle 2 done: slice `runner-yaml` shipped; next `flake`").
- On a blocked cycle, `status` names the blocker and the item is `- [!]` in `backlog.md`.
