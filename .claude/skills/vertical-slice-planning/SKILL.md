---
name: vertical-slice-planning
description: "Turns merge inventory + contracts into ONE narrow, parity-backed, reviewable slice with migration intent, parity tests, validation gates, and a rollback path — then emits a narrow idd task. ALWAYS use when planning a merge, deciding what to migrate first, decomposing a unification, or scoping a single PR-sized increment. Use this when the answer must be 'one slice', not 'a big plan'."
---

# Vertical-Slice Planning

Convert facts into one executable increment. The governing rule from `AGENTS.md`: **one vertical slice, one migration intent, one validation path per PR.** A plan that touches everything is a failed plan — your discipline is saying no to scope.

## Why this exists
Cloud and local agents merge safely only in small, serialized, test-backed steps. The planner's value is choosing the *smallest behavior-preserving slice that still delivers value and can be validated independently*, and specifying exactly how its correctness will be proven and undone if needed.

## Procedure

### 1. Read the evidence
Load `_workspace/01_analyst_inventory.md`, `01_analyst_feature_matrix.md`, `01_analyst_env_secret_contract.md`, and `01_analyst_baseline.md`. Plan only on evidence; if it is missing, request it from `merge-analyst` first.

### 2. Select ONE slice
Pick the smallest increment that is: behavior-preserving, independently validatable, and valuable on its own. Prefer slices that resolve a flagged conflict or establish a canonical interface both sides can migrate onto. If several are needed, list the rest as a backlog but **plan only the first**.

### 3. Verify Rust-native feasibility
The slice must be implementable in Rust-native code (std-only for `idd`). If it appears to need another language or a new `idd` dependency, that is a signal to **re-scope**, not to plan around it — flag it to the leader.

### 4. Specify the slice
Fill the template below completely. Migration intent and rollback are not optional — a slice you cannot undo is not ready.

### 5. Emit a narrow task
Record the slice as an `idd task` so it can drive a single PR/issue:
```bash
# from intent-driven-development/
rtk cargo run --bin idd -- task --out <ws>/AI_MERGE/07_tasks --kind <kind> --title "<one-line slice title>"
```

## Slice template — write to `_workspace/02_planner_slice.md`
```markdown
# Slice: <one-line title>

## Scope (in / out)
- In: <the single behavior being migrated>
- Out: <explicitly excluded — guards against creep>

## Migration intent (old → new)
- Old path: <file/symbol> → New path: <file/symbol>
- Deprecation: old path stays callable until parity proven (no deletion this slice)

## Parity tests (proof old behavior survives)
1. <test name / assertion — old vs new produce same result>
2. ...

## Validation gates (must pass before PR)
- drift-check.sh clean (Rust-native invariant)
- cargo fmt --check / clippy -D warnings / test --all --locked (affected crate)
- idd validate (no critical findings) + manifest refreshed if control-plane files changed

## Rollback path
- <exact steps/commit to revert this slice safely>

## Conflicts touched
- <none, or note for AI_MERGE/05_conflict_risk_register.md>

## Backlog (not planned this round)
- <future slices, if any>
```

## Principles
- Additive before destructive: deprecate, never delete, within a slice.
- One integration branch has merge authority; scope the slice to land cleanly on it.
- Hand the parity tests + gates to `merge-qa` up front so the pass criteria are agreed before implementation.
