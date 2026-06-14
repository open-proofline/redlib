# Container and Deployment Review

Use this for Dockerfiles, compose files, reverse proxy examples, Cloudflare-related examples, CI image publishing, runtime packaging, and deployment documentation.

Treat this as sensitive operator-surface work. Review:

- non-root runtime
- minimal runtime image
- CA certificates for outbound HTTPS
- `.dockerignore`
- build caching
- pinned or floating base image implications
- multi-arch build behavior
- health checks
- logging behavior
- environment variables
- secret handling
- generated image metadata
- docs and examples

Do not document private deployment topology, residential IPs, Cloudflare tunnel IDs, private hostnames, VLAN details, tokens, credentials, request bodies, raw logs containing user data, exploit details, abuse/evasion tactics, or sensitive operator details.

Keep public examples generic and safe. Do not imply production readiness, anonymity, staffed operations, emergency response, or integration with Proofline safety/evidence/account systems.

Separate generic upstreamable container hygiene from Proofline-specific deployment, branding, governance, or support changes.
