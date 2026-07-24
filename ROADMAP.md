# Hedwig roadmap

Hedwig advances through proof, not feature count. The delivery order is:
secure the integration boundary, make the six-instruction core easy to use,
observe independent use, harden governance, then consider mainnet.

## Grant context

- Total grant: USDG 3,000.
- Received: USDG 1,500.
- Remaining: USDG 1,500.
- No written portal milestone or second-tranche checklist has been provided.

The working rule is therefore to keep shipping verifiable roadmap progress.
Grant amounts are funding facts, not evidence that a milestone or adoption
claim has been accepted.

## Current release slice

### Six-instruction core

The source implements:

1. `create_org`
2. `create_role`
3. `assign_role`
4. `check_role`
5. `set_role_enabled`
6. `revoke_role`

The local core suite contains 28 LiteSVM integration tests plus its generated
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
artifact was deployed at slot `478667066`; the live dump matches it byte for
byte. A fresh authenticated actor then created a Hedwig role and membership,
initialized a consumer-owned counter, and incremented it from zero to one
through `check_role` CPI.

This proves the live integration path. The consumer is maintained by the Hedwig
builder and does not count as independent use.

### TypeScript SDK alpha

The private repository-local `@hedwig-sol/sdk` alpha contains typed PDA helpers,
argument validation, generated IDL types, and build/send pairs for all six
instructions. The demo imports this package instead of rebuilding raw Anchor
calls. Its suite contains 27 tests.

**State:** built and tested locally as `0.1.0-alpha.0`. It is not published to
npm.

### Audit and devnet promotion

The release gate covered the core, consumer, SDK, demo, CI, docs, and deployment
process at one commit. The 2026-07-24 promotion proved:

- no unresolved Critical or High code finding that applies to devnet;
- all 949 audit checks classified with zero pending rows;
- the consumer built first and Hedwig built last;
- the deployment command pinned to the fixed program ID;
- the compiled ELF matched every deployed code byte, with only zero-filled
  loader allocation padding after it; and
- the exact six-instruction live lifecycle completed, including the disabled
  Role state and closed Member account.

The single-key upgrade authority remains a named mainnet blocker. It does not
invalidate the completed devnet-only upgrade. See the
[audit](docs/audits/2026-07-24-full-audit.md) and
[core promotion record](docs/audits/2026-07-24-devnet-promotion.md). The
[consumer promotion record](docs/audits/2026-07-24-consumer-devnet-integration.md)
contains its deployment and CPI transaction evidence.

The [grant progress ledger](docs/grant-progress.md) maps shipped work to public
artifacts. It is the public claim record for the remaining USDG 1,500; the
roadmap remains the decision record.

## Next evidence gate: one independent integration experiment

The next product question is whether another Solana team can use Hedwig to gate
a real state-changing action without builder help hiding integration problems.
Subject to separate outreach approval, run one concierge devnet integration and
record:

- the team's existing authorization behavior;
- time to the first successful role check;
- holder-authentication mistakes and repeated account choreography;
- whether the team keeps the integration after the test; and
- whether package distribution, a Rust CPI helper, or another missing surface
  blocked them.

This experiment is evidence, not a partner claim by default. The repository
currently has no verified design partner, customer, revenue, or production use.

## Later gates

### Independent design partners

Expand from the first experiment only if the signal is useful. Three independent
teams must publish or link reproducible devnet integration evidence. Builder
fixtures and the reference consumer do not count.

### Agent runtime egress hardening

Pilot [Hermes Agent's IronProxy](https://hermes-agent.nousresearch.com/docs/user-guide/egress/iron-proxy)
for one Docker-sandboxed Hedwig/Estaleiro workflow after the first independent
integration experiment. This belongs to the offchain agent runtime, not the
Hedwig program or authorization model. Its
[implementation notes](https://hermes-agent.nousresearch.com/docs/developer-guide/egress-internals)
define the security-sensitive lifecycle and current backend limits.

The pilot must:

- use one supported provider and a host allowlist;
- expose only proxy tokens inside the container, not the real provider secret;
- deny private, link-local, and metadata-service destinations;
- fail closed if the proxy, CA, mapping, or audit stream is unavailable;
- preserve a request log without secret values; and
- document bypass limits, including raw sockets, host compromise, allowlisted
  exfiltration, and unsupported signature-based credentials.

Adopt it only if the Docker proof passes and the operational burden is
acceptable. Hermes currently wires IronProxy to Docker, not every terminal
backend. This work hardens builder credentials; it is not evidence of Hedwig
adoption and does not block the next devnet partner test.

### Governed upgrade authority

Move the devnet program upgrade authority from the deployer key to a 2-of-3
Squads multisig and rehearse one governed upgrade before mainnet.

### Production integration

Build a Squads-compatible proposal guard, or another production-bound consumer
supported by observed demand, only after integration evidence establishes the
right target. It must authenticate its actor before checking a role.

### External security review and hosted docs

Publish an independent review of a stable candidate with scope, commit, findings,
and remediation status. Hosted docs may mirror the repository, which remains
canonical.

### Mainnet

Deploy only after multisig custody, external review, reproducible release
evidence, and an external production-bound integration are complete.

### Consider freezing v1

Remove upgrade authority only after the stable mainnet interface has external
operating evidence and the published freeze proposal shows that immutability is
safer than continued governed upgrades.

## Deferred until evidence changes

Do not add role hierarchy, eligibility modules, badges, claimable roles,
delegation, instruction allowlists, spending caps, an agent control surface, an
indexer dashboard, or a standalone Rust CPI crate because they sound useful.
Build one only when observed integrations expose the missing primitive.

The durable architecture decisions live in [`docs/adr/`](docs/adr/index.md).
Status claims must link to tests, audit evidence, deployment output, or external
artifacts.
