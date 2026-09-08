# Devnet program presence check

Date: 2026-09-06. Read-only check against Solana devnet using `solana program show`.

| Program | Address | Last deploy slot | ProgramData bytes |
| --- | --- | --- | --- |
| Hedwig core | `H4J9wWhraK2Zvn4o9aFheFVmAf7nfaBNPw3d7w77X1eC` | `478655638` | `227912` |
| Reference consumer | `52D3pTYvMwLYbiigY5xg55n4HmtEzTKCEicx1Cojzo9a` | `478667066` | `176264` |

Both report owner `BPFLoaderUpgradeab1e11111111111111111111111` and upgrade
authority `8gbaJEfM5VDs9BpFLgwMTq7s2FkVpEri8ZnPbxn4HPqY`.

This confirms program presence and the recorded deployment slots. No transaction
was submitted, no program bytes were changed, and no fresh binary hash comparison
was performed. The counter state-change proof remains the
[July 24 consumer record](2026-07-24-consumer-devnet-integration.md).
