# Config: retry policy for implementation loop

## Motivation

The `implementation_loop` in `crates/runner/src/runner.rs` already has stall detection
(`STALL_THRESHOLD = 3` consecutive no-progress runs) and spawn-failure counting. But there
is no way to configure **how** the runner retries when a task fails — it simply increments the
stall counter on every failure.

This change adds a `retry_on_failure` config field that controls how many consecutive times
the implementation loop re-issues the same `- [ ]` task before giving up (default: 1, i.e.
fail-fast to match current behaviour). This lets callers of `rusty-idd run` tune resilience
without editing source or changing `STALL_THRESHOLD`.

## Scope

- **Add** `retry_on_failure: u32` field to `TuiConfig` (serde default = 1).
- **Wire** the field into `implementation_loop`: after a task finishes without progress,
  bump a per-task retry counter; when it exceeds `retry_on_failure`, break with Stalled.
- **Preserve** existing `STALL_THRESHOLD` as an absolute upper bound (whichever comes first).
- **No changes** to batch processing, apply mode, or the data layer.

## Non-goals

- Per-task retry counts (config is global per run).
- Exponential back-off / jitter.
- Retry for post-implementation hook failures (those keep their current single-attempt policy).
