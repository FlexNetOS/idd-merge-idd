# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Harness: Merge Dev Operation

**Goal:** Build **rusty-idd** — unify the three directors into one Rust-native Cargo workspace — by turning each merge/migration intent into a reviewable, Rust-native slice with full PR evidence.

**Trigger:** For any merge/migration/unification dev operation in this repo — building rusty-idd, the workspace restructure, porting the OpenSpec lifecycle to Rust, scanning repos, sequencing the epic, planning a slice, implementing a migration, QA-ing a change, checking Rust-native drift, or assembling merge-PR evidence (and follow-ups: re-run, refine the plan/epic, redo only the implementation/QA/lifecycle-port) — use the `merge-orchestrator` skill. For **autonomous, multi-slice, or cross-session runs** — working the slice backlog one item per cycle, running unattended, resuming after a handoff, or self-restarting with `/new` — use the `idd-merge-loop` skill (the durable-backlog + continuity layer on top of the orchestrator; pairs with `session-relay` + the `continuity-steward` agent). Simple one-off questions may be answered directly.

**Change history:**
| Date | Change | Target | Reason |
|------|--------|--------|--------|
| 2026-06-04 | Initial setup | All (4 agents, 6 skills) | - |
| 2026-06-04 | Add `lifecycle-porter` agent + `lifecycle-porting` skill; generalize `drift-check.sh` (layout-agnostic, core-crate dep check); add epic/slice-type layer to planner+orchestrator; re-scope Rust-native invariant to the core crate | agents, skills, CLAUDE.md | Research found the harness was aimed at the current 2-crate snapshot; retargeted it to *build rusty-idd* (workspace restructure + Node→Rust lifecycle port) |
| 2026-06-04 | Execute epic slices 2–3: `intent-driven-development`→`crates/core`, `openspec-tui-main`→`crates/tui`; relocate+upgrade CI to root `.github/` (workspace-aware, drift gate, fmt/clippy non-blocking); fix tui CWD-race flake; refresh layout docs | repo layout, CI, CLAUDE.md, docs/rusty-idd | Continue the rusty-idd unification; the drift gate's retarget proved out (root lock 234 pkgs, core still zero-dep) |
| 2026-06-04 | Slice 5: fmt + clippy cleanup across both crates; flip CI fmt/clippy to blocking | both crates, CI | Workspace now fully lint-clean; CI fully enforcing |
| 2026-06-04 | Slice 4: split `crates/tui` → `crates/runner` (runner/config/data lib) + `crates/tui` (UI); tui re-exports runner modules | crates, Cargo manifests | So the future `crates/cli` can reuse the execution layer without ratatui |
| 2026-06-04 | Slice 6: build `crates/spec` (the OpenSpec lifecycle engine) — pure model + comrak parse/emit + transactional merge + validate + archive; semantic golden tests vs oracle fixtures | new crate, root Cargo | The crux: Rust-native lifecycle (no Node in the product); byte-exact parity a documented non-goal |
| 2026-06-04 | Slice 7: build `crates/cli` (`rusty-idd`) unifying core (delegated, byte-identical) + spec (validate/archive/show + FS edge) + headless run + tui; split `crates/tui` into lib+bin so the loop is callable | new crate, tui lib split | The unified superflow binary; old `idd`/`openspec-tui` bins kept until slice 8 |
| 2026-06-04 | Slice 8 (final): retire the `idd` and `openspec-tui` bins (core/tui are now libs); point docs/run-commands at `rusty-idd`; confirm zero Node in the product | core/tui manifests, docs | rusty-idd unification epic complete — one Rust-native binary |
| 2026-06-04 | Package/lib rename for coherence: `intent-driven-development`→`rusty-idd-core`, `openspec-runner`→`rusty-idd-runner`, `openspec-tui`→`rusty-idd-tui` (libs `rusty_idd_*`); fix slice-8-stale `--bin idd` refs in skills | all crate manifests + code refs + skills | The whole workspace is now `rusty-idd-*`; directory names unchanged |
| 2026-06-05 | "Upgrades + fixes" epic via merge-orchestrator (PRs #13–#21): U1 flake.nix retarget, U2 `validate --strict` summary/exit reconcile, U3 runner `serde_yaml`→`serde_norway`, U4 `archive --no-validate/-y` wiring, U5 RENAME+MODIFY op-order pinned to oracle (found+fixed an inverted merge), U6 byte-exact `emit_spec`, U7/U8/U9 deferred spec edges `schema`/`adr`/`scaffold` (`spec status/next/adr/new/scaffold`) | crates/{cli,spec,runner,tui}, docs/rusty-idd, fixtures 06–08 | Post-unification maintenance + completion of the designed-but-deferred lifecycle engine; every slice gate-green |
| 2026-06-05 | Add autonomous-operation layer: `idd-merge-loop` skill (durable backlog, one slice/cycle, commit-per-cycle, handoff at budget) + `session-relay` skill (HAND OFF/RESUME) + `continuity-steward` agent + `_workspace/.gitignore` (commit only continuity files) + external runner `scripts/ralph-idd.sh` (opt-in `IDD_APPLY=1`) | new agent + 2 skills + runner, CLAUDE.md | Harness upgrade kit: run the merge/port work unattended + resume cold across sessions/restarts with zero loss |
| 2026-06-05 | Branching model + mandatory auto-merge: loop pushes per cycle and ships every run as a PR `--base develop` with **auto-merge** (no work stranded, no next-run conflict); `develop` = integration branch (protected, required check `rust`), **`main` = protected trunk** reached only via `develop`→`main` promotion PR gated by `rust` + new `.github/workflows/promote-verify.yml` (clean-merge + locked build/test + drift + fmt/clippy + `cargo audit`); default branch → `develop`; repointed `idd-merge-loop`/`session-relay`/`ralph-idd.sh`/CLAUDE.md | skills + runner + new workflow + repo settings/protection | User mandate: "the loop must create a PR with automerge or we lose work / the next loop conflicts" → dev work off `main` onto `develop`, `main` enhanced-verified |

## Session start protocol (mandatory)

**Branching model:** dev work lands on **`develop`** (the integration branch — protected, required check `rust`, auto-merge on). **`main` is the protected release trunk** and takes `develop` only via a promotion PR gated by `rust` **+** the enhanced `promote-verify` workflow (clean-merge + locked build/test + drift + fmt/clippy + `cargo audit`). **Never push or admin-merge `main` directly.** `develop` is the default branch.

1. **Sync first.** `rtk git fetch --all` then confirm the working branch is level with `origin/develop` (`rtk git status -sb`). Do not start work on a stale tree.
2. **Work in a fresh git worktree, every session.** This repo is the integration root; never mutate it from an ad-hoc checkout. Create an isolated worktree off the synced base before touching code:
   ```bash
   rtk git worktree add ../idd-<task-slug> -b <task-slug> origin/develop
   ```
   Use the `EnterWorktree` tool when available; otherwise the command above. One vertical slice per worktree, per `AGENTS.md` rule 4. Open the slice's PR `--base develop` with auto-merge. Remove the worktree when the slice is merged.
3. Read `AGENTS.md` (operating rules) and, for `idd`/lifecycle work, `crates/core/README.md` and `docs/rusty-idd/` before changing control-plane behavior.

## Repository shape

This **is** a Cargo workspace now (the rusty-idd unification, in progress — see `docs/rusty-idd/`). The root `Cargo.toml` is a virtual manifest (`resolver = "3"`) with members under `crates/`. Mixed editions coexist per-crate.

| Path | Kind | What it is |
|------|------|-----------|
| `crates/core/` | Rust **lib** `rusty_idd_core`, edition 2021, **zero-dep / std-only** | The core control-plane logic (was `intent-driven-development`; its `idd` bin was retired — use `rusty-idd <verb>`). Generates the AI-merge **control plane** (markdown + JSON contracts, CI gates, agent templates). `cli::run` is what `crates/cli` delegates to. |
| `crates/runner/` | Rust lib `rusty_idd_runner`, edition 2024 | The non-UI execution layer split out of the TUI: `runner` (spawn an agent CLI, stream progress, stall detection, batch), `data` (parse `tasks.md`, list changes), `config` (`TuiConfig`). Consumed by `crates/tui` and `crates/cli`. |
| `crates/tui/` | Rust **lib** `rusty_idd_tui`, edition 2024 | ratatui/crossterm TUI (was `openspec-tui-main`; its `openspec-tui` bin was retired — use `rusty-idd tui`). The UI layer (`app`, `ui`); the run loop is `rusty_idd_tui::run()`, called by `rusty-idd tui`. Depends on `crates/runner`. |
| `crates/spec/` | Rust lib, edition 2021 (`rusty_idd_spec`) | The OpenSpec lifecycle engine ported to Rust. Hexagonal: a **pure** `model/` (Requirement/Scenario/SpecDoc/Delta + transactional `apply_delta` merge — MODIFIED is whole-block replace), with comrak (`parse`/`emit`), serde (`validate` JSON), and `archive` orchestration at the edges. The CLI/FS edge lives in `crates/cli`. Design: `docs/rusty-idd/spec-engine-design.md`; verified vs `docs/rusty-idd/oracle-fixtures/`. |
| `crates/cli/` | Rust crate, edition 2021 (**`rusty-idd` bin**) | The unified CLI (clap). Flat core verbs `init/scan/plan/task/validate/manifest/github` **delegate verbatim** to `core::cli::run` (byte-identical to `idd`); `spec validate/archive/show` wraps the spec engine (incl. the transactional archive + dir move); `run` is a headless task runner over `crates/runner`; `tui` calls `rusty_idd_tui::run()`. The sole binary — the old `idd` and `openspec-tui` bins were retired in slice 8. |
| `intent-driven-template/` | **Not code** — template assets | OpenSpec scaffolding: `.agents/`, `.opencode/`, `openspec/` schema/templates. The lifecycle being ported into `crates/spec`. |

### `crates/core` (`idd`) architecture

CLI dispatch lives in `crates/core/src/cli.rs` (`run(args)` matches subcommands: `init`, `scan`, `plan`, `task`, `validate`, `manifest`, `github`). Flags are parsed by a hand-rolled `parse_flags` (`--k v` and `--k=v`) — no `clap` here (the future unified `crates/cli` will use clap). The pipeline is `scanner` (walks a repo → `RepoInventory` in `model.rs`) → `planner` (renders inventories + merge plan + tasks) → `templates` (static `&str` bodies) → `validation` (severity-graded; **critical findings make `idd validate` exit non-zero**) → `manifest` (deterministic `.idd/MANIFEST.tsv`). `env_contract.rs` detects env/secret keys. All file writes go through `fs_utils::write_string_preserving_existing` (**backup-on-overwrite**).

### `crates/tui` (`rusty-idd-tui`) + `crates/runner` architecture

**`crates/tui`** is the UI layer: `main.rs` boots the terminal; `app.rs` holds the screen state machine; `ui.rs` renders. It depends on **`crates/runner`** for everything non-UI and re-exports its modules at the crate root (so `crate::config`/`crate::data`/`crate::runner` still resolve in `app.rs`/`ui.rs`).

**`crates/runner`** (`rusty_idd_runner` lib) is the execution layer: `data.rs` parses `tasks.md` progress and lists changes; `config.rs` holds `TuiConfig` (persisted to `openspec/tui-config.yaml`); `runner.rs` spawns the external agent in a worker thread, streams `ImplUpdate` over an mpsc channel, kills children via a shared `Arc<Mutex<Option<Child>>>` + `AtomicBool` cancel flag, and stalls after `STALL_THRESHOLD` (3) no-progress runs. Logic is covered by inline `#[cfg(test)]` modules. (CWD-mutating tests are serialized by a `CWD_GUARD` mutex in `data.rs`.)

## Build, test, lint

Run at the **workspace root**. CI lives at `.github/workflows/ci.yml` (workspace-aware):

```bash
rtk cargo build --workspace
rtk cargo test --workspace                  # all tests
rtk cargo test -p rusty-idd-tui <name>       # a single test in one crate
rtk cargo test --workspace --locked         # CI mode — fails on Cargo.lock drift
rtk cargo fmt --all -- --check              # enforced in CI
rtk cargo clippy --workspace --all-targets --all-features -- -D warnings   # enforced in CI (warnings = errors)
```

> All CI gates are **blocking**: the Rust-native drift gate, `build --workspace`, `test --workspace`, `fmt --check`, and `clippy -D warnings`. The workspace is fully fmt/clippy-clean as of the cleanup slice. See `docs/rusty-idd/slice-sequence.md`.

Notes:
- **One binary: `rusty-idd`** (`crates/cli`). The old `idd` and `openspec-tui` bins were retired in slice 8; those crates are now libs. Run a core verb: `cargo run --bin rusty-idd -- scan --repo <path>` (byte-identical to the former `idd scan`); spec engine: `rusty-idd spec validate|archive|show`; execution: `rusty-idd run <change>` or `rusty-idd tui`.
- `crates/core` is edition 2021 (MSRV 1.74); `crates/runner`/`crates/tui` are **edition 2024** (e.g. `let ... && let ...` chains in `runner.rs`). The workspace sets `resolver = "3"` to carry both.
- `rusty-idd tui` (the former `openspec-tui`) must run from a directory containing an `openspec/` folder; `crates/tui/flake.nix` is the documented nix dev shell.
- **No Node in the shipped product.** The OpenSpec lifecycle is native Rust (`crates/spec`). The Node OpenSpec CLI was only ever a dev-time conformance oracle (`bunx`, captured into `docs/rusty-idd/oracle-fixtures/`); the `rusty-idd` binary's dependency tree is 100% Rust.

## Rust-native invariant (critical — verify, don't assume)

The constraint is scoped to the **zero-dependency core crate** — `crates/core` (`rusty-idd-core`, formerly `intent-driven-development`): **Rust-native, std-only, no network calls** — its `[dependencies]` table stays empty. This is a design principle (`README.md` → "Design principles"), not an accident. The **other crates carry their researched dependencies** — `crates/tui` uses ratatui/crossterm, `crates/spec` uses comrak/serde, `crates/cli` uses clap. What never changes: no crate's `src/` may contain non-Rust source, and the core crate stays dependency-free. The invariant is the *core crate*, not the whole workspace.

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
- **`develop` is the integration branch** (auto-merge on green `rust`); other branches are research/disposable. Keep PRs to one vertical slice, `--base develop`. **`main` is the protected release trunk** — reached only via a `develop`→`main` promotion PR gated by `rust` **+** `promote-verify`; never pushed or admin-merged directly.
- Never commit real secrets. `idd` maps secret *references*, it does not materialize values — use `.env.example` / `.env.schema.example.json` / CI secret backends.
- Every PR that changes the control plane updates the relevant `AI_MERGE/` record and includes the required evidence (build/test/lint/secret-scan results, migration note, rollback path, manifest update or justification).
- If two agents conflict, stop and record it in `AI_MERGE/05_conflict_risk_register.md` before continuing.

## Harness: autonomous / resumable operation (upgrade path)

This repo's harness can be upgraded to **autonomous, resumable, self-restarting** operation:
a durable on-disk backlog → one item per cycle → hand off to a fresh session at a cycle budget
→ optional fully-unattended self-restart with a clean context each cycle ("/new" effect). Truth
lives on disk (backlog + checkpoints + commits) so any restart resumes cold with zero loss.

- Generic pattern + templates: `~/Desktop/meta/HARNESS-UPGRADE-KIT.md`
- Tailored kit for THIS repo:  `~/Desktop/meta/harness_hub/upgrade-kits/idd-merge-idd.md`
- Integrates with your existing skills: merge-orchestrator, merge-verification, pr-evidence-bundle (the loop drives them per cycle).
