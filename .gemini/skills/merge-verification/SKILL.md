---
name: merge-verification
description: "Runs the full merge gate for idd-merge-idd: Rust-native drift detection, parity preservation, env/secret-contract and manifest coherence, CI gates (fmt/clippy/test), and secret scanning — by reading BOTH sides of each boundary. ALWAYS use when verifying a merge slice, QA-ing a change, checking for language/dependency drift, or deciding whether a slice is PR-ready. Use this and not a generic 'run the tests' when the change is part of a repo merge/migration."
---

# Merge Verification

Verify a merge slice the way boundary bugs actually appear: a change passes every isolated check yet breaks where two things connect. The method is **read both sides, then run the gate** — never judge one side alone.

## Why this exists
For this repo the highest-value boundary is the **Rust-native invariant** (the std-only core crate carries zero deps; every crate's `src/` is `.rs`-only). Agent tooling can silently introduce a foreign-language file or a new dependency "to get the slice done." That is the drift this skill is built to catch — and the rule is to **port to Rust-native**, not adopt the foreign artifact.

The unification *restructures* the tree (two top-level crates → a `rusty-idd/crates/*` workspace). The gate is **layout-agnostic** so it keeps working across that restructure — earlier hardcoded paths went blind exactly when crates moved.

## Verification order (highest priority first)
Run in this order; stop and report if a higher-priority gate fails, because lower ones become moot.

### 1. Rust-native drift (deterministic — use the bundled script)
Run `scripts/drift-check.sh` from the repo root. It discovers crates by their `Cargo.toml` (works at the repo root *or* under `crates/*`) and checks:
- every discovered crate `src/` tree is `.rs`-only — and **errors loudly if it finds no src trees at all** (so a restructure can't make it pass vacuously),
- the **zero-dep core crate** (`crates/core` / `crates/idd` / today `intent-driven-development`) keeps its *own* `[dependencies]` table empty — parsed from the crate manifest, **not** a lockfile package count (the workspace lockfile legitimately grows once sibling crates have deps),
- no stray foreign package manifests (`.omc`, `*.ecc`, `package.json`, `pyproject.toml`, `go.mod`, `requirements.txt`) outside the whitelisted asset trees (`intent-driven-template/`, `.gemini/`, `.github/`, `docs/`, `openspec/`, `.opencode/`, `.agents/`, which are legitimately non-Rust).

```bash
bash .gemini/skills/merge-verification/scripts/drift-check.sh .
```
Exit 1 ⇒ drift **or unverifiable**. Do not proceed: send the implementer a fix request to **port the foreign logic to Rust** (std-only for the core crate). A new dependency on the **core** crate is drift unless the planner's slice spec recorded an explicit justification; deps on the spec/runner/tui crates are expected and fine.

### 2. Parity — or the slice's declared correctness gate (read both sides)
Most slices are **migration slices**: open the **old path** and the **new path** together and confirm:
- the old path still exists and is callable (deprecate-before-delete honored — nothing was deleted during migration),
- the parity tests named in `_workspace/02_planner_slice.md` exist and pass,
- behavior matches (same inputs → same outputs across old/new).
A migration that deletes the old path before parity is proven is a fail, even if tests are green.

Two slice types replace parity with a different correctness gate (the planner declares which in the slice spec — verify against *that*):
- **Structural slice** (e.g. create the workspace, move a crate into `crates/*`): there is no behavior to preserve. Gate = **the whole workspace still builds and all prior tests pass** (`cargo build/test --workspace`), and nothing was deleted that wasn't moved.
- **Lifecycle-port slice** (the OpenSpec engine → `crates/spec`): there is no old Rust path. Gate = **conformance to the golden fixtures** captured from the `npx openspec` oracle (`validate --json` and `archive` outputs match). Treat a fixture diff as the parity failure.

### 3. Contract & manifest coherence (read both sides)
- Env/secret contract: compare keys in `AI_MERGE/03_env_and_secret_contracts.*` against actual `env::var(...)`/secret references in the code. A key in code but absent from the contract (or vice-versa) is a mismatch.
- Manifest: after any change to generated control-plane files, `.idd/MANIFEST.tsv` must match reality. Run `idd manifest --workspace <ws>` and confirm it reports no surprising additions/changes.

### 4. CI gates (local must equal CI)
Mirror `.github/workflows/ci.yml` exactly. Run at the workspace root with `--workspace`:
```bash
# at the workspace root (members: crates/core, crates/tui)
rtk cargo fmt --all -- --check
rtk cargo clippy --all-targets --all-features -- -D warnings
rtk cargo test --all --locked          # --workspace once the root Cargo.toml exists
```
Local green but a CI step red (e.g. unformatted code, a clippy warning, `Cargo.lock` drift under `--locked`) is a fail — CI treats warnings as errors.

### 5. Secret hygiene
- No real secret values committed. `idd validate --workspace <ws>` scans for leaked-secret patterns, committed `.env` files, and dangerous workflow shapes; a **critical** finding makes it exit non-zero.
- Confirm secrets are mapped as *references*, not materialized.

## Incremental, not end-of-line
Run this gate **after each module the implementer completes**, not once at the very end. Early boundary mismatches propagate into later modules and get more expensive to fix.

## Report format
Write `_workspace/04_qa_report.md` exactly in this shape:

```markdown
# Merge QA Report — <slice name>

| # | Boundary check | Verdict | Evidence | Fix request |
|---|----------------|---------|----------|-------------|
| 1 | Rust-native drift | pass/fail/unverified | <cmd output> | <file:line + how, if fail> |
| 2 | Parity / declared gate (old↔new, or build-green, or fixture-conformance) | ... | ... | ... |
| 3 | Contract/manifest | ... | ... | ... |
| 4 | CI gates | ... | ... | ... |
| 5 | Secret hygiene | ... | ... | ... |

## Summary
passed: N  failed: N  unverified: N
PR-ready: yes/no
```

Rules: every row gets a verdict of **pass**, **fail**, or **unverified** — never blank, never "looks fine". `unverified` is for a gate that could not be run (record why); it is not a pass. PR-ready is **yes** only when all rows pass.
