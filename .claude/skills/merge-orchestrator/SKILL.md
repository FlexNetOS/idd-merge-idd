---
name: merge-orchestrator
description: "Coordinates the merge-dev agent team for idd-merge-idd: Analyze → Plan → Implement → Verify → assemble PR evidence, producing one reviewable, parity-backed, Rust-native vertical slice. ALWAYS use for merge/migration/unification dev operations in this repo — merging repos, planning a slice, implementing a migration, QA-ing a merge, checking Rust-native drift, or assembling merge-PR evidence. Also use for follow-up work: re-run, run again, update, revise, refine the plan, redo only the implementation/QA of a slice, improve the previous result, or 'based on the previous run'. Simple one-off questions may be answered directly without the team."
---

# Merge Orchestrator

Coordinates a four-member agent team to run one **merge dev operation** end to end and deliver a single PR-ready vertical slice. Governing rules live in `AGENTS.md` (merge discipline) and `CLAUDE.md` (Rust-native invariant + worktree protocol).

## Execution Mode: Hybrid (agent team, with a Producer–Reviewer loop)

| Phase | Mode | Reason |
|-------|------|--------|
| Phase 2 Analyze | Agent team | Analysts share discoveries / flag conflicts in real time |
| Phase 3 Plan | Agent team | Planner pulls directly from analysts, negotiates feasibility |
| Phase 4 Implement ↔ Verify | Agent team (Producer–Reviewer) | Real-time fix loop between implementer and QA minimizes rework |
| Phase 5 PR assembly | Leader (direct) | Single deterministic packaging step; no team needed |

One team spans phases 2–4 (the members are needed throughout); the leader handles phase 5 after the team disbands.

## Agent Composition
| Member | Agent type (`subagent_type`) | Role | Skill | Output |
|--------|------------------------------|------|-------|--------|
| merge-analyst | `merge-analyst` | Inventory + feature matrix + env/secret contract + Rust-native baseline | `repo-inventory` | `_workspace/01_analyst_*.md` |
| merge-planner | `merge-planner` | Select ONE slice; migration intent, parity tests, gates, rollback | `vertical-slice-planning` | `_workspace/02_planner_slice.md` |
| rust-implementer | `rust-implementer` | Implement the slice Rust-native, deprecate-before-delete | `rust-native-implementation` | code + `_workspace/03_implementer_changes.md` |
| merge-qa | `merge-qa` | Cross-boundary verification (drift/parity/contract/CI/secret), incremental | `merge-verification` | `_workspace/04_qa_report.md` |

All Agent/TeamCreate calls use `model: "opus"`.

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
1. `TeamCreate(team_name: "merge-dev-team", members: [merge-analyst, merge-planner, rust-implementer, merge-qa])`, each `model: "opus"` with a prompt pointing to its agent definition and skill.
2. `TaskCreate` the slice's tasks with dependencies, ~5 per member:
   - analyst: scan repos, build feature matrix, map env/secret contract, record Rust-native baseline;
   - planner: select slice, write slice spec, emit idd task (`depends_on` analyst tasks);
   - implementer: implement slice, build (`depends_on` planner);
   - qa: verify each module incrementally (`depends_on` implementer, reopenable).
3. Analysts work first; on completion they SendMessage the planner with artifact paths and alert QA if the **baseline already shows drift**.

### Phase 3: Plan
Planner reads `_workspace/01_analyst_*.md`, selects ONE slice, writes `02_planner_slice.md`, and SendMessages the slice path to the implementer plus the parity tests/gates to QA (so pass criteria are agreed up front). If the only viable slice violates the Rust-native invariant, planner escalates to the leader to re-scope.

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

## Data Flow
```
[Leader] Phase1 sync+intent → _workspace/00_input/
   │ TeamCreate(merge-dev-team) + TaskCreate
   ▼
[merge-analyst] 01_analyst_*  ──SendMessage(paths)──► [merge-planner] 02_planner_slice
                   │ (baseline drift alert)                    │ (slice + gates)
                   └────────────► [merge-qa] ◄──────────── parity tests/gates
                                     ▲   │
[rust-implementer] code + 03_changes ┘   ▼ fix requests (file:line)
        ▲───────── Producer–Reviewer loop (≤3) ──────► 04_qa_report (PR-ready?)
   │ TeamDelete
   ▼
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
1. Implementer adds a crate to `idd` to finish faster.
2. QA's drift-check exits 1 (Cargo.lock package count > 1) → marks Rust-native drift **fail**, SendMessages a port-to-std fix request.
3. Implementer removes the dependency, re-implements with `std`, rebuilds.
4. QA re-verifies → drift-check clean, gates green → PR-ready: yes.
5. Report notes the drift was caught and corrected before PR.
