# Tasks: retry_on_failure for implementation loop

## Implementation

- [x] T1: Add `retry_on_failure: u32` field to `TuiConfig` in `config.rs` with default 1, update serialization tests (golden YAML must include the field).
- [x] T2: Wire `retry_on_failure` into `implementation_loop`: add per-task retry counter, break when exceeded. Preserve existing STALL_THRESHOLD logic as absolute upper bound.
- [x] T3: Add unit tests in `config.rs` for deserialization default (SCEN-1.2), explicit value (SCEN-1.3), and round-trip serialization (REQ-2).
- [ ] T4: Add unit test in `runner.rs` that verifies the loop stalls after exactly `retry_on_failure + 1` runs with no progress (use `true` command, verify via messages channel).

## Verification

- [ ] T5: Run workspace build + tests to confirm nothing regresses.
- [ ] T6: Run drift check (`drift-check.sh`) to ensure core invariant is preserved.
