# Codex Prompts

This directory records reusable Codex workflows for the Proofline-operated Redlib fork.

Codex output is maintainer-reviewed work. It is not an audit, certification, legal review, security review, endorsement, production-readiness claim, or maintenance by OpenAI.

## Directory Structure

Keep the Codex workflow in this structure:

```text
codex/
  README.md
  prompts/
  archive/
  work-orders/
```

Reusable prompts live in `codex/prompts/`. They are scoped workflows that can be run again against the current repository after reading the current source-of-truth files.

Historical one-off prompts, if used, live in:

```text
codex/archive/
codex/work-orders/
```

Historical prompts are reference material only. Do not re-run them without checking current `README.md`, `AGENTS.md`, `SECURITY.md` if present, relevant docs, and reusable prompts.

Generated local artifacts do not belong under `codex/`. Put temporary reviews, issue drafts, logs, screenshots, reports, and generated scripts outside `codex/` unless a maintainer explicitly defines a repository workflow for them.

## Naming Conventions

Reusable prompts use this filename pattern:

```text
NN-short-kebab-title.md
```

Rules:

- two-digit numeric prefix
- kebab-case title
- `.md` extension
- no spaces
- no date prefix
- one reusable workflow per file

Historical prompts use this filename pattern:

```text
YYYY-MM-DD-short-kebab-title.md
```

Rules:

- date prefix
- kebab-case title
- `.md` extension
- no numeric reusable-workflow prefix
- each file should be marked historical/reference-only near the top

One-off implementation tasks belong in `codex/work-orders/` or `codex/archive/`, not as new reusable prompts. Add a reusable prompt only for a repeated workflow.

## Normal Reusable Prompt Order

Use prompts in this order when the task calls for them:

1. `00-project-context-check.md`
2. `05-codex-change-control.md`
3. `10-rust-dependency-maintenance.md`
4. `20-code-review.md`
5. `30-container-and-deployment-review.md`
6. `40-documentation-update.md`
7. `50-upstream-contribution-review.md`

## Current Project Constraints

Treat `README.md`, `AGENTS.md`, `SECURITY.md` if present, `docs/` if present, `Cargo.toml`, Docker/container files, and current source code as project truth. Do not rely on stale assumptions from a prompt when repository files disagree.

Core constraints:

- This repository is a Proofline-operated Redlib fork for a public community service.
- Upstream Redlib remains the source of truth for core Redlib behavior.
- Proofline-specific changes should stay small, transparent, instance-scoped, and easy to separate from upstreamable work.
- Generic improvements should remain suitable for upstream Redlib contribution when practical.
- This repository is not Proofline Server, not the Proofline web-client/account portal, not a mobile client, and not the Proofline protocol repository.
- Do not add Proofline Server behavior, account-portal behavior, safety/evidence workflows, admin/operator surfaces, recording/capture, browser/backend/trusted-contact decryption, key escrow, notifications, emergency-services integration, hosted-account billing, payment-gated Proofline product access, or Proofline protocol behavior.
- Do not add ads, behavioral tracking, invasive analytics, or privacy-hostile features.
- Do not claim anonymity. Be precise about privacy, proxying, caching, logging, request forwarding, and limitations.
- Keep public community-service docs clear that the service is best-effort and separate from Proofline safety/evidence/account systems.
- Do not include private deployment details, residential IPs, Cloudflare tunnel IDs, private hostnames, VLAN details, tokens, secrets, credentials, request bodies, raw logs containing user data, exploit details, abuse/evasion instructions, or sensitive operator details in public docs, tests, screenshots, logs, issues, PRs, or Codex artifacts.
- Treat Dockerfiles, compose files if reintroduced, reverse-proxy examples, Cloudflare-related examples, container publishing, and CI as security-sensitive operator work.
- GitHub Actions workflows are not currently present. Do not create replacement CI unless the task explicitly asks for CI work.

## Validation

For docs-only or prompt-only changes:

```bash
git diff --stat
git diff -- AGENTS.md codex README.md docs SECURITY.md
git diff --check
```

Inspect changed Markdown manually. Run a Markdown link checker only if one already exists in this repository. Rust tests are not required unless code changed or the task explicitly asks for them.

For Rust dependency or code changes:

```bash
cargo fmt --check
cargo check
cargo test
cargo clippy --all-targets --all-features
cargo build --release
git diff --check
```

Do not claim validation passed unless the command actually ran. If a command fails, report the exact command and distinguish task-related failures from pre-existing upstream or local failures.

## Artifact Hygiene

Generated issue drafts, logs, screenshots, reports, and Codex artifacts must not contain secrets, tokens, credentials, private deployment details, residential IPs, Cloudflare tunnel IDs, private hostnames, VLAN details, request bodies, user data, exploit details, abuse/evasion instructions, or sensitive operator details.

If sensitive material is discovered, describe only the category and file path. Do not quote the sensitive value.

## When To Update Prompts

Treat current code and source-of-truth docs as project truth. Reusable prompts are workflow helpers, and historical prompts are reference-only.

Update reusable Codex prompts when their assumptions, guardrails, validation commands, or repeated workflow steps have changed. Leave historical prompts untouched unless the maintainer explicitly requests otherwise.

| Project change | Prompt/doc action |
|---|---|
| Codex workflow or prompt naming changes | Update `codex/README.md` and affected reusable prompts. |
| Rust dependency maintenance workflow changes | Update `10-rust-dependency-maintenance.md`. |
| Docker, container, deployment, Cloudflare, or CI workflow changes | Update `30-container-and-deployment-review.md` and relevant docs after the implementation is real. |
| Documentation source-of-truth or public-claim policy changes | Update `40-documentation-update.md` and `codex/README.md` if the repeated workflow changes. |
| Upstream contribution policy changes | Update `50-upstream-contribution-review.md`. |
| New repeated Codex workflow | Add one reusable `NN-short-kebab-title.md` prompt and list it here. |
| One-off implementation, refactor, or work order | Add a dated historical prompt under `archive/` or `work-orders/` only if the maintainer wants it preserved. |

Do not add a reusable prompt for every one-off idea.
