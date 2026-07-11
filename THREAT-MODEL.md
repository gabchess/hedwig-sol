# Threat Model

This document describes the trust assumptions, known risks, and mitigations for the Hedwig onchain roles program.

---

## Trust assumptions

| Component | Who controls it | Risk if compromised |
|---|---|---|
| Org authority | The deploying wallet | Can create unlimited roles under the org |
| Role admin | Set by org authority; defaults to same wallet | Can assign or revoke role memberships, and toggle the role's `enabled` flag |
| Program upgrade authority | Deployer key (M0); 2-of-3 multisig (M1 target) | Can replace program logic entirely |

The program enforces all access control via Anchor constraints and PDA derivation. No off-chain component is involved in role checks.

---

## Upgrade authority risk

**Current state (M0):** The program upgrade authority is a single deployer keypair. This is the highest risk in the current deployment.

**Mitigation plan:**
- M1: upgrade authority transfers to a 2-of-3 Squads multisig. Any program upgrade requires 2 of 3 keyholders to sign.
- M2: the program is either frozen (upgrade authority removed) or moved to a governance-controlled multisig with a 90-day community notice requirement before any upgrade.

**Why not freeze at M0:** The program is in active development. Freezing before the invariant test suite is complete would lock in any bugs found during M1 testing.

---

## Account-spoofing considerations

**PDA derivation:** All accounts are PDAs. An attacker cannot fake a Member PDA without controlling the seeds (role key + holder pubkey). Anchor's account validation verifies the derivation on every instruction.

**Role enabling flag (circuit breaker):** The `enabled` field on Role accounts lets the role admin disable an entire role for all holders simultaneously. The `set_role_enabled` instruction toggles this field, is gated to the role admin (`has_one = admin`), and emits a `RoleEnabledSet` event. If a role is being abused, disabling it halts check_role and blocks new assign_role calls for that role without requiring the admin to revoke each Member PDA individually. This is implemented and covered by tests in `test_set_role_enabled.rs`.

**Expired memberships:** The `expires_at` field is checked in check_role against the onchain clock. Expired memberships return `MembershipExpired`. Callers cannot bypass this check because check_role is the authoritative source.

---

## Account confusion attacks

Anchor uses discriminators on all account types. A `Role` account cannot be passed as an `Org` account without Anchor rejecting the discriminator mismatch. This prevents account confusion attacks across the instruction set.

---

## Denial of service

There is no global state. Each org, role, and member account is independent. An attacker cannot block access to one org's roles by attacking another org's accounts.

---

## Invariant test coverage

The test suite covers the following invariants:

- A holder with a revoked Member PDA fails check_role
- A holder with an expired membership fails check_role
- A disabled role fails check_role for all holders
- A non-admin cannot assign or revoke role memberships
- A non-admin cannot toggle a role's enabled flag
- A non-authority cannot create roles under an org
- Closing a Member PDA decrements the role member_count by exactly 1
- Re-assigning the same role to the same holder fails (init constraint prevents duplicate PDAs)

22 tests pass across this suite as of the 2026-07-10 hardening pass. check_role proves PDA membership; it does not by itself prove the transaction caller controls the `holder` pubkey. Callers that need caller-identity guarantees must add their own `Signer` or validated-PDA check before trusting check_role. A documented reference consumer for that pattern is planned, not yet shipped.

---

## Out of scope for M0

- Spending caps on agent roles (M2+)
- Time-locked security council with auto-expiry (M2+)
- Cross-program role propagation hooks
- Token-2022 non-transferable badge representation
- Formal verification

---

## Reporting vulnerabilities

Until a formal bug bounty program is established, report vulnerabilities directly to the maintainer. Do not disclose publicly before a fix is available.
