# 0004: Audit integrations before devnet promotion

**Status:** Accepted
**Date:** 2026-07-24

## Context

ADR 0003 set the right product direction but put the devnet upgrade before the
consumer and SDK proof. It also assumed the SDK should be published at once.
The approved delivery slice now requires the opposite promotion order: prove
the integration boundary locally, audit the exact source, then upgrade devnet.
No npm publication is authorized in this slice.

## Decision

Use this order for the current devnet release:

1. Build and test a minimal Anchor consumer that authenticates its actor before
   mapping that actor to Hedwig's holder.
2. Build a private local TypeScript SDK for all six instructions and make the
   devnet demo use it.
3. Audit the program, consumer, SDK, demo, CI, and deployment process at one
   commit.
4. Build the consumer first and Hedwig last, then promote only the reviewed
   Hedwig artifact to the fixed devnet program ID.
5. Dump the live program and require byte equality with the reviewed artifact.
6. Run the six-instruction demo and keep the SDK unpublished.

Publish the SDK only after an independent integration shows that registry
distribution is the actual blocker, or after a separate explicit release
decision.

This ADR supersedes ADR 0003 only for the order and publication timing above.
Its small-core, evidence, governance, mainnet, and freeze gates remain active.

## Consequences

- Good: the deployment is the output of the audit instead of an input to it.
- Good: the secure consumer proves Hedwig's most important integration rule.
- Good: local package use tests the SDK without claiming distribution or use.
- Bad: external teams cannot install the SDK from npm yet.
- Neutral: the consumer remains builder-owned evidence and does not count as
  adoption, whether local or deployed.

## Rejected alternatives

- **Upgrade before the integration audit:** rejected because the live artifact
  would not include the evidence used to approve it.
- **Publish the alpha now:** rejected because this delivery slice does not
  authorize npm publication and no external team has shown it to be the blocker.
- **Expand the onchain core:** rejected because the current evidence calls for
  integration proof, not another role primitive.

## Related

- [ADR 0001](0001-small-core.md)
- [ADR 0003](0003-adoption-led-interfaces.md)
- [`ROADMAP.md`](../../ROADMAP.md)
- [`THREAT-MODEL.md`](../../THREAT-MODEL.md)
- [`docs/integration-guide.md`](../integration-guide.md)
- [`docs/audits/2026-07-24-full-audit.md`](../audits/2026-07-24-full-audit.md)
