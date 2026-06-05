# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Harness: Merge Dev Operation

**Goal:** Build **rusty-idd** — unify the three directors into one Rust-native Cargo workspace — by turning each merge/migration intent into a reviewable, Rust-native slice with full PR evidence.

**Trigger:** For any merge/migration/unification dev operation in this repo — building rusty-idd, the workspace restructure, porting the OpenSpec lifecycle to Rust, scanning repos, sequencing the epic, planning a slice, implementing a migration, QA-ing a change, checking Rust-native drift, or assembling merge-PR evidence (and follow-ups: re-run, refine the plan/epic, redo only the implementation/QA/lifecycle-port) — use the `merge-orchestrator` skill. Simple one-off questions may be answered directly.

**Change history:**
| Date | Change | Target | Reason |
|------|--------|--------|--------|
| 2026-06-04 | Initial setup | All (4 agents, 6 skills) | - |
| 2026-06-04 | Add `lifecycle-porter` agent + `lifecycle-porting` skill; generalize `drift-check.sh` (layout-agnostic, core-crate dep check); add epic/slice-type layer to planner+orchestrator; re-scope Rust-native invariant to the core crate | agents, skills, CLAUDE.md | Research found the harness was aimed at the current 2-crate snapshot; retargeted it to *build rusty-idd* (workspace restructure + Node→Rust lifecycle port) |
| 2026-06-04 | Execute epic slices 2–3: `intent-driven-development`→`crates/core`, `openspec-tui-main`→`crates/tui`; relocate+upgrade CI to root `.github/` (workspace-aware, drift gate, fmt/clippy non-blocking); fix tui CWD-race flake; refresh layout docs | repo layout, CI, CLAUDE.md, docs/rusty-idd | Continue the rusty-idd unification; the drift gate's retarget proved out (root lock 234 pkgs, core still zero-dep) |
| 2026-06-04 | Slice 5: fmt + clippy cleanup across both crates; flip CI fmt/clippy to blocking | both crates, CI | Workspace now fully lint-clean; CI fully enforcing |
| 2026-06-04 | Slice 4: split `crates/tui` → `crates/runner` (runner/config/data lib) + `crates/tui` (UI); tui re-exports runner modules | crates, Cargo manifests | So the future `crates/cli` can reuse the execution layer without ratatui |

## Session start protocol (mandatory)

1. **Sync first.** `rtk git fetch --all` then confirm the working branch is level with `origin/main` (`rtk git status -sb`). Do not start work on a stale tree.
2. **Work in a fresh git worktree, every session.** This repo is the integration root; never mutate it from an ad-hoc checkout. Create an isolated worktree off the synced base before touching code:
   ```bash
   rtk git worktree add ../idd-<task-slug> -b <task-slug> origin/main
   ```
   Use the `EnterWorktree` tool when available; otherwise the command above. One vertical slice per worktree, per `AGENTS.md` rule 4. Remove the worktree when the slice is merged.
3. Read `AGENTS.md` (operating rules) and, for `idd`/lifecycle work, `crates/core/README.md` and `docs/rusty-idd/` before changing control-plane behavior.

## Repository shape

This **is** a Cargo workspace now (the rusty-idd unification, in progress — see `docs/rusty-idd/`). The root `Cargo.toml` is a virtual manifest (`resolver = "3"`) with members under `crates/`. Mixed editions coexist per-crate.

| Path | Kind | What it is |
|------|------|-----------|
| `crates/core/` | Rust crate, edition 2021 (`idd` bin), **zero-dep / std-only** | The core CLI (was `intent-driven-development`). Generates the AI-merge **control plane** (markdown + JSON contracts, CI gates, agent templates). Not an AI agent — it produces artifacts agents execute. |
| `crates/runner/` | Rust lib, edition 2024 (`openspec_runner`) | The non-UI execution layer split out of the TUI: `runner` (spawn an agent CLI, stream progress, stall detection, batch), `data` (parse `tasks.md`, list changes), `config` (`TuiConfig`). Consumed by `crates/tui` and (later) `crates/cli`. |
| `crates/tui/` | Rust crate, edition 2024 (`openspec-tui`) | ratatui/crossterm TUI (was `openspec-tui-main`); the UI layer (`app`, `ui`, `main`). Depends on `crates/runner` for execution; re-exports its modules so `crate::config`/`crate::data`/`crate::runner` resolve. |
| `crates/spec/` | (planned, slice 6) | The OpenSpec lifecycle engine ported to Rust — see `docs/rusty-idd/spec-engine-design.md`. |
| `intent-driven-template/` | **Not code** — template assets | OpenSpec scaffolding: `.agents/`, `.opencode/`, `openspec/` schema/templates. The lifecycle being ported into `crates/spec`. |

### `crates/core` (`idd`) architecture

CLI dispatch lives in `crates/core/src/cli.rs` (`run(args)` matches subcommands: `init`, `scan`, `plan`, `task`, `validate`, `manifest`, `github`). Flags are parsed by a hand-rolled `parse_flags` (`--k v` and `--k=v`) — no `clap` here (the future unified `crates/cli` will use clap). The pipeline is `scanner` (walks a repo → `RepoInventory` in `model.rs`) → `planner` (renders inventories + merge plan + tasks) → `templates` (static `&str` bodies) → `validation` (severity-graded; **critical findings make `idd validate` exit non-zero**) → `manifest` (deterministic `.idd/MANIFEST.tsv`). `env_contract.rs` detects env/secret keys. All file writes go through `fs_utils::write_string_preserving_existing` (**backup-on-overwrite**).

### `crates/tui` (`openspec-tui`) + `crates/runner` architecture

**`crates/tui`** is the UI layer: `main.rs` boots the terminal; `app.rs` holds the screen state machine; `ui.rs` renders. It depends on **`crates/runner`** for everything non-UI and re-exports its modules at the crate root (so `crate::config`/`crate::data`/`crate::runner` still resolve in `app.rs`/`ui.rs`).

**`crates/runner`** (`openspec_runner` lib) is the execution layer: `data.rs` parses `tasks.md` progress and lists changes; `config.rs` holds `TuiConfig` (persisted to `openspec/tui-config.yaml`); `runner.rs` spawns the external agent in a worker thread, streams `ImplUpdate` over an mpsc channel, kills children via a shared `Arc<Mutex<Option<Child>>>` + `AtomicBool` cancel flag, and stalls after `STALL_THRESHOLD` (3) no-progress runs. Logic is covered by inline `#[cfg(test)]` modules. (CWD-mutating tests are serialized by a `CWD_GUARD` mutex in `data.rs`.)

## Build, test, lint

Run at the **workspace root**. CI lives at `.github/workflows/ci.yml` (workspace-aware):

```bash
rtk cargo build --workspace
rtk cargo test --workspace                  # all tests
rtk cargo test -p openspec-tui <name>       # a single test in one crate
rtk cargo test --workspace --locked         # CI mode — fails on Cargo.lock drift
rtk cargo fmt --all -- --check              # enforced in CI
rtk cargo clippy --workspace --all-targets --all-features -- -D warnings   # enforced in CI (warnings = errors)
```

> All CI gates are **blocking**: the Rust-native drift gate, `build --workspace`, `test --workspace`, `fmt --check`, and `clippy -D warnings`. The workspace is fully fmt/clippy-clean as of the cleanup slice. See `docs/rusty-idd/slice-sequence.md`.

Notes:
- `crates/core` is edition 2021 (MSRV 1.74); `crates/tui` is **edition 2024** (e.g. `let ... && let ...` chains in `runner.rs`). The workspace sets `resolver = "3"` to carry both.
- `idd` end-to-end smoke: `cargo run -p intent-driven-development --bin idd -- help`, or the real flow per `crates/core/README.md` ("Primary workflow").
- `openspec-tui` (`cargo run -p openspec-tui`) must run from a directory containing an `openspec/` folder; `crates/tui/flake.nix` is the documented nix dev shell.

## Rust-native invariant (critical — verify, don't assume)

The constraint is scoped to the **zero-dependency core crate** (today `intent-driven-development`/`idd`; in the target `rusty-idd` workspace, `crates/core`): **Rust-native, std-only, no network calls** — its `[dependencies]` table stays empty. This is a design principle (`README.md` → "Design principles"), not an accident. The **other crates are allowed their researched dependencies** — `openspec-tui` already uses ratatui/crossterm/serde, and the ported spec engine will use comrak/serde_norway/clap. What never changes: no crate's `src/` may contain non-Rust source, and the core crate stays dependency-free. The invariant is the *core crate*, not the whole workspace.

**Guard against language/dependency drift on every change:**

1. **Detect.** Before finishing any task, run the layout-agnostic drift gate (it survives the `crates/*` restructure — it discovers crates by their `Cargo.toml`, checks the core crate's own `[dependencies]`, not a lockfile count):
   ```bash
   bash .claude/skills/merge-verification/scripts/drift-check.sh .   # exit 0 = clean
   ```
   Be especially alert to agent tooling that *auto-generates a package in another language or format* (a stray `.omc`, an `ecc`-style auto-pushed package, a Node/Python helper, a new dependency **on the core crate**) and commits it as a convenience. Treat any such artifact as drift to be caught, not adopted. (Legitimately non-Rust asset trees — `intent-driven-template/`, `.claude/`, `.github/`, `docs/`, `openspec/` — are whitelisted by the gate.)
2. **Transform.** If foreign-language code, or a dependency on the **core** crate, has crept in, **port it to Rust-native** (`std` only for the core) rather than wrapping or shelling out — or relocate the dependency to the appropriate non-core crate edge. A dep on the core crate requires an explicit, recorded reason; the default answer is no. A dep on spec/runner/tui is expected, not drift.
3. **Sync.** After porting, re-run the CI triplet (`fmt --check`, `clippy -D warnings`, `test --all --locked` — add `--workspace` once the root manifest exists), refresh the affected `idd` manifest (`idd manifest --workspace <ws>`), and note the old→new path in the PR migration note.

## Merge/PR discipline (from AGENTS.md)

- Preserve working behavior: **deprecate before deleting**; keep old paths until parity tests pass. Migrations are additive first, destructive cleanup last.
- One integration branch has merge authority; other branches are research/disposable. Keep PRs to one vertical slice.
- Never commit real secrets. `idd` maps secret *references*, it does not materialize values — use `.env.example` / `.env.schema.example.json` / CI secret backends.
- Every PR that changes the control plane updates the relevant `AI_MERGE/` record and includes the required evidence (build/test/lint/secret-scan results, migration note, rollback path, manifest update or justification).
- If two agents conflict, stop and record it in `AI_MERGE/05_conflict_risk_register.md` before continuing.
