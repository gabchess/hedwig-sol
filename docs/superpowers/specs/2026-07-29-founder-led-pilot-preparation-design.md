# Founder-led pilot preparation

**Status:** Approved on 2026-07-29

**Product gate:** Prepare one real seven-day pilot before adding Hedwig features

**Authority:** Research, draft, document, and commit. Do not contact prospects, start the Colosseum Eternal timer, deploy, or change the onchain program.

## Context

Hedwig has a reviewed six-instruction core, a secure reference consumer, a private TypeScript SDK alpha, and reproducible devnet evidence. The reference consumer proves that the integration works. It does not prove that another team needs it.

The next question is narrower: will one qualified Anchor team use Hedwig as the canonical role store for a real authorization path across two related programs?

The first acquisition artifact is a manual authority map. It records the current direct and CPI checks, then shows the smallest Hedwig-backed alternative. This work must test the product thesis before it creates another feature request.

## Decision

Run a founder-led, repo-evidence-first preparation pass:

1. Find 15 Anchor teams with public evidence of a relevant authorization path.
2. Score the teams against one fixed qualification rubric.
3. Research the five strongest accounts and one technical owner per account.
4. Draft one personal X or LinkedIn message and one follow-up for each owner.
5. Prepare the authority-map interview, pilot terms, migration worksheet, SDK release checklist, and response ledger.
6. Stage everything for manual review. Send nothing.

Warm introductions from auditors or Solana ecosystem operators may support the motion, but introducers do not count toward the 15 research-qualified teams.

## Success criteria

The preparation stage is complete when:

- 15 accounts have scores, evidence links, a named technical owner, and a recorded disqualification risk;
- the top five score at least 70 out of 100 and each has a hook that could not be sent unchanged to another team;
- five primary messages and five shorter follow-ups are ready for manual approval;
- the authority-map interview and seven-day pilot terms can be used without writing new product code;
- the non-public research pack records the private research and the public repository records the gate without exposing prospect names; and
- the next action is a clear manual send, revise, or stop decision.

Preparation does not count as customer validation. No public document may claim contact, interest, adoption, partnership, revenue, or production use until the evidence exists.

## Qualification model

A qualified account should meet all five conditions:

1. It maintains at least two related Anchor programs or one clear direct/CPI authorization path.
2. Public code or documentation suggests duplicated, fragmented, or migration-sensitive authority logic.
3. A public signal points to an audit, launch, migration, or material release within about eight weeks.
4. A technical owner is reachable and appears able to approve an integration in one sprint.
5. The candidate path can use Hedwig's current primitive without a new onchain instruction.

Disqualify an account when the evidence shows a single unrelated program, no Anchor path, no reachable technical owner, no credible timing signal, or a dependency on unbuilt Hedwig behavior.

### Score

| Dimension | Points | Evidence |
| --- | ---: | --- |
| Architecture fit | 30 | Two related programs, direct/CPI path, or shared operator boundary |
| Pain evidence | 25 | Repeated checks, separate authority stores, manual migration, or incident-sensitive revocation |
| Timing | 20 | Audit, launch, migration, active build cycle, or current authorization work |
| Buyer access | 15 | Named maintainer, founder, security lead, or senior protocol engineer with an active profile |
| Seven-day tractability | 10 | One path can be mapped and tested without protocol redesign |

Scores must cite public evidence. Unknown facts receive zero points rather than an optimistic estimate.

## Prospect record

Each account record contains:

- team and repository links;
- relevant programs and the suspected authorization path;
- the exact public evidence for every score;
- a one-sentence pain hypothesis, marked as a hypothesis;
- one technical owner, role, X profile, LinkedIn profile, and preferred channel;
- one account-specific hook;
- the smallest useful authority-map offer;
- risks, missing facts, and the reason to disqualify if research fails; and
- preparation status: candidate, qualified, top five, drafted, held, or rejected.

Research stops once the record can support or reject a pilot invitation. The output is a decision record, not a company dossier.

## Manual authority map

The map covers one state-changing path across two programs. For each relevant instruction, it records:

- the actor and how the consumer authenticates that actor;
- the authority account, signer, PDA, or constraint checked today;
- every CPI edge and the accounts passed across it;
- the administrator and upgrade authority;
- scope, expiry, revocation, and incident behavior; and
- the migration step and rollback point.

The proposed map shows where Hedwig's `Org`, `Role`, and `Member` state would become canonical. It must keep the consumer responsible for authenticating its actor. A Hedwig membership proves role membership; it does not prove control of the holder or grant transaction authority.

The before/after result ends with one recommendation:

- merge the current Hedwig integration;
- revise a bounded part of the path, including a narrower experiment; or
- reject or defer the fit because the product or migration risk fails.

## Seven-day pilot

The pilot is free for the first qualified team. It covers one real two-program path:

1. Map the current authority flow.
2. Confirm the trust boundaries and duplicated checks.
3. Design the smallest Hedwig-backed direct/CPI path.
4. Use role scope and membership expiry where the existing product supports them.
5. Validate multisig custody and document migration and rollback.
6. Produce a before/after map and a merge, revise, or reject recommendation.

The team provides the relevant repository access, one technical owner, timely feedback, and an explicit merge, revise, or reject decision. An anonymized case study requires separate consent. The pilot creates no obligation to adopt Hedwig.

Multisig support remains a claim to test. The preparation pack must distinguish current program behavior, a documented migration procedure, and any compatibility that still needs a Squads dry run.

## Outreach design

Messages come personally from Hedwig's builder.

Use X when the owner is active there and can receive a message. Use LinkedIn when it offers the clearer current profile or contact path. Do not chase a person across channels after an ignored request.

Every primary message:

- stays below 500 characters;
- reacts to one verified detail;
- states one plausible authorization problem without presenting it as known fact;
- offers a concrete manual authority map;
- uses one low-friction question; and
- avoids a feature list, calendar link, attachment, generic praise, or fake familiarity.

The ask is whether the owner wants the map, not whether they have time for a generic call. A follow-up is shorter, stands alone, and adds one useful angle. LinkedIn connection requests are blank.

Messages remain staged until the project owner approves the exact recipient, channel, and copy. Sending authority is separate from research, drafting, committing, and publishing authority.

## Outreach workflow source

Use the MIT-licensed `reach-out` skill from `swan-gtm/gtm-skills`, pinned to upstream commit `4ac068ae577a24bc2de32a69c014e137e6ec7d96`.

Import only the `reach-out` directory and its references. Preserve the upstream source and license. Store Hedwig's ICP, sender voice, product proof, and approval rules outside the imported files so upstream updates remain reviewable.

Do not import `build-list`, `research`, `score`, or `outreach-execution` for this stage. Their current procedures assume Swan-specific CRM, enrichment, sender, or memory systems that Hedwig does not use.

Install the selected skill in the shared global skill root only after the written spec and execution plan pass review. Verify discovery in both supported local runtimes. If one runtime needs a compatibility link, add only that link and document it.

## Documentation split

Private prospect and contact data belongs in the non-public research pack. It will contain:

- the 15-account scorecard and evidence;
- the top-five research cards;
- staged messages and follow-ups;
- the authority-map interview;
- the seven-day pilot terms;
- the multisig migration worksheet;
- the public SDK release checklist; and
- a response ledger with dates, approvals, replies, rejection reasons, and next action.

The public repository will record:

- that pilot preparation is the active product gate;
- the qualification method and proof standard;
- which preparation artifacts exist;
- the current no-contact and no-adoption status; and
- links to public integration, migration, SDK, and devnet evidence.

Public files must not expose prospect names, contact details, private replies, repository access, or unsupported partner claims.

## Decision gates

The following actions need separate approval:

| Action | Current authority |
| --- | --- |
| Research public teams and contacts | Approved |
| Draft and revise internal outreach | Approved |
| Install the pinned outreach skill | Pending written-spec and plan review |
| Commit preparation documents | Approved within the reviewed plan |
| Push a branch or open a pull request | Pending plan review |
| Send any X or LinkedIn message | Not approved |
| Publish the SDK or another package | Not approved |
| Start the Colosseum Eternal timer | Not approved |
| Change Hedwig product code | Blocked until a qualified pilot signs |
| Deploy or change upgrade authority | Not approved |

## Validation and kill rule

After the project owner approves sending, the first outreach week targets:

- 15 research-qualified contacts;
- five discovery calls;
- three completed manual authority maps; and
- one written seven-day pilot.

If no qualified team signs by the end of that week, stop adding Hedwig features. Record rejection reasons, separately decide whether to publish the existing SDK and primitive, and decide whether to narrow, reposition, or stop the pilot motion. An internal fixture or builder-owned consumer cannot satisfy this gate.

The Colosseum Eternal timer starts only after the list exists, outreach is approved, the first qualified call is booked, the SDK release path is clear, and governed-custody feasibility has a testable plan.

## Risks

- Public code can suggest pain but cannot prove it. Treat the authority problem as a hypothesis until the owner confirms it.
- A high score can hide a slow buyer. Buyer access and one-sprint authority remain separate score dimensions.
- The pilot can turn into consulting. Hold scope to one state-changing path across two programs.
- Multisig wording can outrun evidence. Label untested custody behavior and require a dry run before a compatibility claim.
- Preparation can look like progress without contact. Public status must say "prepared," never "validated."
- Private research can leak into grant evidence. Publish the method and aggregate status, not the prospect list.

## Sources

- Hedwig public `main` at merge commit `0373f6cdfed857e73c4bc2a4261a47bb305b96b9`.
- [`reach-out` source](https://github.com/swan-gtm/gtm-skills/tree/4ac068ae577a24bc2de32a69c014e137e6ec7d96/skills/ido-goldberg/reach-out), MIT licensed.
- [Colosseum Eternal](https://colosseum.com/eternal), reviewed 2026-07-29. Starting the four-week sprint remains outside current authority.
