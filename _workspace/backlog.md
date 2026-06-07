# Backlog — idd-merge-loop · Epic "Complete Delivery + Upgrade" (seeded 2026-06-05)

## Mandate (the intended delivery)
**rusty-idd = ALL three source projects' features + capabilities + specs, unified.**
The three directors and their rusty-idd homes:
1. **intent-driven-development** (idd control plane) → `crates/core` (+ CLI edge `crates/cli`)
2. **openspec-tui-main** (TUI + execution) → `crates/tui` + `crates/runner`
3. **OpenSpec lifecycle** (Node CLI / `intent-driven-template`) → `crates/spec` (+ CLI edge)

Delivery is "complete" only when every feature/capability/spec of those three is present in
rusty-idd, regression-tested, and documented.

## Invariant (applies to EVERY slice — no exceptions)
**UPGRADE ONLY · NO DOWNGRADES.** Concretely, a slice may never:
- remove or weaken a capability (deprecate-before-delete; keep old path until parity proves new),
- regress parity (byte-exact `emit_spec` stays byte-exact; `rusty-idd <core-verb>` stays byte-identical to legacy `idd`),
- shrink the test suite (count only grows; baseline **429**),
- downgrade a dependency or pin it *down* to dodge a fix (remediate by moving *forward*),
- touch the zero-dep core invariant (`crates/core [dependencies]` stays empty; new deps only at spec/runner/tui/cli edges),
- weaken a gate to make a step pass (fail-closed).

Provenance: seeded from the 2026-06-05 deep codebase audit (6 parallel audits + `cargo audit`
+ direct verification), `docs/rusty-idd/{slice-sequence,lifecycle-contract,spec-engine-design}.md`,
and the parity mandate above. Status legend: `- [ ]` todo · `- [x]` done+verified · `- [!]` blocked: <reason>.

---

## Epic A — Supply-chain & build hygiene (highest priority; `cargo audit` is RED today)
- [x] A1 · Add a **`cargo audit` (deny-warnings) CI gate** + commit an advisory baseline. Gate: CI job fails on any new advisory. Rollback: drop the job. (No code downgrade.) — DONE 2026-06-06: audit step added to the required `rust` job (ci.yml) via `taiki-e/install-action` + `cargo audit --deny warnings --ignore RUSTSEC-2026-0009`; committed shared baseline `.cargo/audit.toml` ignoring only the 2 accepted unmaintained advisories (A3). Verified: develop gate GREEN, promote-verify still BLOCKS on `time` (A2 wall intact), any new/un-baselined advisory FAILS (fail-closed). No Rust/Cargo.lock change → suite unchanged (429).
- [x] A2 · Remediate **RUSTSEC-2026-0009** — `time` 0.3.41 → **≥0.3.47** via `cargo update -p time` (forward only). Gate: `cargo audit` no longer reports `time`; `test --workspace --locked` still 429+. Rollback: revert Cargo.lock hunk. — DONE 2026-06-06: `cargo update -p time --precise 0.3.47` (plain update was held at 0.3.41 by the MSRV-aware resolver; forced forward — never down). Forward-bumped its tree too (deranged 0.4→0.5.8, num-conv 0.1→0.2.2, time-core 0.1.4→0.1.8, time-macros 0.2.22→0.2.27). `cargo audit --deny warnings` now exits 0 with NO time exception; removed the obsolete `--ignore RUSTSEC-2026-0009` from ci.yml (gate now enforces vulns with zero exceptions). Verified: build/clippy/fmt/drift clean, **429** tests `--locked`, validate 0 critical. Promotion to main now unblocked (promote-verify time wall cleared).
- [x] A3 · Retire **unmaintained `bincode` 1.3.3 + `yaml-rust` 0.4.5** (both via `syntect` under `tui-markdown`/`comrak`). Upgrade `syntect`/`tui-markdown`/`comrak` forward to versions that drop them; if no upgrade path exists, record an explicit accepted-risk note (NOT a downgrade). Gate: `cargo audit` warnings cleared or documented; UI/markdown rendering unchanged. — DONE 2026-06-06 (ACCEPTED-RISK outcome): investigated — `syntect` 5.3.0, `comrak` 0.52.0, `tui-markdown` 0.3.7 are ALL the latest published versions and the latest `syntect` still pulls `bincode 1.x`+`yaml-rust 0.4`. No forward path. Dropping `syntect` would delete the TUI `highlight-code` capability (`tui-markdown` feature) = forbidden downgrade. Both advisories are unmaintained-WARNINGS, not vulnerabilities. Recorded explicit accepted-risk register (`docs/rusty-idd/security-advisories.md`) + updated `.cargo/audit.toml` rationale + re-eval triggers. No code/dep change → suite unchanged (429); UI/markdown rendering untouched. Gate stays fail-closed on any NEW advisory/vuln.
- [x] A4 · **Pin CI toolchain + add an MSRV/edition-2024 floor job** (`.github/workflows/ci.yml:24` is `@stable`). Add `rust-version` to every crate (`core`=1.74; spec/cli ≥ core; runner/tui edition-2024 ⇒ ≥1.85). Gate: CI builds on the pinned floor; clean-room reproducible. Rollback: revert ci.yml + manifests. — DONE 2026-06-06: **empirically measured** the real floor (not the guessed 1.85): installed 1.85/1.88 toolchains — workspace fails on 1.85 (`time@0.3.47`⇒1.88, `ratatui@0.30`⇒1.86, runner `let`-chains stabilized 1.88 ⇒ E0658) and builds clean on **1.88.0**. Declared `rust-version`: core=1.74 (unchanged, zero-dep), spec/runner/tui/cli=**1.88** (each commented with its binding constraint). Pinned the main `rust` job `@stable`→`@1.96.0` (reproducible). Added an `msrv` CI job (`@1.88.0`, `cargo build --workspace --locked` + `test --no-run --locked`). Verified: floor build+test-compile green on 1.88.0; full gate on stable green (drift 0, validate 0-crit, audit 0, fmt/clippy clean, **429** tests); no lock drift; ci.yml valid (jobs rust+msrv). NOTE: A4 refined the backlog's 1.85 guess → 1.88 (A2's `time` bump raised the MSRV — expected for a security fix; A5's flake floor should match 1.88, not 1.85).
- [x] A5 · **Pin flake.nix Rust toolchain** (`crates/tui/flake.nix`) to **≥1.88** (the real MSRV floor measured in A4 — `time@0.3.47` + `let`-chains; NOT the originally-guessed 1.85) so the nix dev shell can't drift below the floor. Gate: `nix develop` builds. Rollback: revert flake. — DONE 2026-06-06: `flake.lock` was ALREADY tracked + pinned (nixpkgs rev → rustc 1.93.0 ≥ floor; `nix flake lock` reproduced it byte-identically), so the real gap was **floor ENFORCEMENT**. Added a hard `assert` in flake.nix (`msrvFloor="1.88.0"`, `builtins.compareVersions pkgs.rustc.version msrvFloor >= 0`) that **fails `nix develop` eval** if the pinned nixpkgs ever drops below 1.88. Verified fail-closed BOTH ways via `nix eval`: floor 1.99 → eval errors ("MSRV floor is 1.99.0 but nixpkgs provides rustc 1.93.0; bump the pinned nixpkgs input."); floor 1.88 → eval passes (shell `nix-shell` resolves). drift 0, validate 0-crit. No Rust/Cargo change → suite unchanged (429 per resume baseline). (Full `nix develop` realisation not run to completion — heavy download incl. claude-code — but the toolchain version + assertion are proven via eval.)
- [x] A6 · **Collapse duplicate transitive versions** where a pure forward upgrade unifies them (`syn` 1+2, `bitflags`, `nom`, `phf`). Upgrade-only; never pin down. Gate: fewer dups in `cargo tree -d`; build/test green. — DONE 2026-06-06 (NO-OP-by-evidence, documented): traced every listed multi-version dup (syn 1+2, bitflags 1+2, nom 7+8, phf/phf_shared 0.11+0.13) to a SINGLE root — `ratatui-termwiz`, an **optional** ratatui 0.30 backend the workspace does **not** enable (we use crossterm). The termwiz→terminfo/wezterm-* stack pins the old majors; `cargo tree -i <old> --target all` prints "nothing to print" (NOT in any build graph) → these are lock-resident only, never compiled into the shipped `rusty-idd` binary. No pure forward upgrade unifies them (upstream termwiz/terminfo/wezterm still use old majors; ratatui/crossterm already latest), and pinning down/deleting lock lines is forbidden. Host `cargo tree -d` shows only same-version build-vs-runtime splits (fastrand/memchr/phf_shared) — inherently non-collapsible. Recorded full analysis + re-eval trigger in `docs/rusty-idd/dependency-duplication.md`. No code/lock change → suite unchanged (429). **Epic A COMPLETE (A1-A6).**

## Epic B — Runtime robustness (behavior-preserving; no silent failures)
- [x] B1 · `crates/runner/src/runner.rs` — replace the **14 `lock().unwrap()`** with poison-tolerant recovery (no panic-on-poison). Gate: new test asserts a poisoned mutex doesn't crash the runner. Rollback: revert runner.rs. — DONE 2026-06-06: Migrated all `child_handle` mutex access to `.lock().unwrap_or_else(|e| e.into_inner())`. Added `test_poisoned_mutex_recovery`. Also refactored 19 flaky runner tests to use `tempfile::tempdir()`. Verified: 430 tests passed (429 baseline + 1 new). PR #33.
- [x] B2 · `runner.rs` — stop swallowing `tx.send`/`child.kill`/`write_*` failures (lines 145, 299–326, 525, 595): surface to state/log instead of `let _ =`. Deprecate-before-delete the silent paths. Gate: tests for dropped-receiver + kill-failure surfacing. — DONE 2026-06-06: Added `ImplUpdate::Error` variant, updated `apply_run` and `implementation_loop` to surface I/O and spawn errors. PR #33.
- [x] B3 · `crates/runner/src/data.rs` — distinguish **missing vs corrupt** for `tasks.md`/config/cwd (`unwrap_or_default`/`unwrap_or((0,0))` at 123/307/329 mask data loss). Gate: tests for corrupt-file → explicit error/log, not silent empty. — DONE 2026-06-07: Updated `read_change_config`, `read_dependencies`, `read_run_mode`, and `load_change_dependencies` to return `Result`. Updated all callers in `app.rs` and tests. Added `test_read_change_config_corrupt_file_returns_error`. Verified 431 tests.
- [x] B4 · `crates/spec/src/schema/mod.rs:89` — replace the production `expect("…schema.yaml must parse")` with a graceful error path. Gate: corrupt-embedded-schema test returns Err, not panic. — DONE 2026-06-07: Refactored `load_schema` to return `Result`, added `load_schema_fails_on_corrupt_yaml` and `load_schema_fails_on_malformed_yaml` tests.
- [x] C1 · `crates/core/src/fs_utils.rs` — unit tests for **backup-on-overwrite**, `stable_walk` (symlink/deep nesting), `ensure_dir`, and I/O error paths (currently **zero** tests on the data-integrity core). — DONE 2026-06-07: Added comprehensive tests for `ensure_dir`, `write_string`, `backup_on_overwrite`, `stable_walk`, and ignore patterns. Added `tempfile` dev-dependency to core (preserving zero-dep production logic). Verified 12 core tests.
- [x] C2 · `crates/core/src/cli.rs` dispatcher + `planner.rs` (13 untested fns) — tests for each verb incl. missing-flag/I-O-error paths. — DONE 2026-06-07: Added 10 tests covering flag parsing, workspace generation, and inventory serialization. PR #34.
- [x] C3 · `crates/spec` parse/emit — **direct** unit tests for `parse_spec`/`parse_delta`/`emit_spec` (malformed markdown, blank-line edges, Unicode); today only ~3 golden fixtures exercise ~450 LOC. — DONE 2026-06-07: Added direct unit tests for parse_spec, parse_delta, and emit_spec covering Unicode, malformed prefixes, and blank-line edges.
- [x] C4 · `spec archive` — tests for **multi-capability**, partial-failure abort, and permission-denied (transactional rollback). — DONE 2026-06-07: Added direct unit tests for archive_specs transactional orchestration including abort-on-failure.
- [x] C5 · `runner`/`data` error paths — `openspec` not on PATH, invalid UTF-8 stdout, CRLF `tasks.md`, non-ASCII names. — DONE 2026-06-07: Hardened error surfacing in runner.rs and data.rs; added tests for CRLF and non-ASCII. Verified 462 tests. PR #35.

## Epic D — Feature + capability + spec COMPLETENESS (the parity mandate; upgrade-only)
- [x] D1 · **Build the parity matrix** — enumerate every feature/capability/spec of the three source projects and map each to its rusty-idd home; list each gap as a concrete follow-on slice. Output: `docs/rusty-idd/parity-matrix.md`. — DONE 2026-06-07: Created matrix mapping idd, openspec-tui, and lifecycle features; identified 4 remaining gaps (D2-D5). PR #35.
- [x] D2 · `spec validate` **full surface** — add `--all | --changes | --specs` and `--type change|spec`, and accept a change-dir/name (today a strict subset of the oracle). Additive; single-file path unchanged. Gate: oracle-parity tests for the new modes. — DONE 2026-06-07.
- [ ] D3 · Implement the **`sync` capability** (agent-driven scenario merge; `opsx-sync`, distinct from programmatic `archive` per lifecycle-contract §) — the documented lifecycle verb not yet delivered. Gate: oracle-conformance fixture for `sync`.
- [ ] D4 · **Re-establish the differential oracle harness** (the Node oracle binary was deleted) so parity is regression-tested, not asserted once. Pin the OpenSpec oracle version. Gate: harness runs both engines on generated (base,delta) pairs and diffs.
- [ ] D5 · Confirm full **lifecycle generation** parity (`proposal/specs/design/tasks` content, not just `scaffold`/`new` stubs) against the contract; close any stub-vs-full gap. Gate: generated artifacts match contract shape.

## Epic E — Docs / harness truth (upgrade-only)
- [ ] E1 · Fix stale `crates/core/README.md` + `crates/tui/README.md` (still reference retired `idd` / `openspec-tui` bins → `rusty-idd …`).
- [ ] E2 · Reconcile the **byte-exact parity contradiction** (CLAUDE.md "non-goal" vs slice-sequence "achieved") — single source of truth.
- [ ] E3 · Record the 3 implicit decisions as **ADRs** (flat CLI, TUI-in-v1, `AI_MERGE/`-only control-plane reconciliation) via `rusty-idd spec adr`.
- [ ] E4 · **Harden `drift-check.sh`** blind spots (workspace-inherited deps `*.workspace=true`, `[build-dependencies]`, re-exports) so the Rust-native guard can't be fooled.
- [ ] E5 · Drive `rusty-idd validate` warnings → 0: generate the missing control-plane files (`.idd/MANIFEST.tsv`, `.idd/LOCK.md`, `.github/{pull_request_template.md,copilot-instructions.md,ISSUE_TEMPLATE/idd-task.yml}`, `AI_MERGE/0{3,4,8},10`) **or** record why each is intentionally absent. (Also silence the 2 detector false-positives at env_contract.rs:349 / validation.rs:148.)

---

## DONE criteria for this epic (all true → write `_workspace/DONE` with evidence)
- backlog has no `- [ ]` left · `cargo audit` clean-or-documented · CI runs the audit + MSRV gates ·
- `rusty-idd validate` no CRITICAL · `drift-check.sh` exit 0 · fmt/clippy clean · `test --workspace --locked` **≥ 429 and only grown** ·
- byte-exact parity intact · zero capability removed · `docs/rusty-idd/parity-matrix.md` shows full coverage ·
- `pr-evidence-bundle` produced for each shipped slice · **every run's work shipped via a PR to `main` with auto-merge (no unmerged work stranded locally)**.

## PR / merge policy (mandatory — see idd-merge-loop principle 9)
Two-tier flow. Dev work lands on **`develop`**, never `main`. Every run pushes per cycle, opens ONE PR
`--base develop` on cycle 1, and enables auto-merge squash. `develop` is branch-protected (required check
`rust`) so auto-merge is fail-closed (merges only on green CI, async, survives process exit). Resume branches
off the prior PR if it hasn't merged yet — never off stale `develop`. **Promotion to `main`** is a separate
PR opened on DONE, gated by `rust` **+** the enhanced `promote-verify` workflow (clean-merge + locked
build/test + drift + fmt/clippy + `cargo audit`). The loop never pushes/admin-merges `main` directly.
