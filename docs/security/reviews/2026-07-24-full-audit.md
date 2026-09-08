# Hedwig Full Audit

Date: 2026-07-24

Reviewed source: `5d157c9d2bfc52060c513ff39ed53effc301e2eb`

This is an AI-assisted structured security and code review. Hedwig has not
completed an independent human security audit, and this record does not
guarantee that the code has no defects.

Deployment note: the reviewed consumer was assigned its generated deploy ID,
rebuilt, reran the full 42-test Rust suite, and was deployed separately after
the core promotion. The clean public source commit above includes that ID
alignment. Its exact-byte and live CPI proof is recorded in
[`2026-07-24-consumer-devnet-integration.md`](../../deployment/evidence/2026-07-24-consumer-devnet-integration.md).

## Verdict

The review found no unresolved Critical or High program-code flaw in the
reviewed source. The fixed-ID devnet upgrade, deployed-byte identity check,
unchanged-authority check, and live six-instruction demo passed on 2026-07-24.

Hedwig is not ready for mainnet. The live upgrade authority is one local signer,
and `main` has no GitHub branch protection. Mainnet needs governed upgrade
custody, required review and CI rules, monitoring, recovery practice, and a
separate release review.

## Reviewed surfaces

- The six Anchor instructions and all Org, Role, and Member account constraints
- The secure consumer CPI and its signer-to-holder boundary
- The repository-local TypeScript SDK and SDK-driven demo
- Rust, LiteSVM, TypeScript, build, CI, dependency, and deployment paths
- Public status, integration, roadmap, operations, and security claims
- Structured controls for Solana and Anchor program security, release practice,
  dependencies, deployment, and public claims

At the audit checkpoint, the reference consumer was local proof code. It was
later deployed to devnet and remains builder-owned evidence, not independent
adoption. The SDK is a repository-local alpha and is not published to npm.

## Evidence

- 42 Rust tests passed, including 40 integration tests and two program-ID tests.
- 27 SDK tests passed.
- Workspace Rust builds, both SBF builds, Clippy with warnings denied, SDK
  typechecks and build, app typecheck, Prettier, and Git diff hygiene passed.
- The app lockfile dependency audit found zero known vulnerabilities at the
  configured severity gate.
- Duplicate Org, Role, Member, and consumer Counter transactions use fresh
  blockhashes, assert exact errors, and assert unchanged state.
- Consumer tests reject holder substitution, missing signatures, wrong
  authority, expiry, disabled and revoked roles, wrong role, program
  substitution, foreign-owned state, duplicate initialization, and overflow.
- CI action revisions, Rust, Anchor, and Agave inputs are pinned. The Agave
  installer is checked against a fixed SHA-256 digest.
- The fixed program ID was upgraded at slot `478655638`, every compiled ELF byte
  matched the fresh live dump, and all loader allocation bytes after the ELF
  were zero.
- The live SDK demo finalized all six instruction transactions, fetched the
  disabled Role as `enabled=false`, and found the revoked Member PDA closed.

## Checklist result

The structured review classified the applicable controls and converted the
release-relevant gaps into the findings below. Mainnet governance, formal
methods, fuzzing, automated monitoring, release automation, and compliance
process remain outside the verified devnet release.

## Findings

### High: single-signer upgrade custody blocks mainnet

The devnet upgrade authority is the configured local deployer signer. A host or
key compromise could replace all program logic and bypass every onchain
invariant.

This is accepted only for the approved devnet upgrade. Before mainnet, move
authority to the planned 2-of-3 governed custody, rehearse an upgrade and
recovery, and add authority monitoring.

### Resolved promotion gate: reviewed and live code match

The saved predeploy devnet dump has SHA-256
`006a09bffd1a317c1aeef8f1c774d0c6704bc36962209da241459482b6e17d39`.
It is the old five-instruction artifact and does not match the reviewed local
six-instruction build.

The reviewed ELF has SHA-256
`42670041e7df0f9832930bfa511b884e8b59ceebc8e14b0638c4627a83e6aed3`.
Agave extended ProgramData in a 10,240-byte step, so the fresh dump includes
5,104 zero bytes after the 222,808-byte ELF. The ELF prefix compares equal, all
trailing bytes are zero, and the zero-padded reviewed account image is fully
byte-equal to the live dump with SHA-256
`7218f6f8f538cfb3583159e6b18c39668ce643b65cae2aa4bf61c5209d71a543`.

### Medium: TypeScript gates are local, not enforced by CI

CI enforces Rust formatting, Rust and SBF builds, Clippy, Rust tests, and the app
dependency audit. SDK tests and typechecks plus the app typecheck pass locally
but are not yet in GitHub Actions.

The repository uses Yarn at the root. Adding a CI install needs an explicit
package-install policy decision. Until then, these remain required local release
gates and a recurring assurance gap.

### Medium: `main` has no branch protection

The GitHub branch-protection query returned `404 Branch not protected`. A direct
push can bypass review and pinned CI. This does not change the reviewed commit,
but it blocks a mainnet-grade release process.

### Low: deploy keypair address does not match the fixed Hedwig ID

`declare_id!` and `Anchor.toml` use
`H4J9wWhraK2Zvn4o9aFheFVmAf7nfaBNPw3d7w77X1eC`, while the checked-out generated
program keypair resolves to another address. Default deploy tooling could target
the wrong program.

The devnet operation uses an explicit `--program-id` argument, verifies the
authority before and after the upgrade, and compares a fresh live dump byte for
byte. Do not deploy this release through an implicit Anchor program-keypair
path.

## Architecture assessment

The core remains small and direct. Hedwig keeps six role instructions, typed
Anchor accounts, canonical PDAs, stored bumps, checked counters, and no token or
treasury logic. The consumer proves the important integration rule: authenticate
the actor in the consuming program, then pass that same signer as Hedwig's
holder.

Hierarchy, badges, claimable roles, spending limits, indexing, Rust CPI helpers,
and other extensions remain outside this release. Those features need
integration evidence first. The next product step is one external devnet
integration with measured setup friction.

## Residual assurance work

Before any mainnet proposal:

1. Move upgrade authority to governed custody and protect the release branch.
2. Add deterministic SDK and app gates to CI under an approved Yarn install
   policy.
3. Add dependency advisory coverage for Rust and the root Yarn graph.
4. Add fuzz or property tests, compute-unit profiling, release monitoring, and
   an incident drill.
5. Commission an independent human review against a commit-pinned mainnet
   candidate.
