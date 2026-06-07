# 0001. Unified flat CLI

- Status: accepted
- Date: 2026-06-07

## Context

The initial state of the project involved multiple binaries with distinct purposes: `idd` for repository unification and `openspec-tui` for change management. This led to fragmented user experience and deployment complexity.

## Decision

We will unify all core capabilities into a single Rust-native binary named **`rusty-idd`**. This CLI uses a flat subcommand structure:
- Repository unification: `init`, `scan`, `plan`, `task`, `validate`, `manifest`, `github`.
- Spec engine: `spec <validate|archive|show|sync|status|next|adr|new|scaffold>`.
- Execution: `run` (headless), `tui` (interactive).

## Consequences

- **Positive**: Single installation point; unified help and discovery; shared configuration logic; simpler integration into CI/CD pipelines.
- **Neutral**: Subcommand depth increases for some specific operations.
- **Negative**: Binary size increases as it carries the TUI and spec engine together.
