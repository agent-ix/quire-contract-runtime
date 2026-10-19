---
id: SR-13022
title: "IR-878 RT PR 132 EARS review: FR-277 and FR-278 statements"
type: SpecReview
analysis: ears-conformance
scope: "agent-ix/quire-contract-runtime@300924158cacde4428d59de719497fdbd56b7b37; spec/exact/functional/FR-277-checked-equality-evaluation.md, spec/exact/functional/FR-278-runtime-checked-invariant-causes.md"
review_set: subset
---

## Summary

Ticket: IR-878. Both new requirement statements are ubiquitous EARS: one subject, one SHALL, and a testable response. The AC cells are verbatim re-homes and were not re-graded here, because the planner ruling forbids changing their text.

## Examined criteria

- `FR-277` statement: "The runtime SHALL execute checked equality through its real `CheckedEquality::evaluate` implementation in `src/exact/equality.rs`, consuming the quire-exact planned relation and `Meter` effects."
- `FR-278` statement: "Every runtime checked-invariant producer SHALL emit the precise shared quire-exact `CheckedInvariantCause` assigned to its condition, retaining nested `IllTypedCause` payloads."

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Verdict

**PASS**: both statements conform to the ubiquitous EARS pattern. The truth of FR-277's statement against current code is covered in SR-13020 FND-001.
