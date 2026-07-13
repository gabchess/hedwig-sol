# Hedwig

**Grant roles, not keys.**

Composable onchain roles for Solana. Assign a named role to any pubkey, set
optional membership expiry, disable the role in an incident, and let any Solana
program verify active membership via CPI.

Hedwig is a devnet-stage Anchor program. The repository implements six
instructions and covers them with 21 LiteSVM integration tests. The recorded
devnet lifecycle exercises `create_org`, `create_role`, `assign_role`,
`check_role`, and `revoke_role`. `set_role_enabled` is implemented and tested
locally, but is not in the current devnet deployment.

Devnet program: `H4J9wWhraK2Zvn4o9aFheFVmAf7nfaBNPw3d7w77X1eC`

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

| Surface | Current state |
|---|---|
| Anchor program | Six instructions implemented |
| Tests | 21 LiteSVM integration tests |
| Devnet evidence | Five-instruction membership lifecycle recorded |
| Circuit breaker | Implemented and tested locally; devnet redeploy required |
| TypeScript SDK | RFC only; not built or published |
| Secure CPI consumer | Planned reference integration |
| Upgrade authority | Single deployer key; 2-of-3 Squads transfer planned before mainnet |
| Network | Devnet; mainnet is planned |

See [ROADMAP.md](ROADMAP.md) for evidence-gated delivery milestones and
[THREAT-MODEL.md](THREAT-MODEL.md) for the current trust boundaries.

Deployment evidence was checked on 2026-07-13 with `solana program show`: the
live program was last deployed at slot `468922773` on 2026-06-12, before
`set_role_enabled` was added. Its upgrade-authority pubkey is
`8gbaJEfM5VDs9BpFLgwMTq7s2FkVpEri8ZnPbxn4HPqY`.

## Instructions

| Instruction | Authorization | Effect |
|---|---|---|
| `create_org` | Authority signs | Creates the authority's org namespace |
| `create_role` | Org authority signs | Creates an enabled named role under the org |
| `assign_role` | Role admin signs | Creates a member PDA, optionally with expiry |
| `revoke_role` | Role admin signs | Closes the member PDA and returns its rent to the admin |
| `check_role` | No signer required | Returns success only for an enabled, unexpired membership |
| `set_role_enabled` | Role admin signs | Enables or disables checks and new assignments for the role |

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
    ctx.accounts.hedwig_program.to_account_info(),
    hedwig_sol::cpi::accounts::CheckRole {
        member: ctx.accounts.member.to_account_info(),
        role: ctx.accounts.role.to_account_info(),
        holder: ctx.accounts.actor.to_account_info(), // actor is authenticated here
    },
))?;
```

The CPI returns `Ok(())` on active membership and a Hedwig error otherwise. With
the `?` shown above, a failed check aborts the consuming instruction. Hedwig does
not return a boolean and does not grant transaction authority by itself.

The Rust CPI interface currently comes from the program crate. The planned
TypeScript SDK and reference consumer are tracked in [ROADMAP.md](ROADMAP.md).

## Run locally

Requirements: Rust, Anchor CLI 1.0.2, and Solana/Agave CLI 4.0.1 or newer.

```bash
cargo fmt --check
cargo build
cargo build-sbf --manifest-path programs/hedwig_sol/Cargo.toml
cargo test
```

The tests run with LiteSVM and do not require a network connection. To run the
recorded membership lifecycle against devnet, see [app/README.md](app/README.md).

## Repository guide

- [docs/architecture.md](docs/architecture.md): domain boundaries and code map
- [docs/adr/index.md](docs/adr/index.md): durable product and architecture decisions
- [THREAT-MODEL.md](THREAT-MODEL.md): assets, trust boundaries, and open risks
- [ROADMAP.md](ROADMAP.md): shipped evidence and remaining milestones
- [docs/sdk-rfc.md](docs/sdk-rfc.md): historical SDK RFC; the SDK is not shipped
- [CONTRIBUTING.md](CONTRIBUTING.md): contributor workflow and verification gates

## License

MIT. See [LICENSE](LICENSE).
