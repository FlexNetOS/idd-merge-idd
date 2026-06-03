# Intent Driven Development Merge Workspace

This workspace is designed to merge three complementary projects into one integrated repository for intent-driven development, automated AI implementation, and workflow orchestration.

## Workspace goals

- Combine `intent-driven-development`, `intent-driven-template`, and `openspec-tui-main` into a single collaboration environment.
- Deliver intent-driven development tooling and patterns.
- Enable automated AI workflows that transform a user request into a final deliverable.
- Provide agent-readable control planes, issue/PR guardrails, and merge-safe execution guidance.

## Included folders

- `intent-driven-development/` — core CLI and planner for repository unification, manifest generation, and validation.
- `intent-driven-template/` — template assets, OpenSpec configuration, skills, and agent-ready task scaffolding.
- `openspec-tui-main/` — an OpenSpec TUI implementation with skills and agent interaction patterns.

## Open this workspace

Use `idd-merge-workspace.code-workspace` to open the merged workspace in VS Code.

## Recommended workflow

1. Open the `.code-workspace` file.
2. Review `AGENTS.md` and `intent-driven-development/docs/AGENT_WORKFLOW.md`.
3. Use the `intent-driven-development` CLI and validate the agent control plane.
4. Keep agent tasks narrow and use the integration branch model described in the docs.

## Notes

- This workspace is intentionally structured for AI-assisted repo engineering and agent-driven merge orchestration.
- Do not commit real secrets. Use `.env.example`, `.env.schema.example.json`, and approved secret backends.
- Preserve old behavior until parity tests prove migration success.
