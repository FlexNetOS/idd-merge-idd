# Intent Driven Development Merge Workspace

This workspace is designed to merge three complementary projects into one integrated repository for intent-driven development, automated AI implementation, and workflow orchestration.

## Workspace goals

- Combine `intent-driven-development`, `intent-driven-template`, and `openspec-tui-main` into a single collaboration environment.
- Deliver intent-driven development tooling and patterns.
- Enable automated AI workflows that transform a user request into a final deliverable.
- Provide agent-readable control planes, issue/PR guardrails, and merge-safe execution guidance.

## Layout

This is a **Cargo workspace** (the rusty-idd unification, in progress — see `docs/rusty-idd/`):

- `crates/core/` — the std-only `idd` CLI and planner for repository unification, manifest generation, and validation (was `intent-driven-development`).
- `crates/tui/` — the OpenSpec TUI with skills and agent interaction patterns (was `openspec-tui-main`).
- `crates/spec/` — planned: the OpenSpec lifecycle engine ported to Rust.
- `intent-driven-template/` — template assets, OpenSpec configuration, skills, and agent-ready task scaffolding (the lifecycle being ported into `crates/spec`).

## Build

From the workspace root: `cargo build --workspace` / `cargo test --workspace`. CI is `.github/workflows/ci.yml`.

## Recommended workflow

1. Review `AGENTS.md`, `crates/core/docs/AGENT_WORKFLOW.md`, and `docs/rusty-idd/` (the unification plan).
2. Use the `idd` CLI (`cargo run --bin idd`) and validate the agent control plane.
3. Keep agent tasks narrow and use the integration branch model described in the docs.

## Notes

- This workspace is intentionally structured for AI-assisted repo engineering and agent-driven merge orchestration.
- Do not commit real secrets. Use `.env.example`, `.env.schema.example.json`, and approved secret backends.
- Preserve old behavior until parity tests prove migration success.
