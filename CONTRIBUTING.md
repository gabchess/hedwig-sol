# Contributing to Hedwig

Hedwig is a small Solana-native roles primitive. Keep changes narrow, auditable, and honest about what is built versus planned.

## Local checks

Run the same checks CI expects before opening a PR:

```bash
cargo fmt --check
cargo build
cargo build-sbf --manifest-path programs/hedwig_sol/Cargo.toml
cargo test
```

The test suite uses LiteSVM and does not require a network connection.

## Repo map

- Program source: `programs/hedwig_sol/src/`
- Program tests: `programs/hedwig_sol/tests/`
- Devnet demo: `app/demo.ts`
- Architecture notes: `docs/architecture.md`
- Threat model and audit status: `THREAT-MODEL.md`
- Roadmap: `ROADMAP.md`

## Contribution rules

- Keep the core instruction set small: `create_org`, `create_role`, `assign_role`, `revoke_role`, `check_role`, and `set_role_enabled`.
- Do not add nested role hierarchies or agent-to-agent delegation to the core program. Build those patterns as wrapper programs.
- Do not treat `check_role` as identity authentication. A consuming program must authenticate the holder before trusting the role check.
- Keep unfinished work in docs or issues, not as TODO/FIXME stubs in source.
- Update README, roadmap, and threat-model claims together when a change affects shipped status, risks, or public guarantees.

## Pull request bar

A useful PR says what changed, why it matters, and which check proves it. If the change touches authorization, include at least one negative test for the failure path.
