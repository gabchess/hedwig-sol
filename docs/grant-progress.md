# Grant progress ledger

This ledger records Hedwig's progress toward its USDG 3,000 grant. USDG 1,500
has been received and USDG 1,500 remains. No written portal checklist defines
release of the second tranche. Public software and deployment claims below link
to code, tests, or live evidence. Builder-recorded private preparation is
labeled as aggregate status and is not third-party-verifiable here.

This file records delivery. It does not claim that the grant issuer accepted a
milestone, that a repository fixture is a customer, or that unobserved demand
exists.

Public delivery review:
[`#3: Ship secure consumer integration and grant evidence`](https://github.com/gabchess/hedwig-sol/pull/3)

## Shipped on 2026-07-29

| Delivery | Public evidence | State |
| --- | --- | --- |
| Narrow founder-led pilot decision | [pilot program](pilot-program.md), [design](superpowers/specs/2026-07-29-founder-led-pilot-preparation-design.md) | Shipped |
| Dependency-aware preparation plan | [implementation plan](superpowers/plans/2026-07-29-founder-led-pilot-preparation.md) | Shipped |
| Public preparation and claim record | [pilot preparation record](progress/2026-07-29-pilot-preparation.md), [roadmap](../ROADMAP.md) | Shipped |
| Private account and message preparation | Builder-recorded private aggregate; public method and counts only | 15 research-qualified accounts; five message pairs (five primary, five follow-ups); zero sent |
| Private pilot operating pack | Builder-recorded private aggregate; public artifact names only | Ready for an approved outreach run |

The named account records, contact routes, source notes, scores, and message
copy remain private. This public ledger supports the method and aggregate
counts; it does not let a third party verify each private row or treat research
qualification as customer validation.

No external conversation, design partner, signed pilot, customer, revenue, or
production use was created by this preparation work.

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

Secure one signed seven-day pilot with a qualified Anchor team for one real
state-changing path across two related programs. The gate requires:

- past-behavior evidence from the team's current authorization path;
- one manual before/after authority map;
- a named technical owner and final-decision owner;
- an agreed direct or CPI path; and
- an explicit merge, revise, or reject end state.

No product feature work starts before that signal. Only the signed scope and
first-party team evidence can close this gate. A score, a draft, a polite reply,
or the builder-owned reference consumer cannot.

## Later security work

- Move upgrade authority to a rehearsed 2-of-3 Squads multisig before mainnet.
- Obtain an external security review of a stable candidate.
- Pilot Hermes IronProxy for one Docker-sandboxed agent workflow before adopting
  it across Hedwig/Estaleiro automation. This protects offchain provider
  credentials; it does not change Hedwig's onchain guarantees.

The roadmap is canonical for delivery order:
[`ROADMAP.md`](../ROADMAP.md).
