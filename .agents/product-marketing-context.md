# Hedwig pilot context

## Product truth

Hedwig is shared authorization for related Anchor programs: one canonical role store, with each consumer still responsible for authenticating its actor.

The current program has six instructions:

1. `create_org`
2. `create_role`
3. `assign_role`
4. `check_role`
5. `set_role_enabled`
6. `revoke_role`

An `Org` defines the namespace. A `Role` defines one named authority in that namespace. A `Member` records that one pubkey holds the role, with optional expiry. `check_role` lets a consumer verify active membership through CPI.

The first acquisition artifact is a manual authority map. It compares the team's current direct and CPI checks with the smallest path that could use Hedwig.

## Ideal early team

The first team:

- maintains at least two related Anchor programs or one real direct/CPI authorization path;
- repeats or splits authority logic across that path;
- has an audit, launch, migration, or material release within about eight weeks;
- has a technical owner who can make a merge decision in one sprint; and
- can test the current Hedwig core without a new onchain instruction.

## Disqualifiers

Do not target:

- a standalone program with no shared authorization path;
- a project that does not use Anchor;
- a team with no public authority path or reachable technical owner;
- a path that needs role hierarchy, delegation, instruction allowlists, spending caps, or another unbuilt feature;
- an auditor, investor, or ecosystem group when the goal is a product pilot; or
- a builder-owned Hedwig fixture presented as external demand.

## Current proof

- The six-instruction core has 28 LiteSVM integration tests plus one program-ID test.
- The secure reference consumer has 12 LiteSVM integration tests plus one program-ID test.
- The private `@hedwig-sol/sdk` alpha has 27 tests and covers all six instructions.
- The core is live on devnet at `H4J9wWhraK2Zvn4o9aFheFVmAf7nfaBNPw3d7w77X1eC`.
- The reference consumer is live on devnet at `52D3pTYvMwLYbiigY5xg55n4HmtEzTKCEicx1Cojzo9a`.
- A fresh authenticated actor changed consumer-owned state through Hedwig CPI.
- The release records bind reviewed source, artifacts, deployments, and live transactions.

The reference consumer is builder-owned integration evidence. It is not adoption.

## Claims we cannot make

Hedwig has no verified design partner, customer, revenue, production use, independent external audit, or mainnet release.

The SDK is private `0.1.0-alpha.0`, tested locally, and not published.

The current role scope comes from the `Org` and `Role` PDAs. Hedwig does not yet provide resource-level permissions, instruction allowlists, or delegation.

Multisig administration remains a compatibility test. A Squads dry run must prove the relevant signer and proposal path before any public compatibility claim.

Migration support means a documented before/after map, transaction sequence, rollback point, and owner decision. It is not a new onchain migration instruction.

## Seven-day offer

The first qualified team gets a free seven-day pilot for one state-changing path across two programs.

Hedwig provides:

1. a map of the current actors, authority accounts, constraints, CPI edges, administrators, and upgrade authorities;
2. the smallest proposed Hedwig-backed path;
3. scoped and expiring membership where the current product supports it;
4. a multisig custody dry-run plan;
5. migration and rollback steps; and
6. a merge, revise, or reject recommendation.

The team provides repository access for the selected path, one technical owner, timely feedback, and an explicit merge, revise, or reject decision. The pilot creates no obligation to adopt Hedwig. A public name or case study needs separate consent.

## Sender voice

Messages come from Hedwig's builder and speak to another Solana builder.

- Lead with the technical consequence, not praise.
- Use plain language and short sentences.
- State uncertainty directly.
- Offer a useful artifact before asking for time.
- Treat the recipient as a peer who can reject the fit.
- Use the recipient's public working language.
- Avoid sales jargon, fake familiarity, feature lists, and inflated proof.

## Outreach rules

Every message:

- uses one verified hook that could not be sent unchanged to another team;
- treats the authority problem as a hypothesis until the owner confirms it;
- offers to map one real two-program path;
- has one question that can be answered in one line;
- stays below 500 characters;
- contains no attachment, calendar link, meeting request, or markdown link; and
- remains on one channel unless the recipient replies.

Use X when the owner is active there and can receive a message. Use LinkedIn when it has the clearer current profile or contact path. LinkedIn connection requests are blank.

## Approval gate

Research and drafting are approved. Every message is unsent.

The project owner must approve the exact recipient, channel, and copy before outreach. Preparation does not authorize package publication, deployment, submission, spending, the Colosseum Eternal timer, or a product-code change.
