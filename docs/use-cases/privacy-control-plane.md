# Privacy control-plane authorization

Status: candidate use case. KageB v0.1 does not integrate Hedwig. This is not
a design partner and not a completed grant milestone.

## Problem

A privacy coordinator may need role-based approval for plan requests, policy
changes, keyper administration, or an emergency stop. These are control-plane
actions: they decide whether an operation may proceed, not the protected
transaction itself.

Hedwig can guard that control plane without entering the protected execution
path.

## Why a control-plane check, not a per-transaction CPI

The integration pattern documented elsewhere in this repository puts Hedwig's
`check_role` CPI inside the transaction it protects, so the consumer program
authenticates the actor and checks its role in the same instruction. That
pattern fits when the protected action and the authorization check are the
same event.

A privacy coordinator's protected transactions are different: high-volume,
latency-sensitive, and not the right place to add a dependency. Putting a
Hedwig account and CPI into every one of those transactions would add compute
and account cost to each one, couple the protected execution path to Hedwig's
deployed program address, and leave a stable, public fingerprint (the same
program ID and accounts, every time) inside a flow the operator may not want
to advertise as role-gated.

An off-critical-path check avoids all three. Hedwig authorizes the control
request before the protected transaction exists. The protected transaction
carries no Hedwig account and no Hedwig instruction, so it costs nothing extra
and reveals nothing about how the request was approved.

## Vocabulary

This use case reuses Hedwig's existing terms: `Org`, `Role`, `Member`, and
holder (the pubkey a `Member` account names, per
[the architecture doc](../access-control/architecture.md)).

It adds two terms specific to the control-plane split, because that split is
the point of this design:

- **Control key**: the holder that signs a control request and is checked
  against Hedwig, exactly as `check_role` verifies any holder.
- **Execution signer**: the key that signs the protected transaction. It is
  never checked against Hedwig and never appears as a holder.

Keeping these two keys separate, in code and in operator instructions, is
what keeps Hedwig off the protected path.

## Candidate flow

1. A control key signs canonical bytes containing a version, command,
   resource ID, nonce, and expiry.
2. An adapter verifies the signature and rejects expired or replayed
   requests.
3. The adapter verifies the Hedwig program owner plus the expected Role and
   Member PDAs.
4. The adapter checks the holder, the role's enabled state, and the
   membership's expiry through `check_role`.
5. The privacy service accepts the control command.
6. The protected transaction remains unchanged: it uses the execution signer,
   not the control key, and contains no Hedwig account or instruction.

The adapter may authorize a request. It never signs, submits, or modifies the
protected transaction.

## KageB mapping

A later KageB coordinator could use Hedwig to authorize:

- changing an operator policy;
- approving a keyper-set change;
- pausing new epochs;
- triggering an emergency stop.

The first spike should stay offchain and verify Hedwig account evidence
before a coordinator action. A one-time onchain session gate may be tested
later. Per-transaction Hedwig CPI is rejected for the reason above: it would
label the protected flow.

## Proof required

- Expired, disabled, wrong-owner, wrong-PDA, wrong-holder, tampered, and
  replayed requests fail.
- The gate's enabled and disabled modes produce byte-for-byte identical
  protected transaction accounts and instructions.
- Evidence states only whether Hedwig authorized the control request.
- Public copy says Hedwig did not sign, execute, or enter protected
  transactions.
- The control key and execution signer remain separate in code, tests, and
  operator instructions.
