---
name: merge-verification
description: "Runs the full merge gate for idd-merge-idd: Rust-native drift detection, parity preservation, env/secret-contract and manifest coherence, CI gates (fmt/clippy/test), and secret scanning — by reading BOTH sides of each boundary. ALWAYS use when verifying a merge slice, QA-ing a change, checking for language/dependency drift, or deciding whether a slice is PR-ready. Use this and not a generic 'run the tests' when the change is part of a repo merge/migration."
---

# Merge Verification

Verify a merge slice the way boundary bugs actually appear: a change passes every isolated check yet breaks where two things connect. The method is **read both sides, then run the gate** — never judge one side alone.

## Why this exists
For this repo the highest-value boundary is the **Rust-native invariant** (the `idd` crate is std-only, zero deps; both crates are `.rs`-only). Agent tooling can silently introduce a foreign-language file or a new dependency "to get the slice done." That is the drift this skill is built to catch — and the rule is to **port to Rust-native**, not adopt the foreign artifact.

## Verification order (highest priority first)
Run in this order; stop and report if a higher-priority gate fails, because lower ones become moot.

### 1. Rust-native drift (deterministic — use the bundled script)
Run `scripts/drift-check.sh` from the repo root. It checks:
- both `src/` trees are `.rs`-only,
- `intent-driven-development/Cargo.lock` holds exactly 1 package (std-only),
- no stray foreign package manifests (`.omc`, `package.json`, `pyproject.toml`, `go.mod`, `*.ecc`, …) crept in near the root.

```bash
bash .claude/skills/merge-verification/scripts/drift-check.sh .
```
Exit 1 ⇒ drift. Do not proceed: send the implementer a fix request to **port the foreign logic to Rust** (std-only for `idd`). A new dependency is drift unless the planner's slice spec recorded an explicit justification.

### 2. Parity (read both sides)
Open the **old path** and the **new path** together. Confirm:
- the old path still exists and is callable (deprecate-before-delete honored — nothing was deleted during migration),
- the parity tests named in `_workspace/02_planner_slice.md` exist and pass,
- behavior matches (same inputs → same outputs across old/new).
A migration that deletes the old path before parity is proven is a fail, even if tests are green.

### 3. Contract & manifest coherence (read both sides)
- Env/secret contract: compare keys in `AI_MERGE/03_env_and_secret_contracts.*` against actual `env::var(...)`/secret references in the code. A key in code but absent from the contract (or vice-versa) is a mismatch.
- Manifest: after any change to generated control-plane files, `.idd/MANIFEST.tsv` must match reality. Run `idd manifest --workspace <ws>` and confirm it reports no surprising additions/changes.

### 4. CI gates (local must equal CI)
Run from the affected crate directory — there is no root workspace. Mirror `.github/workflows/ci.yml` exactly:
```bash
# in intent-driven-development/  or  openspec-tui-main/
rtk cargo fmt --all -- --check
rtk cargo clippy --all-targets --all-features -- -D warnings
rtk cargo test --all --locked
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
| 2 | Parity (old↔new) | ... | ... | ... |
| 3 | Contract/manifest | ... | ... | ... |
| 4 | CI gates | ... | ... | ... |
| 5 | Secret hygiene | ... | ... | ... |

## Summary
passed: N  failed: N  unverified: N
PR-ready: yes/no
```

Rules: every row gets a verdict of **pass**, **fail**, or **unverified** — never blank, never "looks fine". `unverified` is for a gate that could not be run (record why); it is not a pass. PR-ready is **yes** only when all rows pass.
