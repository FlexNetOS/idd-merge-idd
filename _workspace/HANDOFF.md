# HANDOFF — idd-merge-loop

_Checkpoint written: 2026-06-05T06:58:33Z. Authoritative cold-start resume signal (not the weave inbox)._

## Status: IDLE — no in-flight loop. Both epics complete; `main` clean.

There is **no slice in flight** and the backlog is **clear**. This is a clean-slate
checkpoint: a resume should confirm the green baseline, then either DISCOVER a new
backlog (new work) or stop (nothing to do).

## Resume command
```
/idd-merge-loop resume from _workspace/HANDOFF.md
```
Unattended variants (external runner; see CLAUDE.md):
```
bash .claude/skills/idd-merge-loop/scripts/ralph-idd.sh                 # SAFE (no auto-apply)
IDD_APPLY=1 bash .claude/skills/idd-merge-loop/scripts/ralph-idd.sh     # unattended + apply
touch _workspace/STOP                                                   # kill switch
```

## Worktree + branch
- Repo: `~/Desktop/idd-merge-idd` (integration root).
- Branch: `main`, **level with `origin/main`** (HEAD `86feeec`). Working tree clean.
- Next session: per CLAUDE.md, cut a fresh worktree off the synced base before any code change:
  `rtk git worktree add ../idd-<slug> -b <slug> origin/main`.
  **Do not** name a worktree with the substring `archive` (trips `tui::test_find_change_dir_active`).

## Backlog status
- **0 todo / 0 blocked.** No `_workspace/backlog.md` yet — none needed; nothing pending.
- If new work arrives, DISCOVER seeds it: `cargo run --quiet --bin rusty-idd -- scan/plan` + `docs/rusty-idd/slice-sequence.md`.

## In-flight cycle
- None. `cycles_this_session` n/a (loop has not been run; this checkpoint was written by hand at session wrap-up).

## Landed this session (all merged to `main`)
- **"Upgrades + fixes" epic — PRs #13–#21** (9 slices, each gate-green):
  #13 flake.nix retarget · #14 `spec validate --strict` summary/exit reconcile ·
  #15 runner `serde_yaml`→`serde_norway` · #16 `spec archive --no-validate/-y` wiring ·
  #17 RENAME+MODIFY op-order pinned to oracle (fixed an inverted merge) · #18 byte-exact `emit_spec` ·
  #19 `schema/` edge (`spec status`/`next`) · #20 `adr/` edge (`spec adr`) · #21 `scaffold/` edge (`spec new`/`scaffold`).
  Test count 387 → 429.
- **Harness upgrade — PR #22**: added `idd-merge-loop` + `session-relay` skills, `continuity-steward`
  agent, `scripts/ralph-idd.sh` runner (opt-in `IDD_APPLY=1`), `_workspace/.gitignore`, CLAUDE.md
  trigger pointer + change-history rows.

## Open findings / blockers
- None. No `NEEDS-HUMAN` walls. Lifecycle engine (`crates/spec`) is feature-complete
  (model/parse/emit byte-exact/validate/archive/schema/adr/scaffold); `cli/` edge in `crates/cli`.

## Decisions & dead-ends (don't re-litigate)
- Byte-exact `emit_spec` is achieved by a hand-rolled tightening emitter, **not** by tuning
  `format_commonmark` options (design §7 "Splice vs rebuild" → rebuild won).
- RENAME applies **before** MODIFY/REMOVE; a MODIFIED referencing the old name aborts (oracle-verified, fixtures 06–08).
- Unattended apply is the **runner** (`IDD_APPLY=1`), not a slash command — an agent can't type `/new`.
- `_workspace/` audit intermediates (`0{1,2,3,4}_*.md`) stay **untracked**; only `backlog.md`/`loop_state.md`/`HANDOFF.md` are committed.
- CLAUDE.md "Change history" now carries the epic + harness rows (chose this over a separate doc).

## Verify-on-resume (run FIRST; confirm green before new work)
```bash
cargo run --quiet --bin rusty-idd -- validate          # fail-closed: CRITICAL -> non-zero
bash .claude/skills/merge-verification/scripts/drift-check.sh .   # exit 0 = no Rust-native drift
rtk cargo fmt --all -- --check
rtk cargo clippy --workspace --all-targets --all-features -- -D warnings
rtk cargo test --workspace --locked                    # expect: 429 passed
```
Last known-green: all of the above on `86feeec` (drift 0, fmt clean, clippy clean, 429 tests).
