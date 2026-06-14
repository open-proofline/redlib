# Codex Prompt: Container And Deployment Review

Review container, deployment, reverse-proxy, Cloudflare-related, and image-publishing changes as security-sensitive operator-surface work.

Default mode is review-only unless the maintainer explicitly asks for fixes.

Do **not** rewrite Dockerfiles.
Do **not** create replacement CI.
Do **not** add deployment providers.
Do **not** document private deployment topology.

## Source Of Truth

Read current source-of-truth files as relevant:

- `README.md`
- `AGENTS.md`
- `SECURITY.md`, if present
- `docs/`, if present
- `codex/README.md`
- Dockerfiles
- compose files, if present
- `.dockerignore`
- `.env.example`
- container or service files such as `redlib.container` and `contrib/`
- `.github/`, if present
- relevant source and tests if container behavior depends on app behavior

Do not rely on stale assumptions from this prompt if repository files disagree.

## Review Scope

Review:

- Dockerfiles
- compose files, if present
- reverse-proxy examples
- Cloudflare-related examples
- image publishing
- CI related to container builds, if present
- runtime image selection
- Alpine-based runtime considerations
- CA certificates for outbound HTTPS
- non-root runtime user
- file ownership and permissions
- `.dockerignore`
- build cache strategy
- build toolchain isolation
- single-container deployment goal when relevant
- multi-arch implications
- health checks
- logging and redaction
- environment variables
- secret handling
- generated image metadata
- documentation and examples

## Constraints

- Keep public examples generic and safe.
- Do not include private deployment topology.
- Do not include residential IPs.
- Do not include Cloudflare tunnel IDs.
- Do not include private hostnames.
- Do not include VLAN details.
- Do not include tokens, secrets, credentials, request bodies, raw logs containing user data, exploit details, abuse/evasion tactics, or sensitive operator details.
- Do not present Cloudflare, proxying, or Redlib as anonymity.
- Do not expose admin/operator surfaces.
- Do not imply production readiness, staffed operations, emergency response, or integration with Proofline safety/evidence/account systems.
- Do not create replacement CI unless the task explicitly asks for CI work.
- Separate generic upstreamable container hygiene from Proofline-specific deployment, branding, governance, or support changes.

## Output

Return:

1. Container/deployment findings
2. Security-sensitive concerns
3. Minimal fixes
4. Validation commands
5. Follow-up work

If asked to fix:

- keep edits scoped to container/deployment files named by the task
- do not change app code unless required to fix the container task
- do not add private operator details
- run relevant validation and `git diff --check`
