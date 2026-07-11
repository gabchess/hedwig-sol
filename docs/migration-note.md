# Architecture And ADR Migration Note

Branch: `architecture-adr-screaming-alignment`

## What Changed

This branch does not reorganize the Rust code. The code already expresses Hedwig's domain clearly at the right layer:

- `Org`, `Role`, and `Member` in `state.rs`
- `create_org`, `create_role`, `assign_role`, `revoke_role`, `check_role`, and `set_role_enabled` in `instructions/`
- instruction-named tests in `programs/hedwig_sol/tests/`

The branch instead moves the missing architecture context into the repo:

- `docs/architecture.md` explains domain boundaries, folder conventions, and the current screaming-architecture audit.
- `docs/adr/index.md` consolidates the prior ADR guidance into a repo-local decision index.
- `README.md` links the new architecture and ADR docs.

## Why This Is Minimal

Renaming or nesting the current Rust modules would increase review risk without improving clarity. Anchor reviewers expect the framework layout, and the current six-instruction core is small enough that extra folders would add ceremony.

The real gap was documentation portability: too much taste and architecture guidance lived in external references. This branch makes those decisions legible to any future LLM, contributor, grant reviewer, or auditor.

## Follow-Up Rule

Any future note that affects Hedwig's architecture should land in `docs/adr/` as a real decision body. It can cite external provenance, but it cannot require external notes to understand the rule.
