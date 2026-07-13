# 0002: Conventional shell, domain-visible core

**Status:** Accepted
**Date:** 2026-07-13

## Context

Solana and Anchor contributors benefit from a familiar repository shape, while generic technical folders can hide what a program actually does. Hedwig's program is already small and its instruction modules already use authorization language. A broad physical reorganization would add review risk without making the domain clearer.

## Decision

Keep the conventional Anchor repository shell and make Hedwig's authorization domain visible at every boundary inside it. Accounts use the nouns `Org`, `Role`, and `Member`. Instruction modules use the actions `create_org`, `create_role`, `assign_role`, `revoke_role`, `check_role`, and `set_role_enabled`. Tests follow the same vocabulary. Public documentation begins with the authorization model, then explains Anchor and Solana mechanics.

Add a folder or abstraction only when it represents a stable domain boundary or removes demonstrated repetition. Prefer deletion, platform conventions, and the smallest working surface over speculative structure. Do not weaken validation, error handling, or security to reduce line count.

## Consequences

- Good: an Anchor reviewer recognizes the project layout immediately and can identify the product domain from file and type names.
- Good: architecture changes are judged by domain value, not by folder symmetry.
- Bad: framework-level files such as `lib.rs`, `state.rs`, and `error.rs` remain generic at the outer shell.
- Neutral: documentation and ADRs carry the cross-file domain map without forcing a risky source move.

## Rejected alternatives

- **Reorganize the entire repository into custom domain folders:** rejected because it would fight Anchor conventions and add migration risk.
- **Group instructions into generic handlers or actions:** rejected because those names hide the authorization use cases.
- **Create a folder for every account or concern:** rejected because the six-instruction core does not justify that ceremony.
- **Keep architectural intent only in external notes:** rejected because contributors must be able to understand the repository from the repository itself.

## Related

- [`docs/architecture.md`](../architecture.md)
- [ADR 0001](0001-small-core.md)
