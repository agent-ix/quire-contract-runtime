---
id: "SR-1402"
title: "IR-345 spec review (scope-boundary): AD-004 Interim paragraph and owner-answer boundary (PR 101)"
type: SpecReview
analysis: scope-boundary
review_set: subset
scope: "agent-ix/quire-contract-runtime@e3c517ccc56eab1d72c6e797456537508ebfeec1; spec/assurance/AD-004-runtime-crate-layout.md (What this crate keeps, Interim, end state, Owner answers); read against FR-275 Open questions, SR-1398 FND-003, Linear IR-349 owner answers"
relationships:
  - target: ix://agent-ix/quire-contract-runtime/AD-004
    type: references
  - target: ix://agent-ix/quire-contract-runtime/FR-275
    type: references
---

# SR-1402: IR-345 scope-boundary review of PR 101

## Summary

Ticket: IR-345. The Interim paragraph grants no exception and sets no expiry. It asks for neither,
and it requests no vendoring approval. It records the ported items as the defect FR-275 already
records: AC-13 is unmet and the residue list is non-empty. The end state says "no keep, no
exception, no copy without an expiry", which fits the owner's answer to Q-1.

Two statements are the AD's own design choices, and neither is attributed to the owner:

- deletion "in step 5, in the PR that adds the dependency on the leaf", which follows from
  Decision B;
- Q-4's "the 6 sites outside `exact` are not part of this answer". The sites are measured, and
  the owner's question was framed as "RT 66 sites".

The interim policy is a different matter.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | This regresses SR-1398 FND-003. The Interim paragraph now states an interim policy as settled fact: "Until IR-583 lands the items stay in the tree ... steps 3 and 4 only repoint what they must". PR 100 cited FR-275's open owner question ("how the runtime handles its residue copy until QSL-358 lands (open, owner decision) ... no interim policy is stated") and offered early deletion as the alternative. This PR drops both. The owner's Q-1 answer on IR-349 is "QSL puts evaluation in a no_std+alloc leaf crate ...; RT depends on it and deletes its ported copy". It chooses (a), but it does not rule on interim handling or on deleting early. Keeping the copy until IR-583 lands is a fair reading, but it is the AD's inference. Step 1 still says it amends "the Open question on the interim copy", and no owner answer exists to amend it to. Label the interim handling as the AD's reading of the Q-1 answer, and keep the FR-275 open question cited. Or record that the owner ruled on it, with the source. | AD-004 "What this crate keeps" Interim; Migration step 1; FR-275 Open questions |

## Verdict

**Fix FND-001 before merge.** It is wording only. Apart from this, the keep/delete boundary is
unchanged from PR 100 and stays compliant.

## Dispositions

Disposition pass 1. I reviewed agent-ix/quire-contract-runtime at
7b25ebfddd762e668ca4f2d6fbcb976b58a225bf (fix commit 7b25ebf).

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 7b25ebf. The Interim paragraph quotes FR-275's open owner question again. It says the owner's Q-1 answer "says nothing about interim handling or early deletion". It labels the rest "this AD's inference from that answer, not a ruling", and restores early deletion as the alternative, with its CG cost. It grants no exception, sets no expiry and asks for neither. Step 1 leaves the open question open. |

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-002 | low | Group 3 says the leaf's "size and shape (a relocated mechanism, or QSL's own evaluator) are being put to the owner". I found no record of this. IR-583 has no comments. IR-349 and IR-345 have none since the owner's answers at 2026-10-04T15:12Z except review comments. IR-583's description leaves the surface to "Read RT AD-004". The sentence decides nothing, and "not decided here" is right. But it asserts an owner process the AD cannot cite. Reword it to say the surface is not yet stated by IR-583 and is not decided here, or cite the record once one exists. This is a wording fix only. | AD-004 end state Group 3 |

### Dispositions, round 2

Disposition pass 2, reviewed at agent-ix/quire-contract-runtime@3e96b68bb65abe9fa4f1e9f9bf679b2734194bda (fix commit 3e96b68).

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-002 | fixed | 3e96b68. Group 3 now reads "What they are amended to depends on the surface IR-583 states. IR-583 has not yet stated it, and this AD does not decide its size or shape". This matches the record: IR-583 has no comments, and its description defers the surface to AD-004. A grep for "being put" or "put to the owner" finds nothing left in the AD. |
