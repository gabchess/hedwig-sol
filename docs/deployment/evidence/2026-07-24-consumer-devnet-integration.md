# Secure consumer devnet integration

Date: 2026-07-24

Status: verified builder-owned reference integration. This is not an
independent design partner, customer, or production deployment.

## Reviewed source

- Hedwig program:
  `H4J9wWhraK2Zvn4o9aFheFVmAf7nfaBNPw3d7w77X1eC`
- Reference consumer:
  `52D3pTYvMwLYbiigY5xg55n4HmtEzTKCEicx1Cojzo9a`
- Consumer source:
  `programs/hedwig_consumer/src/lib.rs`
- Live proof client:
  `app/consumer-demo.ts`
- Local suite: 12 consumer integration tests plus the consumer program-ID test

The consumer requires an authenticated signer, derives its counter from that
signer, stores the same authority, and passes that signer to Hedwig as the
holder. `Program<HedwigSol>` pins the CPI target. Failed Hedwig checks propagate
and abort the consumer instruction before its state change.

## Build and deployment

- Deployment signature:
  `4jdcuVZEHAkNjch31qmTGupwsVNuZee97L3BE87wohJqi785dmpZ3dPDvsQqz2KYC8h6SHnMnppj7GHt6aZ8koCh`
- Deployment slot: `478667066`
- Slot time: `2026-07-24T23:35:46Z`
- ProgramData:
  `2q7MuMJAHvX3jSxhL3gH7JXP2pdsrvnzCHq1pWtZgjn5`
- Upgrade authority:
  `8gbaJEfM5VDs9BpFLgwMTq7s2FkVpEri8ZnPbxn4HPqY`
- Artifact size: `176264` bytes
- Artifact SHA-256:
  `10909aba9e9474a4b9c98ec3f9ebd2ab312b7fcf370f9564c3153b63d2bba2a5`
- Live dump SHA-256:
  `10909aba9e9474a4b9c98ec3f9ebd2ab312b7fcf370f9564c3153b63d2bba2a5`
- Byte comparison: exact full-file match

Before deployment, the generated program keypair was found not to match the
consumer's old undeployed declared ID. The declared ID, Anchor configuration,
and client constant were corrected to the generated deploy key
`52D3...zo9a`. Both SBF artifacts and the full 42-test Rust suite then passed.
The consumer had no prior live address or external integration to migrate.

## Live state-change proof

Fresh actor:
`HpBXjrxJUbvbkFdjGKcuqeXLVbLBhT4KzWMtoenmJdVm`

| Step                         | Result                                                                                     |
| ---------------------------- | ------------------------------------------------------------------------------------------ |
| Fund fresh actor             | `4n9zHzpN3G1qw2N3inpVgkmgpvxvRvKYFVLCuYSJJcMsTkxdrr1A1kSAdbMfaM5L2Bn8jS4G68BJw6X2iBe4PQY8` |
| Create org                   | `4sn1aNaKcxQthThyN81XctbHh1dLAsPXNdPgXaFR9gbfMZrRMH4rx1cQuzXW5AmBkZDVeKHmQ1niP6PZWqu25TMg` |
| Create `operator` role       | `46SK61L7rCPLjja4xYdPZcXrDCmBXgZiQxe6jKQLn7saRYLBPhZu3CzpWmWDAu1XDmm911rmDA9FMykQxLe4ANcn` |
| Assign actor to role         | `2nsyYKWjHWH2nPiScCozL5B9dSnynYCar4hPtgJ6VJ6o8LpYJJYJLxgxrk6Zi1RfcgwvaB8upoUA6eWu7KHzxc7L` |
| Initialize consumer counter  | `2hWg7HCzZfTtyw8CVfJ2CMrdYDHq4n8EEoeUyDCmgk6L6XuY6g4AE2WjfqzWHJRHuNe29AV2nJJyGfb5PBz9ha3C` |
| Increment through Hedwig CPI | `1LBw17SBGhVRijDwGWjvpvfZhheB1EwqiSXYRmu55TAte3yGUa7xoGRFJuyu3ragszQXwqHFbHmMeJYLgH5dnjA`  |

Derived accounts:

- Org: `J12w5krU2BxpCMQTRRnYG4p6DSweMK8wzVKi6HZGRRs4`
- Role: `3ESP4cYqqnCyz55z8jXsc6yLy8y1yCEmGmVJ8wm6dSNo`
- Member: `2eFex8mUN66zRcKoVy4WbUAKHNKJ7YGc6wkQb1P35wUP`
- Consumer counter:
  `3j3KNBCUnHWF8pcFHQVgQDo7bUREyUrE25RoNHiTJDvR`

The final read proved:

- the counter account is owned by the fixed consumer program;
- its discriminator is `Counter`;
- its stored authority is the authenticated actor;
- its stored role is the role checked through Hedwig;
- its value changed from zero to one; and
- the Hedwig Member holder equals the same actor.

## Claim boundary

This proof closes the gap between local CPI tests and a live devnet
state-changing integration. It does not show that another team can integrate
Hedwig without builder help, that a team will keep the integration, or that
there is recurring or paid demand. Those remain the next customer-validation
gate.
