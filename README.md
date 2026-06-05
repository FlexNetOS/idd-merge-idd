# Intent Driven Development Merge Workspace

This workspace is designed to merge three complementary projects into one integrated repository for intent-driven development, automated AI implementation, and workflow orchestration.

## Workspace goals

- Combine `intent-driven-development`, `intent-driven-template`, and `openspec-tui-main` into a single collaboration environment.
- Deliver intent-driven development tooling and patterns.
- Enable automated AI workflows that transform a user request into a final deliverable.
- Provide agent-readable control planes, issue/PR guardrails, and merge-safe execution guidance.

## Layout

This is a **Cargo workspace** producing one Rust-native binary, **`rusty-idd`** (the unification — see `docs/rusty-idd/`):

- `crates/core/` — std-only control-plane logic for repo unification, manifesting, validation (was `intent-driven-development`).
- `crates/runner/` — the task-execution engine + OpenSpec data layer (split out of the TUI).
- `crates/tui/` — the ratatui OpenSpec TUI (was `openspec-tui-main`).
- `crates/spec/` — the OpenSpec lifecycle engine ported to Rust (parse / validate / transactional archive — no Node).
- `crates/cli/` — **`rusty-idd`**, the unified binary wiring the above together.
- `intent-driven-template/` — template assets / OpenSpec scaffolding.

## Build & run

From the workspace root: `cargo build --workspace` / `cargo test --workspace`. CI is `.github/workflows/ci.yml`.

```bash
cargo run --bin rusty-idd -- scan --repo <path>     # core control-plane verbs (init/scan/plan/task/validate/manifest/github)
cargo run --bin rusty-idd -- spec validate <file>   # OpenSpec lifecycle: validate / archive / show
cargo run --bin rusty-idd -- run <change>           # headless task runner
cargo run --bin rusty-idd -- tui                    # interactive TUI
```

## Recommended workflow

1. Review `AGENTS.md`, `crates/core/docs/AGENT_WORKFLOW.md`, and `docs/rusty-idd/` (the unification record).
2. Use the `rusty-idd` CLI and validate the agent control plane.
3. Keep agent tasks narrow and use the integration branch model described in the docs.

## Notes

- This workspace is intentionally structured for AI-assisted repo engineering and agent-driven merge orchestration.
- Do not commit real secrets. Use `.env.example`, `.env.schema.example.json`, and approved secret backends.
- Preserve old behavior until parity tests prove migration success.
