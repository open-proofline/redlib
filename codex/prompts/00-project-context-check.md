# Codex Prompt: Project Context Check

Read current project context before making changes.

Do **not** change files.
Do **not** add features.

## Goal

Summarize the current repository state and likely impact of the requested task before implementation work begins.

## Required Inspection

Start with:

```bash
git status --short
git branch --show-current
git rev-parse HEAD
git remote -v
find . -maxdepth 2 -type f | sort
```

Inspect source-of-truth files as relevant:

- `README.md`
- `AGENTS.md`
- `SECURITY.md`, if present
- `docs/`, if present
- `codex/README.md`
- `codex/prompts/`
- `Cargo.toml`
- `Cargo.lock`
- Rust source layout under `src/`
- tests and test modules
- Dockerfiles
- compose files, if present
- container or deployment files
- `.github/`, if present

If the task references an issue, pull request, report, branch, or external repository, inspect that source before making assumptions. If a referenced source is unavailable, say so.

## Classification

Classify the requested work as one or more of:

- Proofline-local instance work
- upstreamable Redlib work
- Rust dependency maintenance
- container or deployment work
- documentation or prompt work
- review-only work
- too broad or ambiguous
- sensitive operator/security work

Keep Proofline-local instance changes separate from upstreamable Redlib improvements.

## Risk Review

Before editing, identify:

- files likely to change
- files that must not change
- whether the task could affect core Redlib behavior
- whether Docker, deployment, Cloudflare, container publishing, or CI is involved
- whether public docs could overclaim anonymity, privacy, reliability, or Proofline integration
- whether logs, tests, screenshots, issues, PRs, reports, or Codex artifacts could expose private operator details or user data
- validation commands likely needed

## Repository Constraints

- This is a Proofline-operated Redlib fork for a public community service.
- Upstream Redlib remains the source of truth for core Redlib behavior.
- Do not add Proofline Server, account portal, safety/evidence, mobile, protocol, notification, emergency-services, billing, decryption, key escrow, admin/operator, recording/capture, ads, tracking, or privacy-hostile behavior.
- Do not claim anonymity. Be precise about privacy, proxying, caching, logging, request forwarding, and limitations.
- Do not include private deployment details, residential IPs, Cloudflare tunnel IDs, private hostnames, VLAN details, secrets, tokens, credentials, request bodies, raw logs containing user data, exploit details, abuse/evasion instructions, or sensitive operator details in public artifacts.

## Output

Return:

1. Current branch, commit, and dirty working tree summary
2. Repository shape
3. Source-of-truth files inspected
4. Task classification
5. Files likely affected
6. Files or areas that must not change
7. Risk notes
8. Recommended next reusable prompt
9. Validation plan
10. Clarifying questions, only if required to avoid a bad change
