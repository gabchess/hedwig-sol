# Founder-led pilot preparation

Date: 2026-07-29

## Decision

Hedwig is shared authorization for related Anchor programs: one canonical role
store, with each consumer still responsible for authenticating its actor.

The acquisition proof is a manual before/after authority map. The product proof
is one signed seven-day pilot for a real state-changing path across two related
programs. The end state is merge, revise, or reject.

Sonda's preregistered verdict remains `AGUARDAR MAIS EVIDÊNCIA`. Hedwig has no
verified customer conversation or demand signal. No feature work starts before
one qualified team signs the pilot.

## Qualification method

The private account screen uses a fixed 100-point rubric:

| Dimension | Points |
| --- | ---: |
| Two-program or direct/CPI architecture fit | 30 |
| Public pain evidence | 25 |
| Current timing signal | 20 |
| Reachable technical owner | 15 |
| Seven-day tractability | 10 |

Unknown facts score zero. Public code supports a hypothesis, not proof of pain.
Candidates fail the screen when they lack Anchor source; have neither two
related Anchor programs nor one clear direct/CPI authorization path; or lack a
public authority surface, a reachable owner, timing evidence, or fit with
Hedwig's current six-instruction core.

## Prepared output

- 15 research-qualified accounts scored from public primary sources
- five detailed research cards
- five message pairs (five primary messages and five follow-ups), all under
  500 characters
- zero messages sent
- zero external conversations
- zero signed pilots

Prospect names, handles, contact details, message copy, and response state stay
in the private project vault.

The private operating pack contains:

- an authority-map interview;
- seven-day pilot terms;
- a multisig and authority migration worksheet;
- an SDK release checklist;
- an outreach and response ledger; and
- a Colosseum Eternal readiness gate.

## Outreach workflow provenance

The shared `reach-out` skill was installed unchanged from
`swan-gtm/gtm-skills` commit
`4ac068ae577a24bc2de32a69c014e137e6ec7d96`. Both supported local runtimes
resolve to the same installation, and its pinned file hashes match. Its
targeted-outreach rules produced the staged sequence: one verified hook, one
hypothesis, one concrete offer, one answerable question, and manual approval
before send.

## Authority gates

- Outreach: denied until the project owner approves each recipient, channel, and exact copy.
- SDK publication: not approved.
- Deployment or upgrade-authority change: not approved.
- Colosseum Eternal stopwatch: `NOT STARTED`.
- Colosseum submission: not approved.
- Product code change: outside this preparation scope.

## Verification

The preparation gate checks:

```bash
rg -c '^\| P[0-9]{2} \|' 01-Prospect-Scorecard.md
rg -c '^### P[0-9]{2}:' 02-Top-Five-Research.md
rg -c '^> ' 03-Staged-Messages.md
awk '/^> / { if (length(substr($0,3)) >= 500) exit 1 }' 03-Staged-Messages.md
git diff --check
```

Observed result: 15 score rows, five research cards, ten message lines, no
message at or above 500 characters, and no whitespace errors.

Source review rejected two initial top-five hypotheses because they joined
different actors or instructions into one path. Those records moved to hold;
two candidates with directly cited state-changing CPI authorization paths
replaced them. One non-Anchor record also left the qualified account set.

Repository verification also passed:

- `cargo fmt --all -- --check`;
- `cargo build` and 42 Rust tests across the workspace;
- SBF builds for the Hedwig core and consumer programs;
- repository-configured JavaScript and TypeScript Prettier checks;
- SDK build, typecheck, and 27 tests;
- demo app typecheck;
- private-name, secret-shape, claim, and whitespace scans; and
- a branch diff check confirming no product-code or deployment change.

Final evidence, standards, and security reviews reported no unresolved P0,
P1, or P2 finding.

The pull request will bind the final review result to its merged revision
without exposing the private prospect pack.

## Next decision

The project owner can approve, revise, or hold the five staged message pairs.
Approval of one pair does not approve the other four, start the Eternal timer,
publish the SDK, deploy code, or change custody.
