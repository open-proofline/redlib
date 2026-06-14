# Codex Prompt: Code Review

Review current changes for correctness, maintainability, privacy, security, and scope control.

Default mode is review-only: inspect current changes and report findings without editing files. Enter fix mode only when the maintainer explicitly asks for fixes or the task explicitly says to fix findings.

Do **not** add features unless required to fix a bug.
Do **not** make broad refactors during review unless required to fix a blocking issue.

## Source Of Truth

Before reviewing, read current source-of-truth files as relevant:

- `README.md`
- `AGENTS.md`
- `SECURITY.md`, if present
- `docs/`, if present
- `codex/README.md`
- relevant prompt under `codex/prompts/`
- relevant source files
- relevant tests
- `Cargo.toml` and `Cargo.lock`, if dependencies changed
- Docker/container files, if touched
- relevant issue or PR, if this is issue/PR work

Do not rely on stale assumptions from this prompt if the repository has changed.

## Global Constraints

- Keep changes scoped to the task.
- Preserve upstream Redlib behavior unless the local fork delta is explicit.
- Keep Proofline-local instance changes separate from upstreamable improvements.
- Do not add Proofline Server behavior, account-portal behavior, safety/evidence workflows, admin/operator surfaces, recording/capture, browser/backend/trusted-contact decryption, key escrow, notifications, emergency-services integration, hosted-account billing, payment-gated Proofline product access, or Proofline protocol behavior.
- Do not add ads, behavioral tracking, invasive analytics, or privacy-hostile features.
- Do not claim anonymity. Check privacy, proxying, caching, logging, request forwarding, and limitation wording for precision.
- Do not include private deployment details, residential IPs, Cloudflare tunnel IDs, private hostnames, VLAN details, tokens, secrets, credentials, request bodies, raw logs containing user data, exploit details, abuse/evasion instructions, or sensitive operator details in public artifacts.

## Review Focus

Check for:

- correctness and behavior changes
- Rust error handling and panic risks
- async/request behavior
- request forwarding, proxy behavior, redirects, and upstream Reddit interaction if touched
- caching behavior, cache invalidation, and request coalescing if touched
- rate limiting, retry, backoff, or OAuth rollover behavior if touched
- configuration and environment variable behavior
- logging and redaction
- privacy claims and privacy regressions
- template escaping and HTML safety
- response headers and cookie behavior if touched
- test coverage and whether tests rely on live network behavior
- dependency and lockfile changes
- build reproducibility
- documentation drift
- Docker/container/deployment risk if touched
- no private operator details or sensitive data introduced
- no Proofline safety/evidence/account behavior accidentally introduced
- no Proofline branding, service copy, deployment topology, or support copy mixed into upstreamable work

## Output Format

Return findings first, ordered by severity:

1. Critical issues
2. Important issues
3. Minor issues
4. Suggested minimal fixes
5. Tests/validation recommended

If there are no findings, say so clearly and mention any remaining test gaps or residual risk.

If asked to fix findings:

- keep edits small
- do not add unrelated features
- do not include sensitive vulnerability details in public artifacts
- run relevant validation
- summarize what changed
