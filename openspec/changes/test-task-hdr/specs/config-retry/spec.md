# Spec: retry_on_failure for implementation loop

## Capability: TuiConfig.retry_on_failure

### REQ-1: Config field exists

TuiConfig SHALL have a `retry_on_failure` field of type `u32`.

#### SCEN-1.1: Default value
When `TuiConfig::default()` is used, `retry_on_failure` SHALL equal `1`.

#### SCEN-1.2: Deserialization default
When deserializing YAML that omits `retry_on_failure`, the field SHALL default to `1`.

#### SCEN-1.3: Explicit value deserialization
When deserializing YAML with `retry_on_failure: 5`, the field SHALL equal `5`.

### REQ-2: Serialization round-trip

Serializing a TuiConfig with a custom `retry_on_failure` and deserializing it SHALL produce
a struct with an equal `retry_on_failure` value.

### REQ-3: Loop respects config

The `implementation_loop` function SHALL use `config.retry_on_failure` as the per-task retry limit.

#### SCEN-3.1: Default causes immediate stall
With `retry_on_failure = 1` (default), a task that runs twice with no progress SHOULD stall on
the second no-progress run (first is the initial attempt, second is the retry).

#### SCEN-3.2: Higher value allows retries
With `retry_on_failure = N`, a task SHOULD be re-attempted up to N+1 times total before stalling.

### REQ-4: STALL_THRESHOLD remains an absolute upper bound

When both per-task and global stall limits are exceeded, the loop SHALL stall.
The global `STALL_THRESHOLD = 3` limit is NOT changed by this feature.
