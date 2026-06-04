---
name: pr-evidence-bundle
description: "Assembles the AGENTS.md-mandated PR evidence for a merge slice (build/test/lint/secret-scan results, migration note, rollback path, manifest update) and follows worktree→branch→commit→PR discipline. ALWAYS use when opening a PR for a merge slice, packaging evidence for review, or finalizing a slice in idd-merge-idd. Use this so no PR ships without its required evidence."
---

# PR Evidence Bundle

A merge PR in this repo is not just a diff — `AGENTS.md` requires it to carry the evidence a reviewer (human or agent) needs to trust the slice. Assemble that bundle and follow the repo's branch/worktree discipline.

## Why this exists
The repo's safety model is serialized, test-backed increments on one authoritative integration branch. Missing evidence (no parity note, no rollback, stale manifest) is how a "small" merge silently breaks behavior. This skill guarantees every required artifact is present before the PR opens.

## Required PR evidence (from AGENTS.md — all mandatory)
Pull these from the QA report and change log; do not hand-wave any item:
1. **Build result** — `cargo build` from the affected crate.
2. **Test result** — `cargo test --all --locked`.
3. **Lint/typecheck** — `cargo fmt --all -- --check` and `cargo clippy --all-targets --all-features -- -D warnings`.
4. **Secret scan** — `idd validate` (no critical findings) and drift-check clean.
5. **Migration note** — old path → new path (from the slice spec / change log).
6. **Rollback path** — exact revert steps (from the slice spec).
7. **Manifest update** — refreshed `.idd/MANIFEST.tsv` if control-plane files changed, or a note explaining why unchanged.
8. **Updated `AI_MERGE` record** — the slice's task/plan/conflict entries reflect what shipped.

Gate: only assemble the PR when `_workspace/04_qa_report.md` says **PR-ready: yes**. If QA marks anything fail/unverified, return to the producer–reviewer loop.

## Branch / worktree discipline
Per `CLAUDE.md`, work in a fresh worktree off the synced base — never mutate the integration root directly:
```bash
rtk git fetch --all
rtk git worktree add ../idd-<slice-slug> -b <slice-slug> origin/main
# ... implement / verify happen on this branch ...
rtk git add <files> && rtk git commit -m "<slice title>"   # always rtk, even in && chains
rtk git push -u origin <slice-slug>
```
End commit messages with the required co-author trailer. Keep the PR to one slice.

## PR body template
```markdown
## Summary
<one slice, one migration intent — what changed and why>

## Evidence
- Build: <result>
- Test (--all --locked): <result>
- Lint (fmt --check / clippy -D warnings): <result>
- Secret scan (idd validate): <result, critical=0>
- Rust-native drift-check: <clean / details>

## Migration note
- Old: <path/symbol> → New: <path/symbol>  (old path deprecated, not deleted)

## Rollback
- <exact steps to revert>

## Manifest / AI_MERGE
- <.idd/MANIFEST.tsv refreshed | unchanged because …; AI_MERGE record updated>
```

## Principles
- One integration branch has merge authority; everything else is research/disposable.
- Never commit real secrets — map references, not values.
- If two agents conflicted during the slice, record it in `AI_MERGE/05_conflict_risk_register.md` before opening the PR.
- Do not open or merge a PR without explicit user go-ahead; surface the assembled bundle and let the user decide.
