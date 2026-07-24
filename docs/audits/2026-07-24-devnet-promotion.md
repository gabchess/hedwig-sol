# Devnet Promotion Record

Date: 2026-07-24

Program: `H4J9wWhraK2Zvn4o9aFheFVmAf7nfaBNPw3d7w77X1eC`

Cluster: Solana devnet

Reviewed source: `5d157c9d2bfc52060c513ff39ed53effc301e2eb`

## Promotion constraints

- Upgrade the existing fixed program ID only.
- Keep the current upgrade authority unchanged.
- Deploy the Hedwig program only. Do not deploy the reference consumer.
- Do not transfer or freeze authority.
- Do not deploy to mainnet.
- Keep the predeploy dump until the postdeploy proof is complete.

## Predeploy state

| Field                  | Evidence                                                                                                                              |
| ---------------------- | ------------------------------------------------------------------------------------------------------------------------------------- |
| ProgramData account    | `C8zmPRJyQmDrG2fM3uCKft7dZxjMHf5zAxRewffr6yGz`                                                                                        |
| Upgrade authority      | `8gbaJEfM5VDs9BpFLgwMTq7s2FkVpEri8ZnPbxn4HPqY`                                                                                        |
| Existing deployed slot | `468922773`                                                                                                                           |
| Existing data length   | `217672` bytes                                                                                                                        |
| Predeploy dump SHA-256 | `006a09bffd1a317c1aeef8f1c774d0c6704bc36962209da241459482b6e17d39`                                                                    |
| Reviewed local SHA-256 | `42670041e7df0f9832930bfa511b884e8b59ceebc8e14b0638c4627a83e6aed3`                                                                    |
| Ship-gate result       | Expected predeploy FAIL: old binary, single EOA, no CU fixture; fresh-deploy rent model did not account for existing ProgramData rent |

The predeploy dump is stored outside the repository at
`/tmp/hedwig-devnet-predeploy.so` for immediate rollback during this promotion.

## Exact upgrade command

```bash
solana program deploy target/deploy/hedwig_sol.so \
  --program-id H4J9wWhraK2Zvn4o9aFheFVmAf7nfaBNPw3d7w77X1eC \
  --url devnet
```

The first deploy command stopped before code replacement because the loader
requires at least 10,240 additional ProgramData bytes. The program was extended
by that minimum in finalized transaction
`5xKtube6ie7k3xXSHSZvn86QUFL5S4PyAJBkBETUrqjqa1kCFCE131Bexhw8nYJLfa3xXqFRMPsVE84cq97YZaCt`
at slot `478655264`. The devnet faucet was rate-limited, so one abandoned
authority-owned buffer, `EDDVRMj7QiyGwbvRkkiDtkJYRM66r365aFr7riR88TQp`, was
closed in finalized transaction
`4EqvWXQ8gtPSrnm1FUKpgrs71nKcywtptMd7nxLjiqqTZEZQCKsX5KeccHhtWwvfU8Sc2MzymLJU3Sti1heh2rL7`
at slot `478655555`. Its 1.51614552 devnet SOL rent returned to the same
deployer. The predeploy dump remained available for rollback.

## Postdeploy proof

| Field                        | Evidence                                                                                               |
| ---------------------------- | ------------------------------------------------------------------------------------------------------ |
| Deployment signature         | `957WYWCgyWeqBAsBAURdfhR3VC6aQpPj4bGdvZieRxJpxsAvdopgqgdYcFeeHwguG38P8pydMxT5m3AUmj1XeiZ`              |
| Deployed slot                | `478655638`                                                                                            |
| ProgramData length           | `227912` bytes                                                                                         |
| Postdeploy dump SHA-256      | `7218f6f8f538cfb3583159e6b18c39668ce643b65cae2aa4bf61c5209d71a543`                                     |
| Reviewed ELF identity        | First `222808` live bytes equal the reviewed ELF; all `5104` trailing loader-allocation bytes are zero |
| Exact account-image equality | Zero-padded reviewed image and fresh live dump both hash to `7218f6...1a543`; full `cmp` passed        |
| Upgrade authority unchanged  | `8gbaJEfM5VDs9BpFLgwMTq7s2FkVpEri8ZnPbxn4HPqY`                                                         |
| Postdeploy ship-gate         | Build PASS, rent PASS, CU WARN without a simulation fixture, authority FAIL until governed custody     |
| Disabled Role state          | Fetched as `enabled=false` after `set_role_enabled`                                                    |
| Revoked Member state         | Account absent after `revoke_role`                                                                     |

## Live six-instruction lifecycle

| Instruction        | Signature                                                                                  |
| ------------------ | ------------------------------------------------------------------------------------------ |
| `create_org`       | `33CXdsKqsePRaXerF7mQXuLppdg8ACVciuLhNweU6rqDz2YKnmJZX9aYdxXLvL67bDETpemc15asmYUSsgJ3Wdz4` |
| `create_role`      | `VVsG3PqkeRFnuoAp5CBnWaysdybb8mWU9yVaSgzDuKHMEQYhqFDt875wLDLEqJbGPm1SSEpzQCX5u3hcVSDwc92`  |
| `assign_role`      | `3TzanWJYJWmBPgwuj9rLfhYHzx4EcSSqc374thWsDCGeZDwbV8iRVBKi9J6p33eY4Enhkv8RBejhoaU8Lu1yUQ83` |
| `check_role`       | `2f6PaSra8uwgVDsaUMHWViW41xsq5WDdujj6AJjJFEvDj4tHu5XRUX894Erzgn19QmkwDSU9pziUYpzzU5kPauSb` |
| `set_role_enabled` | `4ivsEv1WeH574xoQjcY7xPr2zF5FsbLktkpzhAepVTyjkebbCuNcuh7gnkPJKoeZhqZVHXrisTgvuCyh7ksiMjin` |
| `revoke_role`      | `5WWvVh9eSJgNqaQ4d2AHyht1jVgj7u7XeX6sFpGT99199sR888qxWJHWVaNvkLgnZgmFYaSBm6MAV5M9FDu9PBvf` |

This promotion changes the Hedwig program only. The reference consumer remains
local and undeployed. Authority was neither transferred nor frozen.
