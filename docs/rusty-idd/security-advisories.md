# Security advisories — status & accepted-risk register

This file records every advisory `cargo audit` reports against the rusty-idd
workspace, the disposition of each, and the re-evaluation trigger. It is the
human-readable companion to the machine-readable baseline `.cargo/audit.toml`
(auto-loaded by every `cargo audit`) and the CI gate in
`.github/workflows/ci.yml`.

Policy (per the loop's UPGRADE-ONLY / NO-DOWNGRADE invariant): advisories are
remediated by moving **forward** (newer dependency) — never by downgrading or by
removing a capability. When no forward path exists and the advisory is a
non-vulnerability *unmaintained* warning, it may be **accepted-risk** here with an
explicit rationale and a re-evaluation trigger, rather than walling all work.

## Remediated

| Advisory | Crate | Disposition | Slice |
|----------|-------|-------------|-------|
| RUSTSEC-2026-0009 | `time` 0.3.41 → **0.3.47** | Fixed by forward upgrade (`cargo update -p time --precise 0.3.47`). DoS via stack exhaustion. No longer reported. | A2 |

## Accepted risk (no forward upgrade path)

Both entries below are **unmaintained warnings, not vulnerabilities** (no CVSS
severity; `cargo audit` classifies them `Warning: unmaintained`). They are pulled
in **transitively** and there is **no forward upgrade path** as of 2026-06-06:

- They both come exclusively from **`syntect` 5.3.0**, which is the **latest**
  published version (crates.io) and still depends on `bincode 1.x` + `yaml-rust 0.4`
  (used to load/serialize its syntax-definition sets).
- `syntect` reaches the tree via two latest-version consumers:
  - `comrak` 0.52.0 (latest) → `crates/spec` (CommonMark parse/emit), and
  - `tui-markdown` 0.3.7 (latest), pulled with **`features = ["highlight-code"]`** →
    `crates/tui`. That feature **is** the TUI's markdown code-block syntax-highlighting
    capability. Dropping `syntect` would remove that capability — a **downgrade**,
    which the invariant forbids.

Therefore these cannot be removed without either (a) `syntect` upstream dropping
them, or (b) deleting a shipped capability. Neither is acceptable today, so they are
accepted-risk and ignored in the shared baseline (warnings-only; the fail-closed
gate still fails on any **new** advisory or any **vulnerability**).

| Advisory | Crate | Why ignored | Re-evaluate when |
|----------|-------|-------------|------------------|
| RUSTSEC-2025-0141 | `bincode` 1.3.3 (via `syntect`) | unmaintained-only; required transitively by `syntect` (latest), which backs the TUI `highlight-code` capability and `comrak` | a `syntect` release drops `bincode 1.x`, or `comrak`/`tui-markdown` move off `syntect` |
| RUSTSEC-2024-0320 | `yaml-rust` 0.4.5 (via `syntect`) | unmaintained-only; same `syntect` dependency as above | a `syntect` release moves to `yaml-rust2`/`saphyr`, or the markdown stack drops `syntect` |

**Periodic check:** when bumping `syntect`/`comrak`/`tui-markdown`, re-run
`cargo audit` and, if either advisory clears, remove its line from
`.cargo/audit.toml` and move the row here to *Remediated*.
