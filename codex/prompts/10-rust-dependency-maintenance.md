# Codex Prompt: Rust Dependency Maintenance

Use this for Rust dependency updates, lockfile refreshes, and dependency-driven build failures.

Do **not** mix dependency work with Dockerfile rewrites, CI creation, deployment changes, formatting churn, or unrelated refactors.

## Goal

Refresh or repair Rust dependencies with the smallest reasonable change while preserving Redlib behavior and keeping Proofline-local concerns separate from upstreamable fixes.

## Baseline

Before changing dependencies:

```bash
git status --short
git branch --show-current
git rev-parse HEAD
git remote -v
```

Inspect:

- `Cargo.toml`
- `Cargo.lock`
- relevant source files and tests
- current dependency diff, if a lockfile is already dirty
- the exact failing command, if this is build-failure work

Run the relevant failing validation command before fixing when practical.

## Dependency Strategy

- Prefer compatible lockfile refreshes before broad manifest upgrades.
- Use `cargo update -p <package>` for narrow updates when appropriate.
- Use `cargo update` only when broad lockfile refresh is the explicit task.
- Use `cargo tree -d` to inspect duplicate dependency versions when relevant.
- Use `cargo tree -i <package>` to understand why a dependency is present.
- Use `cargo audit` only if it is already installed or documented in this repo. Do not install new tools unless the maintainer asks.
- Keep `Cargo.toml` changes minimal and justified.
- Do not change runtime behavior unless a dependency API change requires it.
- If dependency API changes require code edits, keep them narrow and explain the compatibility reason.
- Do not hide upstream, network, or pre-existing test failures.

## Failure Capture

For a build failure, record:

- exact command
- compiler or tool error category
- package versions involved
- whether the failure is caused by lockfile refresh, manifest change, local code, external network/service behavior, or pre-existing repository state

Do not paste secrets, private hostnames, tokens, request bodies, user data, exploit details, abuse/evasion instructions, or sensitive operator details into logs, docs, screenshots, reports, issues, PRs, or Codex artifacts.

## Validation

After dependency work, run:

```bash
cargo fmt --check
cargo check
cargo test
cargo clippy --all-targets --all-features
cargo build --release
git diff --check
```

If a command fails, say exactly which command failed, why it failed, and whether it appears related to the current dependency work or pre-existing upstream/local behavior.

## Output

Summarize:

1. baseline state
2. dependency changes
3. failing command and cause, if any
4. fix applied
5. files changed
6. validation results
7. follow-up items, especially Dockerfile, CI, or upstream contribution work intentionally deferred
