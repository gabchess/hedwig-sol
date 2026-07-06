# RFC: Hedwig SDK surfaces

Status: Draft. Date: 2026-07-06. Discussion via GitHub issues.

---

## Problem

An integrating program should be able to gate an instruction on "does this pubkey hold role X in org Y" without reading Hedwig's Rust source. Today Hedwig ships 5 instructions and a devnet lifecycle demo (`app/demo.ts`), proving the primitive works end to end. Missing: an importable surface. No crate, no package, nothing a third-party program can pull in without copying PDA derivation and account layouts by hand.

---

## Decisions already made

- **Roles stay flat in the core program, forever.** Hierarchy composes on top via wrapper programs, since `Role.admin` can be any pubkey, including a PDA owned by another program. The core program never grows a tree-walking authorization model.
- **Trust path is freeze-track, not multisig.** Mainnet v1 deploys intending to burn the upgrade authority after the invariant test suite and an external review pass. A small, finished program is easier to freeze and audit than one kept "open" indefinitely.
- **First consumer is a dogfood demo, not a partner integration.** An autonomous trading agent, built in a separate repo (`hedwig-agent-demo`), holds a time-boxed "trader" role and a "settler"-gated settlement path. Building the first real consumer in-house surfaces SDK gaps before any external team hits them.
- **Token-2022 badge issuance is deferred.** It is a presentation layer on top of role state, not part of the roles primitive, and does not block anything here.

---

## Proposed surfaces

### hedwig-cpi (Rust crate)

The primary surface, since Hedwig's consumers are programs, not end users. Exposes two helpers:

- A `check_role` CPI helper for Anchor programs, wrapping the existing `check_role` instruction.
- A direct read-and-verify helper that deserializes the Member and Role PDAs in-process and runs the same checks (expiry, `enabled` flag, PDA derivation) without a CPI call.

The open cost question: CPI carries fixed compute and call-depth overhead a direct account read does not. Some consumers may prefer the cleaner interface; others may be compute-constrained. This RFC does not pick a default. The `hedwig-agent-demo` build measures both paths and the crate README documents the numbers once known.

### @hedwig-sol/sdk (TypeScript client)

Secondary, built after the crate. PDA derivation helpers and instruction builders for the create/assign/revoke flows, the operations `app/demo.ts` already performs by hand. A thin wrapper over the Anchor IDL, nothing more: no indexer, no caching layer, no off-chain state.

### Worked example (hedwig-agent-demo)

A separate repo with an autonomous trading agent whose signing key holds a "trader" role with `expires_at` set (a session-key pattern: the key stops working on schedule, no revoke transaction needed) and whose settlement path checks a "settler" role. The org authority can flip `Role.enabled` to false as a kill switch, halting every check for that role without touching individual memberships. This demo validates both patterns and produces real CPI-vs-direct-read compute numbers instead of estimates.

---

## Explicit non-goals

- Role hierarchy or inheritance inside the core program.
- An indexer service for role state or history.
- Token-2022 badge issuance (future, separate layer, does not touch core state).
- Multisig upgrade governance (superseded by the freeze-track plan above).
- Any off-chain permission cache. Role state is read onchain, always.

---

## Non-obvious decisions

- Flat-forever makes the freeze-track plan credible: a program that never grows new authorization logic is a program that is actually finished, and a finished program is one worth freezing.
- The CPI crate outranks the TypeScript SDK in priority because Hedwig's target integrators are other Solana programs calling in via CPI, not end users clicking a UI. The TS client matters for tooling, but it is not the adoption bottleneck.
- The CPI helper and the direct-read helper ship in one crate, not two, because they answer the same question (does this pubkey hold this role) and choosing between them is a compute tradeoff, not a separate integration decision.

---

## Open questions

- CPI call overhead versus direct PDA deserialization: exact compute unit delta, answered by the `hedwig-agent-demo` build.
- Rent-return recipient on `revoke_role`: confirm current behavior is correct, don't assume it, audit during the demo build.
- Whether any client needs event or indexer support before the v1 freeze, or onchain-only reads stay sufficient through mainnet.

---

## Milestones

Sequenced, not dated. Cadence is weekly ships.

| Step | Deliverable |
|---|---|
| 1 | `hedwig-agent-demo` repo: trading agent with time-boxed trader role, settler-gated settlement |
| 2 | Friction report from building the demo against raw CPI calls |
| 3 | `hedwig-cpi` v0: CPI helper + direct-read helper, informed by the friction report |
| 4 | `@hedwig-sol/sdk` v0: PDA derivation + instruction builders |
| 5 | Invariant test suite (per THREAT-MODEL.md M1 commitment) |
| 6 | Mainnet deploy, then freeze |
