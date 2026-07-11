# Hedwig

**Grant roles, not keys.**

An onchain roles primitive for Solana: define a role, assign it to a wallet, a program, or an agent, and revoke it in one transaction.

**Status: devnet-live.** All 6 core instructions, including the `set_role_enabled` circuit breaker, are deployed and CI-green: `cargo build-sbf` and `cargo test` both exit 0, 22 tests passing, 0 failed. A static security audit ran 2026-07-10 with zero critical (P0) findings and three high-severity (P1) items; two are closed (see [Security](#security)), one remains open. See [THREAT-MODEL.md](THREAT-MODEL.md) for the full findings.

---

## The problem

Solana has no composable, onchain primitive for "who is authorized to do what." That gap already has a body count.

**Multisig signer management is all-or-nothing.** When a contributor leaves a 3-of-5 multisig, the team runs a full proposal cycle to remove them. There is no "this person held the Treasurer role, revoke it everywhere" as a single atomic action. The ~$285M Drift Protocol exploit (April 2026) was social engineering of multisig signers, not a contract bug: the code held, the access model did not.

**Upgrade authority is a single hot wallet on most Solana programs.** Moving authority to a DAO governance program forces teams to publicly disclose security patches before deployment, creating an exploit window between announcement and fix. A time-limited, role-gated security council with automatic expiry is the obvious fix. Solana has no protocol primitive for it today.

**Autonomous agents ship with full wallet access and no onchain permission boundary.** Prompt-based restrictions are not authorization. In one reported incident, an autonomous agent sent $250K-$400K to a stranger because nothing onchain stopped it. An onchain role that scopes what an agent can authorize is a different kind of guardrail: enforced by the runtime, not the prompt.

Same primitive, three audiences: a DAO ops lead who needs single-action offboarding the moment a contributor leaves, a security-conscious protocol founder who needs a time-limited security council instead of a single hot wallet, and an AI-agent builder who needs a permission boundary enforced onchain, not just described in a prompt. Hedwig does not distinguish between them. A `holder` is a pubkey. Whether that pubkey belongs to a human wallet, a multisig, or an autonomous agent's keypair is invisible to the program, and that is the point: one scoped-authority primitive for all three.

---

## The solution

Hedwig defines named roles, assigns them to a wallet, a program, or an agent, and revokes them in a single transaction. No centralized registry, no dependency on any DAO tool, composable via CPI.

Program-derived accounts (PDAs) are the onchain proof of authority. An `Org` PDA is the top-level namespace, each `Role` PDA lives under an org, and each `Member` PDA proves one pubkey holds one role. The full seed and field layout is in [Architecture](#architecture) below.

Any Solana program can CPI into Hedwig to verify role membership in a single instruction. The call either succeeds (holder has the role, role is enabled, membership has not expired) or returns an error the calling program can inspect.

---

## What's built vs planned

| | Built (devnet) | Planned |
|---|---|---|
| Core instructions | `create_org`, `create_role`, `assign_role`, `revoke_role`, `check_role`, `set_role_enabled` | None. The instruction set is stable at six. |
| Testing | 22 tests: negative-authorization invariants (non-admin, wrong holder, wrong role, expired, revoked, duplicate assignment), full lifecycle, create_org edge cases, circuit-breaker coverage | Continued expansion as new surface area lands |
| SDK | RFC only (`docs/sdk-rfc.md`) | TypeScript SDK, published to npm as `@hedwig-sol/sdk` |
| Upgrade authority | Single deployer keypair | 2-of-3 Squads multisig |
| Reference integration | Devnet e2e demo script (`app/demo.ts`) | Documented secure CPI consumer pattern showing how to authenticate a `holder` before trusting `check_role` |
| Network | Devnet | Mainnet, after the above closes |

We would rather tell you what is not done than let you find out later. See [Roadmap](#roadmap) below for the full milestone breakdown.

---

## Design decisions

Hedwig deliberately keeps its core surface small, so the program can eventually be frozen and trusted as an immutable primitive. Three places where that discipline is a decision, not an oversight:

**Flat roles, not a hierarchy.** An org has roles, roles have members, and that is the entire tree: no nested sub-roles, no role-spawns-role state machine. A program small enough to eventually freeze needs to stay small enough to audit. Role trees and scoped sub-role composition belong in wrapper programs built on top of Hedwig, not inside the core.

**No agent-to-agent delegation in the core.** `assign_role` and `revoke_role` are admin-only (see [Instructions](#instructions) below). A holder, including an agent holder, cannot grant a scoped sub-role to another agent; only the role admin can assign or revoke. Delegation chains are exactly the kind of complexity a flat-forever core is designed to exclude. A wrapper program that lets an admin grant a bounded, scoped delegation capability to a holder is a reasonable pattern to build; it is out of scope here.

**No pluggable eligibility modules.** Hedwig's core does not decide who should be allowed to hold a role: stake-gated, NFT-gated, vote-gated, or otherwise. That logic lives in the calling program, checked via CPI before it assigns or trusts a role. Same posture as the two points above: the primitive stays small, the integrator owns the gating logic.

These are the tradeoffs for a program built to be frozen and trusted, not gaps on the way to a bigger feature set.

---

## Instructions

| Instruction | Who signs | What it does |
|---|---|---|
| `create_org` | authority | Creates the org PDA |
| `create_role` | org authority | Creates a named role under the org |
| `assign_role` | role admin | Grants a role to any pubkey; optional expiry, rejects a past or negative `expires_at` |
| `revoke_role` | role admin | Closes the member PDA, returns rent |
| `check_role` | anyone (CPI-friendly) | Verifies membership; errors if the check fails |
| `set_role_enabled` | role admin | Toggles a role's `enabled` flag, the circuit breaker. Disabling a role blocks `check_role` and `assign_role` for every holder without revoking individual memberships |

All instructions except `check_role` emit an event for indexers and off-chain tooling. `check_role` does not mutate state, so it emits nothing; it is a pure read used inside a CPI.

`check_role` proves membership. It does not by itself prove that the caller controls the `holder` pubkey being checked; the consuming program must independently authenticate whoever it treats as the holder (a `Signer`, a validated PDA, or an established identity from its own constraints). See [THREAT-MODEL.md](THREAT-MODEL.md) for the full authentication-composition note.

---

## Quickstart

**Requirements:** Rust, anchor-cli 1.0.2, solana-cli (Agave) 4.0.1+

```bash
# Build (SBF artifact required before tests)
anchor build

# Run tests (LiteSVM, no network required)
cargo build-sbf --manifest-path programs/hedwig_sol/Cargo.toml
cargo test

# Deploy to devnet
solana config set --url devnet
anchor deploy --provider.cluster devnet
```

Devnet program: `H4J9wWhraK2Zvn4o9aFheFVmAf7nfaBNPw3d7w77X1eC`

CPI example:

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

## Security

We ran a static audit before seeking external integrations, not after. Result: zero P0 (critical) findings, no path to falsify an `Org`, `Role`, or `Member` account, no bypass of admin/authority constraints. Three P1 (high) findings came out of that audit. Two are closed:

1. **Closed.** The circuit-breaker gap. `set_role_enabled` did not exist when the audit ran; it exists now, is admin-gated, emits an event, and is covered by tests.
2. **Closed.** Authorization invariant coverage. The test suite grew from one happy-path test to 22 tests covering non-admin actions, wrong holder, wrong role, expired and revoked memberships, duplicate assignment, and the circuit breaker itself.
3. **Open.** `check_role` proves membership, not caller identity. Integrators must authenticate the `holder` themselves. A reference consumer demonstrating the correct pattern (a `Signer` or validated-PDA check before trusting `check_role`) is still planned, not shipped.

Full findings and remediation status: [THREAT-MODEL.md](THREAT-MODEL.md) and the Roadmap section below.

---

## Architecture

For the repo-level architecture map, folder conventions, and ADR pointers, see [docs/architecture.md](docs/architecture.md) and [docs/adr/index.md](docs/adr/index.md).

### PDA tree

```
Authority wallet
    |
    +-- Org PDA  [seeds: "org", authority]  {fields: authority, name, role_count}
            |
            +-- Role PDA  [seeds: "role", org, "treasurer"]  {fields: org, name, admin, member_count, enabled}
            |       |
            |       +-- Member PDA  [seeds: "member", role, holder_A]  {fields: role, holder, granted_at, expires_at}
            |       +-- Member PDA  [seeds: "member", role, holder_B]
            |
            +-- Role PDA  [seeds: "role", org, "security-council"]
                    |
                    +-- Member PDA  [seeds: "member", role, multisig_key]
```

### Role checks

Role state lives entirely onchain. No indexer, no off-chain cache. A CPI check reads two accounts (the Member PDA and the Role PDA) at the cost of any other account read on Solana.

### Privacy

The `holder` field is a pubkey. Off-chain labels that map pubkeys to human identities are optional and stored off-chain. Public badges (e.g. Token-2022 non-transferable tokens) are a future roadmap item, not required.

### One org per authority

Each authority wallet can currently create exactly one org (the org PDA derives from `["org", authority]` alone). This is a namespace decision, not a bug; multi-org support would require a seed change and a migration plan, and is an open decision, not a silent default. See the Roadmap section below.

---

## Upgrade authority

The program upgrade authority is currently held by the deployer key. This is the single highest risk in the current deployment, and we say so plainly rather than bury it.

- Next: transfer upgrade authority to a 2-of-3 Squads multisig
- After that: document the path to freezing the program (removing upgrade authority entirely) with community notice

See [THREAT-MODEL.md](THREAT-MODEL.md) for the full analysis.

---

## Roadmap

Milestone-based roadmap with falsifiable done-when conditions: [ROADMAP.md](ROADMAP.md).

---

## Development

See [Quickstart](#quickstart) above. Standing context for contributors and AI agents: [AGENTS.md](AGENTS.md).

---

## License

MIT. See [LICENSE](LICENSE).

---
