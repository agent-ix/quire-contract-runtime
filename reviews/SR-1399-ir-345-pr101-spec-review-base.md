---
id: "SR-1399"
title: "IR-345 spec review (base): AD-004 owner answers recorded (PR 101)"
type: SpecReview
analysis: base
review_set: subset
scope: "agent-ix/quire-contract-runtime@e3c517ccc56eab1d72c6e797456537508ebfeec1; spec/assurance/AD-004-runtime-crate-layout.md (git diff origin/main...HEAD only); owner answers read on Linear IR-349, IR-346, IR-582, IR-583"
relationships:
  - target: ix://agent-ix/quire-contract-runtime/AD-004
    type: references
---

# SR-1399: IR-345 base review of PR 101

## Summary

Ticket: IR-345. PR 101 records the owner's answers to Q-1 to Q-4 and Q-6 in AD-004 and gates the
migration steps on IR-582 and IR-583. I read the owner answers myself on Linear. The planner's
comment on IR-349 (2026-10-04T15:12Z) gives the answers to Q-1 to Q-4. The comments on IR-346 and
IR-582 (2026-10-04T14:46Z) give the answer to O-1/Q-6, "Extract both". I compared each Answer cell
with those comments.

Measured clean:

- Q-1, Q-2, Q-3, Q-4 and Q-6 say what the recorded answers say.
- The Linear relations hold: IR-582 and IR-583 both block IR-349, read with `linear api`.
- Q-4 says "66 attributes in `src/exact`". That count is 66; there are 6 more in `snapshot_json`,
  `observation` and `verdict`, which makes AD-003's 72.
- Five kept `scalar` enums carry `#[non_exhaustive]`: `IntegerDivisionConsumer`,
  `IntegerDivisionDisposition`, `ShortCircuitConnective`, `IeeeUnsupportedCause` and
  `IeeeDisposition`.
- The file is complete. Its headings match main's, except that "Open questions" is renamed. No line
  is duplicated, and the relationships are unchanged and resolve.
- The AD cites no SHA and no CG line number.
- `quire validate` passes 98 of 98 docs. `quire coverage --strict` reports 56 unbacked and 5
  contradicted, and its sorted output is identical to main's.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The Q-5 row cites an owner answer that was never given: "open, routed to QSL (IR-349 answer)". The owner's answer comment on IR-349 covers Q-1 to Q-4 only. No comment on IR-349, IR-345, IR-346, IR-582 or IR-583 records any owner statement on Q-5. The preamble itself says IR-349 holds Q-1 to Q-4. Q-5 was always QSL's question, so the routing is fine, but citing "IR-349 answer" invents an owner source. Drop the citation, and say that Q-5 was not put to the owner on 2026-10-04 and stays open with QSL. | AD-004 "Owner answers and open questions" Q-5 row |
| FND-002 | low | The end state now has three shared crates, but several places still say two. Decision C: "the feature is what pulls in the two crates". Decision D: "Dependency spelling is FR-275's, extended to `quire-semantic-value`", where FR-275's spelling names the QSL repository and the evaluation leaf is not covered. The no-shim bullet in the end state and L-1 forbid a `pub use` of `quire_exact` or `quire_semantic_value` only, so a re-export of the leaf is not excluded, and consumers are told to import only those two paths. Step 5 adds the leaf dependency without the `deny.toml` `allow-git` entry for the leaf's repository; the edge table gives that crate a licence exception only. Step 5's CG column also leaves out the emitted manifest naming the leaf. | AD-004 Decisions C, D; end state no-shim bullet; L-1; edge table; Migration step 5 |

## Verdict

**Mergeable after FND-001 is fixed.** It is a wording fix, but a recorded decision must not cite an
owner answer that does not exist. Fix FND-002 in the same round. See SR-1400 to SR-1402 for the
other findings.

## Dispositions

Disposition pass 1. I reviewed agent-ix/quire-contract-runtime at
7b25ebfddd762e668ca4f2d6fbcb976b58a225bf (fix commit 7b25ebf) and re-measured every item myself.

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 7b25ebf. The Q-5 row now reads "open, routed to QSL by this lane; no owner answer is recorded (the IR-349 answer covers Q-1 to Q-4 only)". The preamble says the same. A grep finds "IR-349 answer" only in the Q-1 to Q-4 rows, and the Linear record supports each of those. |
| FND-002 | fixed | 7b25ebf. Decision C now says "the shared crates", and Decision D covers "each of the three shared crates", each naming its own repository, with step 2 amending FR-275's wording. The no-shim bullet and L-1 both cover the evaluation leaf. Step 5 adds the leaf's `deny.toml` `allow-git` entry and licence exception, and its CG column now has the emitted manifest name the leaf. Step 3 still says "manifest names the two crates", which is correct, because the leaf only arrives in step 5. |
