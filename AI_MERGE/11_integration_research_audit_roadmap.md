# Integration Research, Audit & Roadmap

> **Scope:** Connect `prompt_hub` → `weave` → `rusty-idd` into the unified autonomous delivery pipeline defined in `docs/rusty-idd/proposal.md`.
> **Date:** 2026-06-06
> **Method:** repo-audit skill (hotfiles, ownership, secret scan) + deep source analysis across all 3 repos.

---

## 1. Executive Summary

**The vision:** `user-request → prompt_hub → weave+rtk → rusty-idd → product`

**The reality today:** Three independent Cargo workspaces with **zero cross-repo imports, zero shared libraries, and zero orchestration scripts**. They are sibling peers under FlexNetOS (`.meta.yaml`) with no `depends_on` edges.

**Corrected understanding of weave:** `weave` is **still in construction mode**. It is not a finished agent-mesh utility — it is a **Rust port of Pipedrive (CRM/pipeline) + MCP server + agent-mesh add-ons**. Its final surface is uncertain. The CLI may or may not survive. The MCP layer is the most stable integration target.

**Corrected understanding of prompt_hub ↔ rusty-idd:** `prompt_hub` **can generate the Rust-native OpenSpec** artifacts using `rusty-idd`'s spec engine as the format authority. rusty-idd owns the OpenSpec contract; prompt_hub is a producer.

**The path forward:** 
- **Primary integration:** `prompt_hub → rusty-idd` via OpenSpec data contract (prompt_hub generates, rusty-idd consumes).
- **Secondary integration:** `rusty-idd ↔ weave` via **MCP client only**, feature-gated, experimental — because weave is in flux.
- **No deep code coupling** until weave stabilizes.

---

## 2. Per-Repo Integration Surface (Research)

### 2.1 rusty-idd (`/home/drdave/Desktop/idd-merge-idd`)

| Layer | Surface | Stability |
|-------|---------|-----------|
| **Binary** | `rusty-idd` (single binary, `crates/cli`) | Stable |
| **Core verbs** | `scan`, `plan`, `task`, `validate`, `manifest`, `github`, `init` | Stable |
| **Spec lifecycle** | `spec validate`, `spec archive`, `spec show`, `spec sync`, `spec status`, `spec next`, `spec adr`, `spec scaffold`, `spec new` | Stable (golden-oracle parity) |
| **Runner** | `run <change>` (headless), `tui` (interactive) | Stable |
| **Core API** | `rusty_idd_core::cli::run(argv)` — std-only, zero-dep | Stable |
| **Spec API** | `rusty_idd_spec::{parse_spec, validate_spec, sync_one, archive_specs, scaffold_render, …}` | Stable |
| **Runner API** | `rusty_idd_runner::{config, data, runner}` | Stable |
| **Config files** | `openspec/tui-config.yaml`, `openspec/config.yaml`, `openspec/schemas/intent-driven/schema.yaml`, `change-config.yaml` | Stable |
| **Manifest** | `.idd/MANIFEST.tsv` (FNV-1a 64-bit) | Stable |
| **Contract** | `AGENTS.md` (static template in core) | Stable |
| **Dir structure** | `openspec/changes/<name>/{proposal,design,spec,tasks,adr}.md` + `archive/` | Stable |

**Key constraint:** `crates/core` is **std-only / zero-dependency**. No network, no serde, no async. Any integration that needs tokio/HTTP/serde must live in `crates/cli`, `crates/runner`, or a new adapter crate.

**Key debt:** `runner::data::list_changes` and `get_change_status` still spawn the **Node `openspec` CLI** as an external binary. The Rust spec engine exists but these runner paths are legacy.

---

### 2.2 prompt_hub (`/home/drdave/Desktop/meta/prompt_hub`)

| Layer | Surface | Stability |
|-------|---------|-----------|
| **Library** | `prompt-hub` crate (`lib.rs`) — `PromptHub::new(db_path, config).await` | Stable core; many feature-gated stubs |
| **CLI** | `prompthub <command>` (~35 subcommands) | Stable for CRUD + search + audit + export/import |
| **HTTP API** | `prompthub-server` (Axum, port 8080) — 15 routes | Stable CRUD; some routes scaffolded |
| **Core methods** | `.register()`, `.get(role, intent)`, `.search(query, mode)`, `.vibe_code()`, `.gather_context()`, `.estimate_cost()`, `.fallback_chain()` | Stable |
| **Templates** | `base_orchestrator`, `base_architect`, `base_implementer`, `base_critic`, `base_reviewer`, `handoff_standard` | Stable |
| **Config** | `~/.config/prompthub/config.toml` (TOML), `PROMPTHUB_CONFIG` env | Stable |
| **Export formats** | JSONL, YAML, Markdown | Stable |
| **WebSocket** | `WebSocketSync` (scaffolded, not production) | Unstable |
| **MCP** | **None** | N/A |
| **gRPC** | **None** | N/A |

**Stubbed features (not blocking, but incomplete):** `diff`, `unlock`, `lineage`, `preview`, `scan`, `deploy`, `summarize`, `voice`, `onboard`, `heal`, `quota`, `tui`, `server` (CLI stub), SmartEngine ONNX wiring.

**Hotfiles:** `storage.rs`, `search.rs`, `models.rs`, `hub.rs`, `routes.rs`, `main.rs` (CLI), `templates.rs`.

---

### 2.3 weave (`/home/drdave/Desktop/meta/weave`)

**Nature of the repo:** Under construction. Combines:
- **MCP server** (35+ tools) — stable surface
- **Rust port of Pipedrive** (CRM/pipeline: deals, stages, activities, contacts) — in progress
- **Agent-mesh add-ons** (messaging, injection, federation) — in progress

| Layer | Surface | Stability |
|-------|---------|-----------|
| **Binary** | `weave` (single binary, **no `[lib]`**) | v0.1.0 MVP |
| **CLI** | ~30 subcommands | **Uncertain future** — user "not sure on the cli" |
| **MCP server** | `weave mcp` — 35+ tools over stdio JSON-RPC 2.0 | **Most stable integration target** |
| **Storage** | SQLite (default) or libSQL/Turso | Schema is internal impl detail |
| **Config** | `~/.config/weave/config.toml` + env vars | Stable |

**Critical limitation:** `weave` is **binary-only** and **in construction**. No `libweave`. The CLI may change or be reduced. The only safe integration surface is **MCP**.

**Integration rule for weave:** Treat it as an **external MCP service**. No library dependency. No CLI parsing. Feature-gate everything.

**Known issues:** Unresolved merge conflicts in `src/main.rs` (lines 483/521) and `src/store_libsql.rs` (lines 96/210, 593/602).

---

## 3. Gap Analysis

### 3.1 Data Format Gaps

| What flows | From | To | Gap |
|------------|------|-----|-----|
| User intent | prompt_hub `vibe_code()` | rusty-idd OpenSpec | `VibeResult` has **no OpenSpec export**. Need to add `prompthub export-openspec` or equivalent. |
| Prompt bundle | prompt_hub templates | rusty-idd runner config | rusty-idd `TuiConfig` expects a **single command template** (`claude --print …`). No `prompt_hub` role-aware bundle loading. |
| Agent coordination | rusty-idd runner | weave CRM/mesh | rusty-idd spawns child processes directly. No MCP client integration to weave (`weave_job_create`, `weave_send`, etc.). |
| Progress telemetry | rusty-idd runner | weave CRM/pipeline | `ImplUpdate` (Progress/Finished/Stalled/Error) goes to TUI channel only. No external broadcast to weave. |
| Context gather | prompt_hub `gather_context()` | rusty-idd `scan` | `gather_context()` reads a project path and returns `ProjectContext`. rusty-idd `scan` returns `RepoInventory`. **No shared type, no merge logic.** |
| Audit trail | rusty-idd `.idd/MANIFEST.tsv` | prompt_hub audit | prompt_hub has `audit_trail()` for prompts. No concept of repo-manifest or env-contract audit. |
| CRM pipeline | weave (Pipedrive port) | rusty-idd changes | No mapping between weave "deals/stages" and rusty-idd "OpenSpec changes/tasks". |

### 3.2 API Gaps

| Gap | Severity | Notes |
|-----|----------|-------|
| weave is **in construction** — surface unstable | High | CLI uncertain. CRM/pipeline shape evolving. **Only MCP is safe to target.** |
| weave has **no library API** | High | Forces MCP client integration. No `use weave::...`. |
| prompt_hub has **no MCP server** | Medium | weave speaks MCP; prompt_hub does not. Asymmetry in agent-tool integration. |
| rusty-idd has **no async runtime** | Medium | core is std-only; runner is sync. prompt_hub and weave are async (tokio). Cannot directly `await PromptHub::new()`. |
| No **shared crate** for data types | Medium | Each repo redefines similar concepts (Task, Change, Message, Peer). |
| rusty-idd runner **still spawns Node `openspec`** | High | Legacy dependency that spec engine was supposed to replace. |
| No **orchestration script** (docker-compose/justfile) | Low | All three must be started manually. No unified startup. |

### 3.3 Configuration Gaps

| Config | rusty-idd | prompt_hub | weave |
|--------|-----------|------------|-------|
| Config file | `openspec/tui-config.yaml` | `~/.config/prompthub/config.toml` | `~/.config/weave/config.toml` |
| Env prefix | None | `PROMPTHUB_*` | `WEAVE_*` |
| DB path | N/A (file-based) | `prompthub.db` (libsql) | `~/.local/share/weave/messages.db` |
| Identity | N/A | `AgentIdentity` (token-based) | `WEAVE_SESSION` (name-based) |
| Auth | N/A | RBAC (argon2id) | Signed identity (Ed25519, optional) |

**No unified identity:** A user/agent has one identity in prompt_hub, a different name in weave, and no identity in rusty-idd.

---

## 4. Risk Register

| # | Risk | Likelihood | Impact | Mitigation |
|---|------|------------|--------|------------|
| R1 | weave is in construction — surface may change under us | High | High | Integrate **only via MCP**, feature-gated. No lib dependency. Re-evaluate after weave stabilizes. |
| R2 | weave merge conflicts block clean builds | High | Low | Fix conflicts in `main.rs` + `store_libsql.rs` before any integration PR |
| R3 | rusty-idd core purity regresses if async/serde deps leak in | Medium | High | New integration code lives in `crates/cli` or new `crates/adapter`, never in `core` |
| R4 | prompt_hub stubs (preview, scan, deploy) are needed for full pipeline | Medium | Medium | Defer pipeline features that depend on stubs; implement stubs in parallel |
| R5 | No shared identity/auth across repos | Medium | Medium | Define env-contract mapping (table in §3.3); implement in Slice 2 |
| R6 | `runner::data` spawns Node `openspec` — breaks zero-Node goal | High | High | **Prerequisite:** port `list_changes`/`get_change_status` to use `rusty-idd-spec` instead |
| R7 | Over-eager deep coupling (making all one workspace) | Medium | High | Keep repos independent; integrate via contracts and MCP, not code coupling |
| R8 | weave CLI may be deprecated/removed | Medium | Medium | Do not build CLI-parsing integration. MCP only. |

---

## 5. Proposed Integration Contracts

### Contract 1: Intent Export (prompt_hub → rusty-idd)

**Authority:** `rusty-idd` owns the OpenSpec format. `prompt_hub` generates artifacts that conform to it.

```bash
# Option A: prompt_hub calls rusty-idd CLI
prompthub vibe "Build me a login page" --json \
  | rusty-idd spec from-intent --out-dir ./openspec/changes/login-page/

# Option B: prompt_hub exports directly (if it links rusty-idd-spec)
prompthub vibe "Build me a login page" \
  --export-format openspec \
  --out-dir ./openspec/changes/login-page/
```

**Recommended (Option A):** Keep repos decoupled. prompt_hub emits a structured intent JSON. rusty-idd's spec engine converts it to OpenSpec. This makes rusty-idd the format authority and prompt_hub a producer.

**Output:** `openspec/changes/<name>/{proposal,design,spec,tasks,adr}.md` + `change-config.yaml`

**Fields:**
- `proposal.md` — the "what & why" (from prompt_hub `VibeResult`)
- `design.md` — the "how" (from prompt_hub `gather_context()` + rusty-idd `scan`)
- `spec.md` — requirements (from vibe decomposition)
- `tasks.md` — checklist with role assignments (from prompt_hub templates)
- `change-config.yaml` — `depends_on`, `run_mode`, `prompt_bundle: <role>`

### Contract 2: Prompt Bundle Resolution (rusty-idd → prompt_hub)

```yaml
# openspec/tui-config.yaml
command: "claude --print --dangerously-skip-permissions {prompt}"
prompt_source: "prompt_hub"  # NEW
prompt_bundle: "base_implementer"  # NEW
prompt_role: "implementer"  # NEW
```

rusty-idd runner, before spawning a task:
1. Reads `prompt_source: prompt_hub`
2. Calls `prompthub get <role> <intent>` (or HTTP `GET /api/v1/prompts/search?role=...&intent=...`)
3. Renders the retrieved prompt template with task context
4. Passes rendered prompt to `command`

**Requirement:** New `rusty-idd-runner` config fields + subprocess/HTTP client logic.

### Contract 3: Agent Coordination (rusty-idd ↔ weave)

**Integration surface:** MCP only. No CLI parsing. No lib linking.

```json
// rusty-idd launches `weave mcp` as a subprocess
// Then sends JSON-RPC over stdin:

{"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":"weave_job_create","arguments":{"title":"Implement login OAuth","assignee":"claude-desktop","kind":"build"}}}

{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"weave_send","arguments":{"from":"rusty-idd-login-page","to":"all","body":"progress: 3/5 tasks"}}}

{"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"weave_ask","arguments":{"from":"rusty-idd-login-page","to":"planner","body":"Stalled on OAuth callback URL"}}}
```

**Requirement:** `rusty-idd-runner` gains an **MCP client** module (feature-gated: `--features weave`). It spawns `weave mcp`, speaks JSON-RPC, and maps `ImplUpdate` events to weave tools.

**Why MCP only:** weave is in construction. The CLI may change. The MCP tool schema is the most stable contract.

### Contract 4: Session Handoff (weave → rusty-idd)

The `session-relay` skill already uses `HANDOFF.md` as the authoritative payload, with weave as heartbeat only. This contract **does not change**.

```
weave send to:all tag:relay:handoff "resume via /idd-merge-loop resume from _workspace/HANDOFF.md"
# HANDOFF.md contains the real checkpoint
```

### Contract 5: Unified Env/Secret Contract

All three repos read from a **shared env contract file** (produced by rusty-idd `scan`):

```yaml
# .env.contract.yaml (proposed)
identities:
  prompt_hub:
    agent_id: "prompthub-alpha"
    token_ref: "PROMPTHUB_ADMIN_TOKEN"
  weave:
    session: "rusty-idd-runner"
    db: "~/.local/share/weave/messages.db"
  rusty-idd:
    manifest: ".idd/MANIFEST.tsv"
    changes_dir: "openspec/changes"

secrets:
  - name: PROMPTHUB_ADMIN_TOKEN
    provider: github-actions
    required: false
  - name: WEAVE_LIBSQL_AUTH_TOKEN
    provider: github-actions
    required: false
```

**Requirement:** Extend `rusty-idd-core::env_contract` to emit this unified contract.

---

## 6. Integration Roadmap — Phased Slices

Each slice is one reviewable PR. Each slice updates `.idd/MANIFEST.tsv` and `AI_MERGE/11_integration_research_audit_roadmap.md`.

### Phase 0: Prerequisites (no cross-repo code yet)

| Slice | Task | Validation |
|-------|------|------------|
| P0.1 | **Fix weave merge conflicts** in `src/main.rs` and `src/store_libsql.rs` | `cargo build --release` green |
| P0.2 | **Retire Node `openspec` from rusty-idd runner** — port `list_changes`/`get_change_status` to use `rusty-idd-spec` | `cargo test --workspace` green; no `openspec` binary required |
| P0.3 | **Inventory cross-repo manifest** — generate feature matrix + contract map for all 3 repos | `AI_MERGE/` record updated |

### Phase 1: Data Contract Integration (file-based, no runtime coupling)

| Slice | Task | Validation |
|-------|------|------------|
| P1.1 | **prompt_hub OpenSpec exporter** — `prompthub export-openspec <change-name>` emits `openspec/changes/<name>/{proposal,design,spec,tasks}.md` | Export round-trips through `rusty-idd spec validate` |
| P1.2 | **Unified env contract** — extend rusty-idd `env_contract` to map identities across prompt_hub/weave/rusty-idd | Contract file validates against schema |
| P1.3 | **Shared AGENTS.md contract** — rusty-idd-generated `AGENTS.md` includes prompt_hub/weave sections | `rusty-idd validate` passes |

### Phase 2: Runtime Integration (subprocess/CLI contracts)

| Slice | Task | Validation |
|-------|------|------------|
| P2.1 | **rusty-idd runner: prompt_hub prompt resolution** — config gains `prompt_source: prompt_hub`, runner calls `prompthub get` before task spawn | Parity test: task runs with prompt_hub-provided template vs. static template |
| P2.2 | **rusty-idd runner: weave telemetry** — runner calls `weave job create`, `weave send`, `weave ask` at lifecycle points | `weave inbox --me rusty-idd-$CHANGE` shows progress messages |
| P2.3 | **rusty-idd CLI: `orchestrate` verb** — new top-level command that drives the full pipeline: `rusty-idd orchestrate --request "..."` (calls prompt_hub vibe → export → validate → run) | End-to-end test with mock AI agent |

### Phase 3: Deep Integration (MCP-first, no lib coupling)

| Slice | Task | Validation |
|-------|------|------------|
| P3.1 | **weave CRM mapping** — define how rusty-idd OpenSpec changes map to weave CRM deals/stages/activities | Documented mapping validated by test |
| P3.2 | **prompt_hub MCP server** — add `prompthub mcp` subcommand exposing `prompt_hub_register`, `prompt_hub_get`, `prompt_hub_search` tools | MCP tool schema validated |
| P3.3 | **rusty-idd `crates/adapter`** — new crate with async tokio runtime, MCP client for weave, HTTP client for prompt_hub. Feature-gated. | Build green; no `core` purity regression |

**Note:** `weave` lib extraction is **deferred** until weave stabilizes out of construction mode. We integrate via MCP only.

### Phase 4: Polish & Hardening

| Slice | Task | Validation |
|-------|------|------------|
| P4.1 | **Integration test suite** — shell script that starts prompt_hub server + weave MCP + rusty-idd, runs full pipeline end-to-end | CI green |
| P4.2 | **Observability wiring** — OpenTelemetry traces span prompt_hub → weave → rusty-idd | Traces visible in Jaeger/OTel collector |
| P4.3 | **Documentation & migration notes** — update all `AGENTS.md`, `README.md`, architecture diagrams. Document weave's construction-mode status. | Human review + agent self-test |

---

## 7. Dependency Matrix

```
                    Phase 0      Phase 1      Phase 2      Phase 3      Phase 4
prompt_hub          P0.3 ──────► P1.1 ──────► P2.1 ──────► P3.2 ──────► P4.1
                       │            │            │            │            │
weave               P0.1 ──────►     ──────► P2.2 ──────► P3.1 ──────► P4.1
                       │            │            │            │            │
rusty-idd           P0.2 ──────► P1.2 ──────► P2.3 ──────► P3.3 ──────► P4.1
                       │            │            │            │            │
contracts/meta         └────────► P1.3 ──────►     ──────►     ──────► P4.3
```

**Critical path:** P0.1 → P0.2 → P1.1 → P2.1 → P2.3 → P4.1

---

## 8. Rollback Strategy

- **Phase 0–1:** File-based contracts. Rollback = delete exported files + restore config. No code coupling.
- **Phase 2:** Subprocess integration. Rollback = revert config to `prompt_source: static` and remove weave hook calls.
- **Phase 3:** Library coupling. Rollback = remove `crates/adapter` from workspace; revert CLI to Phase 2 behavior.
- **Phase 4:** Observability. Rollback = feature-gate OTel behind `--features telemetry`.

---

## 9. Evidence Checklist (per PR)

Per AGENTS.md rule #5, every integration slice must include:
- [ ] Build command result (`cargo build --workspace` or repo-specific)
- [ ] Test command result (`cargo test --workspace`)
- [ ] Lint/typecheck result (`cargo fmt --check`, `cargo clippy --workspace -D warnings`)
- [ ] Secret scan result (`cargo audit` + grep scan)
- [ ] Migration note (old path → new path)
- [ ] Rollback path
- [ ] Updated `.idd/MANIFEST.tsv` or note explaining why unchanged
- [ ] Updated `AI_MERGE/11_integration_research_audit_roadmap.md` progress

---

## 10. Open Questions

1. **weave integration surface:** The roadmap targets **MCP only**. Because weave is in construction (Pipedrive CRM + MCP + mesh) and the CLI is uncertain, we do not build CLI-parsing integration or library coupling. Everything is feature-gated behind `--features weave`.
2. **prompt_hub OpenSpec generation:** The recommended path is **Option A** — prompt_hub emits structured intent JSON, rusty-idd's spec engine converts it to OpenSpec. This keeps rusty-idd as the format authority. Option B (direct export) is possible if prompt_hub later links `rusty-idd-spec`.
3. **What is `rtk`'s role?** It is a command wrapper (infrastructure), not a code dependency. The integration does not need to touch `rtk-tokenkill`.
4. **Should the three repos become one workspace?** The roadmap says **no** — keep independent, integrate via contracts and MCP. If the user wants a mega-workspace, that becomes a separate epic with its own inventory/manifest/contract map.

---

*End of report. Ready for review and slice selection.*
