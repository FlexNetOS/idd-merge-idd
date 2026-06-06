---
name: idd-merge-loop
description: "Autonomous, resumable merge/port loop for idd-merge-idd: works a durable on-disk backlog ONE vertical slice per cycle, commits per cycle, and hands off to a fresh session at a cycle budget — so long runs survive context rot and cold restarts with zero loss. Each cycle drives the existing worker skills (vertical-slice-planning → rust-native-implementation → merge-verification → pr-evidence-bundle). ALWAYS use to run the merge/unification work unattended or across many slices/sessions, to resume after a handoff, or to self-restart with /new. Trigger keywords: run the merge loop, idd-merge-loop, work the backlog, one slice per cycle, autonomous/unattended run, resume the loop, pick up where it left off, continue in a new session, ralph loop, /new runner. For a single interactive slice with the full agent team, use merge-orchestrator instead; this loop is the durable multi-slice/continuity layer on top of it."
---

# idd-merge-loop — durable backlog, one slice per cycle, resumable

A **Ralph loop** for the rusty-idd merge/port work. It exists because a single long session rots (context fills, quality drops, tokens burn). The defense: run the work as a **chain of short cycles**, each writing a durable checkpoint, so any session — or a fresh process — resumes cold from disk with zero loss.

Two layers:
- **In-session loop** (this skill): work one backlog slice per cycle, commit, self-pace; at the cycle budget, hand off.
- **External self-restart** (`scripts/ralph-idd.sh`): spawns a fresh `gemini -p` per iteration (a new process = a clean context = the `/new` effect) until a terminal sentinel.

This is the **continuity layer on top of `merge-orchestrator`**. The orchestrator runs ONE slice with the 5-agent team end-to-end; this loop sequences slices across sessions with a durable backlog + commit-per-cycle + handoff. Each cycle either invokes `merge-orchestrator` for the slice, or drives its worker skills directly (`vertical-slice-planning` → `rust-native-implementation` → `merge-verification` → `pr-evidence-bundle`).

## Non-negotiable principles
1. **Write state down every cycle.** Never hold the plan only in context.
2. **Truth on disk** under `_workspace/` (backlog + ledger + checkpoint + commits). A fresh process resumes from committed state alone.
3. **One slice per cycle; commit AND push per cycle.** Area-prefixed subject; push the run branch every cycle so an interrupted run strands nothing. PR evidence via `pr-evidence-bundle`.
4. **The committed `HANDOFF.md` is the authoritative resume signal** — not the weave inbox (a self-addressed weave message doesn't land in your own inbox; a same-machine successor shares your identity). weave is an observable heartbeat (`to:"all"`), not the payload.
5. **Fail-closed.** Destructive/irreversible steps are dry-run first + opt-in (`IDD_APPLY=1`). `rusty-idd validate` is fail-closed: never tick a slice whose validation has CRITICAL findings. Never weaken a guard to make a step pass.
6. **Human walls STOP the loop** (sudo / interactive auth / a hardware wall) → write `NEEDS-HUMAN` with the reason and halt; don't spin or force. Opening + auto-merging the run's PR is **not** a wall (authorized + fail-closed); but a PR whose required `rust` check has **failed** is a wall → leave it open, record `NEEDS-HUMAN: PR #N red`.
7. **Rust-native by mandate.** `crates/core` stays zero-dep; `drift-check.sh` must be green every cycle (new deps only at spec/runner/tui/cli edges).
8. **Bounded.** A max-iterations backstop + an always-checked `STOP` kill switch live in the runner.
9. **Every run ends on a PR with auto-merge — MANDATORY (user-authorized).** A run must never end with unmerged work on a local branch: that loses work and makes the next run conflict off a stale base. **Dev work lands on `develop`, never `main`.** So before writing *any* terminal/handoff sentinel, the run pushes its branch, opens (or reuses) a PR `--base develop`, and enables auto-merge (`gh pr merge --auto --squash`). `develop` is branch-protected (required check `rust`) so auto-merge is **fail-closed**: GitHub merges only on green CI, async, even after the process exits. A red PR is left open + recorded — never force-merged. **Promotion to `main` is a separate gate:** when the backlog clears (DONE), open ONE `develop`→`main` promotion PR with auto-merge; `main` requires `rust` **+** the enhanced `promote-verify` workflow (clean-merge + locked build/test + drift + fmt/clippy + security audit). The loop never pushes `main` directly and never force-promotes.

## Phase 0 — Context check (initial / resume / new)
1. `_workspace/HANDOFF.md` exists **and** the request is "resume" / "pick up" / "continue" → **RESUME**: use the `session-relay` skill (RESUME entry); it reads the committed `HANDOFF.md`, runs the verify-on-resume baseline, resets `cycles_this_session=0`, and continues at the backlog's current item.
2. `_workspace/backlog.md` exists, no resume intent, user gives new scope → carry the backlog forward (append new items) or, for a wholly new mission, move `_workspace/` aside (timestamped) and re-DISCOVER.
3. Neither exists → **DISCOVER** (below), then start cycling.

Always first: confirm a synced base and a fresh worktree per `GEMINI.md` (sync `rtk git fetch --all`, level with `origin/develop`, work in `../idd-<slug>` off the synced base **`origin/develop`** — dev work never branches off `main`). **Avoid worktree names containing "archive"** — `tui::test_find_change_dir_active` asserts a CWD-derived path lacks that substring. **If the prior run left an open (unmerged) PR**, do not branch off stale `develop` — branch off that PR's branch instead (see **Branch & PR lifecycle**).

## DISCOVER (the backlog writes itself)
Generate the slice backlog from the repo's own merge-planning pipeline (the `idd` bin was retired in slice 8 → use `rusty-idd`):
```bash
cargo run --quiet --bin rusty-idd -- scan --repo <path>   # walk repo(s) -> RepoInventory
cargo run --quiet --bin rusty-idd -- plan                 # render inventories + merge plan + tasks
```
The backlog = the generated merge tasks / vertical slices **in dependency order**. Also fold in the plan-of-record slices from `docs/rusty-idd/slice-sequence.md` and any `_workspace/02_planner_epic.md`. Write them to `_workspace/backlog.md`:
```
- [ ]  todo        - [x]  done + verified        - [!] blocked: <reason>
```
**GitHub Integration**: Mirror the backlog to GitHub Issues for visibility. For each `- [ ]` item, run `gh issue create --title "Merge Slice: <name>" --body-file _workspace/02_planner_slice.md --label "merge-loop"` (if not already created) and record the Issue# in `backlog.md`.

Seed `_workspace/loop_state.md` (template in `references/loop-state-template.md`).

## One iteration (a cycle)
1. **Read state** — `backlog.md` + `loop_state.md`.
2. **Stop-checks** (every terminal exit first ensures the run's PR is open + auto-merge enabled + PR# recorded — see **Branch & PR lifecycle**) —
   - no `- [ ]` left → **DONE**: run the full verify suite once more; if all green, ensure the run PR (→`develop`) is open with auto-merge AND open the `develop`→`main` promotion PR (auto-merge, gated by `rust`+`promote-verify`), then write `_workspace/DONE` (with evidence + both PR#s) and stop.
   - `cycles_this_session >= cycle_budget` → **HAND OFF** via `session-relay` (HAND OFF entry — it pushes, ensures the PR + auto-merge, records PR# in `HANDOFF.md`), then stop (no wakeup).
   - `_workspace/STOP` exists → still push + ensure the PR/auto-merge for work already committed (don't lose it), then stop.
3. **Pick the top `- [ ]`** in dependency order (one slice).
4. **Do the slice** — drive the worker skills (or `merge-orchestrator`) for exactly this slice:
   `vertical-slice-planning` (scope it: parity test + gate + rollback) → `rust-native-implementation` (additive, deprecate-before-delete, edition-aware, core stays std-only; all writes via `fs_utils::write_string_preserving_existing` — backup-on-overwrite, never clobber) → `merge-verification` → `pr-evidence-bundle`. Destructive steps: **dry-run first; apply only under `IDD_APPLY=1`**.
5. **VERIFY across the boundary** (not existence-only — run the real gates in a fresh shell):
   ```bash
   cargo run --quiet --bin rusty-idd -- validate            # fail-closed: CRITICAL -> non-zero -> do NOT tick
   bash .gemini/skills/merge-verification/scripts/drift-check.sh .    # exit 0 = no Rust-native drift
   rtk cargo fmt --all -- --check
   rtk cargo clippy --workspace --all-targets --all-features -- -D warnings
   rtk cargo test --workspace --locked                      # CI mode: fails on Cargo.lock drift
   ```
   (Delegate this to `merge-verification`; it reads both sides of each boundary.) A WARNING-only result may pass; a CRITICAL/ERROR fails the cycle — mark `- [!] blocked: <reason>`, never a false `- [x]`.
6. **Write state back** — flip the item `- [x]` (verified) or `- [!] blocked`; bump `cycles_this_session` and `cycles_total`; update `last_item`/`status`/`last_update`.
7. **Commit + push + ensure PR** — `git add` the slice's code + `_workspace/backlog.md` + `_workspace/loop_state.md`; commit (area-prefixed subject); **`git push -u origin <run-branch>`**. If the run's PR isn't open yet, open it now (body via `pr-evidence-bundle`, `gh pr create --base develop`) and enable auto-merge (`gh pr merge --auto --squash <pr>`); record the PR# in `loop_state.md`. This makes even a one-cycle run durable — work is on a PR heading to `develop` the moment it's committed, not stranded locally.
8. **Self-pace** — `ScheduleWakeup` to re-enter the next cycle; use a long delay if waiting on a slow external step (CI). At budget, HAND OFF instead and do **not** schedule a wakeup.

## Branch & PR lifecycle (mandatory — prevents work loss + next-run conflicts)
Two-tier flow: **dev work → `develop` (loop has access); `develop` → `main` only via a gated promotion PR.** `main` never takes direct dev work.
- **One run branch.** A run cuts ONE worktree/branch off the synced base (`idd-<slug>` off `origin/develop`, no "archive" substring) and does its ≤`cycle_budget` slices there, one commit each (slices stay individually reviewable in the squash).
- **Push every cycle** (step 7). Nothing lives only locally.
- **Run PR opens on the first cycle**, `--base develop`, with **auto-merge squash** enabled immediately. `develop`'s required `rust` check gates it (fail-closed): green → GitHub squash-merges async (even after this process exits) and deletes the branch; red → PR stays open, record `NEEDS-HUMAN: PR #N red`.
- **Promotion to `main`** happens on **DONE** (backlog clear): open ONE `develop`→`main` PR with auto-merge. `main` requires `rust` **+** `promote-verify` (the enhanced workflow: clean-merge check, locked build/test, drift, fmt/clippy, `cargo audit`). It merges only when all pass — so `main` is always clean. Never push/admin-merge `main` directly.
- **Resume must not branch off a stale base.** On RESUME, after syncing: if the prior run's PR is **merged**, cut the new branch off the now-advanced `origin/develop`; if it's **still open** (CI pending or red), branch the next worktree **off that PR's branch** (stack) — never start parallel work off a `develop` behind the open PR, or you recreate the exact conflict this policy exists to prevent. `HANDOFF.md`/`loop_state.md` carry the open PR# + branch for this.
- **No duplicate PRs.** Reuse the run's existing PR across its cycles (`gh pr view <branch>` / recorded PR#); only open a new one per run branch.

## DONE criteria (all true → write `_workspace/DONE` with evidence)
- `rusty-idd validate` clean (no CRITICAL) ·
- `rtk cargo build --workspace` + `test --workspace --locked` green ·
- `fmt --check` + `clippy -D warnings` clean · `drift-check.sh` exit 0 ·
- `pr-evidence-bundle` produced for the shipped slice(s) ·
- the run's PR (→`develop`) is open with auto-merge (or merged) — **no unmerged work stranded locally** ·
- **Comment on PR**: post the final `_workspace/DONE` evidence as a comment: `gh pr comment <pr> -F _workspace/DONE`.
- on backlog-clear, a `develop`→`main` promotion PR is open with auto-merge (gated by `rust`+`promote-verify`) ·
- backlog has no `- [ ]` left.
Record the passing command output **and the PR number(s)/URL(s)** in `_workspace/DONE`.

## Sentinel contract (the runner reads exactly one per process)
| Sentinel (`_workspace/…`) | Meaning | Runner action |
|---------------------------|---------|---------------|
| `HANDOFF.md` | more work remains | spawn the next fresh process |
| `DONE` | finished + verified (evidence inside) | exit 0 |
| `NEEDS-HUMAN` | sudo / interactive auth / review gate / hardware wall (reason inside) | halt for human |
| `STOP` | kill switch (human `touch`es it) | halt |

Write **exactly one** sentinel per process when running under the external runner, then stop.

## External self-restart (`/new` effect)
An agent cannot type `/new` (it's a REPL command, not a tool) — but a new process is a clean context. The bundled runner does it:
```bash
bash .gemini/skills/idd-merge-loop/scripts/ralph-idd.sh                 # SAFE: dry-run/plan, commits non-destructive progress
IDD_APPLY=1 bash .gemini/skills/idd-merge-loop/scripts/ralph-idd.sh     # UNATTENDED APPLY: opt in deliberately
touch _workspace/STOP                                                   # kill switch, any time
```
Each iteration spawns a fresh `gemini -p "/idd-merge-loop resume …"`, which runs up to one budget of cycles, writes one sentinel, and exits; the runner respawns until a terminal sentinel or `MAX_ITERS`.

## Repo-specific guardrails (do not violate)
- **`fs_utils::write_string_preserving_existing`** is backup-on-overwrite — respect it; don't clobber existing files outside the normal write path.
- **`rusty-idd validate` is the fail-closed gate** — never tick a slice whose validation has CRITICAL findings.
- **Rust-native mandate** — `crates/core` `[dependencies]` stays empty; `drift-check.sh` green every cycle; port foreign-language drift to Rust rather than wrapping it.
- **Parity** — `rusty-idd <core-verb>` stays byte-identical to the former `idd`; deprecate-before-delete.
- **Every run ships a PR with auto-merge** (mandatory, user-authorized — principle 9). One PR per run branch (bundling that run's vertical slices, each its own commit); `--base develop`; auto-merge squash; fail-closed on the required `rust` check. `develop`→`main` is a separate promotion PR (on DONE) gated by `rust`+`promote-verify`. **Never push or admin-merge `main` directly.** Required PR *evidence* (`pr-evidence-bundle`) still attaches; opening + auto-merging is automatic, not human-gated. Only a **red** required check is a wall.

## Data transfer
Task-based (coordination) + File-based (`_workspace/` audit trail) + Message-based / weave heartbeat. Keep the per-slice orchestrator/worker artifacts in `_workspace/` (`0{1,2,3,4}_*.md`); the loop's own durable files are `backlog.md`, `loop_state.md`, `HANDOFF.md`, and the sentinels.

## Test Scenarios
**Happy path:** DISCOVER seeds a 3-item backlog → cycle 1 implements+verifies+commits+**pushes** slice 1 (`- [x]`) and **opens PR `--base develop` with auto-merge squash** → cycle 2 commits+pushes slice 2 onto the same PR → at `cycle_budget` (3) HAND OFF commits `HANDOFF.md` (recording the PR#), ensures auto-merge, heartbeats `relay:handoff` → CI greens → GitHub squash-merges the PR into `develop` + deletes the branch → a fresh session `/idd-merge-loop resume from _workspace/HANDOFF.md` sees the PR merged, branches off the now-advanced `origin/develop`, continues at slice 3 → backlog clear → loop opens a `develop`→`main` promotion PR (auto-merge, gated by `rust`+`promote-verify`) → `_workspace/DONE` with the green suite + both PR#s.

**Error path A (drift):** cycle finds Rust-native drift (a dep crept onto `crates/core`) → mark `- [!] blocked: core dep drift`, do NOT tick → relocate the dep to the correct edge → re-verify clean → `- [x]`.
**Error path B (red PR):** run ends, PR opened with auto-merge, but the required `rust` check fails on CI → PR does **not** merge (fail-closed) → write `NEEDS-HUMAN: PR #N red (<failing gate>)` and halt; the next run, finding the PR still open, branches off its branch rather than stale `develop`.

## References
- `references/loop-state-template.md` — the `loop_state.md` ledger schema.
- Per-slice workers: `vertical-slice-planning`, `rust-native-implementation`, `merge-verification`, `pr-evidence-bundle`; team coordination: `merge-orchestrator`; continuity: `session-relay` + the `continuity-steward` agent.
