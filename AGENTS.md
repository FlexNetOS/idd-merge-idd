# AGENTS.md — Intent Driven Development

## North Star

Unify repositories by preserving working behavior, making contracts explicit, and merging only through reviewable, test-backed increments.

## Operating Rules

1. Do not flatten two repos before generating inventories, manifest, feature matrix, and contract maps.
2. Do not introduce a new secret provider unless the env/secrets contract says why.
3. Do not delete source code during migration; deprecate first, remove after parity tests pass.
4. Keep PRs narrow: one vertical slice, one migration intent, one validation path.
5. Every generated PR must update the relevant `/AI_MERGE` record.
6. Never commit real secrets. Use `.env.example`, `.env.schema.example.json`, GitHub Actions secrets, OIDC, or provider references.
7. If two agents conflict, stop and update `/AI_MERGE/05_conflict_risk_register.md` before continuing.
8. Treat `.idd/MANIFEST.tsv` as the audit baseline for generated control-plane artifacts.
9. Break work into sub-59-minute tasks when using cloud coding agents with hard session limits.
10. One integration branch has authority. Other branches are research, staging, or disposable.

## Required PR Evidence

- Build command result
- Test command result
- Lint/typecheck result
- Secret scan result
- Migration note explaining old path -> new path
- Rollback path
- Updated manifest or note explaining why unchanged

## Merge Authority

Parallel agents may analyze and propose branches, but only the integration branch is authoritative.
