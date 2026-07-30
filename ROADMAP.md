# Hedwig roadmap

Hedwig is shared authorization for related Anchor programs: one canonical role
store, with each consumer still responsible for authenticating its actor.
Hedwig advances through proof, not feature count. The delivery order is to
secure the integration boundary, test one real two-program path, harden
governance, then consider mainnet.

## Grant context

- Total grant: USDG 3,000.
- Received: USDG 1,500.
- Remaining: USDG 1,500.
- The maintainer reports that the authenticated portal exposes a final-tranche
  request form and the grant program's general completion policy.
- This portal state is not publicly verifiable, and no project-specific
  milestone or KPI text was found.

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
[review](docs/audits/2026-07-24-full-audit.md) and
[core promotion record](docs/audits/2026-07-24-devnet-promotion.md). The
[consumer promotion record](docs/audits/2026-07-24-consumer-devnet-integration.md)
contains its deployment and CPI transaction evidence.

The [grant progress ledger](docs/grant-progress.md) maps shipped work to public
artifacts. It is the public claim record for the remaining USDG 1,500; the
roadmap remains the decision record.

### Founder-led pilot preparation

The [seven-day pilot offer](docs/pilot-program.md) defines one bounded
two-program authorization path, a manual before-and-after authority map, a
custody dry run, migration and rollback steps, and a merge, revise, or reject
decision. The public
[preparation record](docs/progress/2026-07-29-pilot-preparation.md) records the
claim boundary.

No signed pilot, design partner, customer, revenue, production use, or external
adoption has been verified.

## Next evidence gate: sign one seven-day pilot

The next product question is whether one qualified Anchor team will expose a
real state-changing path across two related programs and sign the bounded
pilot. The pilot must:

- verify the technical owner and current release timing;
- interview past authorization behavior before presenting Hedwig;
- produce one manual before/after authority map;
- sign a seven-day scope for one two-program direct or CPI path; and
- end with an explicit merge, revise, or reject decision.

Do not add product features before this signal. If no qualified team signs
after one week of pilot validation, stop feature work, record the rejection
reasons, and review the product thesis. SDK publication, deployment, and
custody changes remain separate decisions.

The signed scope is the demand signal. The later integration is technical
evidence, not a customer-success claim by default.

## Later gates

### Independent design partners

Expand from the signed pilot only if the signal is useful. Three independent
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
delegation, instruction allowlists, spending caps, an agent control surface, an
indexer dashboard, or a standalone Rust CPI crate because they sound useful.
Build one only when observed integrations expose the missing primitive.

The current architecture is documented in
[`docs/architecture.md`](docs/architecture.md). Status claims must link to
tests, review evidence, deployment output, or external artifacts.
