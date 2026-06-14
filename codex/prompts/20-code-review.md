# Code Review

Review changes as maintainer-reviewed work, not as an audit, certification, endorsement, or security review.

Prioritize findings before summaries. Check:

- correctness and behavior changes
- scope control
- privacy claims and privacy regressions
- error handling
- logging and generated artifacts
- tests and validation
- maintainability
- dependency and lockfile changes
- separation between upstreamable work and Proofline-local changes

Confirm no private operator details or sensitive data were introduced, including secrets, tokens, credentials, request bodies, raw logs containing user data, private hostnames, residential IPs, Cloudflare tunnel IDs, VLAN details, exploit details, or abuse/evasion instructions.

Confirm no Proofline safety/evidence/account behavior was added, including recording/capture, browser/backend/trusted-contact decryption, key escrow, notifications, emergency-services integration, hosted-account billing, payment-gated product access, admin/operator surfaces, or Proofline protocol behavior.

Confirm upstreamable changes are not polluted with Proofline branding, governance copy, support copy, deployment topology, private infrastructure assumptions, or instance-specific source links.

Report validation exactly. Do not claim a command passed unless it actually ran.
