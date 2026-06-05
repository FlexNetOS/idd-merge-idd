//! openspec-runner — the non-UI execution layer extracted from the TUI.
//!
//! Holds the task-execution engine (`runner`: spawn an agent CLI, stream
//! progress, stall detection, batch ordering), the OpenSpec data layer
//! (`data`: parse `tasks.md`, list changes), and the run configuration
//! (`config`: `TuiConfig`). The TUI consumes these; the future unified
//! `crates/cli` will reuse them to drive task execution without ratatui.

pub mod config;
pub mod data;
pub mod runner;
