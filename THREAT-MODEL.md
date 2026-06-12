# Threat Model

This document describes the trust assumptions, known risks, and mitigations for the Hedwig onchain roles program.

---

## Trust assumptions

| Component | Who controls it | Risk if compromised |
|---|---|---|
| Org authority | The deploying wallet | Can create unlimited roles under the org |
| Role admin | Set by org authority; defaults to same wallet | Can assign or revoke role memberships |
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

**Role enabling flag:** The `enabled` field on Role accounts lets an org authority disable an entire role category for all holders simultaneously. This is a circuit-breaker for incidents: if a role is being abused, disabling it halts all check_role calls for that role without requiring individual revocations.

**Expired memberships:** The `expires_at` field is checked in check_role against the onchain clock. Expired memberships return `MembershipExpired`. Callers cannot bypass this check because check_role is the authoritative source.

---

## Account confusion attacks

Anchor uses discriminators on all account types. A `Role` account cannot be passed as an `Org` account without Anchor rejecting the discriminator mismatch. This prevents account confusion attacks across the instruction set.

---

## Denial of service

There is no global state. Each org, role, and member account is independent. An attacker cannot block access to one org's roles by attacking another org's accounts.

---

## M1 invariant test commitment

Before M1 ships, the test suite will cover the following invariants:

- A holder with a revoked Member PDA fails check_role
- A holder with an expired membership fails check_role
- A disabled role fails check_role for all holders
- A non-admin cannot assign or revoke role memberships
- A non-authority cannot create roles under an org
- Closing a Member PDA decrements the role member_count by exactly 1
- Re-assigning the same role to the same holder fails (init constraint prevents duplicate PDAs)

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
