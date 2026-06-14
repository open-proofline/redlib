# Project Context Check

Use this before editing.

1. Inspect repository state:
   - `git status --short`
   - `git branch --show-current`
   - `git rev-parse HEAD`
   - `git remote -v`
2. Inspect repository shape:
   - top-level files
   - `README*`
   - `LICENSE*`
   - `Cargo.toml`
   - `Cargo.lock`
   - Rust source layout
   - tests
   - Dockerfiles
   - compose files
   - CI/workflow files
   - docs and configuration files
   - existing `AGENTS.md` and `codex/` files
3. Identify fork context:
   - whether an upstream Redlib remote is present
   - default branch names from remotes when available
   - whether the requested change is Proofline-instance-specific, upstreamable, or both
4. Summarize before editing:
   - current branch and commit
   - dirty working tree entries
   - relevant repository shape
   - scope classification
   - main risks
   - validation commands to run

Do not assume the upstream project is healthy, current, or inactive without evidence.

Do not include private deployment details, residential IPs, Cloudflare tunnel IDs, private hostnames, VLAN details, tokens, secrets, credentials, request bodies, raw logs containing user data, exploit details, abuse/evasion instructions, or sensitive operator details in public docs, issues, tests, screenshots, logs, summaries, or Codex artifacts.
