# AGENTS.md: hedwig-sol

Standing context for any AI coding agent working in this repo. Read this before touching anything.

## What Hedwig is

Hedwig is a composable onchain roles primitive for Solana. Its account model is `Org PDA -> Role PDA -> Member PDA`. A consuming program can authenticate an actor, then CPI into `check_role` to verify that the corresponding holder has an enabled, unexpired membership.

Framing: roles as a composable onchain primitive for organizations, usable by a person, a team, or an autonomous agent. Hedwig is an original, Solana-native program. The name is locked to "Hedwig" alone.

## Stack

- Solana / Rust / Anchor 1.0.2
- Devnet program: `H4J9wWhraK2Zvn4o9aFheFVmAf7nfaBNPw3d7w77X1eC`
- TS SDK (`@hedwig-sol/sdk`): private local `0.1.0-alpha.0`, built and tested,
  not published
- Secure consumer: devnet reference at
  `52D3pTYvMwLYbiigY5xg55n4HmtEzTKCEicx1Cojzo9a`
- MIT license

## Architecture

6 instructions total: `create_org`, `create_role`, `assign_role`, `revoke_role`, `check_role`, `set_role_enabled`. Program source at `programs/hedwig_sol/src/` (~580 LOC): `lib.rs`, `state.rs`, `error.rs`, `constants.rs` + one file per instruction.

## Conventions

- CI gate (`.github/workflows/ci.yml`): match every Rust, SDK, and app gate it
  declares before proposing a change.
- Tests use LiteSVM: 28 core integration tests under
  `programs/hedwig_sol/tests/` and 12 consumer integration tests under
  `programs/hedwig_consumer/tests/`.
- Project source carries zero TODO/FIXME/unimplemented markers. Keep it that way; anything unfinished goes in docs/tracker, not a code stub.
- `app/demo.ts` uses the local SDK for the exact six-instruction lifecycle and
  reads back disabled Role and closed Member state. Do not run it before the
  six-instruction devnet promotion is verified.
- `app/consumer-demo.ts` proves an authenticated member can change state in the
  separate devnet consumer through Hedwig CPI.
- Build both `programs/hedwig_sol` and `programs/hedwig_consumer` SBF artifacts
  before `cargo test --workspace`; the LiteSVM fixtures load those artifacts.

## What NOT to touch without explicit sign-off

- The devnet program ID: treat as a fixed identity for this milestone.
- The ignored local program keypair does not derive the fixed devnet program
  ID. Any upgrade must pass
  `--program-id H4J9wWhraK2Zvn4o9aFheFVmAf7nfaBNPw3d7w77X1eC` explicitly.
- The single-deployer-key upgrade authority: a **named, tracked risk** in `THREAT-MODEL.md`; the roadmap requires a 2-of-3 Squads multisig before mainnet. Do not change it silently.
- `docs/sdk-rfc.md`: locked historical design doc. Do not edit it in place; `docs/adr/0003-adoption-led-interfaces.md` records the active decisions that supersede parts of it.
- Naming/branding: "Hedwig" only.

## Goal-contract discipline

Before starting any task here, state a **DONE WHEN** in one sentence: a binary, verifiable end-state, such as a file exists, a named test passes with exit code 0, or a grep returns N hits. Not "improve the tests" or "make this more secure." If the task can't be stated that way, stop and ask before touching code. Apply this every time: named artifact, bounded scope, checkable end-state.
