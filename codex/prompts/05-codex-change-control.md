# Codex Change Control

Use this before implementation when a request could expand beyond a small change.

1. Decide whether the request is scoped enough for a small PR-sized change.
2. If scope is unclear, narrow the work to the smallest defensible change or ask for clarification.
3. Keep Proofline-local patches separate from upstreamable Redlib improvements.
4. Do not mix unrelated Docker, CI, dependency, formatting, refactor, deployment, or docs churn into the current change.
5. Defer unrelated findings into follow-up notes instead of implementing them.
6. Treat Docker, compose, reverse proxy examples, Cloudflare-related examples, CI image publishing, and deployment guidance as security-sensitive operator-surface work.
7. Preserve upstream Redlib behavior unless the task explicitly requires a fork-local delta.
8. Validate with the smallest command set that covers the risk, and report failures exactly.

Do not add Proofline Server behavior, account-portal behavior, safety/evidence workflows, admin/operator surfaces, recording/capture, decryption, key escrow, notifications, emergency-services integration, billing, payment-gated access, or Proofline protocol behavior in this repository.

Do not include secrets, tokens, private deployment details, request bodies, user data, exploit details, abuse/evasion instructions, or sensitive operator details in public artifacts.
