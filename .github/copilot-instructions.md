# Repository Instructions for AI Coding Agents

Follow `AGENTS.md` first. Treat `/AI_MERGE` as the current control plane.

Preferred workflow:

1. Read the task file completely.
2. Inspect the repo inventory and env/secrets contract.
3. Make the smallest behavior-preserving change.
4. Run relevant tests and `rusty-idd validate` when available.
5. Update the affected `/AI_MERGE` documents.
6. Never commit secret values.

Do not perform broad cleanup, style-only rewrites, dependency swaps, or folder flattening unless the task explicitly says so.
