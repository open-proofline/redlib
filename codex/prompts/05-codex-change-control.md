# Codex Prompt: Change-Control Check

Review the requested Codex task before making changes.

Default mode is review-only.

Do **not** change code.
Do **not** change documentation.
Do **not** add features.

Only enter edit mode when the maintainer explicitly asks for implementation or documentation changes.

## Goal

Decide whether the task is scoped enough to begin, needs clarification, should become follow-up/backlog work, or requires sensitive operator/security handling.

## Source Of Truth

Read current source-of-truth files as relevant:

- `README.md`
- `AGENTS.md`
- `SECURITY.md`, if present
- `docs/`, if present
- `codex/README.md`
- relevant reusable prompt under `codex/prompts/`
- `Cargo.toml` and `Cargo.lock`, for dependency work
- relevant Rust source and tests, for code work
- Docker/container/deployment files, for operator-surface work
- relevant issue or PR, if this is issue/PR work

Do not rely on stale assumptions from this prompt if the repository has changed.

## Global Constraints

- Keep changes small, scoped, and PR-sized.
- Do not combine broad mixed tasks such as dependency updates, Dockerfile rewrites, and CI fixes unless the maintainer explicitly scopes that combined change.
- Separate Proofline-local instance work from upstreamable Redlib improvements.
- Separate documentation/prompt work from app, dependency, Docker, deployment, and CI work.
- Preserve upstream Redlib behavior unless a local fork delta is explicit.
- Do not add unrelated features.
- Do not add Proofline Server behavior, account-portal behavior, safety/evidence workflows, admin/operator surfaces, recording/capture, browser/backend/trusted-contact decryption, key escrow, notifications, emergency-services integration, hosted-account billing, payment-gated Proofline product access, or Proofline protocol behavior.
- Do not add ads, behavioral tracking, invasive analytics, or privacy-hostile features.
- Do not claim anonymity. Be precise about privacy, proxying, caching, logging, request forwarding, and limitations.
- Treat Dockerfiles, compose files if reintroduced, reverse-proxy examples, Cloudflare-related examples, container publishing, and CI as security-sensitive operator work.
- Do not include private deployment details, residential IPs, Cloudflare tunnel IDs, private hostnames, VLAN details, tokens, secrets, credentials, request bodies, raw logs containing user data, exploit details, abuse/evasion instructions, or sensitive operator details in public artifacts.

## Check

Assess whether the task has:

- a clear goal
- likely affected files or areas
- files or areas that must not change
- explicit out-of-scope items
- validation commands
- a rollback or checkpoint state, or a clean enough working tree
- clear distinction between required work and future work
- clear handling for sensitive operator, deployment, or security details
- clear separation between local-only and upstreamable work

## Backlog And Follow-Up Gate

If the request introduces future work that is not required for the current task, recommend a follow-up item instead of implementing it.

Use follow-up handling for:

- Dockerfile overhaul when a task is only dependency or prompt work
- replacement CI when a task is not explicitly CI work
- deployment-provider cleanup when a task is only docs/prompt work
- upstream contribution prep when local Proofline changes are still mixed in

Security vulnerabilities should not become public issue text. Non-sensitive hardening can become a normal follow-up item.

## Output

Return a short readiness assessment:

- `Ready`: the task is scoped enough to start.
- `Needs clarification`: one or two specific details are missing.
- `Create follow-up/backlog item`: the request is future work or too broad for the current task.
- `Sensitive operator/security handling`: the task may expose private deployment, security, or abuse details.
- `Not suitable for upstream`: the task is Proofline-local or instance-specific.

Also include:

- likely validation commands
- recommended next reusable prompt
- suggested follow-up title, if applicable
