# Hedwig

**Grant roles, not keys.**

Hedwig is an onchain roles primitive for Solana. Any team or protocol can define named roles, assign them to wallets or programs, and revoke them in a single transaction. No centralized registry, no dependency on any DAO tool.

---

## The problem

Solana teams have no composable, onchain primitive for defining who is authorized to do what.

Three situations make this acute:

**Multisig signer management is all-or-nothing.** When a contributor leaves a 3-of-5 multisig, the team must run a full proposal cycle to remove them. There is no concept of "this person held the Treasurer role, revoke it everywhere" as a single atomic action. The result is either access that outlives the relationship or an operational bottleneck at every team change.

**Upgrade authority is binary and hot.** Most Solana programs run with a single hot wallet as upgrade authority. Moving authority to a DAO governance program forces teams to publicly disclose security patches before deployment, creating an exploit window between announcement and fix. A time-limited, role-gated security council with automatic expiry is the obvious fix. Solana has no protocol primitive for it today.

**Autonomous agents ship with full wallet access.** When an autonomous agent misbehaves, the root cause is often not a code bug: the agent simply had no onchain permission boundary. Prompt-based restrictions are not authorization. An onchain role that scopes what an agent can authorize is.

---

## How it works

Hedwig uses program-derived accounts (PDAs) as the onchain proof of authority.

```
Org PDA
  seeds: ["org", authority]
  fields: authority, name, role_count

Role PDA
  seeds: ["role", org_key, role_name]
  fields: org, name, admin, member_count, enabled

Member PDA
  seeds: ["member", role_key, holder]
  fields: role, holder, granted_at, expires_at
```

Any Solana program can CPI into Hedwig to verify role membership in a single instruction. The call either succeeds (holder has the role, role is enabled, membership has not expired) or returns an error the calling program can inspect.

---

## Instructions

| Instruction | Who signs | What it does |
|---|---|---|
| `create_org` | authority | Creates the org PDA |
| `create_role` | org authority | Creates a named role under the org |
| `assign_role` | role admin | Grants a role to any pubkey; optional expiry |
| `revoke_role` | role admin | Closes the member PDA, returns rent |
| `check_role` | anyone (CPI-friendly) | Verifies membership; errors if check fails |

All instructions emit events for indexers and off-chain tooling.

---

## CPI usage

Any Solana program can gate an instruction on role membership:

```rust
// Verify the caller holds the "admin" role before proceeding.
hedwig_sol::cpi::check_role(
    CpiContext::new(
        ctx.accounts.hedwig_program.to_account_info(),
        hedwig_sol::cpi::accounts::CheckRole {
            member: ctx.accounts.member.to_account_info(),
            role: ctx.accounts.role.to_account_info(),
            holder: ctx.accounts.caller.to_account_info(),
        },
    ),
)?;
```

If `check_role` returns `Ok(())`, the caller holds the role, the role is enabled, and membership has not expired. If it returns an error, the calling instruction reverts.

---

## Architecture

### PDA tree

```
Authority wallet
    |
    +-- Org PDA  [seeds: "org", authority]
            |
            +-- Role PDA  [seeds: "role", org, "treasurer"]
            |       |
            |       +-- Member PDA  [seeds: "member", role, holder_A]
            |       +-- Member PDA  [seeds: "member", role, holder_B]
            |
            +-- Role PDA  [seeds: "role", org, "security-council"]
                    |
                    +-- Member PDA  [seeds: "member", role, multisig_key]
```

### Role checks

Role state lives entirely onchain. No indexer, no off-chain cache. A CPI check reads two accounts (the Member PDA and the Role PDA) and costs roughly the same as any other account read on Solana.

### Privacy

Role accounts support opaque identifiers: the `holder` field is a pubkey. Off-chain labels that map pubkeys to human identities are optional and stored off-chain. Public badges (e.g. Token-2022 non-transferable tokens) are on the roadmap for M1 but are not part of M0 and are never required.

---

## Upgrade authority

The program upgrade authority is currently held by the deployer key. The plan before mainnet:

- M1: transfer upgrade authority to a 2-of-3 Squads multisig
- M2: document the path to freezing the program (removing upgrade authority entirely) with 90-day community notice

See [THREAT-MODEL.md](THREAT-MODEL.md) for the full upgrade authority risk analysis.

---

## Roadmap

| Milestone | Status | Description |
|---|---|---|
| M0 | In progress | Public repo, devnet deploy, 5 core instructions |
| M1 | Planned | TypeScript SDK on npm, assign/check demo script, devnet design partners |
| M2 | Planned | Mainnet deploy, Squads CPI integration example, hosted docs, THREAT-MODEL invariant tests |

Token-2022 non-transferable badge representation, cross-program role propagation hooks, and a full DAO creation UI are out of scope for M0-M2 and documented as future roadmap items.

---

## Development

**Requirements:** Rust, anchor-cli 1.0.2, solana-cli (Agave) 4.0.1+

```bash
# Build
anchor build

# Run tests (LiteSVM, no network required)
cargo test

# Deploy to devnet
solana config set --url devnet
anchor deploy --provider.cluster devnet
```

---

## License

MIT. See [LICENSE](LICENSE).

---

## npm scope

The TypeScript SDK will be published under the `@hedwig-sol` npm scope at M1.
