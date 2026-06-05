---
name: session-relay
description: "Session-continuity protocol for the idd-merge-loop: HAND OFF a running loop to a fresh session at the cycle budget, and RESUME it cold from the committed checkpoint. ALWAYS use when the loop reaches its cycle budget, when handing off to a fresh session, or when resuming/picking up the loop in a new session. Trigger keywords: hand off, handoff, session relay, resume the loop, pick up where it left off, continue in a new session, cycle budget reached, relay handoff/resumed. Used by idd-merge-loop; pairs with the continuity-steward agent."
---

# session-relay — hand off and resume the loop cold

The loop survives context rot by chaining short sessions, each handing a durable checkpoint to the next. This skill is the two ends of that chain. The **committed `_workspace/HANDOFF.md` is the authoritative payload** — weave is only a cross-identity heartbeat, and cron is best-effort.

## Why disk, not the inbox (verified gotcha)
A weave message addressed to your own identity does **not** appear in your own inbox, and a same-machine successor process inherits the same identity — so you cannot hand off to yourself through weave. The handoff must be a committed file on disk. weave `to:"all"` is an observable heartbeat for humans/peers, not the resume mechanism. cron (`CronCreate {recurring:false}`) is best-effort and session-only (`durable:true` is not honored here); the prompt must self-describe the resume. For survives-restart continuation, the committed `HANDOFF.md` is the signal — a human, the external `ralph-idd.sh` runner, or `RemoteTrigger` resumes from it.

## HAND OFF (at the cycle budget)
1. **Spawn `continuity-steward`** (general-purpose agent) → it writes `_workspace/HANDOFF.md` (state + pointers + verify-on-resume, not narrative).
2. **Commit** `HANDOFF.md` together with `backlog.md` + `loop_state.md` (area-prefixed subject, e.g. `chore(loop): handoff after cycle N`). This is the real, durable signal.
3. **Heartbeat** weave `to:"all"` with a `relay:handoff` note (branch + current item + "resume via /idd-merge-loop resume from _workspace/HANDOFF.md"). Observability only.
4. **Best-effort successor** — optionally `CronCreate {recurring:false}` a one-shot whose prompt self-describes the resume (it carries the resume command + worktree). Treat as a convenience, not a guarantee.
5. **Stop** — write the `HANDOFF.md` sentinel as the process's single sentinel (more work remains) and do **not** `ScheduleWakeup`.

## RESUME (a fresh session / runner iteration)
1. **Read the committed `_workspace/HANDOFF.md`** — the authoritative signal. If it's missing, fall back to `backlog.md` + `loop_state.md`; if those are missing too, re-run the loop's DISCOVER.
2. **Verify-on-resume baseline FIRST** — run the commands HANDOFF.md lists (the loop's verify suite: `rusty-idd validate`, `drift-check.sh .`, `rtk cargo fmt --check` / `clippy -D warnings` / `test --workspace --locked`). Confirm proven-green before doing new work; if the baseline is red, the first cycle fixes that, not the next backlog item.
3. **Broadcast** weave `to:"all"` `relay:resumed` (branch + the item being resumed). Observability only.
4. **Reset** `cycles_this_session = 0` in `loop_state.md` (keep `cycles_total`).
5. **Continue** the loop at the backlog's current `- [ ]` item (or finish a half-done slice HANDOFF.md flagged as in-flight).

## Working Principles
- The checkpoint is **state, not story**: resume command, worktree+branch, backlog counts + current item, in-flight cycle, commits landed this session, open blockers, decisions/dead-ends, verify-on-resume commands.
- Never resume from a stale in-memory view — re-read the committed files; code may have changed under you (a peer or the prior session may have advanced the branch).
- Fail-closed carries across the boundary: if verify-on-resume can't run, mark the check unverified and surface it — never assume green.

## Collaboration
- Invoked by `idd-merge-loop` (HAND OFF at budget; RESUME on a fresh session or a `ralph-idd.sh` iteration). Delegates the checkpoint write to the `continuity-steward` agent so the loop's context stays lean.
