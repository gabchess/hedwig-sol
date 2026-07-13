# Hedwig Architecture

Hedwig is an onchain authorization primitive organized around three domain
nouns—orgs, roles, and members—and six domain actions. The repository keeps the
conventional Anchor shell while making the authorization model visible in file
and symbol names.

## Domain boundary

Hedwig answers one question: does this pubkey currently hold this role?

| Domain term | Onchain meaning |
|---|---|
| `Org` | Namespace owned by one authority pubkey |
| `Role` | Named authority under an org, controlled by its role admin |
| `Member` | Account proving one holder pubkey belongs to one role |
| Holder | Pubkey named by a member account; not necessarily a signer |
| Circuit breaker | Role-wide enabled flag managed through `set_role_enabled` |
| Role check | CPI-friendly validation through `check_role` |

Hedwig does not define what a role authorizes inside another program. It does not
authenticate the actor presented by that program, create delegation chains, or
evaluate token- or vote-based eligibility. Those policies belong to the consumer
or to wrapper programs.

## Account graph

```text
authority
  └─ Org ["org", authority]
       └─ Role ["role", org, role_name]
            └─ Member ["member", role, holder]
```

- One authority currently derives one org.
- A role name is part of its PDA seed and cannot be renamed.
- A role admin is initialized to the org authority. The current core has no
  instruction to transfer that admin field.
- A member may expire at a Unix timestamp; `0` means no expiry.
- Revocation closes the member PDA and decrements the role's member count.
- Disabling a role preserves members but blocks checks and new assignments.

Program ownership, Anchor account deserialization, PDA seed and bump checks, and
explicit `has_one` constraints enforce relationships between these accounts.
Discriminators identify account types; they do not establish account identity on
their own.

## Instruction flow

```text
create_org
    └─ create_role
         ├─ assign_role ── check_role
         ├─ revoke_role
         └─ set_role_enabled
```

The mutating instructions authenticate the relevant authority or role admin.
`check_role` is intentionally read-only and requires no holder signature. Its
account constraints bind `Member.role` to the supplied role and `Member.holder`
to the supplied holder; its handler then rejects disabled roles and elapsed
memberships.

For CPI use, the consuming program must first authenticate the actor it maps to
`holder`. A wallet consumer can require a signer. A program identity can use its
own PDA validation. The consumer then invokes `check_role` and propagates its
`Result`; propagating an error aborts the consuming instruction.

## Repository map

| Path | Responsibility |
|---|---|
| `programs/hedwig_sol/src/lib.rs` | Public six-instruction Anchor surface |
| `programs/hedwig_sol/src/instructions/` | One file per authorization action |
| `programs/hedwig_sol/src/state.rs` | `Org`, `Role`, and `Member` account state |
| `programs/hedwig_sol/src/error.rs` | Authorization and validation errors |
| `programs/hedwig_sol/src/constants.rs` | PDA seed and account-size constants |
| `programs/hedwig_sol/tests/` | LiteSVM behavior and lifecycle tests |
| `app/` | Devnet membership-lifecycle client |
| `docs/adr/` | Durable product and architecture decisions |
| `THREAT-MODEL.md` | Trust boundaries and security posture |
| `ROADMAP.md` | Evidence-gated delivery sequence |

This is screaming architecture at the scale the program needs: files and types
name the authorization domain, while Anchor's expected root and crate layout stay
conventional. Extra `domain`, `service`, `manager`, or `utils` layers would add
navigation without clarifying the current six-action core.

## Change rules

- Name new state with domain nouns and new instructions with domain verbs.
- Keep one instruction per file while the surface remains small.
- Put shared code in a generic module only after repeated use proves the need.
- Keep framework and deployment concerns at the repository shell.
- Record durable scope or architecture changes in [the ADR index](adr/index.md).
- Keep roadmap status out of architecture documents; architecture describes the
  current system.

The governing decisions are recorded in [docs/adr/](adr/index.md). Integration
risks, especially holder authentication and upgrade authority, belong in
[THREAT-MODEL.md](../THREAT-MODEL.md), not in this code map.
