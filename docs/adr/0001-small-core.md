# 0001: Small, flat authorization core

**Status:** Accepted
**Date:** 2026-07-13

## Context

Authorization systems tend to accumulate hierarchy, policy evaluation, identity, and workflow features. Each addition expands the onchain trust boundary and makes the primitive harder for consuming programs to understand and audit. Hedwig already has a complete three-account model for its current purpose: `Org -> Role -> Member`.

## Decision

Keep the Hedwig program focused on named role membership. The core creates an organization, creates and enables or disables roles, assigns and revokes memberships, and checks whether a membership is active. Roles remain flat. Hierarchy, inheritance, holder-to-holder delegation, eligibility policy, badges, spending limits, and organization workflows belong in consuming programs or separate wrappers.

The current account cardinality and authority rules are explicit:

- `Org` is derived from `['org', authority]`, so the current interface permits one organization per authority pubkey.
- `Role` is derived from `['role', org, role_name]`; its name is part of its identity and cannot be renamed.
- `Member` is derived from `['member', role, holder]`, so a holder has at most one membership account per role.
- A role's admin is set to the organization authority at creation. The current six-instruction interface has no admin-transfer instruction.
- Membership expiry is optional. Role disablement is the role-wide circuit breaker.

## Consequences

- Good: consumers can reason about one small account graph and one membership predicate.
- Good: application-specific policy can evolve without changing Hedwig's core program.
- Bad: richer organizations require a wrapper or consuming program.
- Neutral: changing organization cardinality or role administration would require a deliberate interface and migration decision; documentation must not imply those capabilities exist today.

## Rejected alternatives

- **Role hierarchy in the core:** rejected because tree traversal and inheritance enlarge the authorization surface.
- **Pluggable eligibility modules in the core:** rejected because identity and policy have different trust boundaries from membership.
- **Holder delegation in the core:** rejected because it introduces a second authority path that the current product does not require.
- **Generic policy engine:** rejected because consuming programs can enforce their own policy after verifying Hedwig membership.

## Related

- [`docs/architecture.md`](../architecture.md)
- [`THREAT-MODEL.md`](../../THREAT-MODEL.md)
- [`docs/sdk-rfc.md`](../sdk-rfc.md), whose flat-role decision remains active
