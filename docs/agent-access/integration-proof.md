# Agent access integration boundary

Hedwig's first validation target is one recurring protected action. The
consuming program authenticates the agent and checks its shared role when the
instruction executes. The evidence is a successful call with valid membership,
a confirmed expiry or revocation, a denied retry, and unchanged protected
state.

This is a design boundary for future integration work. No external team,
customer, production use, or independent adoption has been verified.

## Boundary

The consuming program identifies the actor, credential, permitted action, and
existing checks. It then requires the Hedwig membership check in the
state-changing instruction. A dedicated agent identity and bounded role
lifetime make the result inspectable.

The record includes the successful transaction, the onchain state change that
confirms revocation or expiry, the denied transaction result, and the unchanged
protected account. A browser state label or client-side precheck is not evidence.

## Reference workflow

The initial reference is a generic vault operator with a rebalance bot. The
[reference authorization flow](reference-flow.md) uses simulated vault
positions and the existing protected counter as its only onchain action. It
does not transfer assets, execute a real rebalance, or claim an external vault
integration.

A real vault integration must make the role check mandatory in the onchain
path. A client or SDK precheck can be bypassed when a bot keeps direct manager
authority. The authority path must be reviewed before making a vault security
claim.

## Fit

The integrating team must control the program that enforces the role check. An
external service that never checks Hedwig is outside this boundary. Existing
wallet policies may already cover the need and should be tested first. Shared
membership across programs is a design capability; it is not evidence of
recurring customer demand.

The current core and builder-owned reference consumer have recorded devnet
evidence. They show the technical path and do not establish independent use.
