# Agent access

**Agent access you can revoke and verify.**

Hedwig gives Solana apps a shared, revocable role record for software agents.

Status: positioning reference. Implementation work is paused.

The proposed user is a Solana team that controls a recurring protected action
and needs to grant, expire, or revoke a software agent's role. That customer
profile is a hypothesis. One builder-owned reference consumer is available;
independent adoption and recurring demand are not established.

- [Integration proof](integration-proof.md): one agent and one protected action
- [Reference flow](reference-flow.md): valid access, confirmed revocation, expiry, and denied retry
- [Access control](../access-control/architecture.md): the existing membership model
- [Integration](../access-control/integration-guide.md): consumer enforcement
- [Security boundaries](../../THREAT-MODEL.md): current risks and responsibility

## What the role record controls

The consumer defines what the role permits and must authenticate the actor
before checking membership; see
[Caller authentication is an integration requirement](../../THREAT-MODEL.md#caller-authentication-is-an-integration-requirement).
A role can be assigned to the agent's own authenticated key, so no shared
human key is required by this pattern.

This is application authorization. Hedwig does not provide key custody, a
wallet signing policy, arbitrary instruction allowlists, spending limits,
process shutdown, compute cost caps, or a guarantee against hacks. It does not
replace a wallet policy engine or an independent security review.
