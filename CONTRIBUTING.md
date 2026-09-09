# Contributing to Hedwig

Hedwig is a small Solana-native roles primitive. Keep changes narrow, auditable, and honest about what is built versus planned.

## Local checks

Run the full local release checks before opening a PR. This list matches
[`.github/workflows/ci.yml`](.github/workflows/ci.yml). If the two ever drift,
the workflow file is canonical.

```bash
bash scripts/test-public-boundary.sh
bash scripts/check-public-boundary.sh
npm run evals:test
npm run evals:offline
yarn install --frozen-lockfile
yarn lint
yarn sdk:typecheck
yarn sdk:test
yarn sdk:build
npm ci --prefix app
./node_modules/.bin/tsc -p app/tsconfig.json --noEmit
cargo fmt --all -- --check
cargo build --locked --verbose
cargo build-sbf --manifest-path programs/hedwig_consumer/Cargo.toml -- --locked
cargo build-sbf --manifest-path programs/hedwig_sol/Cargo.toml -- --locked
cargo clippy --workspace --all-targets --all-features -- \
  -A clippy::diverging_sub_expression -A clippy::result_large_err -D warnings
cargo test --locked --workspace --verbose
npm audit --package-lock-only --audit-level=moderate --prefix app
cargo audit
```

The tests use LiteSVM and do not require a network connection. Build both SBF
artifacts before the workspace tests because the fixtures load them at compile
time. CI runs every check above, including the SDK and app TypeScript gates
that were local-only before. Build the SDK and install the app's dependencies
before the app typecheck: `app` depends on the SDK through `file:../sdk`, and
`sdk/dist` is not committed. `cargo audit` needs
`cargo install cargo-audit --locked --version 0.22.2` first.

## Repo map

- Program source: `programs/hedwig_sol/src/`
- Program tests: `programs/hedwig_sol/tests/`
- Secure consumer and tests: `programs/hedwig_consumer/`
- Repository-local TypeScript SDK alpha: `sdk/`
- SDK-driven devnet demo: `app/demo.ts`
- Integration guide: `docs/access-control/integration-guide.md`
- Architecture map: `docs/access-control/architecture.md`
- Threat model and known risks: `THREAT-MODEL.md`
- Roadmap: `ROADMAP.md`
- Public boundary and claim checks: `scripts/`, `evals/`
- CI workflow: `.github/workflows/ci.yml`

## Contribution rules

- Keep the core instruction set small: `create_org`, `create_role`, `assign_role`, `revoke_role`, `check_role`, and `set_role_enabled`.
- Do not add nested role hierarchies or agent-to-agent delegation to the core program. Build those patterns as wrapper programs.
- Do not treat `check_role` as identity authentication. A consuming program must authenticate the holder before trusting the role check.
- Preserve the conventional Anchor shell, but name domain code after Hedwig concepts and actions. Avoid generic `utils`, `manager`, or `service` layers.
- Add an abstraction only after a concrete integration demonstrates repeated need. Prefer deletion and direct code when both are equally safe.
- Keep unfinished work in docs or issues, not as TODO/FIXME stubs in source.
- Update README, roadmap, and threat-model claims together when a change affects shipped status, risks, or public guarantees.

## Public repository boundary

Keep this repository useful to an external reader. Product behavior,
integration guidance, security and deployment evidence, current roadmap status,
and grant delivery records belong here.

Keep private operating instructions, assistant configuration, planning
workflows, approval records, prospect work, and unpublished review provenance
outside this repository. Every proper name, link, and status claim must make
sense without access to a maintainer's local environment. Public grant,
adoption, deployment, and security claims must link to public evidence.

Run `yarn public:check` before opening a pull request.

## Pull request bar

A useful PR says what changed, why it matters, and which check proves it. If the
change touches authorization, include at least one negative test for the failure
path. Confirm that the public repository boundary still holds.

`main` requires a pull request and a passing `test` check. GitHub blocks direct
pushes, force pushes, and branch deletion on `main`, including for repository
admins.
