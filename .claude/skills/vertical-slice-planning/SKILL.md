---
name: vertical-slice-planning
description: "Turns merge inventory + contracts into ONE narrow, parity-backed, reviewable slice with migration intent, parity tests, validation gates, and a rollback path — then emits a narrow idd task. ALWAYS use when planning a merge, deciding what to migrate first, decomposing a unification, or scoping a single PR-sized increment. Use this when the answer must be 'one slice', not 'a big plan'."
---

# Vertical-Slice Planning

Convert facts into an executable sequence, then one increment. The PR-level rule from `AGENTS.md` holds: **one slice, one intent, one validation path per PR.** But a large restructure (building the `rusty-idd` workspace, porting the lifecycle engine) is an **epic**: sequence it first, then plan only the next slice in detail. Refusing the epic is as wrong as planning everything at once.

## Why this exists
Cloud and local agents merge safely only in small, serialized, test-backed steps. The planner's value is choosing the *next* increment that is independently validatable, and specifying exactly how its correctness will be proven and undone — and, for a big restructure, the order the increments must come in.

## Epic first (for a restructure)
When the work is a multi-crate restructure, write the slice sequence to `_workspace/02_planner_epic.md` before detailing slice #1. A typical rusty-idd sequence:
1. **Workspace skeleton** (structural) — root virtual `Cargo.toml` + `resolver = "3"`; gate: empty workspace builds.
2. **Fold in existing crates** (structural) — move `intent-driven-development` → `crates/core` (or `crates/idd`) and `openspec-tui-main` → `crates/tui` + `crates/runner`; gate: `cargo build/test --workspace` green; drift-check still clean (core `[dependencies]` empty).
3. **Port the lifecycle** (lifecycle-port) — build `crates/spec` from the `lifecycle-porter`'s design; gate: golden-fixture conformance vs `npx openspec`.
4. **Integrate** (migration) — `crates/cli` wires the unified subcommands; gate: parity with the prior per-tool behaviors.

## Slice types — choose the correctness gate
Declare each slice's type so QA verifies the right thing:
- **Migration** (default) — old path → new path; gate = parity tests.
- **Structural** — create/move crates; no behavior to preserve; gate = `cargo build/test --workspace` green + nothing deleted that wasn't moved.
- **Lifecycle-port** — no old Rust path; gate = golden-fixture conformance vs the `npx openspec` oracle.

## Procedure

### 1. Read the evidence
Load `_workspace/01_analyst_inventory.md`, `01_analyst_feature_matrix.md`, `01_analyst_env_secret_contract.md`, and `01_analyst_baseline.md`. Plan only on evidence; if it is missing, request it from `merge-analyst` first.

### 2. Select the NEXT slice
Pick the next increment that is independently validatable and valuable (or structurally necessary). For an epic, it's the next item in `02_planner_epic.md`; otherwise the smallest behavior-preserving migration. Tag its **type** (migration / structural / lifecycle-port).

### 3. Verify Rust-native feasibility (at the right scope)
The slice must be implementable in Rust-native code. The **core crate** stays std-only; spec/runner/tui crates may use their researched deps (comrak, serde_norway, ratatui, clap). A foreign-language file, or a dep on the *core* crate, is a signal to **re-scope** — flag it. A dep on a non-core crate is expected, not drift.

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
# Slice: <one-line title>   (type: migration | structural | lifecycle-port)

## Scope (in / out)
- In: <the single thing this slice does>
- Out: <explicitly excluded — guards against creep>

## Intent (old → new, or structural move, or fixture target)
- migration:     Old path <file/symbol> → New path <file/symbol>; old stays callable until parity proven
- structural:    Move <from> → <to> (a move, not a delete); old path removed only after it builds at the new path
- lifecycle-port: Build <crates/spec module>; correctness = conformance to <fixture set>

## Correctness gate (declare exactly what QA verifies)
- migration:     parity tests — <names/assertions: old vs new same result>
- structural:    `cargo build/test --workspace` green; nothing deleted that wasn't moved
- lifecycle-port: golden-fixture conformance vs `npx openspec` (validate --json / archive)

## Validation gates (must pass before PR)
- drift-check.sh clean (core crate stays zero-dep; all src trees .rs-only)
- cargo fmt --check / clippy -D warnings / test --all --locked (affected crate, or --workspace post-restructure)
- idd validate (no critical findings) + manifest refreshed if control-plane files changed

## Rollback path
- <exact steps/commit to revert this slice safely>

## Conflicts touched
- <none, or note for AI_MERGE/05_conflict_risk_register.md>

## Sequence position
- <this is slice N of the epic in 02_planner_epic.md; next: …>
```

## Principles
- Additive before destructive: deprecate, never delete, within a slice.
- One integration branch has merge authority; scope the slice to land cleanly on it.
- Hand the parity tests + gates to `merge-qa` up front so the pass criteria are agreed before implementation.
