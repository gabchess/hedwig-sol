# Seven-day shared authorization pilot

Historical record, superseded 2026-09-06. See the current [agent access integration boundary](../agent-access/integration-proof.md).

If two related Anchor programs each decide who may change state, Hedwig will map one real path and show whether one role store removes work without weakening actor authentication.

This is a free, seven-day pilot for one qualified Solana team. It ends with a merge, revise, or reject decision. It does not require the team to adopt Hedwig.

## Who this is for

The pilot fits a team that:

- maintains at least two related Anchor programs with a real direct/CPI authorization path spanning them;
- repeats or splits authority logic across that path;
- has an audit, launch, migration, or material release within about eight weeks; and
- has a technical owner who can approve an integration in one sprint.

A standalone program with no shared authorization path is not a fit. Neither is a path that needs an unbuilt Hedwig feature.

## What happens in seven days

### 1. Map the current path

We document:

- the state-changing instructions;
- the actor and how each consumer authenticates that actor;
- every authority account, signer, PDA, and constraint;
- CPI edges and the accounts passed across them;
- administrators and upgrade authorities;
- scope, expiry, revocation, and incident behavior; and
- the current migration and rollback path.

### 2. Draw the smallest Hedwig-backed path

The proposed map shows where Hedwig's `Org`, `Role`, and `Member` state could become the canonical membership store.

The consumer keeps responsibility for actor authentication. A Hedwig membership proves that one pubkey holds a role. It does not prove control of that pubkey or grant transaction authority by itself.

### 3. Test the operational edge

The pilot executes a Squads custody dry run that proves the signer and proposal
path. It documents migration order and rollback.

Where the selected path needs it, the pilot also covers:

- membership expiry;
- role disable and revocation behavior;
- audit-readable before/after state.

The pilot records multisig compatibility only after the dry run proves the signer and proposal behavior.

### 4. Make the decision

The pilot ends with one of three calls:

1. merge the current Hedwig integration;
2. revise a bounded part of the path;
3. reject the fit.

Rejection is a valid result. The point is to learn whether the current primitive solves a real authorization path before Hedwig adds features.

## What Hedwig provides

- the current authority map;
- the proposed authority map;
- migration and rollback steps;
- integration support for the selected path;
- a custody dry-run record; and
- the final merge, revise, or reject recommendation.

## What the team provides

- access to the relevant code and documentation;
- one technical owner;
- timely answers about past authorization changes and current operations;
- feedback during the seven days; and
- an explicit merge, revise, or reject decision.

A public name, quote, or case study needs separate consent.

## Current product limits

Hedwig is a devnet-stage six-instruction program. Its repository-local
TypeScript SDK alpha is tested but not published. The live reference consumer
is maintained by the Hedwig builder, so it proves the integration path rather
than external adoption.

Hedwig has no verified design partner, customer, revenue, production use, independent external audit, or mainnet release. The single-key program upgrade authority remains a mainnet blocker.

## Current status

The offer is published. No team has signed the pilot, and no design partner,
customer, or external adoption has been verified. Current program evidence
remains in the [grant progress ledger](../grants/progress.md) and
[roadmap](../../ROADMAP.md).
