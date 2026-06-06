# HANDOFF — idd-merge-loop

_Checkpoint written: 2026-06-06. Authoritative cold-start resume signal (committed to disk, not the weave inbox)._

## Status: EPIC A COMPLETE (A1–A6 all `[x]` + verified). Cycle budget (3) reached → handed off.
This session ran A4, A5, A6 (the prior session ran A1–A3). Next session RESUMEs and runs **cycle 1 at backlog item B1** (Epic B — runtime robustness).

## Resume command
```
/idd-merge-loop resume from _workspace/HANDOFF.md
```
Unattended: `bash .claude/skills/idd-merge-loop/scripts/ralph-idd.sh` (SAFE) · `IDD_APPLY=1 bash .../ralph-idd.sh` (apply) · `touch _workspace/STOP` (kill switch).

## What shipped (all merged to develop unless noted)
| Slice | PR | What |
|---|---|---|
| A1 | #26 ✅ | fail-closed `cargo audit` CI gate + shared `.cargo/audit.toml` |
| A2 | #27 ✅ | `time` 0.3.41→0.3.47 (RUSTSEC-2026-0009 fixed) |
| A3 | #28 ✅ | bincode/yaml-rust accepted-risk (`docs/rusty-idd/security-advisories.md`) |
| A4 | #29 ✅ | pin CI toolchain `@1.96.0` + `msrv` job `@1.88.0`; `rust-version` (core=1.74, rest=1.88) |
| A5 | #30 ✅ | flake.nix hard `rustc>=1.88` assertion (dev shell can't drift below floor) |
| A6 | **this PR — OPEN, auto-merge armed** | dup-versions = documented no-op (`docs/rusty-idd/dependency-duplication.md`); bundles this HANDOFF.md |

develop head before A6: `b07f7fb`. **A6 PR# is recorded in `loop_state.md` open_pr** (find via `gh pr list --head dedup-deps`).

## Reconcile the A6 PR BEFORE picking a base (one-PR-per-cycle rule)
- A6 is the run PR (`dedup-deps` → develop), auto-merge squash armed, fail-closed on `rust`. On resume: **sync, check it.** Merged → branch B1 off advanced `origin/develop`. Still open + green/pending → branch B1 off `dedup-deps` (stack). Red → fix that first (or `NEEDS-HUMAN`), not B1.
- A1–A5 PRs already merged — don't reopen.

## ⚠️ Hard-won policy (already baked into loop_state.md + pr_policy)
- **ONE PR PER CYCLE.** Never add a commit to a PR that already has auto-merge armed (fast CI merges it before the 2nd commit lands — A1/A2 hit this, recovered by cherry-pick). This session waited for each PR to merge, then branched the next cycle off clean develop — worked cleanly for A4→A5→A6.
- `origin` (drdave-flexnetos) is a redirect alias for canonical `FlexNetOS/idd-merge-idd` (one repo). gh PR head OID can lag a few seconds after push — re-check via `git ls-remote`/refs API if it looks stale.

## Backlog (truth = `_workspace/backlog.md`, 25 slices)
**Epic A DONE (A1–A6).** Next: **B1** · `crates/runner/src/runner.rs` — replace the 14 `lock().unwrap()` with poison-tolerant recovery (no panic-on-poison); test asserts a poisoned mutex doesn't crash. Then B2 (stop swallowing `tx.send`/`child.kill`/`write_*` failures), B3 (data.rs missing-vs-corrupt), B4 (spec schema `expect`→graceful). Then Epics C (tests), D (feature/spec completeness), E (docs/harness).
Mandate: rusty-idd = all 3 source projects unified. **Invariant: UPGRADE ONLY / NO DOWNGRADES** (suite only grows from 429; no dep downgraded; core stays zero-dep; gates never weakened).

## State of the supply chain after Epic A
- `cargo audit` clean (the only ignores are the 2 accepted unmaintained warnings from the optional/unused syntect path; `time` fixed). **develop→main promotion is UNBLOCKED** — but promotion only happens on full-backlog DONE (Epics B–E remain), so NO promotion PR yet.
- MSRV floor = **1.88** (edition 2024 + let-chains + time 0.3.47 + ratatui 0.30), enforced by per-crate `rust-version`, CI `msrv` job, and the flake assertion. core stays 1.74.
- Toolchain DETAIL for B-work: `crates/runner`/`tui` are edition 2024; runner.rs uses `let … && let …` chains (need rustc ≥1.88 — installed locally: 1.85.0, 1.88.0, 1.96.0).

## Open blockers / NEEDS-HUMAN
- None. No `STOP`. (A local `cargo audit` fetch hit a transient network error once; `--no-fetch` against the cached db is clean — it's a network blip, not a finding. CI runners fetch fine.)

## Verify-on-resume (run FIRST; confirm green before new work)
```bash
cargo run --quiet --bin rusty-idd -- validate          # expect 0 critical / 11 warning
bash .claude/skills/merge-verification/scripts/drift-check.sh .   # exit 0
cargo audit --deny warnings                             # exit 0 (retry/--no-fetch if a transient fetch error)
rtk cargo fmt --all -- --check
rtk cargo clippy --workspace --all-targets --all-features -- -D warnings
rtk cargo test --workspace --locked                     # expect 429 passed (baseline; only grows — B-work ADDS tests)
```
Last known-green on develop after A1–A5: 429 tests / drift 0 / fmt+clippy clean / audit clean. A6 changes no compiled input.
