# @hedwig-sol/sdk

Thin TypeScript helpers for Hedwig's six onchain instructions.

Current status: private repository-local `0.1.0-alpha.0`. This package has not
been published to npm.

The package exports the fixed program ID, generated Anchor IDL type, PDA
derivation helpers, argument checks, and build/send pairs for:

- `createOrg`
- `createRole`
- `assignRole`
- `checkRole`
- `revokeRole`
- `setRoleEnabled`

## Signers stay explicit

Every sender requires a `signers` array. The provider wallet still signs and
pays for the transaction. Pass any extra transaction signers yourself:

```ts
const signature = await sendAssignRole(
  provider,
  {
    role,
    holder,
    admin: provider.publicKey,
    expiresAt: null,
  },
  { signers: [] }
);
```

The SDK never generates a keypair.

The devnet reference consumer demo uses this SDK to create the org, role, and
membership before a separate program authenticates the holder and calls Hedwig
through CPI. See [`app/consumer-demo.ts`](../app/consumer-demo.ts) and its
[public promotion record](../docs/audits/2026-07-24-consumer-devnet-integration.md).

## Membership is not authentication

`checkRole` proves that the supplied holder pubkey has an active membership.
The holder is intentionally not a signer for that instruction. A consuming
program must authenticate its own authority and pass that authenticated account
to Hedwig as the holder.

## PDA helpers

```ts
const [org] = deriveOrgPda(authority);
const [role] = deriveRolePda(org, "treasurer");
const [member] = deriveMemberPda(role, holder);
```

Org names must contain 1 to 64 UTF-8 bytes. Role names must contain 1 to 32
UTF-8 bytes. Expiry accepts a `Date`, integer Unix seconds, `BN`, `null`, or
`undefined`. `null` and `undefined` map to `NO_EXPIRY`, represented onchain as
zero.
