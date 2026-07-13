# Hedwig devnet membership demo

Runs the five-instruction membership lifecycle against the recorded Hedwig
devnet deployment:
create_org, create_role, assign_role, check_role, revoke_role.

The demo does not exercise `set_role_enabled`. That sixth instruction is
implemented and tested locally but is not in the current devnet deployment.

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

An Org PDA is derived from the wallet, not from the random display name. Each
wallet can therefore run this demo once. Use a fresh funded wallet for another
run.

The script prints one labeled line per instruction with the resulting
transaction signature, reads back the Role and Member account state after
`assign_role`, and ends with `full lifecycle OK on devnet` on success.
