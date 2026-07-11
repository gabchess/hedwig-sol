# AGENTS.md: hedwig-sol

Standing context for any AI coding agent working in this repo. Read this before touching anything.

## What Hedwig is

Solana has no composable onchain primitive for "who is authorized to do what": multisig member changes need a full proposal cycle, upgrade authority is usually a single hot wallet, and autonomous agents get raw private keys instead of scoped permissions. Hedwig fixes that with a PDA-anchored account layout: `Org PDA -> Role PDA -> Member PDA`. Any program can CPI into `check_role` to verify membership (holder + role enabled + not expired) in one instruction.

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
- `app/demo.ts` is the canonical devnet e2e reference for the full role lifecycle.

## What NOT to touch without explicit sign-off

- The devnet program ID: treat as a fixed identity for this milestone.
- The single-deployer-key upgrade authority: a **named, tracked risk** in `THREAT-MODEL.md` (M1 target: 2-of-3 Squads multisig), not an oversight to silently "fix."
- `docs/sdk-rfc.md`: locked design doc, already PR-reviewed. Propose changes as a new PR, don't edit in place.
- Naming/branding: "Hedwig" only.

## Goal-contract discipline

Before starting any task here, state a **DONE WHEN** in one sentence: a binary, verifiable end-state, such as a file exists, a named test passes with exit code 0, or a grep returns N hits. Not "improve the tests" or "make this more secure." If the task can't be stated that way, stop and ask before touching code. Apply this every time: named artifact, bounded scope, checkable end-state.
