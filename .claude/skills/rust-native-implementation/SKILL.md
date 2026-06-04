---
name: rust-native-implementation
description: "Implements ONE planned merge slice in idiomatic, behavior-preserving Rust — deprecate-before-delete, edition-aware, keeping the idd crate std-only. ALWAYS use when writing the Rust code for a merge slice, porting foreign-language logic to Rust-native, or building a migration in this repo. Use this when an agent-generated artifact drifted off Rust and must be re-expressed in Rust."
---

# Rust-Native Implementation

Implement exactly one slice, make it read like the code around it, and never drift off Rust-native. Build and self-check before handing to QA.

## Why this exists
The `idd` crate's identity is **std-only, zero-dependency, no network**. Both crates are pure Rust. The cheap move under pressure is to shell out to a script or add a crate; that betrays the repo's design. When logic arrives in another language or as a generated non-Rust package, the correct response is to **port it to Rust**, not wrap it.

## Repo facts that change how you write code
- **No root workspace.** Run cargo from the crate directory: `intent-driven-development/` or `openspec-tui-main/`.
- **Editions differ.** `intent-driven-development/` = edition 2021, MSRV 1.74 (no edition-2024-only syntax). `openspec-tui-main/` = edition 2024 (let-chains etc. are used there).
- **`idd` is std-only.** Its `Cargo.lock` must stay at exactly 1 package. Adding a crate needs an explicit justification recorded in the slice spec — default no; prefer `std`.
- **`idd` error style:** `Result<_, String>` with `map_err(|e| format!(...))`. Flags are parsed by a hand-rolled `parse_flags` (no `clap`). Match the surrounding idioms.
- **`idd` file writes** go through `fs_utils::write_string_preserving_existing` (backup-on-overwrite). Preserve that contract when touching artifact generation.

## Procedure

### 1. Read the slice spec
Load `_workspace/02_planner_slice.md`. Implement its **In** scope and nothing in its **Out** scope.

### 2. Implement additively (deprecate-before-delete)
Introduce the new path alongside the old one. Leave the old path callable — a thin deprecation shim that delegates to the new code is ideal — until QA confirms parity. **Do not delete source during a migration.**

### 3. If foreign-language or generated non-Rust code is involved, port it
Re-express the logic in Rust (std-only for `idd`). Do not add a dependency or a shell-out to avoid the port. If the port genuinely requires a crate, stop and report it as a Rust-native-drift decision for the planner — do not introduce it unilaterally.

### 4. Build and self-verify (in the crate directory)
```bash
rtk cargo build
rtk cargo test <slice-related-test-name>   # fast inner-loop check
```
Do not hand red code to QA. If the build fails, fix it first.

### 5. Write the change log
`_workspace/03_implementer_changes.md`: files touched, old→new path mapping, build/test output, and any deprecation shims left in place.

## Principles
- Small, legible diffs that mirror existing naming, comment density, and error handling.
- Behavior preservation over cleverness — the goal is a migration that proves parity, not a rewrite.
- Stay in scope; if the slice can't be done as specified, push back to the planner rather than expanding it.
