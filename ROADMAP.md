# Hedwig roadmap

Hedwig gives Solana apps a shared, revocable role record for software agents.
Implementation work is paused after the devnet core and reference consumer.
When work resumes, the next build is admin and authority rotation, closing a
gap named in THREAT-MODEL.md. The next product-validation slice remains one
recurring protected action with an authenticated agent identity. That
validation is an integration hypothesis, not a shipped hosted application.

Funding and accepted delivery facts remain in the [grant ledger](docs/grants/progress.md).

## Current release slice

### Six-instruction core

The source implements:

1. `create_org`
2. `create_role`
3. `assign_role`
4. `check_role`
5. `set_role_enabled`
6. `revoke_role`

The local core suite contains 29 LiteSVM integration tests plus its generated
program ID test. The current devnet program at
`H4J9wWhraK2Zvn4o9aFheFVmAf7nfaBNPw3d7w77X1eC` contains the reviewed
six-instruction artifact deployed at slot `478655638`.

### Secure CPI reference consumer

The repository now contains `programs/hedwig_consumer`, a minimal program that
requires an authenticated signer, binds its counter PDA and stored authority to
that signer, and passes the same account to Hedwig as the holder. Its 12
LiteSVM integration tests cover the successful path and the main authorization,
account, lifecycle, program-substitution, and overflow failures.

**State:** shipped on devnet at
`52D3pTYvMwLYbiigY5xg55n4HmtEzTKCEicx1Cojzo9a`. The reviewed 176,264-byte
artifact was deployed at slot `478667066`; the July 24 live dump matched it
byte for byte. A fresh authenticated actor then created a Hedwig role and membership,
initialized a consumer-owned counter, and incremented it from zero to one
through `check_role` CPI.

This proves the live integration path. The consumer is maintained by the Hedwig
builder and does not count as independent use.

### TypeScript SDK alpha

The repository-local `@hedwig-sol/sdk` alpha contains typed PDA helpers,
argument validation, generated IDL types, and build/send pairs for all six
instructions. The demo imports this package instead of rebuilding raw Anchor
calls. Its suite contains 27 tests.

**State:** built and tested locally as `0.1.0-alpha.0`. It is not published to
npm.

### Review and devnet promotion

The release gate covered the core, consumer, SDK, demo, CI, docs, and deployment
process at one commit. The 2026-07-24 promotion proved:

- no unresolved Critical or High code finding that applies to devnet;
- the consumer built first and Hedwig built last;
- the deployment command pinned to the fixed program ID;
- the compiled ELF matched every deployed code byte, with only zero-filled
  loader allocation padding after it; and
- the exact six-instruction live lifecycle completed, including the disabled
  Role state and closed Member account.

The single-key upgrade authority remains a named mainnet blocker. It does not
invalidate the completed devnet-only upgrade. See the
[review](docs/security/reviews/2026-07-24-full-audit.md) and
[core promotion record](docs/deployment/evidence/2026-07-24-devnet-promotion.md). The
[consumer promotion record](docs/deployment/evidence/2026-07-24-consumer-devnet-integration.md)
contains its deployment and CPI transaction evidence.

The [grant progress ledger](docs/grants/progress.md) maps shipped work to public
artifacts. It is the public claim record for the remaining USDG 1,500; the
roadmap remains the decision record.

## Next build: role admin and org authority rotation

Planned. No code exists yet.

`Role.admin` is fixed at `create_role`, initialized to the org authority, and
`Org.authority` is fixed at `create_org`. The six-instruction surface has no
way to change either field, so a lost, departed, or compromised key
permanently strands every role and membership under it. THREAT-MODEL.md names
this under
["Fixed cardinality and immutable authorities"](THREAT-MODEL.md#fixed-cardinality-and-immutable-authorities).

This ships as one or more new instructions that let the current org authority
rotate `Org.authority`, and let the current role admin rotate `Role.admin`,
each gated on the existing key's signature. It closes when the instruction
exists in the reviewed source, a LiteSVM test proves a successful rotation,
negative tests prove that only the current authority or admin can rotate, and
a test proves the new key retains full control over the org's or role's
existing state after rotation.

## Next evidence gate: one protected action

The [agent access integration proof](docs/agent-access/integration-proof.md) is
ready for later work. It starts with one recurring action and one technical
owner. The buyer remains a hypothesis until an independent team shows a real
access problem and accepts the integration.

1. Identify the actor, its existing credentials, and the action being protected.
2. Prove authenticated success while membership is valid.
3. Prove a later retry fails after confirmed revocation, and separately after expiry.
4. Show the transaction result and unchanged protected state on denial.
5. Record whether an independent team would keep the integration and why.

A builder-owned consumer proves behavior. It does not establish external demand. The
[older two-program pilot](docs/history/2026-07-shared-authorization-pilot.md)
remains a historical record, not the current integration proof.

## Later gates

### Independent design partners

Expand from the agent access proof only if the signal is useful. Three independent
teams must publish or link reproducible devnet integration evidence. Builder
fixtures and the reference consumer do not count.

### Governed upgrade authority

Move the devnet program upgrade authority from the deployer key to a 2-of-3
Squads multisig and rehearse one governed upgrade before mainnet.

### Production integration

Build a Squads-compatible proposal guard, or another production-bound consumer
supported by observed demand, only after integration evidence establishes the
right target. It must authenticate its actor before checking a role.

### External human security review and hosted docs

Publish a human security review of a stable candidate with scope, commit,
findings, and remediation status. Hosted docs may mirror the repository, which
remains canonical.

### Mainnet

Deploy only after multisig custody, external review, reproducible release
evidence, and an external production-bound integration are complete.

### Consider freezing v1

Remove upgrade authority only after the stable mainnet interface has external
operating evidence and the published freeze proposal shows that immutability is
safer than continued governed upgrades.

## Deferred until evidence changes

Do not add role hierarchy, eligibility modules, badges, claimable roles,
delegation, instruction allowlists, spending caps, an
indexer dashboard, or a standalone Rust CPI crate because they sound useful.
Build one only when observed integrations expose the missing primitive.

The current architecture is documented in
[`docs/access-control/architecture.md`](docs/access-control/architecture.md). Status claims must link to
tests, review evidence, deployment output, or external artifacts.
