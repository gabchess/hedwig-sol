# Hedwig threat model

This document defines the security boundary of Hedwig's current repository program. It distinguishes checks enforced by Hedwig from checks an integrating program must perform. The live devnet deployment contains the first five instructions; the locally tested `set_role_enabled` circuit breaker requires a redeploy.

Hedwig has not completed an external security review. The 21 LiteSVM integration tests are evidence of the tested behaviors below, not a substitute for one.

## Assets and security properties

Hedwig protects three kinds of state:

- an `Org` identifies the authority allowed to create roles in its namespace;
- a `Role` identifies its org and the admin allowed to manage memberships or disable the role;
- a `Member` identifies one holder's membership in one role, with optional expiry.

The program is intended to enforce these properties:

1. Only an org authority can create a role under that org.
2. Only a role admin can assign or revoke membership and toggle that role.
3. A successful `check_role` means the supplied holder has the supplied role, the role is enabled, and the membership has not expired.
4. Revoking a membership or disabling its role makes subsequent checks fail.
5. Failed instructions do not partially update counters or close accounts.

## Trust assumptions

| Component | Current controller | Consequence if compromised |
|---|---|---|
| Org authority | Signer that created the org | Can create any role in that org. The authority cannot currently be rotated. |
| Role admin | Org authority recorded when the role is created | Can assign and revoke memberships and enable or disable the role. The admin cannot currently be rotated. |
| Program upgrade authority | Pubkey `8gba...HPqY`, operationally recorded as deployer-controlled | Can replace all program logic. This is the highest current deployment risk. |
| Solana runtime and Clock sysvar | Solana validator consensus | Supply account ownership, transaction atomicity, signatures, and time used by expiry checks. |
| Integrating program | Its own upgrade and instruction authorities | Must authenticate the actor whose membership it asks Hedwig to check. |

No offchain service, indexer, or cache participates in an onchain role check.

## Account identity and substitution

PDA seeds are public; secrecy of seeds is not a security control. Hedwig relies on Solana program ownership and Anchor account validation:

- account discriminators establish the expected account type;
- PDA seeds and stored bumps bind each account to its canonical address;
- `has_one` constraints bind stored relationships such as member-to-role, member-to-holder, org-to-authority, and role-to-admin;
- required authorities and admins are transaction signers on mutating instructions.

A discriminator alone does not prove that an account is the correct org, role, or member. The seed and relationship constraints provide that binding.

The canonical addresses are:

```text
Org:    ["org", authority]
Role:   ["role", org, role_name]
Member: ["member", role, holder]
```

This layout allows one `Org` per authority and one `Role` per name within an org. It also prevents duplicate membership for the same role-holder pair while the original `Member` account exists.

## Caller authentication is an integration requirement

`check_role` proves a statement about the supplied `holder` pubkey. It does not prove that the transaction caller controls that pubkey: `holder` is intentionally not a signer because programs may need to check wallets, multisigs, or program-derived authorities.

A consuming program must bind `holder` to the actor it intends to authorize before trusting a successful CPI. Depending on the integration, that means requiring a `Signer`, validating a PDA owned by the consumer, or applying another explicit identity constraint. Failing to do so can let a caller present someone else's active membership.

A tested reference consumer demonstrating both signer and validated-PDA patterns is a prerequisite for partner integrations; it is not shipped yet.

## Membership lifecycle

### Assignment and expiry

`assign_role` accepts `expires_at = 0` for a non-expiring membership. Any nonzero value must be strictly later than the current Clock timestamp at assignment.

`check_role` treats the membership as valid while `Clock::unix_timestamp <= expires_at` and returns `MembershipExpired` after that point. Expiry blocks future checks but does not close the `Member` account or decrement `member_count`; an admin must still revoke the membership to reclaim rent and update the counter.

### Revocation

`revoke_role` requires the role admin, verifies the member-role relationship, decrements `member_count` with checked arithmetic, and closes the `Member` account. A later check cannot use the closed membership.

### Role circuit breaker

`set_role_enabled` requires the role admin. Disabling a role makes `check_role` fail for every member and blocks new assignments; it does not delete existing memberships. Re-enabling the role restores checks for memberships that have not expired or been revoked.

## Availability and denial of service

Hedwig has no application-global writable account. Write contention and authority failure are therefore scoped to an org, role, or membership rather than shared across every Hedwig user.

This does not eliminate denial-of-service risk. An unavailable or compromised admin can prevent legitimate membership changes for its role, Solana congestion can delay transactions, and a consumer can make its own instruction unusable through incorrect account constraints or compute budgeting. Hedwig does not provide recovery or rotation instructions for an unavailable org authority or role admin in the current program.

## Fixed cardinality and immutable authorities

The current address scheme and instruction set deliberately impose these constraints:

- one org per authority, because the org PDA is derived from `["org", authority]`;
- one role with a given name per org, because the role name is part of the role PDA seeds;
- immutable org authority and role admin, because no rotation instruction exists.

Supporting multiple orgs per authority or authority rotation would change the state model or instruction surface. It requires a new reviewed decision and migration plan, not a silent change to the deployed program.

## Deployment and upgrade authority

As checked on 2026-07-13, the devnet program was last deployed at slot `468922773` on 2026-06-12 and its upgrade authority was `8gbaJEfM5VDs9BpFLgwMTq7s2FkVpEri8ZnPbxn4HPqY`. The repository records that authority as a single deployer-controlled key. A compromise could bypass every invariant described above by deploying different code.

The deployment predates the addition of `set_role_enabled`; circuit-breaker guarantees apply to the current source and tests, not to the live devnet executable until it is redeployed.

Before mainnet, the upgrade authority will move to a 2-of-3 Squads multisig. Removing upgrade authority entirely is a later, evidence-gated decision: stable v1 interfaces, an external security review with all critical and high-severity findings closed, and at least one external production integration must exist first. Freezing earlier would prevent remediation of defects discovered during integration or review.

## Test evidence

The current 21-test LiteSVM suite covers:

- rejected non-authority role creation;
- rejected non-admin assignment, revocation, and role toggling;
- wrong-holder and wrong-role checks;
- past, negative, active, and non-expiring membership timestamps;
- revoked and duplicate memberships;
- disabled-role and re-enable behavior;
- checked `role_count` and `member_count` lifecycle updates.

The suite does not establish correctness of an external consumer, upgrade-key operations, Squads governance, or mainnet deployment. Those require separate evidence in the roadmap.

## Out of scope for the current devnet program

- holder authentication inside `check_role`;
- authority or admin rotation and recovery;
- role hierarchy, inheritance, or holder-driven delegation;
- spending limits or transaction policy enforcement;
- token-gated assignment and Token-2022 badges;
- cross-program propagation hooks;
- formal verification.

These features belong in integrating or wrapper programs unless a future ADR changes the small-core boundary.

## Reporting vulnerabilities

Do not open a public issue for an undisclosed vulnerability. Use GitHub's private vulnerability-reporting flow for this repository if available; otherwise contact the maintainer through the links on the repository owner's GitHub profile and request a private channel. There is no bug bounty program at this stage.
