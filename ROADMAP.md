---
title: "Hedwig Roadmap"
date: 2026-07-10
status: "M0-M1 shipped, M2 through M5 planned"
---

# Hedwig Roadmap

Hedwig is an onchain roles primitive for Solana: define named roles, assign them to a person, a team, or an autonomous agent, and revoke them in a single transaction. This roadmap tracks what is built and what is planned.

## Shipped

### M0: Core primitive (devnet)

Five instructions live on devnet (`create_org`, `create_role`, `assign_role`, `revoke_role`, `check_role`). CI green (`cargo fmt`, `cargo build`, `cargo build-sbf`, `cargo test`). Threat model published. Devnet end-to-end demo script.

### M1: Security hardening

- `set_role_enabled` (sixth instruction): an admin-gated circuit breaker that toggles a role's `enabled` flag and emits `RoleEnabledSet`, so an authority can disable a role in an incident without revoking members individually.
- `assign_role` rejects a past or negative `expires_at` (`expires_at == 0` for never-expires, otherwise strictly in the future).
- Checked arithmetic on every counter (overflow and underflow return a domain error).
- Test suite expanded from one happy-path test to 22: the full negative-authorization invariant set (non-admin assign/revoke, non-authority `create_role`, wrong holder, wrong role, expired membership, revoked membership, duplicate assignment), full lifecycle coverage, `create_org` edge cases, and the circuit-breaker path.
- Verified: `cargo build-sbf` and `cargo test` both exit 0.

**Open, carried to M3:** a documented reference CPI consumer that authenticates the `holder` (via a `Signer` or a validated PDA) before trusting `check_role`. The test suite proves the PDA-derivation invariant holds; the integrator-facing consumer pattern is not shipped yet.

## Planned

### M2: Design decisions and hygiene

Record the remaining design decisions explicitly in the docs: one Org per authority (namespace design), and admin/authority immutability (rotation is out of scope for the current program identity). No code change; a seed change would break the deployed program's identity.

These decisions are already stated in the README; M2 records them in `THREAT-MODEL.md` as well.

**Done when:** the org-cardinality and admin-rotation decisions are stated in `THREAT-MODEL.md`, and the docs match the code.

### M3: SDK alpha and secure CPI reference

Publish the TypeScript SDK (`@hedwig-sol/sdk`) to npm, rewrite the demo to call the SDK instead of raw Anchor, and ship the holder-authentication consumer pattern as a documented example.

**Done when:** `npm view @hedwig-sol/sdk version` returns a published version, the demo uses the SDK, and the secure CPI consumer example is in the repo with a passing test.

### M4: Design partner and multisig upgrade authority

Transfer upgrade authority from the single deployer key to a 2-of-3 Squads multisig, and onboard the first design-partner integration.

**Done when:** `solana program show <PROGRAM_ID> --url devnet` shows the multisig as upgrade authority, and one design-partner integration is live.

### M5: Mainnet

Deploy to mainnet, publish hosted docs, and merge a Squads CPI integration example with a passing test.

**Done when:** the mainnet program account is live, the docs URL resolves, and the CPI example merges green.

## Design decisions

Hedwig deliberately keeps a small surface, so the program can eventually freeze as an immutable primitive:

- **Flat roles, not a hierarchy.** An org has roles, roles have members. Role trees and scoped sub-role composition belong in wrapper programs built on top of Hedwig, not in the core.
- **No agent-to-agent delegation in the core.** `assign_role` and `revoke_role` are admin-only. A wrapper program can add scoped delegation; the core does not.
- **No pluggable eligibility modules.** Eligibility logic (whether a pubkey may hold a role) lives in the calling program, checked via CPI, consistent with the flat-forever design.
