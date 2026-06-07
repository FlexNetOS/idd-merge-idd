# Parity Matrix — rusty-idd

This matrix maps every feature, capability, and spec of the three source projects to its home in `rusty-idd`. **rusty-idd** is complete only when every item here is "pass" and verified.

## 1. intent-driven-development (idd control plane)
The core merge-management engine, now in `crates/core` and `crates/cli`.

| Verb | Home | Status | Notes |
|------|------|--------|-------|
| `init` | `rusty-idd init` | **pass** | Delegated to `core`. |
| `scan` | `rusty-idd scan` | **pass** | Delegated to `core`. |
| `plan` | `rusty-idd plan` | **pass** | Delegated to `core`. |
| `task` | `rusty-idd task` | **pass** | Delegated to `core`. |
| `validate` | `rusty-idd validate` | **pass** | Delegated to `core`. Critical findings fail non-zero. |
| `manifest` | `rusty-idd manifest` | **pass** | Delegated to `core`. |
| `github` | `rusty-idd github` | **pass** | Delegated to `core`. |

## 2. openspec-tui-main (TUI + runner)
The interactive task runner and execution layer, now in `crates/tui` and `crates/runner`.

| Feature | Home | Status | Notes |
|---------|------|--------|-------|
| Terminal UI | `rusty-idd tui` | **pass** | `crates/tui` UI + `crates/runner` execution. |
| Headless Run | `rusty-idd run` | **pass** | `crates/cli` wraps `crates/runner`. |
| Task tracking | `crates/runner` | **pass** | Parsed from `tasks.md` checkboxes. |
| Stall detection | `crates/runner` | **pass** | Aborts after 3 no-progress runs. |
| Dependency sort | `crates/runner` | **pass** | Topological sort of `change-config.yaml` deps. |
| Archive view | `rusty-idd tui` | **pass** | Displays archived changes from `openspec/changes/archive/`. |

## 3. OpenSpec Lifecycle (Node CLI / intent-driven-template)
The intent-driven lifecycle engine, now in `crates/spec` and `crates/cli`.

| Verb | Home | Status | Notes |
|------|------|--------|-------|
| `spec validate` | `rusty-idd spec validate` | **pass** | Structural validation, batch filtering (`--all/changes/specs`), and recursive directory search. |
| `spec archive` | `rusty-idd spec archive` | **pass** | Transactional delta-merge + dir move. |
| `spec show` | `rusty-idd spec show` | **pass** | Markdown rendering in CLI. |
| `spec status` | `rusty-idd spec status` | **pass** | Artifact DAG status check. |
| `spec next` | `rusty-idd spec next` | **pass** | Propose next capability via agent prompt. |
| `spec adr` | `rusty-idd spec adr` | **pass** | ADR scaffolding and monotonic numbering. |
| `spec scaffold` | `rusty-idd spec scaffold` | **pass** | Generates `proposal/specs/design/tasks` stubs with full contract templates. |
| `spec new` | `rusty-idd spec new` | **pass** | Propose new change via agent prompt. |
| `spec sync` | `rusty-idd spec sync` | **pass** | Agent-driven scenario merge with intelligent scenario injection. |

## 4. Differential Parity (Harness)
Regression testing against the original behavior.

| Goal | Mechanism | Status | Notes |
|------|-----------|--------|-------|
| `idd` parity | Byte-identical core output | **pass** | Delegated verbatim. |
| `openspec` parity | Oracle Fixtures (Golden) | **pass** | Fixtures 01-08 passing in `crates/spec`. |
| `openspec` parity | Differential Harness | **pass** | Continuous diff vs Node oracle via `scripts/oracle-sync.sh`. |
| Lifecycle parity | Template content | **pass** | Full content generation vs contract implemented in `scaffold::render`. |

## Summary of Completed Gaps
- **D2**: `spec validate` flags implemented.
- **D3**: `spec sync` capability implemented and verified.
- **D4**: Continuous differential oracle harness active.
- **D5**: Full lifecycle generation parity verified.
