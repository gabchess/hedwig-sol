# Hedwig ADR Index

This is the repo-local, LLM-readable index for architecture decisions that affect Hedwig. It states the active rules directly, so contributors do not need any external context.

Any future contributor or LLM should be able to read this folder and understand the active rules from the repo alone. Do not leave a decision as only a pointer to an external reference; write the decision body here.

## How ADRs Work Here

An ADR records a decision that should shape future code. It is not a meeting note, a changelog, or a task list.

Each ADR should include:

- Context: what pressure forced a decision.
- Decision: the rule we follow.
- Consequences: what gets easier, what gets harder, and what we deliberately reject.
- Code impact: affected paths, naming conventions, and examples.
- Status: proposed, active, superseded, or rejected.

## Active Decisions

| ID | Title | Status | Applies to | Rule |
|---|---|---|---|---|
| 0001 | Human Scaffold Before Agent Fill | Active | Docs, ADRs, architecture notes | Humans define the load-bearing structure before agents fill in detail. |
| 0002 | Separate Oracle Domains | Active | Future integrations | Keep onchain oracle, social oracle, and web-fetch oracle concerns separate until a deliberate bridge exists. |
| 0003 | Code Elegance | Active | All code and docs | The best code is the code never written; keep the core small and defer shortcuts as explicit debt. |
| 0004 | Screaming Architecture For Hedwig | Active | Repo layout and naming | Anchor root stays conventional; program internals and docs must speak the authorization domain directly. |

## ADR 0001: Human Scaffold Before Agent Fill

Status: Active.

Decision: humans define the structure of load-bearing artifacts before agents elaborate them. For Hedwig, that means README, architecture, threat model, roadmap, and ADR structure must be anchored by explicit human intent, not only generated summaries.

Code impact:

- Agents can draft details, but they should not invent new product boundaries.
- The repo should keep a small number of canonical docs instead of many session-fragment notes.
- Architecture docs must make the current decision readable directly, not point only to private memory.

## ADR 0002: Separate Oracle Domains

Status: Active, but mostly future-facing for Hedwig.

Decision: oracle concerns should remain separated by trust boundary. Onchain checks, social assertions, and web-fetched evidence are different systems with different failure modes.

Code impact:

- Hedwig core does not include web, social, or off-chain oracle logic.
- `check_role` proves onchain membership only.
- Eligibility logic and identity assertions live in the consuming program or wrapper.

## ADR 0003: Code Elegance

Status: Active.

Decision: keep the core small enough to audit. If a feature is not required for the primitive, leave it out or move it to a wrapper.

Code impact:

- Flat roles stay in core; hierarchies are out of scope.
- Agent-to-agent delegation stays out of core.
- Pluggable eligibility modules stay out of core.
- Generic helpers should justify their existence. Prefer six clear instruction files over one abstract dispatcher.
- Redundant tests or docs should be removed when they stop protecting behavior.

## ADR 0004: Screaming Architecture For Hedwig

Status: Active.

Decision: Hedwig uses screaming architecture where it creates leverage: inside the program, tests, docs, and public instruction surface. The repository root remains a conventional Anchor project.

Rationale: a Solana reviewer should immediately recognize the project as Anchor, then immediately see that the domain is role-based authorization.

Rules:

- Keep Anchor-standard root folders: `programs`, `migrations`, `app`, `docs`.
- Keep the Rust instruction modules named by domain action.
- Keep account structs named by domain noun.
- Put LLM-readable architecture guidance in `docs/architecture.md`.
- Put repo-relevant ADRs under `docs/adr/`.
- Avoid docs that only point to an external reference; write the decision body here.

Good examples:

- `programs/hedwig_sol/src/instructions/create_org.rs`
- `programs/hedwig_sol/src/instructions/assign_role.rs`
- `programs/hedwig_sol/src/instructions/check_role.rs`
- `programs/hedwig_sol/src/state.rs` with `Org`, `Role`, and `Member`

Bad examples:

- `src/handlers.rs`
- `src/actions/mod.rs`
- `docs/architecture-notes-from-session.md` with only private session references
- `README.md` describing the repo as only "an Anchor program" without the authorization domain

## Folder Convention

The repo should read in two layers:

1. Framework shell: Anchor/ Solana conventional layout.
2. Domain core: Hedwig authorization language.

Current target layout:

```text
hedwig-sol/
  README.md
  THREAT-MODEL.md
  ROADMAP.md
  docs/
    architecture.md
    adr/
      index.md
    sdk-rfc.md
  programs/
    hedwig_sol/
      src/
        lib.rs
        state.rs
        error.rs
        constants.rs
        instructions/
          create_org.rs
          create_role.rs
          assign_role.rs
          revoke_role.rs
          check_role.rs
          set_role_enabled.rs
      tests/
        test_create_org.rs
        test_create_role.rs
        test_assign_role.rs
        test_revoke_role.rs
        test_check_role.rs
        test_set_role_enabled.rs
        test_lifecycle.rs
```

## Review Checklist

Use this checklist before grant, hackathon, or external audit submissions:

- Can a reviewer name the product domain from the file names alone?
- Can an LLM understand the ADRs from the repo alone, with no external context?
- Does every new folder earn its existence?
- Is every decision written out as direct repo prose, not a pointer elsewhere?
- Do tests protect the authorization invariants that docs claim?
- Did `cargo build-sbf --manifest-path programs/hedwig_sol/Cargo.toml` and `cargo test` pass?
