# Codex Prompts

This directory records reusable, scoped Codex workflows for this Proofline-operated Redlib fork.

Upstream Redlib remains the source of truth for core Redlib behavior. Proofline-specific changes should be small, instance-scoped, transparent, and kept separate from upstreamable work.

Codex output is maintainer-reviewed work. It is not an audit, certification, legal review, security review, endorsement, or production-readiness claim.

## Prompt Files

- `00-project-context-check.md`: inspect the current repository shape, fork context, scope, risks, and validation plan before editing.
- `05-codex-change-control.md`: decide whether a request is scoped enough and keep changes small and separable.
- `10-rust-dependency-maintenance.md`: refresh Rust dependencies conservatively and fix dependency breakage narrowly.
- `20-code-review.md`: review correctness, scope, privacy, safety, tests, and maintainability.
- `30-container-and-deployment-review.md`: review Docker, compose, CI image publishing, and deployment examples as sensitive operator-surface work.
- `40-upstream-contribution-review.md`: decide whether a change should be separated for upstream Redlib contribution.

## Validation

Run the commands that match the change. For Rust dependency or code changes, start with:

```bash
cargo fmt --check
cargo check
cargo test
cargo clippy --all-targets --all-features
cargo build --release
git diff --check
```

CI also uses `cargo nextest run` and `cargo clippy -- -D warnings`; use those when checking CI parity.

Do not claim validation passed unless the command actually ran.

## Artifact Hygiene

Generated issue drafts, logs, screenshots, reports, and Codex artifacts must not contain secrets, tokens, credentials, private deployment details, residential IPs, Cloudflare tunnel IDs, private hostnames, VLAN details, request bodies, user data, exploit details, abuse/evasion instructions, or sensitive operator details.

Keep public community-service wording best-effort and separate from Proofline safety/evidence/account systems. Do not claim anonymity; describe privacy, proxying, caching, logging, and limits precisely.
