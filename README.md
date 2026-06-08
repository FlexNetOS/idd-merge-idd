# rusty-idd

**rusty-idd** — one Rust-native binary that unifies three intent-driven development tools: the IDD control plane, the OpenSpec lifecycle engine, and a task execution TUI. Built for the FlexNetOS org.

## Quick start

```bash
cargo install rusty-idd-cli
rusty-idd --help
```

## What it does

| Capability | Command | Source |
|------------|---------|--------|
| Repository scan / plan / manifest | `rusty-idd scan`, `plan`, `manifest` | `crates/core` (zero-dep std-only) |
| OpenSpec lifecycle (`validate`, `archive`, `sync`) | `rusty-idd spec validate`, `archive`, `sync` | `crates/spec` (comrak + serde_norway) |
| Headless task runner | `rusty-idd run <change>` | `crates/runner` |
| Interactive terminal UI | `rusty-idd tui` | `crates/tui` (ratatui + crossterm) |

## Workspace layout

A Cargo workspace producing a single `rusty-idd` binary:

- **`crates/core/`** — std-only control-plane logic for repo unification, manifesting, validation (was `intent-driven-development`).
- **`crates/runner/`** — the task-execution engine + OpenSpec data layer (split out of the TUI).
- **`crates/tui/`** — the ratatui OpenSpec TUI (was `openspec-tui-main`).
- **`crates/spec/`** — the OpenSpec lifecycle engine ported to Rust (parse / validate / transactional archive — no Node).
- **`crates/cli/`** — **`rusty-idd`**, the unified clap binary wiring everything together.

## Build & test

```bash
cargo build --workspace
cargo test --workspace
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

CI gates (blocking): Rust-native drift check, build, test, fmt, clippy, and `cargo audit`. See `.github/workflows/ci.yml` and `.github/workflows/promote-verify.yml`.

## Recommended workflow

1. Review `AGENTS.md`, `crates/core/README.md`, and `docs/rusty-idd/`.
2. Use the `rusty-idd` CLI; validate with `rusty-idd validate`.
3. Keep agent tasks narrow; follow the integration branch model (`develop` → promotion PR to `main`).

## Notes

- No real secrets in repo — use `.env.example`, `.env.schema.example.json`, and approved secret backends.
- Deprecated old behavior is preserved until parity tests prove migration success (deprecate-before-delete).
