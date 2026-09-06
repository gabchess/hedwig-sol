# Hedwig

**Agent access you can revoke and verify.**

Hedwig gives Solana apps a shared, revocable role record for software agents.

Give an agent a role with an expiry. Each integrated Solana program authenticates
that agent and checks its current membership before a protected action. Revoke
membership or disable the role to deny later actions that use that check.

Hedwig is a devnet-stage Anchor program with six instructions, a separate
reference consumer, and a repository-local TypeScript SDK alpha. The consumer
shows an authenticated actor changing a protected counter through CPI.

## Why Hedwig

An agent can keep trying to work after a task ends or an operator intervenes.
For an integrated Solana action, the useful question is whether the program
will still accept that agent's authority when the transaction executes.

Hedwig keeps `Org`, `Role`, and `Member` state in one onchain store. Related
programs can check the same membership during their own transaction. The
consumer defines the actions a role permits and must authenticate the actor.

Revocation takes effect through the updated chain state. It does not undo an
executed transaction, terminate a process, remove unrelated credentials, or
stop actions in programs that do not enforce the check. Transactions ordered
before revocation can still succeed. Expiry and revocation alone are not a
unique capability compared with wallet policy products.

## Try one protected action

**Bring one agent and one protected Solana action. Prove the agent can act while
its role is valid, and cannot after expiry or revocation.**

The [devnet proof offer](docs/agent-access/proof-offer.md) describes a bounded
integration exercise. The [demo behavior](docs/agent-access/demo.md) specifies
what a grant, successful task, and denied retry must prove. The proposed Replit
experience is not implemented in this repository yet.

## Current status

| Surface             | Current state                                                                       |
| ------------------- | ----------------------------------------------------------------------------------- |
| Anchor program      | Six instructions implemented                                                        |
| Rust tests          | 40 LiteSVM integration tests across the core and consumer                           |
| TypeScript tests    | 27 SDK tests plus SDK and app typechecks                                            |
| Devnet evidence     | Upgrade verified at slot `478655638`; six-instruction lifecycle finalized afterward |
| Circuit breaker     | Live `enabled=false` state verified on devnet                                       |
| TypeScript SDK      | Repository-local `0.1.0-alpha.0`; built and tested, not published                   |
| Secure CPI consumer | Deployed at slot `478667066`; live Hedwig-gated state change verified               |
| Upgrade authority   | Single deployer key; 2-of-3 Squads transfer planned before mainnet                  |
| Validation          | Agent access proof proposed; no signed pilot or external adoption                   |
| Network             | Devnet; mainnet is planned                                                          |

See [ROADMAP.md](ROADMAP.md) for evidence-gated delivery milestones and
[THREAT-MODEL.md](THREAT-MODEL.md) for the current trust boundaries.

Binary and lifecycle evidence was checked on 2026-07-24. A
[read-only presence check on 2026-09-06](docs/deployment/evidence/2026-09-06-program-presence.md)
confirmed both program addresses and their recorded deployment slots. The live program was upgraded at
slot `478655638`; its upgrade-authority pubkey remains
`8gbaJEfM5VDs9BpFLgwMTq7s2FkVpEri8ZnPbxn4HPqY`. The reviewed ELF SHA-256 is
`42670041e7df0f9832930bfa511b884e8b59ceebc8e14b0638c4627a83e6aed3`.
See the
[promotion record](docs/deployment/evidence/2026-07-24-devnet-promotion.md) for the
transaction, loader-padding proof, and six lifecycle signatures.

Funding history remains in the [grant progress ledger](docs/grants/progress.md).
No signed pilot, design partner, customer, revenue, production use, or external
adoption has been verified. The single builder-owned consumer proves a
technical integration pattern, not demand from Solana agent teams.

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

A consuming program must authenticate the actor first, for example with a
`Signer<'info>` for a wallet or with its own validated PDA constraints, and pass
that authenticated account as `holder`. It must also bind the supplied role to
its configured required role before the CPI. Otherwise an actor could supply
an unrelated role it controls. The reference consumer uses
`has_one = required_role` on its counter account for this binding:

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
[the integration guide](docs/access-control/integration-guide.md).

The reference consumer is live on devnet. Its first verified state change used
an authenticated signer as the Hedwig holder and incremented a separate
consumer-owned counter from zero to one. This is deployment and integration
evidence, not a design-partner or customer claim.

## Run locally

Requirements: Rust, Anchor CLI 1.0.2, and Solana/Agave CLI 4.0.1 or newer.

```bash
cargo fmt --check
cargo build
cargo build-sbf --manifest-path programs/hedwig_consumer/Cargo.toml
cargo build-sbf --manifest-path programs/hedwig_sol/Cargo.toml
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

- [Agent access](docs/agent-access/README.md): product scope, proof offer, and demo
- [Access control](docs/access-control/architecture.md): role model and code map
- [Integration](docs/access-control/integration-guide.md): actor binding, CPI, and SDK
- [Security](THREAT-MODEL.md): boundaries, risks, and [recorded review](docs/security/reviews/2026-07-24-full-audit.md)
- [Deployment](docs/deployment/operations.md): operations and [devnet evidence](docs/deployment/evidence/2026-07-24-consumer-devnet-integration.md)
- [Grants](docs/grants/progress.md): funding and delivery evidence
- [History](docs/history/README.md): superseded pilot offer and dated records
- [Roadmap](ROADMAP.md): shipped evidence and next product gates
- [Contributing](CONTRIBUTING.md): verification and contribution workflow

## License

MIT. See [LICENSE](LICENSE).
