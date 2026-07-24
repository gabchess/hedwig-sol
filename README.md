# Hedwig

**Grant roles, not keys.**

Composable onchain roles for Solana. Assign a named role to any pubkey, set
optional membership expiry, disable the role in an incident, and let any Solana
program verify active membership via CPI.

Hedwig is a devnet-stage Anchor program. The repository implements six
instructions, a secure reference consumer, and a private TypeScript SDK alpha.
The local suites contain 40 LiteSVM integration tests and 27 SDK tests. The
reviewed core and consumer are both live on devnet. On 2026-07-24, the SDK
completed the six-instruction lifecycle and an authenticated member changed
state in the separate consumer through CPI.

Devnet program: `H4J9wWhraK2Zvn4o9aFheFVmAf7nfaBNPw3d7w77X1eC`

Devnet reference consumer: `52D3pTYvMwLYbiigY5xg55n4HmtEzTKCEicx1Cojzo9a`

## Why Hedwig

Solana programs that need roles usually define their own account layout,
authorization rules, expiry behavior, and revocation flow. Those implementations
do not compose: a role recognized by one program has no standard meaning to
another.

Hedwig makes membership a small onchain primitive:

- an `Org` is a role namespace;
- a `Role` is a named authority within that org;
- a `Member` records that one pubkey holds one role; and
- `check_role` is the CPI entrypoint other programs can use to verify active
  membership.

The holder can be a wallet, multisig, program-derived identity, or agent key.
Hedwig records membership; the consuming program decides what that role permits.

## Current status

| Surface             | Current state                                                                       |
| ------------------- | ----------------------------------------------------------------------------------- |
| Anchor program      | Six instructions implemented                                                        |
| Rust tests          | 40 LiteSVM integration tests across the core and consumer                           |
| TypeScript tests    | 27 SDK tests plus SDK and app typechecks                                            |
| Devnet evidence     | Upgrade verified at slot `478655638`; six-instruction lifecycle finalized afterward |
| Circuit breaker     | Live `enabled=false` state verified on devnet                                       |
| TypeScript SDK      | Private local `0.1.0-alpha.0`; built and tested, not published                      |
| Secure CPI consumer | Deployed at slot `478667066`; live Hedwig-gated state change verified               |
| Upgrade authority   | Single deployer key; 2-of-3 Squads transfer planned before mainnet                  |
| Network             | Devnet; mainnet is planned                                                          |

See [ROADMAP.md](ROADMAP.md) for evidence-gated delivery milestones and
[THREAT-MODEL.md](THREAT-MODEL.md) for the current trust boundaries.

Deployment evidence was checked on 2026-07-24. The live program was upgraded at
slot `478655638`; its upgrade-authority pubkey remains
`8gbaJEfM5VDs9BpFLgwMTq7s2FkVpEri8ZnPbxn4HPqY`. The reviewed ELF SHA-256 is
`42670041e7df0f9832930bfa511b884e8b59ceebc8e14b0638c4627a83e6aed3`.
See the
[promotion record](docs/audits/2026-07-24-devnet-promotion.md) for the
transaction, loader-padding proof, and six lifecycle signatures.

The project received USDG 1,500 from a USDG 3,000 grant. No written checklist
defines the remaining tranche. Progress is tracked through the evidence gates
in [ROADMAP.md](ROADMAP.md) and the public
[grant progress ledger](docs/grant-progress.md), without treating the reference
consumer as independent adoption.

## Instructions

| Instruction        | Authorization       | Effect                                                      |
| ------------------ | ------------------- | ----------------------------------------------------------- |
| `create_org`       | Authority signs     | Creates the authority's org namespace                       |
| `create_role`      | Org authority signs | Creates an enabled named role under the org                 |
| `assign_role`      | Role admin signs    | Creates a member PDA, optionally with expiry                |
| `revoke_role`      | Role admin signs    | Closes the member PDA and returns its rent to the admin     |
| `check_role`       | No signer required  | Returns success only for an enabled, unexpired membership   |
| `set_role_enabled` | Role admin signs    | Enables or disables checks and new assignments for the role |

The current program creates one org per authority. A role's admin is initialized
to that org authority and cannot yet be changed. Disabling a role preserves its
member accounts while causing `check_role` and new `assign_role` calls to fail.

## Account model

```text
Org PDA       ["org", authority]
  └─ Role PDA ["role", org, role_name]
       └─ Member PDA ["member", role, holder]
```

The PDA seeds bind the namespace, role, and holder. Anchor account ownership,
deserialization, seed, bump, and `has_one` constraints establish the account
relationships used by each instruction.

Role names are part of the role PDA seed and are therefore immutable. Membership
expiry uses a Unix timestamp; `0` means no expiry. Revocation closes the member
account, so a later grant creates it again.

## Integrating with `check_role`

`check_role` proves that the supplied `holder` pubkey has a valid member PDA for
the supplied role, that the role is enabled, and that the membership has not
expired. It does **not** prove that the transaction actor controls that holder.

A consuming program must authenticate the actor first—for example with a
`Signer<'info>` for a wallet or with its own validated PDA constraints—and pass
that authenticated account as `holder`:

```rust
hedwig_sol::cpi::check_role(CpiContext::new(
    ctx.accounts.hedwig_program.key(),
    hedwig_sol::cpi::accounts::CheckRole {
        member: ctx.accounts.member.to_account_info(),
        role: ctx.accounts.role.to_account_info(),
        holder: ctx.accounts.actor.to_account_info(),
    },
))?;
```

The CPI returns `Ok(())` on active membership and a Hedwig error otherwise. With
the `?` shown above, a failed check aborts the consuming instruction. Hedwig does
not return a boolean and does not grant transaction authority by itself.

The Rust CPI interface comes from the program crate. The compiling reference
consumer, its negative tests, and the TypeScript client flow are documented in
[the integration guide](docs/integration-guide.md).

The reference consumer is live on devnet. Its first verified state change used
an authenticated signer as the Hedwig holder and incremented a separate
consumer-owned counter from zero to one. This is deployment and integration
evidence, not a design-partner or customer claim.

## Run locally

Requirements: Rust, Anchor CLI 1.0.2, and Solana/Agave CLI 4.0.1 or newer.

```bash
cargo fmt --check
cargo build
cargo build-sbf --manifest-path programs/hedwig_sol/Cargo.toml
cargo build-sbf --manifest-path programs/hedwig_consumer/Cargo.toml
cargo test --workspace
yarn sdk:typecheck
yarn sdk:test
./node_modules/.bin/tsc -p app/tsconfig.json --noEmit
```

The tests run without a network connection. Build both SBF artifacts before the
workspace tests because the LiteSVM fixtures load them at compile time. To run
the membership lifecycle against devnet, see
[app/README.md](app/README.md).

## Repository guide

- [docs/architecture.md](docs/architecture.md): domain boundaries and code map
- [docs/integration-guide.md](docs/integration-guide.md): secure CPI and local SDK use
- [docs/grant-progress.md](docs/grant-progress.md): public delivery and evidence ledger
- [docs/operations.md](docs/operations.md): devnet upgrade, verification, and rollback
- [docs/audits/2026-07-24-full-audit.md](docs/audits/2026-07-24-full-audit.md): commit-pinned audit findings
- [docs/audits/2026-07-24-devnet-promotion.md](docs/audits/2026-07-24-devnet-promotion.md): live deployment and lifecycle proof
- [docs/audits/2026-07-24-consumer-devnet-integration.md](docs/audits/2026-07-24-consumer-devnet-integration.md): live CPI consumer proof
- [docs/adr/index.md](docs/adr/index.md): durable product and architecture decisions
- [THREAT-MODEL.md](THREAT-MODEL.md): assets, trust boundaries, and open risks
- [SECURITY.md](SECURITY.md): private vulnerability reporting
- [ROADMAP.md](ROADMAP.md): shipped evidence and remaining milestones
- [docs/sdk-rfc.md](docs/sdk-rfc.md): historical SDK design record
- [CONTRIBUTING.md](CONTRIBUTING.md): contributor workflow and verification gates

## License

MIT. See [LICENSE](LICENSE).
