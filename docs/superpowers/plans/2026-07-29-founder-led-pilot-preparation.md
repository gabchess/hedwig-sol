# Founder-led pilot preparation plan

**Status:** Implemented and verified on 2026-07-29

**Approved scope:** Prepare the first Hedwig pilot motion. Do not contact prospects, publish the SDK, start the Colosseum Eternal timer, deploy, upgrade, or change program authority.

## Verifiable end state

This stage is complete when:

- the public repository states that one signed seven-day pilot is the active product gate;
- fifteen private prospect records use one fixed evidence rubric;
- five private research cards and five message pairs are ready for manual review;
- the authority-map, pilot, migration, SDK-release, response-ledger, and Eternal-readiness templates are usable;
- public files contain no prospect identity, contact detail, private workspace path, or unsupported adoption claim;
- the Rust workspace, both SBF programs, TypeScript SDK, app typecheck, and configured lint pass; and
- evidence, standards, and security reviewers report no unresolved P0, P1, or P2 finding.

## Implementation

### 1. Freeze the product and action boundary

Record the decision in a public design note before implementation:

- one real authorization path across two related Anchor programs;
- one canonical role store with direct and CPI-safe checks;
- scoped and expiring membership;
- multisig administration and migration support;
- audit-readable state and a manual before/after authority map;
- no new product feature until the pilot demand gate moves.

Keep research, drafting, documentation, and repository work separate from send, publish, timer, deployment, and authority-changing actions.

### 2. Install the outreach workflow

Import only the MIT-licensed `reach-out` skill from `swan-gtm/gtm-skills`, pinned to commit `4ac068ae577a24bc2de32a69c014e137e6ec7d96`.

Install the source once in the shared skill root. Add only the links needed for the two supported local runtimes. Preserve the upstream license and verify the pinned file hashes and both resolved links.

Keep Hedwig-specific context outside the imported skill so later upstream updates remain reviewable.

### 3. Prepare the public pilot offer

Add one public pilot page that defines:

- the team and path qualification gates;
- the free seven-day scope;
- the authority-map deliverable;
- the merge, revise, or reject outcome;
- the team's required access and decision owner;
- the claims Hedwig cannot make before evidence exists; and
- the actions that still need separate approval.

Update the public README, roadmap, threat model, SDK guide, app guide, grant log, and dated progress record to use the same product gate and proof standard.

### 4. Build the private evidence pack

Use primary sources to score fifteen candidates on architecture fit, pain evidence, timing, buyer access, and seven-day tractability. Unknown facts score zero.

For each candidate, record:

- repository and program evidence;
- the observed authority behavior;
- a clearly marked pain hypothesis;
- the smallest useful map;
- one current technical owner and personal contact route;
- a timing signal;
- the disqualification risk; and
- the preparation state.

Research the five strongest candidates in depth. Draft one primary message under 500 characters and one shorter follow-up for each. Each draft must cite one team-specific code hook and ask one answerable question. Mark all drafts `NOT SENT`.

Prepare reusable private templates for:

- the authority-map interview;
- seven-day pilot terms;
- multisig migration and rollback;
- SDK release;
- outreach evidence;
- Colosseum Eternal readiness.

### 5. Verify the evidence and privacy boundary

Before sync or publication:

1. Recalculate every score.
2. Confirm the five selected research cards match the scorecard.
3. Confirm the ten stored character counts.
4. Re-read every cited direct or CPI path in source.
5. Remove claims that join separate code paths or infer shared operators without proof.
6. Scan the public diff for prospect names, contact details, private paths, credentials, and unsupported customer claims.
7. Run evidence, standards, and security reviews until no P0, P1, or P2 finding remains.

Private research belongs in the internal project vault. Public documentation may state the method and preparation count, but not the identities or draft copy.

### 6. Run the software gates

Run:

- Rust formatting, build, and workspace tests;
- SBF builds for the Hedwig program and reference consumer;
- SDK build, typecheck, and tests;
- app typecheck;
- the repository's configured JavaScript and TypeScript lint;
- diff checks that prove no product code changed.

Do not bulk-format unrelated Markdown.

### 7. Publish the public record

Create one clean documentation-only commit and a ready pull request. The pull request must state:

- why pilot evidence is the current gate;
- what preparation artifacts now exist;
- that identities and messages remain private;
- that no outreach, customer validation, SDK publication, timer start, deployment, or program change occurred; and
- the exact verification evidence.

Review the final pull-request diff, checks, comments, and approvals. Merge only when the expected head revision is still current. Keep the remote branch.

### 8. Close the internal evidence loop

After merge, record the pull-request URL and merge revision in the non-public project record. Confirm its repository is clean after sync.

The next human decision remains separate: approve, revise, or hold the exact five private message pairs. No message is sent by this plan.
