---
name: repo-inventory
description: "Inventories repos/crates for a merge: languages, package managers, entrypoints, workflows, agent-control files, the feature matrix, and the env/secret contract — using the idd CLI as the engine. ALWAYS use when scanning a repo before a merge, mapping the merge surface, building a feature matrix, or capturing the Rust-native baseline. Use this rather than ad-hoc grepping when preparing a merge."
---

# Repo Inventory

Make the merge surface explicit before any code moves. The `idd` CLI already detects languages, package managers, entrypoints, workflows, agent-control files, and secret references — drive it rather than re-implementing detection by hand.

## Why this exists
A merge plan is only as good as its evidence. The analyst's job is to hand the planner facts (what exists, what overlaps, what conflicts, which keys are secrets) and to record the **Rust-native baseline** so any later drift is measurable against a known-good starting point.

## Procedure

### 1. Confirm a synced base
Never inventory a stale tree:
```bash
rtk git fetch --all && rtk git status -sb
```

### 2. Scan each repo/crate
`idd scan` emits both human-readable and machine-readable inventories. Run it for each side of the merge:
```bash
# from the workspace root (the idd binary lives in crates/core)
rtk cargo run --bin rusty-idd -- scan --repo <path> --format md   --out <ws>/01_<name>_inventory.md
rtk cargo run --bin rusty-idd -- scan --repo <path> --format json --out <ws>/01_<name>_inventory.json
```
The inventory captures language counts, package managers, entrypoints, workflows, agent-control files, and secret references per repo.

### 3. For a two-repo unification, prefer `idd plan`
When the task is to unify two repos, `idd plan` generates the full control plane (both inventories, feature matrix, env/secret contract, merge plan, conflict register) in one pass:
```bash
rtk cargo run --bin rusty-idd -- plan --repo-a <pathA> --repo-b <pathB> --out <ws> --name <slug>
```
Read the generated `AI_MERGE/02_feature_matrix.md` and `AI_MERGE/03_env_and_secret_contracts.*` and distill them into your `_workspace/` artifacts.

### 4. Build the feature matrix
Cross-tabulate what each side provides. For every feature record: side A has it / side B has it / both (overlap) / conflict. Overlaps and conflicts are the planner's raw material — make them prominent.

### 5. Map the env/secret contract
From the scan's secret references, classify each key:
- **configuration** — safe to document in `.env.example`/schema,
- **secret** — value must never be committed; only the *reference* is mapped.
List provider/source per key (GitHub Actions, dotenv, Vault/OpenBao, SOPS, Doppler, Infisical, direnv, mise, etc. — `idd` detects these).

### 6. Capture the Rust-native baseline
Record the known-good starting state so QA can detect drift later. Use the **layout-agnostic** drift gate rather than hardcoded paths (the tree gets restructured into `crates/*`):
```bash
bash .claude/skills/merge-verification/scripts/drift-check.sh .   # expect: no drift, exit 0
```
Note its output (discovered crate src trees + which crate is the zero-dep core) plus per-crate language counts in `_workspace/01_analyst_baseline.md`. This baseline is what QA re-runs after each slice.

### 7. Hand the lifecycle off to the porter (don't port it yourself)
The OpenSpec lifecycle (`intent-driven-template/openspec/schemas/intent-driven/schema.yaml` + the `.opencode/`/`.agents` skills) is a *porting source*, not just inventory. Note its presence and scope in your inventory, but the constructive extraction into a contract is `lifecycle-porter`'s job (`_workspace/01_lifecycle_contract.md`) — flag it to the porter rather than duplicating it.

## Output artifacts (write to `_workspace/`)
- `01_analyst_inventory.md` — per-repo inventory summary.
- `01_analyst_feature_matrix.md` — feature cross-tab with overlaps/conflicts flagged.
- `01_analyst_env_secret_contract.md` — keys classified config vs secret, with sources.
- `01_analyst_baseline.md` — Rust-native baseline (source purity + `idd` dep count + language counts).

## Principles
- Separate configuration from secrets explicitly; never record a secret value.
- Surface conflicts; do not resolve them here — resolution is the planner's call.
- Prefer `idd`/`rtk` commands (deterministic, token-cheap) over free-form exploration.
