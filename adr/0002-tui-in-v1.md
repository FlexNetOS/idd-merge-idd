# 0002. TUI included in v1 release

- Status: accepted
- Date: 2026-06-07

## Context

OpenSpec users typically expect an interactive way to manage changes and view progress. The `openspec-tui` project provided this, but as a separate project.

## Decision

We will include the interactive TUI as a core command (`rusty-idd tui`) in the first stable release of the unified workspace. The TUI code is folded into `crates/tui` and depends on the `crates/runner` execution layer.

## Consequences

- **Positive**: Immediate high visual impact and utility for end users; parity with established OpenSpec workflow expectations; better feedback loop during complex merges.
- **Neutral**: Adds dependencies on `ratatui` and `crossterm` to the workspace.
- **Negative**: Increases the complexity of the release-candidate gates (TUI must be verified alongside CLI).
