# Grant progress ledger

This ledger records public, verifiable progress toward Hedwig's USDG 3,000
grant. USDG 1,500 has been received and USDG 1,500 remains. No written portal
checklist defines release of the second tranche, so each claim below links to
code, tests, or live evidence.

This file records delivery. It does not claim that the grant issuer accepted a
milestone, that a repository fixture is a customer, or that unobserved demand
exists.

Public delivery review:
[`#3 — Ship secure consumer integration and grant evidence`](https://github.com/gabchess/hedwig-sol/pull/3)

## Shipped on 2026-07-24

| Delivery                                             | Public evidence                                                                                                                  | State                                |
| ---------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------ |
| Six-instruction core, including role circuit breaker | [`programs/hedwig_sol`](../programs/hedwig_sol), [threat model](../THREAT-MODEL.md)                                              | Live on devnet                       |
| Secure actor-bound CPI consumer                      | [`programs/hedwig_consumer`](../programs/hedwig_consumer), [integration guide](integration-guide.md)                             | Tested and live on devnet            |
| Six-instruction TypeScript SDK alpha                 | [`sdk`](../sdk)                                                                                                                  | Private repository package; 27 tests |
| SDK-driven six-instruction lifecycle                 | [`app/demo.ts`](../app/demo.ts), [core promotion record](audits/2026-07-24-devnet-promotion.md)                                  | Verified on devnet                   |
| Hedwig-gated state change in a separate program      | [`app/consumer-demo.ts`](../app/consumer-demo.ts), [consumer promotion record](audits/2026-07-24-consumer-devnet-integration.md) | Verified on devnet                   |
| Commit-pinned security and code audit                | [full audit](audits/2026-07-24-full-audit.md), [949-row matrix](audits/2026-07-24-checklist-matrix.md)                           | Zero pending checks                  |
| Reproducible release checks and CI hardening         | [workflow](../.github/workflows/ci.yml), [operations runbook](operations.md)                                                     | Shipped                              |
| Public architecture and evidence-gated roadmap       | [architecture](architecture.md), [ADRs](adr/index.md), [roadmap](../ROADMAP.md)                                                  | Shipped                              |

## Verification totals

- 42 Rust tests passed: 28 core integration tests, 12 consumer integration
  tests, and two program-ID tests.
- 27 TypeScript SDK tests passed.
- Both SBF programs built from the reviewed source.
- The live core dump matches the reviewed ELF with documented zero-filled
  loader allocation padding.
- The live consumer dump matches its 176,264-byte reviewed artifact exactly.
- The live core lifecycle and consumer CPI state change both finalized on
  devnet.

## Current claim

Hedwig now has a small live role registry, a tested SDK, and a builder-owned
reference integration that authenticates the actor before checking its role.
The integration boundary is implemented, audited, deployed, and reproducible.

Hedwig still has no verified independent design partner, customer, revenue,
production use, independent external audit, or mainnet release.

## Next claim gate

Run one concierge devnet integration with an independent Solana team. Record the
team's prior authorization approach, time to the first successful Hedwig-gated
state change, actor/holder mistakes, repeated account work, missing package or
CPI surfaces, and whether the team keeps the integration.

Only an external artifact or a recorded first-party observation from that team
can close this gate. The builder-owned reference consumer cannot.

## Later security work

- Move upgrade authority to a rehearsed 2-of-3 Squads multisig before mainnet.
- Obtain an external security review of a stable candidate.
- Pilot Hermes IronProxy for one Docker-sandboxed agent workflow before adopting
  it across Hedwig/Estaleiro automation. This protects offchain provider
  credentials; it does not change Hedwig's onchain guarantees.

The roadmap is canonical for delivery order:
[`ROADMAP.md`](../ROADMAP.md).
