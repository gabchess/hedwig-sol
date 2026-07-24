# Integrating Hedwig

Hedwig answers one narrow question: does this pubkey hold this enabled,
unexpired role? A consumer must first prove that the pubkey represents the actor
it intends to authorize.

## Onchain consumer

Use typed Hedwig accounts and pin the program with
`Program<'info, hedwig_sol::program::HedwigSol>`. Bind the consumer's
authenticated actor to the holder account passed to `check_role`:

```rust,ignore
pub authority: Signer<'info>,
pub member: Account<'info, hedwig_sol::Member>,
pub required_role: Account<'info, hedwig_sol::Role>,
pub hedwig_program: Program<'info, hedwig_sol::program::HedwigSol>,
```

```rust,ignore
let cpi_accounts = hedwig_sol::cpi::accounts::CheckRole {
    member: ctx.accounts.member.to_account_info(),
    role: ctx.accounts.required_role.to_account_info(),
    holder: ctx.accounts.authority.to_account_info(),
};

hedwig_sol::cpi::check_role(CpiContext::new(
    ctx.accounts.hedwig_program.key(),
    cpi_accounts,
))?;
```

The `?` is part of the authorization boundary. It aborts the consumer
instruction when Hedwig rejects a disabled, expired, revoked, or mismatched
membership.

The compiling reference is
[`programs/hedwig_consumer/src/lib.rs`](../programs/hedwig_consumer/src/lib.rs).
Its LiteSVM suite covers the successful path, actor substitution, missing
signature, wrong role, expired and disabled membership, revocation, a substituted
Hedwig program, and a wrong-owned role account.

The same consumer is deployed on devnet at
`52D3pTYvMwLYbiigY5xg55n4HmtEzTKCEicx1Cojzo9a`. The live proof in
[`app/consumer-demo.ts`](../app/consumer-demo.ts) creates a fresh authenticated
actor, grants that actor a role, initializes a consumer counter, and increments
it through Hedwig CPI. The
[promotion record](audits/2026-07-24-consumer-devnet-integration.md) contains
the artifact hash, deployment transaction, every proof transaction, and final
account state.

Do not accept an arbitrary holder pubkey from instruction data and pass it to
Hedwig. Membership is a claim about a pubkey, not proof that the transaction
caller controls it.

## TypeScript SDK

The private `@hedwig-sol/sdk` alpha exports:

- fixed program ID and generated IDL types;
- org, role, and member PDA helpers;
- argument validation; and
- build/send pairs for all six instructions.

It is a repository-local package and has not been published to npm. Build it
from the repository root:

```bash
yarn sdk:build
```

The app resolves it through `file:../sdk`. The SDK never creates keypairs and
requires each sender to receive its signer array explicitly. See
[`sdk/README.md`](../sdk/README.md) for the API and
[`app/README.md`](../app/README.md) for the devnet lifecycle.

## Local verification

Both SBF artifacts must exist before the workspace tests run:

```bash
cargo build-sbf --manifest-path programs/hedwig_sol/Cargo.toml
cargo build-sbf --manifest-path programs/hedwig_consumer/Cargo.toml
cargo test --workspace
yarn sdk:typecheck
yarn sdk:test
./node_modules/.bin/tsc -p app/tsconfig.json --noEmit
```

The consumer is a live builder-owned reference integration. It does not count
as an external design partner, recurring use, or production evidence.
