# Codex Prompt: Documentation Update

Update documentation to match the current code and project scope.

Do **not** change code unless documentation reveals a clear inconsistency or broken example and the maintainer explicitly wants it fixed.
Do **not** overpromise production readiness.
Do **not** claim anonymity.

## Source Of Truth

Before making changes, read current source-of-truth files as relevant:

- `README.md`
- `AGENTS.md`
- `SECURITY.md`, if present
- `docs/`, if present
- `codex/README.md`
- `codex/prompts/`
- relevant source files
- relevant tests
- `Cargo.toml`
- Dockerfiles and compose files if documenting deployment
- relevant issue or PR, if this is issue/PR work

Do not rely on stale assumptions from this prompt if the repository has changed.

## Potential Docs To Update

Update only relevant files:

- `README.md`
- `AGENTS.md`
- `SECURITY.md`, if present
- `docs/`, if present
- `codex/README.md`
- `codex/prompts/*.md`
- deployment docs, if present
- container docs, if present
- `.github/ISSUE_TEMPLATE/*.md`, if process changes require it

## Check Documentation For

- current project scope
- upstream Redlib relationship
- Proofline-operated fork boundaries
- public community-service boundaries
- privacy, proxying, caching, logging, request forwarding, and limitation claims
- no anonymity claims
- no Proofline safety/evidence/account integration claims
- no emergency reliability or emergency-services implication
- no official hosted Proofline account behavior unless implemented elsewhere and explicitly scoped
- no ads, behavioral tracking, invasive analytics, or privacy-hostile features
- build, test, and run commands
- Docker/deployment commands
- CI status, if CI exists
- removed or planned deployment providers
- stale Heroku, Fly.io, Render, Railway, Replit, Vercel, or Netlify references
- stale GitHub Actions references if CI has been removed
- Codex workflow accuracy

## Constraints

- Do not describe future Docker, CI, deployment, account, safety/evidence, notification, emergency-services, billing, mobile, or protocol work as implemented.
- Do not imply the service is production-ready, staffed, emergency-reliable, or anonymous.
- Keep public community-service docs clear that the service is best-effort and separate from Proofline safety/evidence/account systems.
- Do not include private deployment details, residential IPs, Cloudflare tunnel IDs, private hostnames, VLAN details, tokens, secrets, credentials, request bodies, raw logs containing user data, exploit details, abuse/evasion instructions, or sensitive operator details.

## Validation

For docs-only or prompt-only changes:

```bash
git diff --stat
git diff -- README.md AGENTS.md SECURITY.md docs codex .github/ISSUE_TEMPLATE
git diff --check
```

Inspect changed Markdown manually. Run a Markdown link checker only if one already exists in this repository. Rust tests are not required unless code changed.

If code changed unexpectedly, stop and explain why.

## Output

Summarize:

1. files changed
2. docs updated
3. docs intentionally not touched
4. stale references left for maintainer review
5. validation result
6. whether code tests were skipped as docs-only
