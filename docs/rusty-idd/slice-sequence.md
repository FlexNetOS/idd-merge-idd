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
| 4 | **Split `crates/tui` → `crates/runner` + `crates/tui`** | refactor | ⏳ next | runner is a lib both cli + tui consume; `--workspace` green | re-merge |
| 6 | **Port lifecycle → `crates/spec`** | lifecycle-port | ⏳ next big one | golden-fixture conformance vs `bunx openspec` (`validate --json`, `archive`) | drop `crates/spec` |
| 7 | **Unified `crates/cli`** | migration | ⏳ | parity: each `rusty-idd <verb>` matches the prior per-tool behavior | keep old entrypoints |
| 8 | **Retire old entrypoints + oracle** | migration | ⏳ | parity proven for all verbs; no Node in shipped product | restore shims |

## Notes
- **Slices 2–3 kept each existing crate whole** (a directory move, not a code refactor). `openspec-tui` became `crates/tui` as one crate; the `runner`/`tui` split (slice 4) is a genuine refactor, not a move, so it was deferred rather than smuggled into a structural slice.
- **The fmt/clippy cleanup (slice 5)** is ~1700 lines of mechanical reformat plus clippy fixes across both crates (assembled from separate upstreams; never linted together). It is its own reviewable PR; until then root CI runs fmt/clippy **non-blocking**.
- **Slice 6 is the crux** and the only high-effort piece; it is test-driven by the oracle fixtures captured in slice 0.
- **Deprecate-before-delete** holds across the epic: old per-tool entrypoints stay callable until slice 7 proves parity; slice 8 removes them only after.

## Open sequencing question
If the TUI must ship in v1 (design.md open question), the `crates/tui` integration comes before the unified-CLI cutover.
