# AGENTS.md

## Project Rules

- This repository is a Proofline-operated Redlib fork for a public community service.
- This repository is not Proofline Server, not the Proofline web-client/account portal, and not a Proofline mobile or protocol repository.
- Upstream Redlib remains the source of truth for core Redlib behavior.
- Keep Proofline-specific instance branding and source links small, isolated, transparent, and easy to separate from upstreamable work.
- Keep upstreamable changes separate from Proofline-specific deployment, branding, governance, or support changes.
- Do not add Proofline Server behavior, account-portal behavior, safety/evidence workflows, admin/operator surfaces, recording/capture, browser/backend/trusted-contact decryption, key escrow, notifications, emergency-services integration, hosted-account billing, payment-gated product access, or Proofline protocol behavior.
- Do not add ads, behavioral tracking, invasive analytics, or privacy-hostile features.
- Do not include private deployment details, residential IPs, Cloudflare tunnel IDs, private hostnames, VLAN details, tokens, secrets, credentials, request bodies, raw logs containing user data, exploit details, abuse/evasion instructions, or other sensitive operator details in public docs, issues, tests, screenshots, logs, or Codex artifacts.
- Do not claim anonymity. Be precise about privacy, proxying, caching, logging, and limits.
- Treat Docker, CI, and deployment changes as security-sensitive operator work.
- Keep public community-service docs clear that the service is best-effort and separate from Proofline safety/evidence/account systems.
- Prefer small, scoped, reviewable changes.
- Treat Codex prompts as scoped change requests, not permission to rewrite adjacent service behavior.

## Current Shape

- Rust 2021 single-crate application named `redlib`, with `src/main.rs` for the binary and `src/lib.rs` for library code.
- `Cargo.toml` declares Rust 1.81 as the minimum supported Rust version.
- Templates live under `templates/`; static assets live under `static/`.
- Inline Rust tests live in `src/`; there is no top-level `tests/` directory.
- Dockerfiles are present at `Dockerfile`, `Dockerfile.alpine`, and `Dockerfile.ubuntu`.
- Compose files are present at `compose.yaml` and `compose.dev.yaml`.
- GitHub Actions workflows live under `.github/workflows/`.
- Runtime and packaging examples include `.env.example`, `redlib.container`, `contrib/`, `heroku.yml`, `flake.nix`, and `seccomp-redlib.json`.
- The upstream remote, when present, should point at `redlib-org/redlib`; verify remotes before assuming branch or fork state.

## Commands

From the repository root:

```bash
cargo fmt --check
cargo check
cargo test
cargo clippy --all-targets --all-features
cargo build --release
git diff --check
```

CI also uses `cargo nextest run` and `cargo clippy -- -D warnings`; use those when reviewing CI parity or larger Rust changes.

Do not claim validation passed unless the command actually ran.

## Review Expectations

Before accepting Codex changes, check:

- changes stay scoped to this Redlib fork
- upstreamable changes are not mixed with Proofline branding or deployment specifics
- core Redlib behavior still follows upstream unless a local fork delta is explicit
- no Proofline safety, evidence, account, mobile, protocol, emergency, billing, notification, decryption, or admin/operator behavior was added
- privacy language is precise and does not claim anonymity
- logs, docs, tests, screenshots, issue drafts, and reports do not include secrets, private operator details, request bodies, user data, exploit details, or abuse/evasion instructions
- Docker, CI, and deployment changes received security-sensitive review
- public service wording remains best-effort and separate from Proofline safety/evidence/account systems
- validation results are reported exactly, including failures
