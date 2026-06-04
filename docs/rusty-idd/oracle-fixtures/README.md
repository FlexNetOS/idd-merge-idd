# Oracle fixtures

Golden fixtures captured from the Node OpenSpec CLI used as a throwaway
conformance oracle. These seed the differential tests for `crates/spec`
(Slice 4). The oracle itself (`/tmp/os-oracle/`) is deleted; the shipped
product has **zero** Node dependency.

- **Oracle**: `bunx @fission-ai/openspec@latest`, **version 1.4.1** (pin this
  for the Slice-4 differential harness).
- Captured: 2026-06-04.

## Files

| File | What it is | Used to verify |
|------|------------|----------------|
| `01-base-spec.md` | Base spec before archive (4 requirements). | merge input |
| `02-delta-spec.md` | Delta with ADDED + MODIFIED + REMOVED + RENAMED. | merge input |
| `03-archived-result.md` | The exact merged base spec the CLI `archive` produced. **Byte-stable target.** | `(01 + 02) → 03` archive golden test |
| `04-validate-spec.json` | `validate --json` of a spec that passes with one WARNING (Purpose too brief). | validate report shape + WARNING handling |
| `05-validate-no-scenario.json` | `validate --json` of a spec whose requirement has no scenario — `valid:false` with an ERROR. | validate ERROR path/message |

## Key verified semantics (see ../lifecycle-contract.md §3, §5, §6)

- **MODIFIED = whole-block replacement** (rate limit 10→20: old scenarios fully
  replaced, not merged). The programmatic `archive` does NOT do partial scenario
  merge — that is the separate agent-driven `sync`.
- **ADDED appends** to the end of the requirement list; **RENAMED keeps
  position**; **REMOVED** deletes the block.
- **Archive is transactional**: ADDED-of-existing ("already exists") and
  MODIFIED-of-missing ("not found") **abort the whole update — no files
  changed**.
- **Formatter tightens blank lines** on re-serialization (blank lines after
  `## Purpose` / `## Requirements` removed in `03`) — the Rust
  `format_commonmark` round-trip must reproduce this byte-for-byte.
- **validate** is per-file structural only (does not cross-check deltas vs base);
  ERROR fails always, WARNING fails only under `--strict`.
