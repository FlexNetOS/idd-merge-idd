# rusty-idd — Epic Slice Sequence

The build order the `merge-orchestrator` harness executes (one slice per loop of orchestrator Phases 3–5). This is the `02_planner_epic.md` the planner will own at runtime; captured here as the plan of record. Each slice names its **type** and **correctness gate** so `merge-qa` verifies the right thing.

| # | Slice | Type | Status | Correctness gate | Reverts to |
|---|-------|------|--------|------------------|------------|
| 0 | **Lifecycle contract + spec-engine design** | analysis | ✅ done (PR #5) | `lifecycle-contract.md` + `spec-engine-design.md` + oracle fixtures exist | — |
| 1 | **Workspace skeleton** | structural | ✅ done (PR #5) | workspace builds; `resolver="3"`; `drift-check.sh` clean | delete root `Cargo.toml` |
| 2 | **Fold in `core`** (`intent-driven-development` → `crates/core`) | structural | ✅ done | `cargo build/test --workspace` green; core `[dependencies]` still empty | `git mv` back |
| 3 | **Fold in `tui`** (`openspec-tui-main` → `crates/tui`) | structural | ✅ done | `--workspace` green (multi-threaded); editions preserved (tui=2024) | `git mv` back |
| 3a | **CI relocate + upgrade** (`→ root .github/`, workspace-aware + drift gate) | structural | ✅ done | GitHub runs it; drift+build+test blocking; fmt/clippy non-blocking | restore old path |
| 3b | **Fix tui CWD-race flake** (serialize `set_current_dir` tests) | fix | ✅ done | `cargo test --workspace` green multi-threaded | revert data.rs |
| 5 | **fmt + clippy cleanup** (format the tree; flip CI fmt/clippy to blocking) | refactor | ✅ done | `cargo fmt --all --check` + `clippy --workspace -D warnings` clean; CI blocking | — |
| 4 | **Split `crates/tui` → `crates/runner` + `crates/tui`** | refactor | ✅ done | `crates/runner` lib (runner/config/data); tui depends on it; `--workspace` green | re-merge |
| 6 | **Port lifecycle → `crates/spec`** | lifecycle-port | ✅ done | semantic golden conformance (parse→merge→emit→re-parse == oracle `03`); validate JSON matches `04`/`05`; transactional merge | drop `crates/spec` |
| 7 | **Unified `crates/cli`** (`rusty-idd`) | migration | ✅ done | `rusty-idd scan` byte-identical to `idd scan`; spec validate/archive + headless run + tui launcher; old bins still work | keep old entrypoints |
| 8 | **Retire old entrypoints + oracle** | migration | ✅ done | `idd`/`openspec-tui` bins removed (core/tui now libs); `rusty-idd` sole binary; Cargo.lock 100% Rust (zero Node) | restore shims |

**🎉 Epic complete.** rusty-idd is one Rust-native Cargo workspace: `crates/{core, runner, tui, spec, cli}` producing the single `rusty-idd` binary. All three directors unified (idd control plane + OpenSpec lifecycle + tui execution), no Node in the product.

## Notes
- **Slices 2–3 kept each existing crate whole** (a directory move, not a code refactor). `openspec-tui` became `crates/tui` as one crate; the `runner`/`tui` split (slice 4) is a genuine refactor, not a move, so it was deferred rather than smuggled into a structural slice.
- **The fmt/clippy cleanup (slice 5)** is ~1700 lines of mechanical reformat plus clippy fixes across both crates (assembled from separate upstreams; never linted together). It is its own reviewable PR; until then root CI runs fmt/clippy **non-blocking**.
- **Slice 6 (`crates/spec`)** built the engine CORE: pure `model` + comrak `parse`/`emit` + transactional `merge` (MODIFIED=whole-block) + `validate` + `archive` orchestration, with semantic golden tests. The once-deferred edges are now all built by the **upgrades+fixes epic**: `schema/` artifact DAG (U7, `spec status`/`next`), `adr/` supersession graph (U8, `spec adr`), `scaffold/` minijinja stubs (U9, `spec new`/`scaffold`), and `cli/` (clap, in `crates/cli`). **Byte-exact** parity with the oracle is now achieved too (U6) via a tightening emitter — no longer a non-goal; both a byte-exact and a semantic golden test gate it.
- **Deprecate-before-delete** holds across the epic: old per-tool entrypoints stay callable until slice 7 proves parity; slice 8 removes them only after.

## Upgrades + fixes epic (post-unification, PRs #13–#21)
Maintenance + completion epic on the unified workspace. Fixes: U1 flake.nix retarget (#13), U2 `validate --strict` summary/exit reconcile (#14), U3 runner `serde_yaml`→`serde_norway` (#15), U4 `archive --no-validate`/`-y` wiring (#16). Upgrades: U5 RENAME+MODIFY op-order pinned to the oracle (#17), U6 byte-exact `emit_spec` (#18), U7 `schema/` edge (#19), U8 `adr/` edge (#20), U9 `scaffold/` edge (#21). Every slice shipped gate-green (build/test/clippy/fmt/drift).

## Open sequencing question
If the TUI must ship in v1 (design.md open question), the `crates/tui` integration comes before the unified-CLI cutover.
