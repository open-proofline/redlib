# Upstream Contribution Review

Use this to decide whether a change should be proposed upstream to Redlib.

Upstreamable changes can include:

- bug fixes
- performance improvements
- caching improvements
- request coalescing
- accessibility improvements
- generic docs improvements
- container hygiene
- CI improvements
- reliability improvements
- safe operator configuration

Not upstreamable as-is:

- Proofline branding
- Proofline service links
- Proofline deployment topology
- private infrastructure assumptions
- Proofline governance, moderation, or support copy
- Proofline safety/evidence/account behavior
- instance-specific operator policy

Require clean branch separation before upstream work. Keep the PR summary concise, describe the user-visible or maintainer-visible effect, and avoid mentioning private deployment details.

If a change contains both generic and Proofline-specific parts, split it before proposing anything upstream.
