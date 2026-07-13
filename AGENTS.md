# AGENTS.md: hedwig-sol

Standing context for any AI coding agent working in this repo. Read this before touching anything.

## What Hedwig is

Hedwig is a composable onchain roles primitive for Solana. Its account model is `Org PDA -> Role PDA -> Member PDA`. A consuming program can authenticate an actor, then CPI into `check_role` to verify that the corresponding holder has an enabled, unexpired membership.

Framing: roles as a composable onchain primitive for organizations, usable by a person, a team, or an autonomous agent. Hedwig is an original, Solana-native program. The name is locked to "Hedwig" alone.

## Stack

- Solana / Rust / Anchor 1.0.2
- Devnet program: `H4J9wWhraK2Zvn4o9aFheFVmAf7nfaBNPw3d7w77X1eC`
- TS SDK (`@hedwig-sol/sdk`): RFC only (`docs/sdk-rfc.md`), not built, not published
- MIT license

## Architecture

6 instructions total: `create_org`, `create_role`, `assign_role`, `revoke_role`, `check_role`, `set_role_enabled`. Program source at `programs/hedwig_sol/src/` (~580 LOC): `lib.rs`, `state.rs`, `error.rs`, `constants.rs` + one file per instruction.

## Conventions

- CI gate (`.github/workflows/ci.yml`): `cargo fmt --check`, `cargo build`, `cargo build-sbf`, `cargo test` on ubuntu-latest. Match it before proposing any change.
- Tests use LiteSVM: per-instruction and lifecycle test files under `programs/hedwig_sol/tests/` (21 integration tests).
- Project source carries zero TODO/FIXME/unimplemented markers. Keep it that way; anything unfinished goes in docs/tracker, not a code stub.
- `app/demo.ts` records the five-instruction devnet membership lifecycle. The live program was last deployed at slot `468922773` on 2026-06-12; it predates `set_role_enabled`, so a redeploy is required for the sixth instruction.

## What NOT to touch without explicit sign-off

- The devnet program ID: treat as a fixed identity for this milestone.
- The single-deployer-key upgrade authority: a **named, tracked risk** in `THREAT-MODEL.md`; the roadmap requires a 2-of-3 Squads multisig before mainnet. Do not change it silently.
- `docs/sdk-rfc.md`: locked historical design doc. Do not edit it in place; `docs/adr/0003-adoption-led-interfaces.md` records the active decisions that supersede parts of it.
- Naming/branding: "Hedwig" only.

## Goal-contract discipline

Before starting any task here, state a **DONE WHEN** in one sentence: a binary, verifiable end-state, such as a file exists, a named test passes with exit code 0, or a grep returns N hits. Not "improve the tests" or "make this more secure." If the task can't be stated that way, stop and ask before touching code. Apply this every time: named artifact, bounded scope, checkable end-state.
