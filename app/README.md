# Hedwig devnet demo

Runs the full Hedwig lifecycle against the live devnet deployment:
create_org, create_role, assign_role, check_role, revoke_role.

## Prerequisites

- Node.js 18+
- A funded devnet wallet at `~/.config/solana/id.json` (or set `ANCHOR_WALLET`
  to a different keypair path)
- `socket` installed (`npm install -g socket`) for the security-gated install

Fund a devnet wallet if needed:

```bash
solana airdrop 1 <your-pubkey> --url devnet
```

## Install

```bash
socket npm install
```

## Run

```bash
npm run demo
```

By default the demo connects to the public devnet RPC
(`https://api.devnet.solana.com`), which rate-limits under load. To use a
dedicated RPC instead, set `HELIUS_RPC_URL` before running:

```bash
export HELIUS_RPC_URL="https://devnet.helius-rpc.com/?api-key=<your-key>"
npm run demo
```

Never commit an API key. Pass it as an environment variable only.

## What it does

Each run creates a fresh org with a random name so it never collides with a
previous run's onchain accounts. It prints one labeled line per instruction
with the resulting transaction signature, reads back the role and member
account state after `assign_role` to prove the onchain data is correct, and
ends with `full lifecycle OK on devnet` on success.
