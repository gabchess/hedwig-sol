# Hedwig architecture decisions

This directory records decisions that define Hedwig's product boundary, repository shape, and delivery order. Use issues and pull requests for work tracking.

## Active decisions

| ADR | Decision | Status |
|---|---|---|
| [0001](0001-small-core.md) | Keep the authorization core small and flat | Accepted |
| [0002](0002-screaming-architecture.md) | Use a conventional Anchor shell with a domain-visible core | Accepted |
| [0003](0003-adoption-led-interfaces.md) | Add interfaces and permanence only after integration evidence | Accepted |
| [0004](0004-audit-before-devnet-promotion.md) | Audit the consumer and private SDK before devnet promotion | Accepted |

## Historical design documents

[`docs/sdk-rfc.md`](../sdk-rfc.md) remains the original SDK design record. ADR 0003 supersedes it where the RFC:

- describes five instructions instead of the implemented six;
- prioritizes a Rust CPI crate and an in-house agent demo ahead of the TypeScript SDK and external design partners;
- excludes multisig upgrade governance; or
- schedules an unconditional mainnet freeze.

Its decisions to keep roles flat, keep the TypeScript SDK thin, and avoid offchain permission caches remain consistent with the active ADRs.

ADR 0004 supersedes ADR 0003 only where ADR 0003 schedules the devnet upgrade
before local integration proof or assumes immediate npm publication.

## Adding an ADR

Use the next number and include: status, date, context, decision, consequences, rejected alternatives, and related material. Add the ADR to the table above. Record only durable decisions; use issues and pull requests for work tracking.
