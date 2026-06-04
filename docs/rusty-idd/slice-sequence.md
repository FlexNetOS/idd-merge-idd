# rusty-idd — Epic Slice Sequence

The build order the `merge-orchestrator` harness executes (one slice per loop of orchestrator Phases 3–5). This is the `02_planner_epic.md` the planner will own at runtime; captured here as the plan of record. Each slice names its **type** and **correctness gate** so `merge-qa` verifies the right thing.

| # | Slice | Type | Owner(s) | Correctness gate | Reverts to |
|---|-------|------|----------|------------------|------------|
| 0 | **Lifecycle contract + spec-engine design** | (analysis) | lifecycle-porter | `01_lifecycle_contract.md` + `05_lifecycle_design.md` exist and cover validate/archive | — |
| 1 | **Workspace skeleton** | structural | rust-implementer | empty workspace builds; `resolver="3"`; `drift-check.sh` clean | delete root `Cargo.toml` |
| 2 | **Fold in `core`** (`intent-driven-development` → `crates/core`) | structural | rust-implementer | `cargo build/test --workspace` green; core `[dependencies]` still empty | `git mv` back |
| 3 | **Fold in `runner`+`tui`** (`openspec-tui-main` → `crates/runner`,`crates/tui`) | structural | rust-implementer | `--workspace` green; TUI runs; editions preserved (tui=2024) | `git mv` back |
| 4 | **Port lifecycle → `crates/spec`** | lifecycle-port | lifecycle-porter + rust-implementer | golden-fixture conformance vs `npx openspec` (`validate --json`, `archive`) | drop `crates/spec` |
| 5 | **Unified `crates/cli`** | migration | rust-implementer | parity: each `rusty-idd <verb>` matches the prior per-tool behavior | keep old entrypoints |
| 6 | **Retire old entrypoints + Node oracle** | migration | rust-implementer | parity proven for all verbs; no Node in shipped product; manifest refreshed | restore shims |

## Notes
- **Slice 0 runs in parallel with the analyst's inventory** (Phase 2) — it's the missing constructive-port work the new `lifecycle-porter` owns.
- **Slices 1–3 are low-risk** structural moves; do them first to make the workspace real before the hard port.
- **Slice 4 is the crux** and the only high-effort piece; it is test-driven by the oracle fixtures captured in Slice 0.
- **Deprecate-before-delete** holds across the whole epic: old per-tool entrypoints stay callable until Slice 5 proves parity; Slice 6 removes them only after.
- Every slice ships as its own PR with the full `pr-evidence-bundle` (build/test/lint/secret-scan/drift verdict/migration note/rollback).

## Open sequencing question
Slices 5–6 assume the unified CLI is the cutover point. If the TUI must ship in v1 (design.md open question), insert a `crates/tui` integration slice between 4 and 5.
