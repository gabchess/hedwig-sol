# 0003: Adoption-led interfaces and permanence

**Status:** Accepted
**Date:** 2026-07-13

## Context

Hedwig's value appears when another program uses a role to protect a real action. The original SDK RFC proposed a crate-first sequence, an in-house agent demo, no multisig governance, and a direct path to freezing. The active grant instead commits to a TypeScript SDK, external design partners, a Squads-compatible integration, and documented security work. Building every possible interface before those integrations would create surface area without evidence of demand.

## Decision

Deliver interfaces in this order:

1. Keep the current six-instruction core stable and redeploy it to devnet.
2. Build a raw Anchor CPI consumer that authenticates the holder whose role gates the protected action.
3. Publish a thin `@hedwig-sol/sdk` with PDA derivation and instruction builders, then move the devnet demo onto it.
4. Onboard three external devnet design partners and publish integration evidence.
5. Transfer devnet upgrade authority from the deployer key to a 2-of-3 Squads multisig before mainnet.
6. Ship one external integration intended for production. Add a standalone Rust CPI helper crate only if at least two integrations report the same reusable friction.
7. Complete an external security review, close all P0 and P1 findings, declare the v1 interface stable, and deploy to mainnet.
8. Freeze the program only after at least one external production integration has exercised the stable mainnet interface and the remaining upgrade risk is lower than the risk of immutability.

Repository documentation is canonical. Hosted documentation may publish or mirror it, but must not become a divergent source of truth.

## Consequences

- Good: each new interface answers observed integration needs instead of predicted ones.
- Good: multisig governance reduces single-key upgrade risk while the interface is still changing.
- Good: the freeze decision becomes an evidence gate rather than a calendar milestone.
- Bad: a Rust CPI convenience crate may ship later than some integrators prefer.
- Bad: mainnet and immutability wait for external evidence and review.
- Neutral: an in-house agent demo can still test a use case, but it does not satisfy the design-partner or production-integration gates.

## Rejected alternatives

- **Ship a Rust CPI crate first:** rejected until repeated integration friction establishes the right reusable API.
- **Use only an in-house consumer before mainnet:** rejected because it cannot establish that the primitive is usable outside the builder's fixtures.
- **Keep a single deployer key until freeze:** rejected because review and partner integration require a safer upgrade path before immutability.
- **Freeze immediately after review:** rejected because a reviewed but unused interface can still be wrong for consumers.
- **Build hosted docs as a separate content system:** rejected because two canonical documentation sources create drift.

## Related

- [`ROADMAP.md`](../../ROADMAP.md)
- [`THREAT-MODEL.md`](../../THREAT-MODEL.md)
- [`docs/sdk-rfc.md`](../sdk-rfc.md), historical where it conflicts with this ADR
- [ADR 0001](0001-small-core.md)
