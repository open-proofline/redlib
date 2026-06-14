# Codex Prompt: Upstream Contribution Review

Decide whether a change should be proposed upstream to Redlib, kept local to the Proofline-operated fork, or split.

Default mode is review-only unless the maintainer explicitly asks for branch cleanup or edits.

## Source Of Truth

Read current source-of-truth files as relevant:

- `README.md`
- `AGENTS.md`
- `SECURITY.md`, if present
- `docs/`, if present
- `codex/README.md`
- relevant source files and tests
- `Cargo.toml` and `Cargo.lock`, if dependencies changed
- Docker/container files, if touched
- current branch diff and commit history
- upstream remote information, if available

Do not assume upstream Redlib is healthy, current, or inactive without evidence.

## Review Questions

Assess:

- Does the change belong upstream Redlib?
- Is the change Proofline-local only?
- Is branch separation clean?
- Did Proofline branding, service links, support copy, governance copy, deployment details, private infrastructure assumptions, or community-service messaging leak into upstreamable work?
- Is the change generic, maintainable, and easy for upstream to review?
- Does the change preserve core Redlib behavior?
- Are tests and validation appropriate for an upstream PR?
- Are dependency or container changes minimal and justified?

## Upstreamable Examples

Likely upstreamable:

- bug fixes
- performance improvements
- caching improvements
- request coalescing
- backoff, retry, or rate-limit correctness
- accessibility improvements
- generic documentation improvements
- container hygiene
- CI improvements
- reliability improvements
- safe generic operator configuration
- Rust dependency API compatibility fixes

## Not Upstreamable As-Is

Keep local or split out:

- Proofline branding
- Proofline service links
- Proofline support, donation, moderation, or governance copy
- Proofline deployment topology
- private infrastructure assumptions
- Cloudflare tunnel details
- residential IP assumptions
- private hostnames or VLAN details
- Proofline-specific community-service messaging
- Proofline safety/evidence/account behavior
- admin/operator surfaces
- recording/capture behavior
- browser/backend/trusted-contact decryption
- key escrow
- notifications
- emergency-services integration
- hosted-account billing or payment-gated Proofline product access
- Proofline protocol behavior

## Sensitive-Data Rules

Do not include secrets, tokens, private deployment details, request bodies, raw logs containing user data, exploit details, abuse/evasion instructions, or sensitive operator details in upstream PR text, public issues, examples, tests, logs, screenshots, or Codex artifacts.

## Output

Return:

1. Upstream suitability verdict
2. Files or commits that should be split out
3. Proofline-local pieces to keep separate
4. Privacy or operator concerns
5. Suggested upstream PR summary
6. Suggested local-only follow-up branch or issue title
7. Validation recommended before upstream submission
