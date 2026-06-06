---
name: lifecycle-porting
description: "How to port the OpenSpec intent-driven lifecycle (proposal→specs→design→adr→tasks + validate + archive/delta-merge) from the Node/TypeScript OpenSpec CLI into a Rust-native crate. Covers the extracted contract, the exact Rust crate toolkit (comrak / serde_norway / clap), the delta-merge algorithm, and the npx-openspec conformance oracle. ALWAYS use when designing or building rusty-idd's spec engine, reimplementing OpenSpec in Rust, or replacing the Node spec CLI. Use this rather than improvising crate choices."
---

# Lifecycle Porting

Reimplement the OpenSpec intent-driven lifecycle in Rust, faithfully and verifiably. This is a **constructive port** of a Node/TypeScript engine, not a migration of existing Rust — there is no old Rust path and the Node CLI is not installed, so correctness is proven by a conformance oracle, not by parity tests.

## Why this exists
The crate choices and the archive/delta-merge semantics are easy to get subtly wrong. Two of the obvious YAML crates are unsafe to use, and the obvious "Gherkin" crate is the wrong tool. This skill encodes the researched, load-bearing decisions so the porter doesn't relearn them by trial and error.

## Step 1 — Extract the lifecycle contract (source of truth)
The only specification of record is the schema + skill prose (the Node CLI is absent). Read:
- `intent-driven-template/openspec/schemas/intent-driven/schema.yaml` — the artifacts, their `instruction` bodies, the `requires:` edges, and the `apply` step.
- `intent-driven-template/.opencode/skills/openspec-*` and `.agents/skills/openspec-*` — the operational lifecycle (new/propose/continue/apply/verify/sync/archive).

Capture in `_workspace/01_lifecycle_contract.md`:
- Artifact DAG: `proposal → specs → design → adr → tasks` with exact `requires:` edges (specs⊃proposal; design⊃proposal; adr⊃design; tasks⊃specs,adr).
- Spec wrapper rules: `### Requirement: <name>` (≥1 scenario each), `#### Scenario: <name>` (exact four-hash heading), GIVEN/WHEN/THEN bodies.
- Delta operations: `## ADDED | MODIFIED | REMOVED | RENAMED Requirements` and their semantics (below).
- ADR rules: immutable once accepted; supersession via a `Supersedes:` field; files live in `<repo>/adr/`, never inside the change folder.

## Step 2 — The Rust crate toolkit (decided — do not improvise)
| Concern | Use | Avoid / why |
|---------|-----|-------------|
| CLI / subcommands | **`clap` v4 (derive)** for the lifecycle CLI (many subcommands). Fall back to `lexopt`/`pico-args` only if matching a no-clap house style. | Hand-rolling a full parser (the de-facto standard is clap). |
| YAML config + schema | **`serde_norway`** (maintained `serde_yaml` fork; mdBook ecosystem). | `serde_yaml` (archived 2024); `serde_yml` (**unsound — RUSTSEC-2025-0068**). |
| Markdown AST + delta-merge | **`comrak`** — true mutable AST + `format_commonmark` round-trip; ideal for splicing `### Requirement:` subtrees. | `pulldown-cmark` for the *merge* (event stream, no AST; round-trip via `pulldown-cmark-to-cmark` is lossy by design). pulldown is fine for read-only header scans (list/status). |
| Gherkin scenarios | **None** — scenarios are Markdown-wrapped; lint GIVEN/WHEN/THEN as text inside the comrak AST. | The `gherkin`/`cucumber` crate (parses `.feature` files — wrong format). Pull it in only to *execute* scenarios later. |
| Artifact scaffolding | **Tera** or **minijinja** (minijinja = lighter/fewer deps) to emit proposal/design/tasks/spec templates. | — |

Keep the *pure model* (requirement/scenario/delta structs + merge logic) dependency-light; confine `comrak`/`serde_norway`/`clap` to the crate edges. The zero-dep invariant is `crates/core`'s, not the spec engine's.

## Step 3 — The delta-merge / archive algorithm (the crux)
Archive applies a change's delta spec onto the base spec. Implement deterministically with comrak:
1. Parse the base `spec.md` to a comrak AST. Scan H3 nodes whose text starts with `Requirement:` → build `name → node-range` map.
2. For each delta section under `## ADDED | MODIFIED | REMOVED | RENAMED Requirements`, parse the delta block to an AST and splice:
   - **ADDED** → append the new `### Requirement:` subtree.
   - **MODIFIED** → **full-subtree replacement** (header text matched whitespace-insensitively). MODIFIED carries the *entire* updated requirement, not a partial diff — replacing partial content silently loses scenarios.
   - **REMOVED** → unlink the subtree; record **Reason** + **Migration** (per schema).
   - **RENAMED** → change heading text only (FROM:/TO:).
3. `format_commonmark` back to Markdown.
Test with golden files: `(base + delta) → expected merged`. Same input ⇒ byte-stable output.

## Step 4 — The conformance oracle (dev-only, then dropped)
The Node OpenSpec CLI is MIT and emits plain Markdown — near-zero lock-in — so use it transiently as a differential-testing oracle:
```bash
npx openspec@latest init                       # scaffold authoring layout (no global install)
npx openspec@latest validate --all --strict --json   # machine-readable validation oracle
npx openspec@latest archive <change>            # reference delta-merge output
```
Capture a fixture corpus: `(spec → validate-json)` and `(delta → archived)` pairs. Build rusty-idd's `validate`/`archive` to reproduce them, then delete Node entirely. If Node/`npx` is unavailable, hand-author from the documented spec format and mark the unverifiable rules — never assume.

## Output (write to `_workspace/`)
- `01_lifecycle_contract.md` — the extracted contract (DAG, wrapper rules, delta semantics, ADR rules).
- `05_lifecycle_design.md` — the `crates/spec` Rust design: modules, artifact state machine, the delta-merge algorithm, the crate matrix with rationale, and the fixture-based test plan.

## Principles
- Shipped product is 100% Rust-native; the oracle is scaffolding, not a dependency.
- Faithful over clever: match OpenSpec's documented semantics exactly (especially MODIFIED = whole-block replace), proven by golden fixtures.
- Reference `oonid/OpenSpec-rs` as a tractability check, never as a dependency.
