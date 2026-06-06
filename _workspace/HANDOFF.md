# HANDOFF — idd-merge-loop

_Checkpoint written: 2026-06-06. Authoritative cold-start resume signal (committed to disk, not the weave inbox)._

## Status: CYCLING — 3 cycles done this session (A1, A2, A3). Cycle budget (3) reached → handed off.
Epic A is 50% complete: **A1, A2, A3 all `[x]` + verified.** Next session RESUMEs and runs **cycle 1 at backlog item A4**.

## Resume command
```
/idd-merge-loop resume from _workspace/HANDOFF.md
```
Unattended: `bash .claude/skills/idd-merge-loop/scripts/ralph-idd.sh` (SAFE) · `IDD_APPLY=1 bash .../ralph-idd.sh` (apply) · `touch _workspace/STOP` (kill switch).

## What shipped this session
- **A1** (PR **#26**, MERGED → develop `4b5cba2`): fail-closed `cargo audit` gate added to the required `rust` CI job (`taiki-e/install-action` + `cargo audit --deny warnings`); shared baseline `.cargo/audit.toml`.
- **A2** (PR **#27**, MERGED → develop `2bfcb4f`): `time` 0.3.41 → **0.3.47** (forward, `--precise`; remediates RUSTSEC-2026-0009). Removed the obsolete `--ignore` from the develop gate. Cleared the promote-verify `time` wall.
- **A3** (this checkpoint's PR — OPEN, auto-merge squash armed; PR# recorded in `loop_state.md` open_pr): bincode/yaml-rust **accepted-risk** (no upgrade path; syntect 5.3.0 latest still pulls them, and dropping `syntect` would remove the TUI `highlight-code` capability = forbidden downgrade). Both are unmaintained-warnings, not vulns. Recorded `docs/rusty-idd/security-advisories.md` + `.cargo/audit.toml` rationale. Bundles this `HANDOFF.md`.

## This session's open PR (reconcile BEFORE picking a base)
- **A3 = the run PR** (`syntect-unmaintained` → develop), **auto-merge squash enabled**, fail-closed on required `rust`. On resume: **sync, then check it.** If **merged** → branch A4 off the advanced `origin/develop`. If **still open + green/pending** → branch A4 off `syntect-unmaintained` (stack). If **red** → fix that first (or `NEEDS-HUMAN`), not A4.
- A1/A2 PRs (#26/#27) already merged — do not reopen.

## ⚠️ Race lesson baked into policy (loop_state.md "Race lesson" + pr_policy)
A1+A2 were first stacked as 2 commits on ONE auto-merging PR; CI greened the 1st commit and GitHub merged the PR before the 2nd commit landed, stranding A2. Recovered via cherry-pick onto a fresh branch. **RULE: ONE PR PER CYCLE.** Never add a commit to a PR that already has auto-merge armed. If the prior cycle's PR is still open, branch the next cycle off ITS branch (stack); else off the advanced develop.

## Backlog (truth = `_workspace/backlog.md`, 25 slices)
Done: **A1, A2, A3** `[x]`. **Next item: A4** · pin CI toolchain + add MSRV/edition-2024 floor job (ci.yml uses `@stable`; add `rust-version` to every crate: core=1.74, spec/cli ≥ core, runner/tui edition-2024 ⇒ ≥1.85). Then A5 (pin flake.nix toolchain ≥1.85), A6 (collapse duplicate transitive versions). Then Epics B→C→D→E.
Mandate: rusty-idd = all 3 source projects unified. **Invariant: UPGRADE ONLY / NO DOWNGRADES** (suite only grows from 429; no dep downgraded; core stays zero-dep; gates never weakened).

## Open blockers / NEEDS-HUMAN
- None. No `STOP`. `cargo audit` is GREEN (time fixed; 2 unmaintained warnings accepted-risk). **develop→main promotion is now UNBLOCKED** (the time wall cleared) — but promotion only happens on full backlog DONE, not now.

## Decisions & dead-ends (don't re-litigate)
- A3 is accepted-risk by design — do NOT try to force-drop syntect (would delete TUI highlight-code = downgrade). Re-evaluate only when syntect publishes a release dropping bincode/yaml-rust (triggers in security-advisories.md).
- `.cargo/audit.toml` is SHARED (read by both ci.yml and promote-verify.yml). Never put a *vulnerability* there — only accepted unmaintained-warnings. Vulns get fixed forward or tolerated via a per-workflow `--ignore` flag, never the shared file.
- `origin` (drdave-flexnetos/idd-merge-idd) is a redirect alias for the canonical `FlexNetOS/idd-merge-idd` (one repo, not a fork). `gh`/`git` both resolve there; gh PR head OID can lag a few seconds after push — re-check via `git ls-remote` / the refs API if it looks stale.

## Verify-on-resume (run FIRST; confirm green before new work)
```bash
cargo run --quiet --bin rusty-idd -- validate          # expect 0 critical / 11 warning
bash .claude/skills/merge-verification/scripts/drift-check.sh .   # exit 0
cargo audit --deny warnings                             # exit 0 (accepted-risk baseline; fails on any NEW advisory)
rtk cargo fmt --all -- --check
rtk cargo clippy --workspace --all-targets --all-features -- -D warnings
rtk cargo test --workspace --locked                     # expect 429 passed (baseline; only grows)
```
Last known-green on develop after A1+A2: 429 tests / drift 0 / fmt+clippy clean / audit clean. A3 changes no compiled input.
