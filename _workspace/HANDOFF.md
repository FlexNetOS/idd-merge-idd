# HANDOFF — idd-merge-loop

_Checkpoint written: 2026-06-06. Authoritative cold-start resume signal (not the weave inbox)._

## Status: READY TO CYCLE — setup session complete, loop has NOT started cycling yet.
A fresh 25-slice backlog (Epics A–E) is seeded and the develop/main branching model is live.
The next session RESUMEs and runs **cycle 1 at backlog item A1**. No slice is in flight.

## Resume command
```
/idd-merge-loop resume from _workspace/HANDOFF.md
```
Unattended (external runner): `bash .claude/skills/idd-merge-loop/scripts/ralph-idd.sh` (SAFE) ·
`IDD_APPLY=1 bash .../ralph-idd.sh` (apply) · `touch _workspace/STOP` (kill switch).

## Branching model (NEW this session — read before any git op)
- **`develop` = default + integration branch.** Protected: required check **`rust`**, no reviews. The loop has full auto-merge access here.
- **`main` = protected release trunk.** Required checks **`rust` + `promote-verify`**, no reviews. Reached ONLY via a `develop`→`main` promotion PR. **NEVER push or admin-merge `main` directly** (the safety classifier enforces this).
- Repo: `allow_auto_merge=true`, `delete_branch_on_merge=true`.
- **Dev work branches off `origin/develop`** (worktree `../idd-<slug>`, NO "archive" substring), one slice/cycle, **commit + push every cycle**, PR `--base develop` opened on cycle 1 with `gh pr merge --auto --squash` (fail-closed on `rust`).
- **On DONE (backlog clear):** open a `develop`→`main` promotion PR with auto-merge. `promote-verify` runs `cargo audit` → **it will BLOCK promotion until A2 fixes the `time` CVE** (by design — main only gets clean code).

## This run's PR
- Handoff/continuity PR (this checkpoint): **PR #25 → develop, auto-merge squash enabled** (continuity files only; harness code already on develop). On resume: if merged → branch off advanced `origin/develop`; if still open → branch off its branch (`loop/handoff-setup`).
- No code/slice PR is in flight.

## Backlog (truth = `_workspace/backlog.md`, 25 slices, dependency order)
Next item: **A1 · add a `cargo audit` CI gate** → then **A2 · fix RUSTSEC-2026-0009 (`time` 0.3.41 → ≥0.3.47), forward only**. Epics: A supply-chain (audit RED today) → B robustness → C tests → D feature/spec completeness → E docs/harness.
Mandate: rusty-idd = all 3 source projects' features+capabilities+specs unified. **Invariant: UPGRADE ONLY / NO DOWNGRADES** (no capability removed, suite only grows from 429, no dep downgraded, core stays zero-dep, gates never weakened).

## Landed this session (all on `origin/develop`, commits 776aa21 + f890571)
- Deep codebase audit (6 parallel audits + `cargo audit` + verification) — found: live `time` CVE + 2 unmaintained deps; CI uses `@stable` (no MSRV floor); runner silent-failure/panic surfaces; fs_utils/parse-emit test holes; doc drift (stale READMEs, byte-exact contradiction); parity gaps (validate subset, `sync` verb, no oracle harness).
- Fresh backlog (`_workspace/backlog.md`) + `loop_state.md`; retired the prior epic's stale `DONE` → `DONE.prev-epic`.
- Mandatory PR + auto-merge policy and develop/main branching model baked into: `idd-merge-loop/SKILL.md` (principle 9 + Branch & PR lifecycle), `session-relay/SKILL.md`, `scripts/ralph-idd.sh`, `.github/workflows/promote-verify.yml` (new), `CLAUDE.md`.
- Repo settings + branch protection applied (above). `promote-verify.yml` seeded to main via auto-merged PR #24.

## Open blockers / NEEDS-HUMAN
- None. No `STOP`. `cargo audit` is RED (the `time` CVE) — that's backlog A2, not a wall; it only blocks `develop`→`main` promotion until fixed.

## Decisions & dead-ends (don't re-litigate)
- Auto-merge is **fail-closed** via branch protection (required checks), NOT by merging blindly — `main` was unprotected, so protection was added. Do not merge red.
- No `--admin`/direct merges to `main` — classifier-enforced; bootstrap of the gate workflow used an auto-merged PR (#24).
- Harness changes were direct-pushed to `develop` during bootstrap; from now on everything goes via PR `--base develop`.
- `main`'s harness docs lag `develop` until the first clean promotion (post-A2) — expected; the loop reads skills/CLAUDE from `develop` worktrees.

## Verify-on-resume (run FIRST; confirm green before new work)
```bash
cargo run --quiet --bin rusty-idd -- validate          # fail-closed: CRITICAL -> non-zero (expect 0 critical / ~11 warning)
bash .claude/skills/merge-verification/scripts/drift-check.sh .   # exit 0
rtk cargo fmt --all -- --check
rtk cargo clippy --workspace --all-targets --all-features -- -D warnings
rtk cargo test --workspace --locked                    # expect 429 passed (baseline; only grows)
```
Last known-green: 429 tests / drift 0 / fmt+clippy clean at `1029091` (develop adds only docs on top — no code change).
