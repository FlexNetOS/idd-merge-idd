# rusty-idd Architecture Diagrams

Connected views showing actual API boundaries, call chains, and data flow between crates.

---

## 1. System Context — Who Talks to What

```
┌─────────────┐     ┌─────────────┐     ┌─────────────┐     ┌─────────────┐
│   Human     │     │  CI/CD      │     │   AI Agent  │     │   GitHub    │
│  Developer  │     │  (Actions)  │     │  (Claude/   │     │   API       │
│             │     │             │     │   Kimi/etc) │     │             │
└──────┬──────┘     └──────┬──────┘     └──────┬──────┘     └──────┬──────┘
       │                   │                   │                   │
       │  $ rusty-idd …    │  $ cargo test     │  reads/writes     │  PRs, Issues
       │  $ rusty-idd tui  │  --workspace      │  AGENTS.md        │  agent-orchestration
       │                   │                   │  openspec/        │  manifest updates
       │                   │                   │  .idd/MANIFEST    │
       └───────────────────┼───────────────────┼───────────────────┘
                           │                   │
                           ▼                   ▼
              ┌─────────────────────────────────────────────┐
              │         rusty-idd (crates/cli binary)       │
              │         unified clap entrypoint             │
              └─────────────────────┬───────────────────────┘
                                    │
              ┌─────────────────────┼─────────────────────┐
              │                     │                     │
              ▼                     ▼                     ▼
    ┌─────────────────┐   ┌─────────────────┐   ┌─────────────────┐
    │  File System    │   │  Child Processes│   │   Network       │
    │  - openspec/    │   │  - claude       │   │  - github.com   │
    │  - .idd/        │   │  - kimi         │   │  - api endpoints│
    │  - AGENTS.md    │   │  - aider        │   │                 │
    │  - .env*        │   │  - custom       │   │                 │
    └─────────────────┘   └─────────────────┘   └─────────────────┘
```

---

## 2. Crate Integration — Runtime Call Graph

This is the connected view. Arrows are **actual function calls** across crate boundaries.

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                           crates/cli (main.rs)                              │
│                         binary: rusty-idd                                   │
└─────────────────────────────────┬───────────────────────────────────────────┘
                                  │ clap parse
           ┌──────────────────────┼──────────────────────┐
           │                      │                      │
           ▼                      ▼                      ▼
  ┌─────────────────┐   ┌─────────────────┐   ┌─────────────────────────────┐
  │  core verbs     │   │  spec verbs     │   │      runtime verbs          │
  │                 │   │                 │   │                             │
  │ scan, plan,     │   │ spec validate   │   │  run <change> ─────────┐    │
  │ task, manifest, │   │ spec archive    │   │  tui  ───────────────┐ │    │
  │ validate, github│   │ spec show       │   │                      │ │    │
  │ init            │   │ spec sync       │   │                      │ │    │
  └────────┬────────┘   └────────┬────────┘   └──────────────────────┼─┼────┘
           │                     │                                   │ │
           │                     │                                   │ │
           ▼                     ▼                                   │ │
  ┌─────────────────┐   ┌─────────────────┐                         │ │
  │  crates/core    │   │  crates/spec    │                         │ │
  │  rusty-idd-core │   │  rusty-idd-spec │                         │ │
  │                 │   │                 │                         │ │
  │ cli::run(argv)  │   │ parse_spec()    │                         │ │
  │ (reconstructs   │   │ validate_spec() │                         │ │
  │  argv as        │   │ sync_one()      │                         │ │
  │  ["idd", verb]) │   │ archive_specs() │                         │ │
  │                 │   │ apply_delta()   │                         │ │
  │                 │   │ load_schema()   │                         │ │
  └─────────────────┘   └─────────────────┘                         │ │
                                                                    │ │
              ┌─────────────────────────────────────────────────────┘ │
              │                                                       │
              ▼                                                       ▼
    ┌─────────────────────────────┐         ┌─────────────────────────────────┐
    │      crates/runner          │         │         crates/tui              │
    │     rusty-idd-runner        │◀────────│       rusty-idd-tui             │
    │                             │  re-export│                               │
    │ config::TuiConfig::load()   │   (pub use)│  run() → Result<(), Error>   │
    │ runner::start_implementation│◀───────────│  app.rs uses:               │
    │ runner::start_apply()       │            │   crate::runner::…          │
    │ runner::stop_implementation │            │   crate::data::…            │
    │                             │            │   crate::config::…          │
    │ Returns: ImplState          │            │                             │
    │ Stream: ImplUpdate          │            │                             │
    │  (Progress/Finished/        │            │                             │
    │   Stalled/Error)            │            │                             │
    └─────────────────────────────┘            └─────────────────────────────┘
```

**Key seam**: `cli` is the **only** crate that imports all four others. `tui` does not import `cli` or `core` or `spec` directly — it only imports `runner` and re-exports it internally.

---

## 3. Data Structures Crossing Crate Boundaries

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                         crates/cli                                          │
│  Parses argv, deserializes JSON, maps errors to exit codes                  │
└─────────────────────────────────┬───────────────────────────────────────────┘
                                  │
        ┌─────────────────────────┼─────────────────────────┐
        │                         │                         │
        ▼                         ▼                         ▼
┌───────────────┐       ┌─────────────────┐       ┌─────────────────┐
│ crates/core   │       │ crates/spec     │       │ crates/runner   │
│               │       │                 │       │                 │
│ Receives:     │       │ Receives:       │       │ Receives:       │
│  Vec<String>  │       │  &str (markdown)│       │  &str change_name│
│  (argv)       │       │  &str (delta)   │       │  &TuiConfig     │
│               │       │                 │       │                 │
│ Returns:      │       │ Returns:        │       │ Returns:        │
│  Result<(),   │       │  ParsedSpec     │       │  ImplState      │
│   String>     │       │  ValidationReport│      │  (channel send) │
│               │       │  SyncResult     │       │                 │
│               │       │  ArchiveManifest│       │ Streams:        │
│               │       │                 │       │  ImplUpdate     │
│               │       │                 │       │   - Progress    │
│               │       │                 │       │   - Finished    │
│               │       │                 │       │   - Stalled     │
│               │       │                 │       │   - Error       │
└───────────────┘       └─────────────────┘       └─────────────────┘
        │                         │                         │
        ▼                         ▼                         ▼
   Writes to FS              Writes to FS              Spawns child
   (MANIFEST.tsv,           (archive/,                processes
   openspec/tasks.md)        delta.md,                 (claude, kimi)
                            reports/)                  Writes log files
```

---

## 4. OpenSpec Lifecycle — Internal Pipeline (crates/spec)

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  Input: openspec/changes/<name>/{proposal,design,spec,tasks,adr}.md        │
└─────────────────────────────────┬───────────────────────────────────────────┘
                                  │
                                  ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│                              parse/                                         │
│  comrak ──▶ AST ──▶ front-matter extraction ──▶ ParsedSpec (model/)        │
└─────────────────────────────────┬───────────────────────────────────────────┘
                                  │
              ┌───────────────────┼───────────────────┐
              │                   │                   │
              ▼                   ▼                   ▼
    ┌─────────────────┐ ┌─────────────────┐ ┌─────────────────┐
    │    validate/    │ │    archive/     │ │  delta_merge/   │
    │                 │ │                 │ │                 │
    │ load_schema()   │ │ Move to         │ │ diff two        │
    │ (serde_norway)  │ │ archive/        │ │ ParsedSpec      │
    │                 │ │ with date       │ │ trees           │
    │ Check against   │ │ prefix          │ │                 │
    │ schema.yaml     │ │                 │ │ Produce         │
    │                 │ │ Update          │ │ delta.md        │
    │ Report:         │ │ manifest        │ │                 │
    │  IssueLevel     │ │                 │ │                 │
    │  Counts/Summary │ │                 │ │                 │
    └────────┬────────┘ └────────┬────────┘ └────────┬────────┘
             │                   │                   │
             ▼                   ▼                   ▼
    ┌─────────────────┐ ┌─────────────────┐ ┌─────────────────┐
    │  Report output  │ │  Archive output │ │  Delta output   │
    │  (TOML/JSON)    │ │  (FS mutation)  │ │  (markdown)     │
    └─────────────────┘ └─────────────────┘ └─────────────────┘

  ┌─────────────────────────────────────────────────────────────────────────┐
  │  Hexagonal wall (per spec-engine-design.md):                            │
  │                                                                         │
  │   ┌─────────────┐                                                       │
  │   │   model/    │ ◀── pure data structs, ZERO external deps            │
  │   │ (ParsedSpec,│     no serde, no comrak, no fs                        │
  │   │  Issue, etc)│                                                       │
  │   └──────┬──────┘                                                       │
   │          │                                                             │
  │   ┌──────┴──────┬──────────────┬──────────────┐                        │
  │   │             │              │              │                         │
  │   ▼             ▼              ▼              ▼                         │
  │ parse/      validate/     archive/     delta_merge/                     │
  │ (comrak)    (serde)       (fs ops)     (diff logic)                     │
  └─────────────────────────────────────────────────────────────────────────┘
```

---

## 5. Runner/TUI Execution Loop — Connected Flow

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                          User Types:                                        │
│                    $ rusty-idd tui                                          │
└─────────────────────────────────┬───────────────────────────────────────────┘
                                  │
                                  ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│                         crates/tui                                          │
│                        rusty_idd_tui::run()                                 │
│                                                                             │
│   ┌─────────────┐    ┌─────────────┐    ┌─────────────┐                    │
│   │ ChangeList  │───▶│ArtifactMenu │───▶│ ArtifactView│  (screen stack)    │
│   │  Screen     │    │  Screen     │    │  Screen     │                    │
│   └─────────────┘    └─────────────┘    └─────────────┘                    │
│          │                                                            │
│          │ User presses "Run" on a change                             │
│          ▼                                                            │
│   ┌─────────────────────────────────────────────────────────────┐      │
│   │  Calls: crate::runner::start_implementation(change, config) │      │
│   │         crate::runner::start_apply(change, config)          │      │
│   └─────────────────────────────────────────────────────────────┘      │
│                                  │                                     │
│                                  │ spawns worker thread                 │
│                                  ▼                                     │
│   ┌─────────────────────────────────────────────────────────────┐      │
│   │  Receives: crate::runner::ImplUpdate via mpsc channel       │      │
│   │   - Progress { current, total }                             │      │
│   │   - Finished                                                │      │
│   │   - Stalled                                                 │      │
│   │   - Error                                                   │      │
│   └─────────────────────────────────────────────────────────────┘      │
└─────────────────────────────────┬───────────────────────────────────────────┘
                                  │
                                  ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│                         crates/runner                                       │
│                                                                             │
│   ┌─────────────────────────────────────────────────────────────────────┐  │
│   │  runner::start_implementation(change_name: &str, config: &TuiConfig)│  │
│   │                                                                     │  │
│   │  1. config.build_command(prompt)                                    │  │
│   │     → splits "claude --print --dangerously-skip-permissions {prompt}"│  │
│   │     → replaces {prompt} token                                       │  │
│   │                                                                     │  │
│   │  2. FOR each unchecked task in openspec/changes/<name>/tasks.md:   │  │
│   │        - data::next_unchecked_task()                                │  │
│   │        - config.render_prompt(name)                                 │  │
│   │        - std::process::Command::spawn()                             │  │
   │        - stream stdout/stderr → log file                             │  │
│   │        - parse_task_progress() → checkbox state                     │  │
│   │        - tx.send(ImplUpdate::Progress)                              │  │
│   │                                                                     │  │
│   │  3. Returns ImplState { rx, join_handle, log_path }                 │  │
│   └─────────────────────────────────────────────────────────────────────┘  │
│                                                                             │
│   Supporting APIs (data/):                                                  │
│   - list_changes() / list_archived_changes()                                │
│   - get_change_status(name) / get_archived_change_status(dir)               │
│   - read_artifact_content(path) / parse_task_progress(path)                 │
│   - load_change_dependencies() / generate_dependency_graph()                │
│   - read_change_config() / write_change_config()                            │
│                                                                             │
│   Supporting APIs (config/):                                                │
│   - TuiConfig::load_from(path) / TuiConfig::save_to(path)                   │
│   - TuiConfig::render_prompt(name) / TuiConfig::build_command(prompt)       │
│   - CONFIG_PATH constant                                                    │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## 6. Core Control Plane — Scan → Plan → Task Chain

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                    $ rusty-idd scan --repo <path>                           │
└─────────────────────────────────┬───────────────────────────────────────────┘
                                  │
                                  ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│                         crates/core                                         │
│                    rusty_idd_core::cli::run(argv)                           │
└─────────────────────────────────┬───────────────────────────────────────────┘
                                  │
        ┌─────────────────────────┼─────────────────────────┐
        │                         │                         │
        ▼                         ▼                         ▼
┌───────────────┐       ┌─────────────────┐       ┌─────────────────┐
│    scanner    │       │     planner     │       │  task engine    │
│               │       │                 │       │                 │
│ fs_utils::    │       │ planner::       │       │ templates::     │
│ stable_walk() │──────▶│ generate_plan() │──────▶│ render_task()   │
│               │       │                 │       │                 │
│ Detects:      │       │ Reads:          │       │ Produces:       │
│  languages    │       │  manifest       │       │  narrow task    │
│  package mgrs │       │  env contract   │       │  descriptions   │
│  entrypoints  │       │  feature matrix │       │  per slice      │
│  workflows    │       │                 │       │                 │
│  agent files  │       │                 │       │                 │
└───────┬───────┘       └─────────────────┘       └─────────────────┘
        │
        ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│                              Outputs                                        │
│                                                                             │
│  .idd/MANIFEST.tsv         ←── inventory + audit baseline                   │
│  openspec/                 ←── spec scaffolding                             │
│  AGENTS.md updates         ←── agent control plane                          │
│  env contract maps         ←── secret provider registry                     │
│  feature parity matrix     ←── migration tracking                           │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## 7. Unification → Integration → Runtime — The Full Stack

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                         LAYER 0: Sources (Retired)                          │
├─────────────────────────────────────────────────────────────────────────────┤
│  intent-driven-devel (Node)  openspec-tui-main (Rust)  OpenSpec CLI (Node) │
└─────────────────────────────────────────────────────────────────────────────┘
                                    │
                                    ▼ merged & ported
┌─────────────────────────────────────────────────────────────────────────────┐
│                      LAYER 1: Workspace Crates                              │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│   ┌─────────────┐  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐      │
│   │    core     │  │    spec     │  │   runner    │  │    tui      │      │
│   │  (std-only) │  │ (comrak +   │  │ (serde +    │  │ (ratatui +  │      │
│   │             │  │  serde_norw │  │  chrono)    │  │  crossterm) │      │
│   │ repo ctrl   │  │ lifecycle   │  │ exec engine │  │ interactive │      │
│   │ plane       │  │ engine      │  │             │  │ frontend    │      │
│   └──────┬──────┘  └──────┬──────┘  └──────┬──────┘  └──────┬──────┘      │
│          │                │                │                │             │
│          └────────────────┴────────────────┴────────────────┘             │
│                                     │                                     │
│                              NO cycles allowed                            │
│                              (DAG only: tui → runner)                     │
│                                                                           │
└───────────────────────────────────────────────────────────────────────────┘
                                    │
                                    ▼ wires together
┌─────────────────────────────────────────────────────────────────────────────┐
│                      LAYER 2: Unified Binary                                │
├─────────────────────────────────────────────────────────────────────────────┤
│                         crates/cli → rusty-idd                              │
│                                                                             │
│   argv parse ──▶ dispatch ──▶ core::cli::run()                              │
│                           ──▶ spec::validate_spec() / archive_specs()       │
│                           ──▶ runner::start_implementation()                │
│                           ──▶ tui::run()                                    │
└─────────────────────────────────────────────────────────────────────────────┘
                                    │
                                    ▼ user interface
┌─────────────────────────────────────────────────────────────────────────────┐
│                      LAYER 3: Consumption                                   │
├─────────────────────────────────────────────────────────────────────────────┤
│   Headless:  $ rusty-idd scan | plan | run | spec validate                  │
│   Interactive: $ rusty-idd tui                                              │
│   Agent-driven: AGENTS.md + .idd/MANIFEST.tsv + openspec/ changes           │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## 8. The Merge Loop — Agent Orchestration (Connected to Runtime)

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                    Agent Team (Skills)                                      │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  ┌─────────────┐    ┌─────────────┐    ┌─────────────┐    ┌─────────────┐  │
│  │   repo-     │    │  vertical-  │    │ rust-native-│    │   merge-    │  │
│  │  inventory  │───▶│ slice-plan  │───▶│ implement   │───▶│ verification│  │
│  │             │    │             │    │             │    │             │  │
│  │ scan both   │    │ one narrow  │    │ behavior-   │    │ drift detec │  │
│  │ sides       │    │ slice +     │    │ preserving  │    │ parity test │  │
│  │ emit feature│    │ parity tests│    │ Rust code   │    │ CI gates    │  │
│  │ matrix      │    │ + rollback  │    │             │    │             │  │
│  └─────────────┘    └──────┬──────┘    └──────┬──────┘    └──────┬──────┘  │
│                            │                    │                    │       │
│                            └────────────────────┴────────────────────┘       │
│                                                 │                            │
│                                                 ▼                            │
│                                    ┌─────────────────────────────┐          │
│                                    │     pr-evidence-bundle      │          │
│                                    │  (build/test/lint/scan/     │          │
│                                    │   migration note/rollback/  │          │
│                                    │   manifest update)          │          │
│                                    └─────────────┬───────────────┘          │
│                                                  │                         │
│                                                  ▼                         │
│                                    ┌─────────────────────────────┐          │
│                                    │     git commit / PR         │          │
│                                    │     .idd/MANIFEST.tsv       │          │
│                                    │     AI_MERGE/ record        │          │
│                                    └─────────────┬───────────────┘          │
│                                                  │                         │
│                                                  ▼                         │
│                                    ┌─────────────────────────────┐          │
│                                    │   session-relay HANDOFF.md  │          │
│                                    │   (resume next slice cold)  │          │
│                                    └─────────────────────────────┘          │
│                                                                             │
│  ═══════════════════════════════════════════════════════════════════════   │
│                                                                             │
│  This loop DRIVES the workspace above: each cycle mutates openspec/        │
│  changes, .idd/MANIFEST.tsv, and crate source, then verifies with          │
│  cargo test --workspace and cargo clippy --workspace.                      │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## 9. User Journey — The Vision (End-to-End Autonomous Delivery)

> **Vision:** A non-technical user makes a request. `rusty-idd` delivers the product with full end-to-end autonomous execution.

This diagram reads left-to-right as a human story. The crates, call graphs, and data structures from diagrams 1–8 are the hidden engine inside Stage 3.

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                         STAGE 0: HUMAN INTENT                               │
│                                                                             │
│   ┌─────────────────────────────────────────────────────────────────────┐   │
│   │  Any user. Any skill level.                                         │   │
│   │                                                                     │   │
│   │  "Build me a login page with email + OAuth.                         │   │
│   │   Make it accessible. Deploy it to a preview URL."                  │   │
│   │                                                                     │   │
│   │  Interaction modes:                                                 │   │
│   │   • One-shot CLI:  $ rusty-idd task "..."                         │   │
│   │   • Interactive TUI: $ rusty-idd tui  → describe → submit         │   │
│   │   • File drop:       Place a mock/wireframe in openspec/inbox/    │   │
│   └─────────────────────────────────────────────────────────────────────┘   │
└─────────────────────────────────┬───────────────────────────────────────────┘
                                  │
                                  ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│                      STAGE 1: INTENT → SPEC                                 │
│                                                                             │
│   rusty-idd translates the human request into a structured OpenSpec change: │
│                                                                             │
│   ┌─────────────┐    ┌─────────────┐    ┌─────────────┐    ┌─────────────┐ │
│   │ proposal.md │───▶│  design.md  │───▶│   spec.md   │───▶│  tasks.md   │ │
│   │             │    │             │    │             │    │ (checklist) │ │
│   │ What & why  │    │ How         │    │ Exact reqs  │    │ Step-by-step│ │
│   └─────────────┘    └─────────────┘    └─────────────┘    └─────────────┘ │
│                                                                             │
│   AI-assisted drafting (human reviews / edits / approves)                   │
│   └── stored in: openspec/changes/<name>/                                   │
│                                                                             │
└─────────────────────────────────┬───────────────────────────────────────────┘
                                  │
                                  ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│                   STAGE 2: VALIDATE & PLAN SLICES                           │
│                                                                             │
│   rusty-idd validates the spec against schema.yaml:                         │
│   ┌─────────────────────────────────────────────────────────────────────┐   │
│   │  $ rusty-idd spec validate openspec/changes/<name>/spec.md        │   │
│   │  ✓ Requirements complete  ✓ Design linked  ✓ Tasks checkable    │   │
│   └─────────────────────────────────────────────────────────────────────┘   │
│                                                                             │
│   Then plans vertical slices (one reviewable PR each):                      │
│   ┌─────────────────────────────────────────────────────────────────────┐   │
│   │  Slice 1: OAuth scaffold + provider config                          │   │
│   │  Slice 2: Email/password auth + validation                          │   │
│   │  Slice 3: Accessibility audit + fixes                               │   │
│   │  Slice 4: Preview deployment wiring                                 │   │
│   └─────────────────────────────────────────────────────────────────────┘   │
│                                                                             │
└─────────────────────────────────┬───────────────────────────────────────────┘
                                  │
                                  ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│              STAGE 3: AUTONOMOUS EXECUTION LOOP                             │
│      (This is where diagrams 1–8 live: crates/cli/core/spec/runner/tui)     │
│                                                                             │
│   ┌─────────────────────────────────────────────────────────────────────┐   │
│   │                         WORKER LOOP                                 │   │
│   │                                                                     │   │
│   │   ┌─────────┐   ┌─────────┐   ┌─────────┐   ┌─────────┐          │   │
│   │   │  Read   │──▶│ Spawn   │──▶│ Receive │──▶│ Verify  │          │   │
│   │   │  next   │   │  AI     │   │  code   │   │  tests  │          │   │
│   │   │  task   │   │  agent  │   │  patch  │   │  pass?  │          │   │
│   │   └────┬────┘   └────┬────┘   └────┬────┘   └────┬────┘          │   │
│   │        │             │             │             │                │   │
│   │        │  tasks.md   │  claude /   │  stdout /   │  cargo test    │   │
│   │        │  checkbox   │  kimi /     │  git diff   │  cargo clippy  │   │
│   │        │  state      │  aider      │             │  secret scan   │   │
│   │        │             │             │             │                │   │
│   │        └─────────────┴─────────────┴─────────────┘                │   │
│   │                      │                                            │   │
│   │                      ▼                                            │   │
│   │              ┌───────────────┐                                    │   │
│   │              │ Mark complete │───────┐  if pass: next task       │   │
│   │              │ or retry/stall│       │  if fail: retry or halt   │   │
│   │              └───────────────┘       │                           │   │
│   │                      ▲               │                           │   │
│   │                      └───────────────┘                           │   │
│   │                                                                     │   │
│   │   Visibility: TUI progress bar  OR  headless log stream           │   │
│   │                                                                     │   │
│   └─────────────────────────────────────────────────────────────────────┘   │
│                                                                             │
│   Guardrails (AGENTS.md enforced):                                          │
│   • Never delete source without parity test                                 │
│   • One slice per cycle                                                     │
│   • Every PR needs evidence bundle                                          │
│   • .idd/MANIFEST.tsv is the audit baseline                                 │
│                                                                             │
└─────────────────────────────────┬───────────────────────────────────────────┘
                                  │
                                  ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│              STAGE 4: REVIEWABLE DELIVERABLE                                │
│                                                                             │
│   Per slice, rusty-idd assembles the PR evidence bundle:                    │
│   ┌─────────────────────────────────────────────────────────────────────┐   │
│   │  [x] Build:   cargo build --workspace   ✓                         │   │
│   │  [x] Test:    cargo test --workspace    ✓                         │   │
│   │  [x] Lint:    cargo clippy --workspace  ✓                         │   │
│   │  [x] Secrets: cargo audit / scan        ✓                         │   │
│   │  [x] Migration note: old path → new path                          │   │
│   │  [x] Rollback path documented                                     │   │
│   │  [x] .idd/MANIFEST.tsv updated                                    │   │
│   └─────────────────────────────────────────────────────────────────────┘   │
│                                                                             │
│   Output formats:                                                           │
│   • Git branch ready for PR                                                 │
│   • AI_MERGE/<slice>.md record                                              │
│   • Updated manifest + env contract                                         │
│                                                                             │
└─────────────────────────────────┬───────────────────────────────────────────┘
                                  │
                                  ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│              STAGE 5: PRODUCT DELIVERED                                     │
│                                                                             │
│   ┌─────────────────────────────────────────────────────────────────────┐   │
│   │                                                                     │   │
│   │   The non-technical user receives:                                  │   │
│   │                                                                     │   │
│   │   • Working, tested code                                            │   │
│   │   • A link to the PR / branch                                       │   │
│   │   • A summary of what was built (in natural language)               │   │
│   │   • Confidence: tests pass, secrets clean, rollback known           │   │
│   │                                                                     │   │
│   │   They did NOT need to know about:                                  │   │
│   │   • Cargo, crates, or Rust editions                                 │   │
│   │   • comrak, serde_norway, or mpsc channels                          │   │
│   │   • OpenSpec schema internals                                       │   │
│   │                                                                     │   │
│   └─────────────────────────────────────────────────────────────────────┘   │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

### How the two views relate

| User Journey Stage | AI/Engineer Navigation (Diagrams 1–8) |
|--------------------|---------------------------------------|
| Stage 0: Intent    | Diagram 1 (System Context) — who interacts |
| Stage 1: Spec      | Diagram 4 (OpenSpec Pipeline) — how spec is parsed/validated |
| Stage 2: Plan      | Diagram 6 (Scan→Plan→Task) — core control plane |
| Stage 3: Execute   | Diagrams 2, 3, 5 (Crate calls, data flow, runner loop) |
| Stage 4: Evidence  | Diagram 8 (Merge Loop) — PR bundle assembly |
| Stage 5: Delivery  | Diagram 7 (Full Stack) — how sources → crates → binary → user |
