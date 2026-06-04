# Lifecycle Contract — OpenSpec intent-driven workflow

Source of truth for the rusty-idd `crates/spec` port. Extracted from the
`intent-driven` schema + skill/command prose, and **verified against the Node
OpenSpec CLI** (`bunx @fission-ai/openspec@latest`, **v1.4.1**) used as a
throwaway conformance oracle in `/tmp/os-oracle/` (now deleted).

Every rule below is tagged:

- **`oracle-verified`** — ran the CLI on a fixture and captured the exact
  behavior (fixtures in `docs/rusty-idd/oracle-fixtures/`).
- **`schema-only`** — stated only by the schema/skill prose; the CLI does not
  enforce it (or it could not be exercised), so the Rust engine must hand-author
  it from the documented format.

> Scope note: the bundled CLI ships `spec-driven` / `workspace-planning`
> schemas, not the project-local `intent-driven` schema. The artifact DAG below
> is `intent-driven`-specific (`schema-only`). The **spec-wrapper, delta-merge,
> validate, and archive** rules are schema-independent in OpenSpec, so they were
> oracle-verified with the bundled CLI and apply unchanged to `intent-driven`.

---

## 1. Artifact DAG (`schema-only`)

From `intent-driven-template/openspec/schemas/intent-driven/schema.yaml`.
Artifacts and their `requires:` edges (a `requires` edge means "the named
artifact must exist before this one can be generated"):

```
proposal   requires: []                generates proposal.md
specs      requires: [proposal]        generates specs/**/spec.md
design     requires: [proposal]        generates design.md
adr        requires: [design]          generates ../../../adr/*.md  (repo-root adr/, NOT under the change)
tasks      requires: [specs, adr]      generates tasks.md
apply      requires: [tasks]           tracks tasks.md (checkbox progress)
```

DAG (edges point producer → consumer):

```
        proposal
        /      \
     specs    design
       |         |
       |        adr
        \       /
         \     /
          tasks
            |
          apply  (tracks tasks.md checkboxes)
```

Notes:
- `specs` and `design` both depend only on `proposal` and are **independent of
  each other** — a real DAG, not a linear chain. The documented "stage gate"
  order `proposal → specs → design → adr → tasks` is the *recommended* authoring
  order; the binding constraint is the `requires:` set.
- `tasks` is the join node: it requires **both** `specs` and `adr`.
- `apply` is not an artifact; it is the execution phase, gated on `tasks`,
  tracking `- [ ]` / `- [x]` checkboxes in `tasks.md`.

Per-artifact instruction summaries (for the scaffolding templates):
- **proposal**: WHY. Sections: Why, What Changes (mark **BREAKING**),
  Capabilities (New = new `specs/<kebab>/spec.md`; Modified = delta on an
  existing capability), Impact.
- **specs**: WHAT. One spec file per capability; OpenSpec Markdown wrapper with
  Gherkin-style content. (Wrapper + delta rules in §2–§3.)
- **design**: HOW. Sections: Context, Goals/Non-Goals, Decisions (with
  alternatives), Risks/Trade-offs, Migration Plan, Open Questions. Must read
  in-force ADRs first and stay coherent with them.
- **adr**: durable decisions distilled from design into `<repo>/adr/`. (§4.)
- **tasks**: `## numbered headings` grouping `- [ ] X.Y description` checkboxes.

---

## 2. Spec wrapper rules

The Markdown headings are the OpenSpec *merge wrapper*; content inside is
Gherkin-style.

| Rule | Tag | Evidence |
|------|-----|----------|
| Requirement heading is **H3** `### Requirement: <name>` | `oracle-verified` | An H3 starting `Requirement:` is parsed as a requirement; the delta-merge keys off this exact heading (fixture 02→03). |
| Scenario heading is **H4** `#### Scenario: <name>` (exact four hashes) | `oracle-verified` | A `### Scenario:` (H3) is mis-parsed as a *new requirement* and then flagged "must have at least one scenario" (fixture `05`, and the `h3-scenario` probe). |
| Every requirement MUST have ≥1 scenario | `oracle-verified` (**ERROR**) | `no-scenario` probe → `ERROR` `requirements.N.scenarios: "Requirement must have at least one scenario"`; fails even non-strict (fixture `05-validate-no-scenario.json`). |
| Requirement body MUST contain `SHALL` or `MUST` | `oracle-verified` (**ERROR**) | `h3-scenario` probe → `ERROR` `requirements.N.text: "Requirement must contain SHALL or MUST keyword"`. |
| Scenario body uses `GIVEN`/`WHEN`/`THEN` (`AND`/`BUT` follow-ons), observable outcomes in THEN | `schema-only` | A scenario with prose but **no** WHEN/THEN bullets validated cleanly even under `--strict` (`no-wt` probe). GIVEN/WHEN/THEN is a **content-lint** target, not a validate rule. |
| Spec file has a `## Purpose` section; "too brief (<50 chars)" is a WARNING | `oracle-verified` (**WARNING**) | fixture `04-validate-spec.json` (`overview: "Purpose section is too brief"`). Delta files (`specs/<cap>/spec.md` inside a change) need no Purpose — they are pure delta op sections. |
| Base spec layout: `# <name> Specification` → `## Purpose` → `## Requirements` → `### Requirement:` blocks | `oracle-verified` | The merged result preserves this layout (fixture `03`). |

---

## 3. Delta operations and merge semantics

A change's delta spec lives at `openspec/changes/<change>/specs/<capability>/spec.md`
and contains only `## <OP> Requirements` sections. Archive applies the delta
onto the base spec at `openspec/specs/<capability>/spec.md`.

The four ops and their **exact archive (programmatic-merge) semantics**, all
**`oracle-verified`** against fixtures `01` (base) + `02` (delta) → `03`
(archived result):

### ADDED — `## ADDED Requirements`
- Appends each new `### Requirement:` block **to the end** of the base spec's
  requirement list (verified: "JSON export" landed after all existing
  requirements, fixture `03` lines 35–41).
- **The target requirement MUST NOT already exist.** ADDED of an existing
  requirement name → `"<cap> ADDED failed for header \"### Requirement: <name>\" - already exists"` → **whole archive aborts, no files changed** (`dup-add` probe). `oracle-verified`.

### MODIFIED — `## MODIFIED Requirements`  ⭐ the load-bearing rule
- **Whole-block replacement.** The delta carries the *entire* updated
  requirement (description + ALL scenarios); the matched base block is replaced
  wholesale. Verified: "Export rate limit" 10→20 with rewritten scenarios — the
  base's original scenarios were **fully replaced**, not merged (fixture
  `01` lines 14–25 → `03` lines 14–25).
- Header matched on requirement name; the modified block keeps the base block's
  **position** in the list.
- **The target MUST exist.** MODIFIED of a non-existent requirement →
  `"<cap> MODIFIED failed for header \"### Requirement: <name>\" - not found"`
  → **abort, no files changed** (`bad-modify` probe). `oracle-verified`.
- ⚠️ This is the single rule most easily gotten wrong. A *partial* MODIFIED
  (e.g. one new scenario, omitting the rest) silently **loses** the omitted
  scenarios at archive time. The schema's MODIFIED workflow ("copy the ENTIRE
  requirement block") exists precisely to prevent this.
- ⚠️ **Programmatic `archive` ≠ agent-driven `sync`.** The `opsx-sync` command
  prose explicitly allows *intelligent partial merges* ("To add a scenario, just
  include that scenario under MODIFIED — don't copy existing scenarios"). That
  is a *human/agent* operation on the live spec, NOT what the CLI `archive`
  does. **rusty-idd's `archive` must implement the programmatic whole-block
  replacement** (faithful to the CLI), and may optionally offer a separate
  agent-style `sync` later. `oracle-verified` distinction.

### REMOVED — `## REMOVED Requirements`
- Unlinks the entire matched `### Requirement:` block from the base spec
  (verified: "Legacy XML export" gone in fixture `03`).
- Delta MUST carry **`**Reason**`** and **`**Migration**`** fields (`schema-only`
  for enforcement — these are documented requirements but the merge consumed the
  block regardless; treat as a content-lint/authoring rule).
- Target must exist (same abort-on-missing transaction semantics as MODIFIED,
  by parity; `schema-only` — not separately exercised).

### RENAMED — `## RENAMED Requirements`
- `- FROM: \`### Requirement: <old>\`` / `- TO: \`### Requirement: <new>\`` —
  changes the **heading text only**; the requirement's body and scenarios are
  preserved and it **keeps its original position** (verified: "Export filename"
  → "Exported file naming" in place, fixture `01` line 27 → `03` line 27).

### Transactionality (`oracle-verified`)
- Archive is **atomic per spec file**: if *any* op fails (ADDED-exists,
  MODIFIED/RENAMED/REMOVED-not-found), the CLI prints the failing header and
  `"Aborted. No files were changed."` — the entire spec update is rolled back.
  rusty-idd MUST reproduce this all-or-nothing behavior.

### Formatting / round-trip (`oracle-verified`, load-bearing for golden tests)
- The merge re-serializes the whole file through a CommonMark formatter. It
  **tightens blank lines**: in fixture `03` the blank line between `## Purpose`
  body and `## Requirements`, and between `## Requirements` and the first
  `### Requirement:`, were **removed** (fixture `01` had them; `03` does not).
  Within requirement blocks the single blank line between scenarios is
  preserved, and a trailing newline is added. The Rust `format_commonmark` round
  trip must be tuned so the same delta yields **byte-identical** output to the
  oracle — this is the differential-test pass criterion for Slice 4.

---

## 4. ADR rules (`schema-only`)

From the `adr` artifact instruction + schema README (not CLI-enforced — ADRs are
plain files the agent writes):

- ADR files live at the **target repo's top-level `adr/`** (beside `openspec/`),
  **never** under `openspec/changes/<change>/`. (`generates: ../../../adr/*.md`.)
- **IRON RULE — immutable once accepted.** Never edit a prior ADR (not Status,
  body, nor date). To revisit a decision, write a **NEW** ADR with
  `Status: accepted, supersedes ADR-NNNN` and a `Supersedes:` field naming the
  prior one.
- **Supersession graph**: an ADR is in-force iff it is `accepted` and no later
  ADR's `Supersedes:` points at it. Consumers (design step) walk `Supersedes:`
  links to derive the in-force set; superseded ADRs are historical context only.
- **Filenames**: `NNNN-kebab-title.md`, `NNNN` a 4-digit monotonic sequence
  across the whole repo, never reused.
- The `adr` step still "completes" if no new ADR is warranted, **as long as
  `adr/` already contains ≥1 ADR file**.

---

## 5. What `validate` enforces (`oracle-verified`)

CLI: `validate [item] [--type change|spec] [--strict] [--json] [--all|--changes|--specs]`.

- **Per-item structural validation only.** It checks one file's internal
  structure; it does **not** cross-check delta ops against the base spec (a
  MODIFIED of a non-existent requirement still `valid: true` — that failure
  surfaces only at archive time, §3).
- **Issue levels: `ERROR` vs `WARNING`.**
  - Non-strict: `valid` is false / item `failed` only if there is ≥1 **ERROR**.
    WARNINGs are reported but the item still `passed`.
  - `--strict`: WARNINGs are **promoted to failures** (base spec with the
    too-brief Purpose `passed` non-strict but `failed` under `--strict`).
- **Enforced ERRORs** (verified): requirement has ≥1 scenario; requirement body
  contains `SHALL`/`MUST`; scenario must be H4 (an H3 scenario is mis-parsed).
- **Enforced WARNINGs** (verified): Purpose section <50 chars ("too brief").
- **JSON shape** (target for rusty-idd's `--json`):
  ```json
  { "items": [ { "id", "type", "valid": bool,
                 "issues": [ { "level": "ERROR|WARNING", "path", "message" } ],
                 "durationMs" } ],
    "summary": { "totals": {items,passed,failed},
                 "byType": { "<type>": {items,passed,failed} } },
    "version": "1.0" }
  ```
  See fixtures `04-validate-spec.json` (passing-with-warning) and
  `05-validate-no-scenario.json` (failing-with-error).

---

## 6. What `archive` does (`oracle-verified`)

CLI: `archive [change] [-y] [--skip-specs] [--no-validate]`.
Description: *"Archive a completed change **and update main specs**"* — the CLI
runs the programmatic delta-merge itself; it is not merely a directory move.

Sequence (observed):
1. Reports task status (warns on incomplete `- [ ]`, does not block with `-y`).
2. Lists `Specs to update`.
3. For each delta spec, applies ADDED/MODIFIED/REMOVED/RENAMED onto the base
   spec (§3), printing `+ N added / ~ N modified / - N removed / → N renamed`.
   Aborts the whole update if any op fails (transactional).
4. Moves the change dir to `openspec/changes/archive/YYYY-MM-DD-<change>/`
   (carrying `.openspec.yaml` and the consumed delta specs with it).
- `--skip-specs`: archive (move) without applying any spec merge — for
  infra/tooling/doc-only changes.
- `--no-validate`: skip the pre-archive validation pass (default is to validate
  first; not recommended).
- The agent-facing skill/command (`opsx-archive`) layers extra UX on top
  (prompt for change, status checks, optional `sync`), but the CLI `archive`
  is the authoritative merge+move primitive rusty-idd must reproduce.

---

## 7. Rules left `unverified` / `schema-only` (and why)

- **REMOVED Reason/Migration enforcement** — documented as MUST, but the merge
  consumed the REMOVED block without erroring on their absence in probes; treat
  as authoring/content-lint, not a hard validate gate. Re-confirm in Slice 4.
- **RENAMED / REMOVED abort-on-missing** — inferred by parity with MODIFIED's
  verified abort; only MODIFIED and ADDED were directly exercised for the
  not-found / already-exists abort. Cheap to re-verify in Slice 4.
- **`intent-driven`-specific DAG / gate ordering** — the bundled CLI ships only
  `spec-driven`/`workspace-planning`; the `requires:` edges are read straight
  from `schema.yaml` (`schema-only`). The spec-format/merge/validate behavior is
  schema-independent and was verified.
- **GIVEN/WHEN/THEN content** — confirmed NOT validate-enforced; it is a
  content-lint target only.
