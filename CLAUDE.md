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

## Session start protocol (mandatory)

1. **Sync first.** `rtk git fetch --all` then confirm the working branch is level with `origin/main` (`rtk git status -sb`). Do not start work on a stale tree.
2. **Work in a fresh git worktree, every session.** This repo is the integration root; never mutate it from an ad-hoc checkout. Create an isolated worktree off the synced base before touching code:
   ```bash
   rtk git worktree add ../idd-<task-slug> -b <task-slug> origin/main
   ```
   Use the `EnterWorktree` tool when available; otherwise the command above. One vertical slice per worktree, per `AGENTS.md` rule 4. Remove the worktree when the slice is merged.
3. Read `AGENTS.md` (operating rules) and, for `idd` work, `intent-driven-development/README.md` before changing control-plane behavior.

## Repository shape

This is **not** a Cargo workspace. It is an integration root bundling three independent projects (see `idd-merge-workspace.code-workspace`). Each Rust crate is built and tested **from its own directory** — there is no root `Cargo.toml`.

| Folder | Kind | What it is |
|--------|------|-----------|
| `intent-driven-development/` | Rust crate (`idd`) | The core CLI. Generates a repo-native AI merge **control plane** (markdown + JSON contracts, CI gates, GitHub agent templates). It is **not** an AI agent — it produces artifacts agents execute through issues/PRs. |
| `openspec-tui-main/` | Rust crate (`openspec-tui`) | A ratatui/crossterm TUI that browses OpenSpec changes and drives an external agent CLI (e.g. `claude`) to implement `tasks.md` task-by-task, with stall detection and dependency-ordered batch runs. |
| `intent-driven-template/` | **Not code** — template assets | OpenSpec scaffolding: `.agents/skills/`, `.opencode/` commands+skills, `openspec/` schemas/templates (markdown/YAML/JSON). No build step. |

### `intent-driven-development` architecture

CLI dispatch lives in `src/cli.rs` (`run(args)` matches subcommands: `init`, `scan`, `plan`, `task`, `validate`, `manifest`, `github`). Flags are parsed by a hand-rolled `parse_flags` (`--k v` and `--k=v`) — there is no `clap`. The pipeline is `scanner` (walks a repo → `RepoInventory` in `model.rs`, detecting languages, package managers, entrypoints, workflows, and secret refs) → `planner` (renders inventories + merge plan + tasks) → `templates` (static `&str` artifact bodies) → `validation` (severity-graded findings; **critical findings make `idd validate` exit non-zero**) → `manifest` (deterministic `.idd/MANIFEST.tsv` audit baseline). `env_contract.rs` detects env/secret keys across many providers. All file writes go through `fs_utils::write_string_preserving_existing` (**backup-on-overwrite** — never clobber prior artifacts).

### `openspec-tui` architecture

`main.rs` boots the terminal; `app.rs` holds the screen state machine and key handling; `ui.rs` renders; `data.rs` parses OpenSpec `tasks.md` progress; `config.rs` holds `TuiConfig` (command/prompt templates, persisted to `openspec/tui-config.yaml`); `runner.rs` spawns the external agent in a worker thread, streams `ImplUpdate` messages over an mpsc channel, kills children via a shared `Arc<Mutex<Option<Child>>>` + `AtomicBool` cancel flag, and stalls after `STALL_THRESHOLD` (3) no-progress runs. Most logic is covered by inline `#[cfg(test)]` modules — there is no separate `tests/` dir.

## Build, test, lint

Always run from the relevant crate directory. Mirror CI exactly (`intent-driven-development/.github/workflows/ci.yml`):

```bash
# from intent-driven-development/  OR  openspec-tui-main/
rtk cargo build
rtk cargo test                      # all tests
rtk cargo test <name_substring>     # a single test by name
rtk cargo test --all --locked       # CI mode — fails on Cargo.lock drift
rtk cargo fmt --all -- --check      # CI fails on unformatted code
rtk cargo clippy --all-targets --all-features -- -D warnings   # CI treats warnings as errors
```

Notes:
- `intent-driven-development` is edition 2021 (MSRV 1.74); `openspec-tui-main` is **edition 2024** — keep that in mind when writing syntax (e.g. `let ... && let ...` chains are used in `runner.rs`).
- `idd` end-to-end smoke: `cargo run --bin idd -- help`, or run the real flow per `intent-driven-development/README.md` ("Primary workflow").
- `openspec-tui` must be run from a directory containing an `openspec/` folder; `nix develop` (flake) is the documented dev shell.

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
