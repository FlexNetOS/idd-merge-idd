---
name: merge-orchestrator
description: "Coordinates the merge-dev agent team for idd-merge-idd: Analyze → Plan → Implement → Verify → assemble PR evidence, producing reviewable, Rust-native slices. ALWAYS use for merge/migration/unification dev operations in this repo — building rusty-idd, the Cargo-workspace restructure, porting the OpenSpec lifecycle to Rust, merging repos, sequencing the unification epic, planning a slice, implementing a migration, QA-ing a merge, checking Rust-native drift, or assembling merge-PR evidence. Also use for follow-up work: re-run, run again, update, revise, refine the plan/epic, redo only the implementation/QA/lifecycle-port of a slice, improve the previous result, or 'based on the previous run'. Simple one-off questions may be answered directly without the team."
---

# Merge Orchestrator

Coordinates a five-member agent team to run a **merge dev operation** end to end. It handles both ongoing maintenance (one PR-ready slice) and the larger **rusty-idd unification epic** (a sequenced restructure executed slice by slice). Governing rules live in `AGENTS.md` (merge discipline) and `GEMINI.md` (Rust-native invariant + worktree protocol).

The mission is to build **rusty-idd**: unify the three directors (`intent-driven-development`, `openspec-tui-main`, the OpenSpec lifecycle from `intent-driven-template`) into one Rust-native Cargo workspace. Two directors are already Rust and fold in as crates; the lifecycle engine is a constructive Node→Rust port owned by `lifecycle-porter`.

## Execution Mode: Hybrid (agent team, with a Producer–Reviewer loop)

| Phase | Mode | Reason |
|-------|------|--------|
| Phase 2 Analyze + extract lifecycle | Agent team | Analyst + lifecycle-porter gather facts and the spec contract in parallel |
| Phase 3 Sequence epic + plan slice | Agent team | Planner sequences the restructure, then details the next slice with porter/implementer input |
| Phase 4 Implement ↔ Verify | Agent team (Producer–Reviewer) | Real-time fix loop between implementer and QA minimizes rework |
| Phase 5 PR assembly | Leader (direct) | Single deterministic packaging step; no team needed |

One team spans phases 2–4. **For the unification epic, Phases 3–5 loop once per slice** (the epic sequence lives in `_workspace/02_planner_epic.md`); the leader carries the sequence across slices and the team persists between them. For a single maintenance change, the loop runs once.

## Agent Composition
| Member | Agent type (`subagent_type`) | Role | Skill | Output |
|--------|------------------------------|------|-------|--------|
| merge-analyst | `merge-analyst` | Inventory + feature matrix + env/secret contract + Rust-native baseline | `repo-inventory` | `_workspace/01_analyst_*.md` |
| lifecycle-porter | `lifecycle-porter` | Extract the OpenSpec lifecycle contract; design the Rust `crates/spec` engine + delta-merge | `lifecycle-porting` | `_workspace/01_lifecycle_contract.md`, `05_lifecycle_design.md` |
| merge-planner | `merge-planner` | Sequence the epic; plan the next slice (type + gate + rollback) | `vertical-slice-planning` | `_workspace/02_planner_epic.md`, `02_planner_slice.md` |
| rust-implementer | `rust-implementer` | Implement the slice Rust-native, deprecate-before-delete | `rust-native-implementation` | code + `_workspace/03_implementer_changes.md` |
| merge-qa | `merge-qa` | Cross-boundary verification (drift/gate/contract/CI/secret), incremental | `merge-verification` | `_workspace/04_qa_report.md` |

All Agent/TeamCreate calls use `model: "opus"`. For a pure maintenance run with no lifecycle work, `lifecycle-porter` may be omitted (4-member team); include it whenever the slice touches the spec engine.

## Workflow

### Phase 0: Context Check (follow-up support)
1. Check whether `_workspace/` exists.
2. Decide the mode:
   - **Absent** → initial run; proceed to Phase 1.
   - **Present + user asks for a partial change** (e.g. "redo only the QA", "refine the plan") → **partial re-run**: skip to the relevant phase and re-invoke only that member, passing the existing artifact paths so it revises in place.
   - **Present + new merge intent** → **fresh run**: move `_workspace/` to `_workspace_prev_<timestamp>/` (pass a timestamp in; do not call Date functions), then Phase 1.

### Phase 1: Preparation
1. Confirm a synced base: `rtk git fetch --all && rtk git status -sb`. Do not start on a stale tree.
2. Identify the merge intent, the repo/crate paths in scope, and which integration branch has authority.
3. Create `_workspace/` and save the intent + scope to `_workspace/00_input/`.

### Phase 2: Team Assembly + Analyze
1. `TeamCreate(team_name: "merge-dev-team", members: [merge-analyst, lifecycle-porter, merge-planner, rust-implementer, merge-qa])`, each `model: "opus"` with a prompt pointing to its agent definition and skill. (Omit `lifecycle-porter` for a maintenance run that doesn't touch the spec engine.)
2. `TaskCreate` the tasks with dependencies, ~5 per member:
   - analyst: scan the three directors, feature matrix, env/secret contract, Rust-native baseline;
   - lifecycle-porter: extract `01_lifecycle_contract.md`, design `crates/spec` in `05_lifecycle_design.md` (`depends_on` nothing — runs parallel to analyst);
   - planner: sequence the epic, plan the next slice (`depends_on` analyst + porter);
   - implementer: implement the slice, build (`depends_on` planner);
   - qa: verify each module incrementally (`depends_on` implementer, reopenable).
3. Analyst and lifecycle-porter run first in parallel; on completion they SendMessage the planner with artifact paths and alert QA if the **baseline already shows drift**.

### Phase 3: Sequence the Epic, then Plan the Slice
1. If the mission is the rusty-idd restructure, planner first writes `_workspace/02_planner_epic.md` — the ordered slices (workspace skeleton → fold in existing crates → port lifecycle → integrate), each tagged with its type (structural / lifecycle-port / migration) and correctness gate.
2. Planner then selects the **next** slice, writes `02_planner_slice.md` (with its slice type), and SendMessages the slice to the implementer plus the gate to QA so pass criteria are agreed up front. For a lifecycle-port slice the gate is the porter's golden fixtures, not parity.
3. If the next slice violates the Rust-native invariant **at the core-crate scope** (a foreign file, or a dep on the core crate — not on spec/runner/tui), planner escalates to the leader to re-scope.

### Phase 4: Implement ↔ Verify (Producer–Reviewer loop)
1. Implementer builds the slice additively (deprecate-before-delete), and on each completed module SendMessages QA "ready" with the change-log path.
2. QA runs `merge-verification` **incrementally** (drift-check first), writes `04_qa_report.md`, and SendMessages fix requests (file:line + how) to the implementer; for contract/manifest gaps it notifies **both** implementer and planner.
3. Loop until QA reports **PR-ready: yes**, capped at 3 rounds. Leader monitors via TaskGet and intervenes on idle/stall.

### Phase 5: PR Assembly (leader, direct)
1. Confirm `04_qa_report.md` says PR-ready: yes. If not, stay in the loop or report outstanding items.
2. Disband the team (`TeamDelete`); preserve `_workspace/`.
3. Use the `pr-evidence-bundle` skill to gather the AGENTS.md-required evidence and prepare the worktree→branch→commit→PR steps.
4. **Do not open/merge a PR without explicit user go-ahead** — surface the assembled bundle and let the user decide.
5. Report a summary: slice shipped, evidence status, drift verdict, open items.
6. **Epic advance:** if `02_planner_epic.md` has remaining slices, update it (mark this slice done), keep the team, and loop back to Phase 3 for the next slice. Otherwise the epic is complete.

## Data Flow
```
[Leader] Phase1 sync+intent → _workspace/00_input/
   │ TeamCreate(merge-dev-team, 5) + TaskCreate
   ▼
[merge-analyst]    01_analyst_*  ─┐
[lifecycle-porter] 01_contract +  ├─SendMessage(paths)─► [merge-planner] 02_epic + 02_slice
                   05_design      ┘  (baseline drift alert)        │ (slice type + gate)
                                          └────────► [merge-qa] ◄── gate (parity | build | fixtures)
                                                        ▲   │
[rust-implementer] code + 03_changes ───────────────────┘   ▼ fix requests (file:line)
        ▲───────── Producer–Reviewer loop (≤3) ──────────► 04_qa_report (PR-ready?)
   │  (epic: loop Phase 3–5 for next slice)
   ▼ TeamDelete (after last slice)
[Leader] pr-evidence-bundle → PR draft (await user go-ahead)
```

## Data Transfer Protocol
- **Task-based** (TaskCreate/Update) for coordination and dependencies.
- **Message-based** (SendMessage) for real-time hand-offs and the fix loop.
- **File-based** (`_workspace/{phase}_{agent}_{artifact}.md`, absolute paths) for every artifact — the audit trail. Final code lands in the crates; intermediates stay in `_workspace/`.

## Error Handling
| Situation | Strategy |
|-----------|----------|
| A member stalls/errs | Leader detects idle → SendMessage status check → restart or reassign its tasks; note any omission in the report |
| QA can't run a gate | Retry once; else mark that check **unverified** (never silent pass) and continue |
| Drift detected | Halt the slice; implementer ports to Rust-native; re-verify before proceeding |
| Loop exceeds 3 rounds | Escalate outstanding items to the user with QA evidence |
| Member data conflict | Cite both sources, keep both; record in `AI_MERGE/05_conflict_risk_register.md` |
| Slice infeasible Rust-native | Planner re-scopes; do not force a foreign dependency |

## Test Scenarios

### Happy path
1. User: "merge repo-a and repo-b" (or "do the merge dev operation").
2. Phase 1 syncs and records intent; Phase 2 assembles the team and analysts produce inventory + baseline.
3. Planner selects one slice with parity tests + rollback.
4. Implementer builds it Rust-native; QA verifies incrementally, drift-check clean, gates green → PR-ready: yes.
5. Leader assembles the evidence bundle and presents the PR draft for user go-ahead.
6. Expected: `_workspace/01..04` artifacts present, drift-check exit 0, PR bundle complete.

### Error path
1. During the lifecycle-port slice, the implementer adds `serde` to the **core** crate to finish faster (the spec crate may use serde_norway; the core may not).
2. QA's drift-check exits 1 — the core crate's `[dependencies]` is no longer empty → marks Rust-native drift **fail**, SendMessages a fix request: move the serialization to the `spec` crate edge, keep the core std-only.
3. Implementer relocates the dependency off the core crate, rebuilds.
4. QA re-verifies → drift-check clean, `cargo build/test --workspace` green, lifecycle fixtures conform → PR-ready: yes.
5. Report notes the drift was caught and corrected before PR.
