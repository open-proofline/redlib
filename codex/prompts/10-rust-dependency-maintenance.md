# Rust Dependency Maintenance

Use this for Rust dependency updates, lockfile refreshes, and dependency-driven build failures.

1. Baseline before changes:
   - inspect `git status --short`
   - inspect `Cargo.toml` and `Cargo.lock`
   - run the relevant failing Rust validation command when possible
2. Prefer compatible lockfile refreshes before broad manifest upgrades.
3. Capture the exact failing command and concise error category.
4. Fix dependency-update breakage narrowly.
5. Do not mix dependency churn with Docker, CI, deployment, formatting, or refactor work unless required to restore the build.
6. Keep upstreamable dependency fixes separate from Proofline-specific branding, deployment, or service copy.
7. Re-run validation:

```bash
cargo fmt --check
cargo check
cargo test
cargo clippy --all-targets --all-features
cargo build --release
git diff --check
```

If a command fails, say exactly which command failed, why it failed, and whether it appears related to the current dependency work or pre-existing upstream behavior.

Do not paste secrets, private hostnames, tokens, request bodies, user data, exploit details, abuse/evasion instructions, or sensitive operator details into logs, issues, docs, screenshots, reports, or Codex artifacts.
