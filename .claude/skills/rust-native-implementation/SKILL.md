---
name: rust-native-implementation
description: "Implements ONE planned merge slice in idiomatic, behavior-preserving Rust — deprecate-before-delete, edition-aware, keeping the idd crate std-only. ALWAYS use when writing the Rust code for a merge slice, porting foreign-language logic to Rust-native, or building a migration in this repo. Use this when an agent-generated artifact drifted off Rust and must be re-expressed in Rust."
---

# Rust-Native Implementation

Implement exactly one slice, make it read like the code around it, and never drift off Rust-native. Build and self-check before handing to QA.

## Why this exists
The `idd` crate's identity is **std-only, zero-dependency, no network**. Both crates are pure Rust. The cheap move under pressure is to shell out to a script or add a crate; that betrays the repo's design. When logic arrives in another language or as a generated non-Rust package, the correct response is to **port it to Rust**, not wrap it.

## Repo facts that change how you write code
- **Workspace state is mid-restructure.** Pre-restructure: two top-level crates, run cargo from `intent-driven-development/` or `openspec-tui-main/`. Target: a `rusty-idd` Cargo workspace — run cargo at the root with `--workspace`. Check the slice type (structural slices *create/move* crates).
- **Editions differ — keep them per-crate.** core (`intent-driven-development` → `crates/core`) = edition 2021, MSRV 1.74. tui (`openspec-tui-main` → `crates/tui`) = edition 2024 (let-chains etc.). A workspace carries both via each crate's own `edition` field; set `resolver = "3"` in the virtual root.
- **The zero-dep invariant is the CORE crate only.** `crates/core` (today `intent-driven-development`) keeps `[dependencies]` empty — `std` only; a dep there needs explicit justification in the slice spec, default no. The spec/runner/tui crates **may** use their researched deps (comrak, serde_norway, clap, ratatui) — that is expected, not drift.
- **`idd` error style:** `Result<_, String>` with `map_err(|e| format!(...))`. The existing `idd` flags use a hand-rolled `parse_flags` (no `clap`); the new unified `crates/cli` uses `clap` (per design). Match the crate you're in.
- **`idd` file writes** go through `fs_utils::write_string_preserving_existing` (backup-on-overwrite). Preserve that contract when touching artifact generation.

## Procedure

### 1. Read the slice spec
Load `_workspace/02_planner_slice.md`. Implement its **In** scope and nothing in its **Out** scope.

### 2. Implement additively (deprecate-before-delete)
Introduce the new path alongside the old one. Leave the old path callable — a thin deprecation shim that delegates to the new code is ideal — until QA confirms parity. **Do not delete source during a migration.**

### 3. If foreign-language or generated non-Rust code is involved, port it
Re-express the logic in Rust. For the **core** crate, `std` only — no dependency, no shell-out. For the lifecycle port (`crates/spec`), use the researched crates (comrak, serde_norway) at the crate edge; that is not drift. The drift rule is: nothing foreign-language in any `src/`, and no dependency on the **core** crate. If a port would add a dep to *core*, stop and report it to the planner.

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
