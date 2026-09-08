# Reference authorization flow

Status: technical reference for a generic vault operator or rebalance bot.
Vault positions and a proposed rebalance are simulated. The only onchain
action is the existing protected counter change. This flow transfers no assets,
performs no real rebalance, and adds no new contract or external integration.

## Scenario

A vault operator gives a dedicated bot a short window to submit a protected
operation. The reference flow shows simulated vault positions and a proposed
rebalance. The bot's attempted operation increments the existing reference
consumer counter only while its required role remains valid. Revocation or
expiry blocks a later attempt that requires that role.

A real vault integration would need to enforce the role check inside the
state-changing onchain path. An SDK-only check can be bypassed if the bot
retains direct manager authority. This rehearsal does not prove that a vault
has installed that gate or that its assets are protected by Hedwig.

## Authorization sequence

1. An admin grants a dedicated actor the `operator` role with a short expiry.
2. The actor signs a transaction for the protected consumer action.
3. The consumer authenticates the signer, checks membership, and changes its counter.
4. The admin revokes membership and the caller waits for chain confirmation.
5. The actor retries; the transaction fails and the counter stays unchanged.

Run expiry as a separate path. Current `check_role` accepts the boundary second
when `Clock.unix_timestamp == expires_at` and rejects strictly after it. A
15-minute grant sets a maximum role lifetime; finishing the task does not
itself trigger revocation without a separate admin action.

## Evidence

Record the actor, role, expiry, confirmed transaction status, and counter before
and after each attempt. A local state label alone is not proof. The denied retry
must still be attempted by the actor after revocation has applied.

Revocation cannot undo prior transactions. Concurrent transactions ordered
before it can succeed. Other agent processes and credentials remain outside
Hedwig's control. Network fees and offchain compute charges are not capped.

## Current proof

The existing consumer is controlled by the Hedwig builder. The July 24 record
shows an authenticated counter increment through CPI. It proves the technical
path, not a customer integration or a hosted application. See the
[consumer evidence](../deployment/evidence/2026-07-24-consumer-devnet-integration.md).
