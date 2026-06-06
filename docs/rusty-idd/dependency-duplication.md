# Dependency duplication — analysis & disposition (backlog A6)

Goal of A6: collapse duplicate transitive crate versions **where a pure forward
upgrade unifies them** (upgrade-only; never pin a crate *down*). This file records
the measured state, why the residual duplicates cannot be collapsed by an
upgrade-only action, and the re-evaluation trigger.

Measured on develop @ A5 (2026-06-06), after A2's forward `time` bump and the A4
MSRV work. Reproduce with `cargo tree -d` (host) and the Cargo.lock version scan.

## Two distinct kinds of "duplicate" here — neither is upgrade-collapsible

### 1. Same-version build-vs-runtime splits (host build graph)
`cargo tree -d` on the host lists exactly three, each at a **single version**:

| Crate | Version | Why it appears twice |
|-------|---------|----------------------|
| `fastrand` | 2.4.1 | once as a **build-dependency** (comrak → `phf_codegen` → `phf_generator`) compiled for the host, once as a normal/dev dep (via `tempfile`) |
| `phf_shared` | 0.13.1 | build-graph (`phf_codegen`/`phf_generator`) vs runtime (`phf`) |
| `memchr` | 2.8.1 | reached by several parents (`nom` 8, `pulldown-cmark`, `quick-xml`, `serde_json`) but **already one version** |

These are **not version conflicts** — there is only one version of each. The split
is intrinsic (Cargo compiles build-scripts/proc-macros in a separate graph from
runtime code). No version bump can "unify" a crate that is already unified.

### 2. Multi-version old majors — from an UNUSED optional ratatui backend
The Cargo.lock *does* carry two majors of several crates:

| Crate | Versions in lock | Old major pinned by |
|-------|------------------|---------------------|
| `syn` | 1.0.109 + 2.0.117 | `wezterm-dynamic-derive` 0.1.1 |
| `bitflags` | 1.3.2 + 2.x | `wezterm-input-types` 0.1.0 |
| `nom` | 7.1.3 + 8.0.0 | `terminfo` 0.9.0 |
| `phf` / `phf_shared` | 0.11.3 + 0.13.1 | `terminfo` 0.9.0 |

**Every one of these old-major consumers is reached only through `ratatui-termwiz`**
(`termwiz` → `terminfo` / `wezterm-*`). `ratatui-termwiz` is an **optional** terminal
backend of `ratatui` 0.30 that this workspace **does not enable** — `crates/tui`
uses `ratatui = "0.30"` + `crossterm = "0.29"` (the default crossterm backend).

Consequences, verified with `cargo tree`:
- `cargo tree -i syn@1.0.109` / `terminfo` / `termwiz` print **"nothing to print"** even
  with `--target all` — the termwiz stack is **not in any build graph** for any target.
- These crates are **lock-resident only** (Cargo locks optional deps it could
  theoretically resolve) and are **never compiled into the shipped `rusty-idd`
  binary**. They add zero code/attack surface to the product.
- They **cannot be unified by a forward upgrade**: the binding versions are dictated
  by the upstream `termwiz`/`terminfo`/`wezterm-*` crates, which still use the old
  majors. `ratatui`/`crossterm` are already at their latest. Forcing the issue would
  mean patching upstream — out of scope and not an upgrade-only action.
- Pinning them *down* or deleting lock lines is forbidden (downgrade / fail-closed),
  and pointless (they don't ship).

## Disposition
**No upgrade-only action is available or beneficial.** The shipped binary is already
free of the old majors; the residual `cargo tree -d` host entries are same-version
build/runtime splits that are inherent, not collapsible. A6 is therefore resolved as
**investigated + documented**, not a lockfile edit.

**Re-evaluate when:** `ratatui` drops or upgrades the `ratatui-termwiz` backend's
terminal stack (`termwiz`/`terminfo`/`wezterm-*`) to the new majors, or if this
workspace ever opts into the termwiz backend (at which point unifying becomes both
possible and worthwhile). Re-run `cargo tree -d` + the Cargo.lock version scan after
any `ratatui`/`crossterm` bump.
