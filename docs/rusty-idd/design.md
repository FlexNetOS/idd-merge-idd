# Design: rusty-idd

Records the architecture decisions for `rusty-idd`, each backed by the deep research run on 2026-06-04. Format follows the intent-driven `design.md` shape (Context / Goals / Decisions / Risks / Migration / Open Questions).

## Context

Three directors (`intent-driven-development`, `openspec-tui-main`, the OpenSpec lifecycle in `intent-driven-template`) must unify into one Rust-native binary. Two are already Rust; the lifecycle engine is a Node/TypeScript CLI (not installed) to be ported. Built and verified by the merge-dev harness, slice by slice.

## Goals / Non-Goals

- **Goals**: one Rust-native binary; preserve the zero-dep core; preserve each tool's behavior; a faithful, testable OpenSpec lifecycle engine; clean seams for agent-parallel construction.
- **Non-Goals**: building rusty-idd's *runtime agent engine* (separate, later work — the current harness builds rusty-idd, it is not rusty-idd's runtime); shipping any Node dependency; rewriting the already-Rust crates (they move, not rewrite).

## Decisions

### D1 — Packaging: Cargo workspace, one binary (confidence: high)
A virtual-root **workspace** with member crates `crates/{core, spec, runner, tui, cli}`; `crates/cli` is the only `[[bin]]`. `resolver = "3"` set explicitly in the virtual root. **Why**: empirically the dominant structure for large Rust CLIs — ripgrep (9 members → `rg`), uv (~60), nushell (~38), cargo all use it; it's the only layout that natively carries **mixed editions** (per-crate `edition`: core=2021, tui=2024) and gives parallel agents compiler-checked seams. Rejected: a flat binary (can't mix editions, no enforced dep wall); lib+many-bins as the primary shape (we want one shipped binary).
- Sources: Cargo Book Workspaces; Rust 2024 resolver guide; ripgrep/uv/nushell `Cargo.toml`.

### D2 — Zero-dependency core preserved (confidence: high)
`crates/core` keeps an **empty `[dependencies]`** (std-only, edition 2021); serde/ratatui/parsers live in adapter crates (hexagonal "pure core, impure shell"). **Why**: verified against the Cargo resolver reference — feature unification changes *features on already-declared deps* and **can never add a dependency edge to a crate that declares none**. So the workspace move does not endanger the core's purity. Enforced by `drift-check.sh` (parses the core manifest) + a CI `cargo tree`/`cargo-deny` gate. Keep serde *out* of core public types; serialization belongs at the edges.
- Sources: Cargo resolver ref; RFC 3692; howtocodeit/Barrage hexagonal-Rust; eizinger (keep derives out of core).

### D3 — Lifecycle engine toolkit (confidence: high)
`crates/spec` is built on **clap v4 (derive)** + **`serde_norway`** + **`comrak`**; Tera/minijinja for artifact scaffolding; **no `gherkin` crate**.
- `serde_yaml` is archived (2024); `serde_yml` is **unsound (RUSTSEC-2025-0068)** — both avoided; `serde_norway` is the maintained fork (mdBook ecosystem).
- `comrak` builds a true mutable Markdown AST with `format_commonmark` round-trip — the right tool for splicing `### Requirement:` subtrees in the `archive`/delta-merge. `pulldown-cmark` is read-only-fast (fine for header scans), but its round-trip is lossy by design.
- OpenSpec scenarios are Markdown-wrapped, not `.feature` files → the gherkin crate is the wrong tool; lint GIVEN/WHEN/THEN as text in the AST.
- Delta-merge semantics to preserve exactly: **MODIFIED = whole-block replacement** (not a partial diff), REMOVED carries Reason+Migration, RENAMED is FROM/TO heading-only, ADDED appends.
- Sources: RustSec advisory; Rain's CLI recommendations; comrak/pulldown-cmark repos; cucumber-rs/gherkin.

### D4 — Authoring + conformance via a throwaway oracle (confidence: high)
Author rusty-idd's own specs as hand-written Markdown now (no binary needed; format documented). For building the engine, use **`npx openspec@latest`** transiently as a **conformance oracle**: capture `(spec → validate --json)` and `(delta → archived)` golden fixtures, reproduce them in Rust, then drop Node. **Why**: OpenSpec is MIT and emits plain Markdown → near-zero lock-in; this is the standard "keep the reference impl as a differential-testing oracle" move. `npx` means no global install.
- Sources: Fission-AI/OpenSpec docs (cli, spec-format); npm `@fission-ai/openspec`.

## Risks / Trade-offs

- **Lifecycle port complexity** → Mitigation: it is the *last* substantive slice (after low-risk structural moves); golden fixtures from the Node oracle make it test-driven; `oonid/OpenSpec-rs` is a tractability reference.
- **Mixed-edition workspace build on old toolchains** → Mitigation: require Rust ≥ 1.85 (the 2024 edition baseline); document MSRV.
- **Core purity regressing via a careless dep** → Mitigation: `drift-check.sh` + CI `cargo-deny` on the core crate; the harness's QA makes this priority-one.
- **Two control planes (idd `AI_MERGE/` vs OpenSpec `openspec/changes/`)** → Open question below.

## Migration Plan

Per `slice-sequence.md`: workspace skeleton → fold in existing crates → port lifecycle → integrate CLI. Each slice independently revertable; structural moves are `git mv`-level until `crates/cli` cuts over. Old per-tool entrypoints stay callable until the unified CLI proves parity.

## Open Questions

- **Control-plane reconciliation**: does rusty-idd keep both idd's `AI_MERGE/` and OpenSpec's `openspec/changes/`, or unify them? (Candidate for an ADR.)
- **CLI namespace**: flat verbs (`rusty-idd propose|scan|run`) vs grouped (`rusty-idd spec …|merge …|run …`)? (Candidate for an ADR — D1 commits to one binary, not to the verb taxonomy.)
- **TUI scope in v1**: ship the ratatui TUI in the first unified binary, or land it after the CLI is stable?
