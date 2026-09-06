# Prove when agent access ends

Bring one agent and one protected Solana action. Prove the agent can act while
its role is valid, and cannot after expiry or revocation.

This is a proposed devnet integration exercise for a technical owner with an
actual recurring action. It is not a production security service, an audit,
or a promise to prevent all misuse.

## Scope

Map the actor, credential, permitted action, and existing checks. Integrate the
current Hedwig membership check into that one action while preserving actor
authentication. Use a dedicated agent identity and a bounded role lifetime.

Record successful execution while authorized. Confirm revocation onchain and
retry the action. Separately prove expiry. Record transaction results and the
protected account state, including unchanged state on denied attempts.

## Concrete rehearsal

The initial scenario is a generic vault operator with a rebalance bot. The
[Rebalance authorization rehearsal](demo.md) shows simulated vault positions
and proposed changes, with the existing protected counter as the only onchain
action. It does not transfer assets, execute a real rebalance, or claim an
external vault integration or customer.

A real integration must make the role check mandatory in the onchain action.
A client or SDK precheck alone does not enforce this limit when the bot still
has direct manager authority. The exercise must identify and remove or gate
that bypass before any real vault security claim is made.

## What the team gets

- One documented authenticated action and its permission boundary.
- A reproducible devnet success and denial flow.
- Transaction and state evidence for the result.
- A decision on whether to keep, revise, or reject the integration.

## Fit and limits

The team must control the program that enforces the role check. An external
service that never checks Hedwig is outside this scope. Existing wallet
policies may already solve the team's need; test that before adding another
access system. Shared membership across programs is a design capability, not
proof that customers have a repeated cross-program problem.

The current core and builder-owned reference consumer have recorded devnet
evidence. There is no verified signed pilot, external adoption, production
release, or independent external audit. Pricing and delivery timing are not
committed by this proposed scope.
