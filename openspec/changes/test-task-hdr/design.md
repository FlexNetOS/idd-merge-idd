# Design: retry_on_failure config

## Context

`implementation_loop` tracks a `stall_count: u32` that increments on:
1. Spawn failure (line ~564 in runner.rs)
2. No-progress after command exit (line ~640 in runner.rs)

When `stall_count >= STALL_THRESHOLD` the loop breaks and sends `ImplUpdate::Stalled`.

There is no per-task notion of retry — every failure counts toward the global stall counter.
For real-world usage this means one bad task kills the whole run even if later tasks are fine
(when running single-change). Operators want a configurable number of re-issues per task.

## Decision

Add `retry_on_failure: u32` to `TuiConfig` with default 1 (fail-fast, current semantics).

In the implementation loop's progress-recheck section (~line 635):
```rust
if completed > prev_completed {
    stall_count = 0;          // reset both counters on progress
    task_failures = 0;        // reset per-task counter
    prev_completed = completed;
} else {
    task_failures += 1;       // bump per-task retry counter
    if task_failures > config.retry_on_failure {
        stalled = true;       // per-task limit exceeded
        break;
    }
    stall_count += 1;         // still count toward global stall
    if stall_count >= STALL_THRESHOLD {
        stalled = true;
        break;
    }
}
```

## Why not a separate config struct?

`TuiConfig` is the canonical place for run-time tuning. Adding a nested `RetryPolicy` would
require new types, more YAML nesting, and more serde defaults — overkill for one field.

## Test plan

1. Default value test: `retry_on_failure` defaults to 1 when omitted from YAML.
2. Round-trip serialization: serializing + deserializing preserves the value.
3. Loop uses config: test that loop breaks after exactly `retry_on_failure` no-progress runs
   (use `true` command that never marks tasks).
