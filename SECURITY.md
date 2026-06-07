# Security Policy

## Secret handling

- Do not commit real secrets.
- Prefer OIDC for cloud credentials in CI instead of long-lived access keys.
- Use GitHub Actions secrets, repository/environment variables, SOPS, Infisical, OpenBao/Vault, or a local secure store by explicit contract only.
- Mask non-GitHub secret values in logs.

## Agent safety

- Do not give broad write authority to parallel agents.
- Use one authoritative integration branch.
- Require review before merging agent-generated code.
- Preserve rollback paths for each migration PR.
