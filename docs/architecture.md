# Hedwig Architecture

This document is the LLM-readable map for Hedwig's current code shape. It is meant to answer the question: "what does this repo do, and where does each decision live?"

## Architecture Posture

Hedwig follows a bounded version of screaming architecture:

- The repository root keeps the standard Anchor project shape because that is how Solana reviewers, Anchor tooling, and CI expect to find a program.
- The domain should become obvious inside `programs/hedwig_sol/src`, where the code names the authorization primitive directly: orgs, roles, members, assignments, revocation, and role checks.
- Documentation mirrors that shape so an LLM can recover the product, security model, and folder conventions without depending on any external notes.

The intended first read path is:

1. `README.md` for the product and current status.
2. `docs/architecture.md` for code layout and domain boundaries.
3. `docs/adr/index.md` for the decision record index.
4. `THREAT-MODEL.md` for security posture and integration risks.
5. `ROADMAP.md` for what remains planned.

## Domain Model

Hedwig's core domain is scoped onchain authorization.

| Domain term | Code location | Meaning |
|---|---|---|
| Org | `programs/hedwig_sol/src/state.rs` | Top-level namespace controlled by an authority pubkey. |
| Role | `programs/hedwig_sol/src/state.rs` | Named permission under one org, controlled by a role admin. |
| Member | `programs/hedwig_sol/src/state.rs` | Proof that one holder pubkey currently has one role. |
| Holder | Instruction accounts and `Member` | Any pubkey: human wallet, multisig, program-derived identity, or agent key. |
| Circuit breaker | `set_role_enabled` | Role-wide pause that blocks new assignment and checks without revoking members. |
| CPI check | `check_role` | The composable verification point consumed by other Solana programs. |

## Folder Boundaries

| Path | Responsibility |
|---|---|
| `programs/hedwig_sol/src/lib.rs` | Anchor program entrypoint and public instruction surface. |
| `programs/hedwig_sol/src/instructions/` | One file per domain action. File names should read like product verbs. |
| `programs/hedwig_sol/src/state.rs` | Onchain account types and PDA shape. |
| `programs/hedwig_sol/src/error.rs` | Domain errors returned by authorization and validation failures. |
| `programs/hedwig_sol/src/constants.rs` | Shared seed and sizing constants. |
| `programs/hedwig_sol/tests/` | LiteSVM behavior tests, named after the instruction or lifecycle they protect. |
| `app/` | TypeScript demo/client surface. It should not redefine the Rust domain model. |
| `docs/` | Human and LLM-facing architecture, ADR, SDK, and integration notes. |
| `migrations/` | Anchor deployment scripts only. |

## Naming Conventions

Use domain names before framework names.

- Instruction files use snake_case domain verbs: `create_org`, `create_role`, `assign_role`, `revoke_role`, `check_role`, `set_role_enabled`.
- Account structs use domain nouns: `Org`, `Role`, `Member`.
- Tests use the behavior they protect: `test_assign_role`, `test_check_role`, `test_lifecycle`.
- Avoid generic modules such as `handlers`, `utils`, `manager`, or `service` unless they describe unavoidable infrastructure.
- Avoid private shorthand in committed docs. The current decision must be written in the repo itself, not left as a pointer to an external reference.

## Architecture notes

Code architecture:

- The Anchor root shape is appropriate and should remain stable.
- The Rust program passes the screaming-architecture test at the instruction layer: each core action is named in Hedwig domain language.
- The state layer is compact and readable: `Org`, `Role`, and `Member` are the right domain nouns.
- A deeper domain-folder split, such as `src/domain/role` or `src/authorization`, is not worth it for a small Anchor program: it would add indirection and could make CPI generation or reviewer navigation worse.

Docs and ADR consistency:

- Decision bodies live in this repo, not in external notes.
- `docs/adr/index.md` is the source-of-truth map for Hedwig-facing ADRs.
- Future ADRs are added to `docs/adr/` with a full decision body, not only a pointer to an external reference.

## Minimal Reorganization Plan

Applied now:

- Add `docs/architecture.md` as the LLM-readable architecture guide.
- Add `docs/adr/index.md` as the repo-local ADR map.
- Add `docs/migration-note.md` as a migration reference.
- Link these docs from `README.md`.

Deferred on purpose:

- Do not rename `programs/hedwig_sol` to `programs/hedwig-sol`; Anchor crate/module naming and existing config expect the underscore.
- Do not split the small instruction set into extra nested folders until the program grows past the current six-instruction core.
- Do not mirror every external note into this repo. Mirror only decisions that affect Hedwig contributors, reviewers, auditors, or LLM agents working on this codebase.
