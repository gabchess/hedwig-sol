# Hedwig roadmap

Hedwig's delivery order follows adoption evidence: secure the integration boundary, make it easy to use, validate it with independent teams, harden governance, then deploy to mainnet. The six-instruction onchain core stays small unless real integrations demonstrate a missing primitive.

## Current baseline: devnet core shipped

The following work is implemented in the public repository:

- six Anchor instructions: `create_org`, `create_role`, `assign_role`, `revoke_role`, `check_role`, and `set_role_enabled`;
- devnet program `H4J9wWhraK2Zvn4o9aFheFVmAf7nfaBNPw3d7w77X1eC`, whose current deployment contains the first five instructions;
- a five-instruction membership-lifecycle demo in `app/demo.ts`;
- 21 LiteSVM integration tests covering lifecycle, authorization failures, expiry, revocation, duplicate assignment, checked counters, and the role circuit breaker;
- a documented threat model and repository-local architecture decisions.

The TypeScript SDK, secure reference consumer, design-partner integrations, Squads governance, external security review, and mainnet deployment are not shipped yet.

The live program was last deployed at slot `468922773` on 2026-06-12. A devnet redeploy is required before `set_role_enabled` is available to consumers.

## Grant milestone 1: safe integration and usable tooling

### 1. Secure CPI reference consumer

Ship a minimal consumer program that calls `check_role` only after authenticating the supplied holder. Cover a wallet signer and a consumer-owned PDA, and document why a bare membership check is insufficient.

**Done when:** the six-instruction program is redeployed to devnet, both authorization patterns have passing integration tests, the unsafe unbound-holder case is rejected by a test, and the example can be built from a clean checkout.

### 2. TypeScript SDK alpha

Publish a thin `@hedwig-sol/sdk` package with typed PDA derivation, account reads, and instruction builders for the existing six-instruction program. Rewrite the devnet lifecycle demo to import the package instead of duplicating raw Anchor setup.

**Done when:** `npm view @hedwig-sol/sdk version` returns the alpha release, its package includes type declarations, a clean install passes the package test suite, and the repository demo imports the published API.

### 3. Three devnet design partners

Onboard three independent Solana teams. Each partner must create an org and a live role tree on devnet and record integration feedback in a public issue or linked public integration artifact.

**Done when:** three partner records identify the organization, devnet accounts or transactions, integration use case, and resulting feedback. Builder-owned fixtures and demos do not count toward the three.

## Grant milestone 2: governed integration and mainnet readiness

### 4. Multisig upgrade authority

Transfer the devnet program upgrade authority from the deployer key to a 2-of-3 Squads multisig and document the upgrade procedure.

**Done when:** `solana program show H4J9wWhraK2Zvn4o9aFheFVmAf7nfaBNPw3d7w77X1eC --url devnet` reports the Squads-controlled authority and a successful governed upgrade rehearsal is recorded.

### 5. Production integration example

Ship a working Squads-compatible proposal guard or, if implementation evidence shows Realms is more tractable, a Realms integration that gates an action through Hedwig. The example must authenticate its actor before checking the role.

**Done when:** the integration has a passing end-to-end test, a reproducible devnet transaction, and setup documentation that a reviewer can follow from a clean checkout.

### 6. External review and hosted documentation

Commission an independent security review of the stable candidate and publish its scope, commit hash, findings, and remediation status. Publish hosted documentation that mirrors the repository's canonical docs for the role model, SDK, integration example, and security boundary.

**Done when:** the review report is public, the reviewed commit is identifiable, every critical and high-severity finding is independently verified closed, any accepted lower-severity risk has a published rationale, and the hosted documentation URL resolves.

### 7. Mainnet deployment

Deploy the reviewed candidate to Solana mainnet with multisig upgrade authority and publish the program ID, reproducible build information, and supported SDK version.

**Done when:** the mainnet program account is independently queryable, its upgrade authority is the documented multisig, the tagged source matches the deployed release process, and one external production integration uses it.

## Post-grant gate: consider freezing v1

Immutability is an outcome of stability and adoption, not a launch shortcut. Remove upgrade authority only when all of the following are true:

- the v1 account and instruction interfaces are tagged stable;
- an external security review is complete and all critical and high-severity findings are closed;
- at least one external production integration has operated on mainnet long enough to surface integration defects;
- maintainers publish a freeze proposal, migration implications, and verification steps before execution.

**Done when:** the published gate evidence is complete and `solana program show <MAINNET_PROGRAM_ID>` reports no upgrade authority. Until then, governance remains with the documented multisig.

## Evidence-gated interface decisions

- The core remains flat: orgs contain roles, and roles contain memberships. Hierarchy, eligibility, spending policy, and holder-driven delegation belong in consumers or wrapper programs.
- A standalone `hedwig-cpi` Rust crate is deferred. Build it only if at least two independent integrations report the same repeated CPI or account-validation friction and the crate removes that duplication without expanding the onchain program.
- An in-house agent demo may test a use case, but it does not replace the three independent design partners or the production-integration requirement.
- Repository documentation is canonical. Hosted documentation mirrors it rather than creating a second source of truth.

The durable rationale for this sequence lives in `docs/adr/`; grant progress should link to verifiable artifacts rather than replace this roadmap with status prose.
