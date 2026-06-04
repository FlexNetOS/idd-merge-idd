# Proposal: rusty-idd — a Rust-native intent-driven dev superflow

> Authored in the `intent-driven` OpenSpec schema shape (proposal → specs → design → adr → tasks).
> The OpenSpec CLI is **not installed**; this is hand-authored Markdown. rusty-idd will validate its own specs once its lifecycle engine exists. See `docs/rusty-idd/design.md` for decisions and `slice-sequence.md` for the build order.

## Why

The repo bundles three "directors" that each govern one phase of the same intent→deliverable lifecycle, but they are separate tools in separate languages with separate control planes:

- `intent-driven-development` (`idd`) — Rust, std-only: the **merge/contract** control plane (scan→plan→task→validate→manifest).
- `openspec-tui-main` (`openspec-tui`) — Rust, ratatui: the **execution** runner (tasks.md task-by-task, stall detection, dependency-ordered batch).
- `intent-driven-template` — **not code**: the OpenSpec **intent-driven lifecycle** (proposal→specs→design→adr→tasks), which depends on the **Node/TypeScript OpenSpec CLI** — which is not even installed here.

The harness already built (the merge-dev agent team) is more capable than any one director alone, but it orchestrates *prose*; the deterministic work still lives in three disconnected tools, one of them non-Rust and non-functional locally. The goal is a single Rust-native binary — **rusty-idd** — that unifies all three into one "superflow," driven by the harness and sitting at the terminus of the mesh pipeline `user-request → prompt_hub → weave+rtk → rusty-idd`.

## What Changes

- **NEW**: `rusty-idd`, a Cargo **workspace** producing one binary, unifying the three directors' capabilities.
- **NEW**: a Rust-native **OpenSpec lifecycle engine** (`crates/spec`) — porting `proposal→specs→design→adr→tasks`, `validate`, and `archive`/delta-merge off the Node CLI. **BREAKING** for the template's workflow: rusty-idd replaces the `npx openspec` runtime dependency (Node retained only as a dev-time conformance oracle, then dropped).
- **CHANGE**: `intent-driven-development` becomes `crates/core` (+ contract/merge modules), preserving its std-only zero-dependency invariant.
- **CHANGE**: `openspec-tui-main` becomes `crates/runner` (+ `crates/tui`), folded in near-as-is.
- **NEW**: `crates/cli` — the single `[[bin]]`, wiring the unified subcommand surface (lifecycle + contract + run).

## Capabilities

New capabilities (each → a future `specs/<name>/spec.md`, kebab-case):

- `workspace-skeleton` — the rusty-idd Cargo workspace (virtual root, `resolver = "3"`, mixed editions per crate).
- `lifecycle-engine` — the ported proposal→specs→design→adr→tasks state machine + `validate` + `archive` delta-merge, in Rust.
- `contract-control-plane` — the `idd` scan/plan/env-secret-contract/manifest capabilities as workspace modules.
- `tasks-runner` — the tasks.md execution engine (stall detection, dependency-ordered batch, apply mode).
- `unified-cli` — the `rusty-idd <verb>` command surface over all of the above.

Modified capabilities: none yet (greenfield workspace; existing crates move, behavior preserved).

## Impact

- **Code**: new root `Cargo.toml`; `intent-driven-development/` and `openspec-tui-main/` move under `crates/`; new `crates/spec` + `crates/cli`. The Rust-native drift gate is already layout-agnostic (handles the move).
- **Dependencies**: `crates/core` stays zero-dep; new deps confined to edges — `comrak`, `serde_norway`, `clap`, ratatui/crossterm (existing), Tera/minijinja (scaffolding). No Node in the shipped product.
- **Connected projects**: rusty-idd is the terminus of `prompt_hub → weave+rtk → rusty-idd`; its CLI/contracts are the integration surface those upstreams target.
- **Risk**: the lifecycle port is the only high-effort piece (no mature Rust prior art; `oonid/OpenSpec-rs` is a nascent reference). Mitigated by the `npx openspec` golden-fixture oracle (differential testing) and by sequencing it after the low-risk structural moves.
- **Rollback**: each slice is independently revertable; the structural moves are `git mv`-level and reversible until `crates/cli` cuts over.
