---
id: SR-35036
title: "spec-review of IR-499 FR-006/TC-016 evidence notes (quire-contract-runtime PR 166)"
type: SpecReview
analysis: base
review_set: subset
scope: "agent-ix/quire-contract-runtime@ce96ab304819f3bf37abe6ce4c31cc1fc69393d0; spec/exact/functional/FR-006-exact-outcomes-and-accounting.md; spec/exact/matrix/TC-016-exact-outcome-envelope.md; spec/exact/matrix/tests.md; base agent-ix/quire-contract-runtime@4e639dcb7f576a029d06a8ec5b5b94e55f34635c"
relationships:
  - target: ix://agent-ix/quire-contract-runtime/FR-006
    type: references
  - target: ix://agent-ix/quire-contract-runtime/TC-016
    type: references
---

# Spec review of PR 166's FR-006 / TC-016 / matrix edits

## Summary

Ticket: IR-499. The spec diff edits prose only: the FR-006 `## Kernel ownership` note, the
TC-016 `## Ownership and evidence` section, and two status cells in `spec/exact/matrix/tests.md`
(FR-006-AC-1 row to ✅; TC-016 row to 🚧 with AC-1 tagged and AC-3 open). No requirement
statement, AC, TC procedure, expected result or relationship changes, so EARS, integrity,
object and dependency sub-analyses have nothing in scope; only the base review ran.
`quire validate --scope .` on the three documents passes at head.

Examined:

- FR-006-AC-1 (unchanged text) against the new evidence note: the note's claims (runtime
  Boolean path, both completed values distinct from the three stops, exhaustive match of the
  consumed kernel reason types, `Incomplete` is a struct) agree with the test.
- TC-016 procedure steps 1 to 3 and expected results (unchanged): step 1 is met by the test;
  step 3 (AC-3 spelling round trip) is stated as open in both the TC and the matrix row.
- Matrix FR-006-AC-1 ✅ and TC-016 🚧 cells: consistent with each other and with FR-006's note.
  The AC-3 row (line 16) stays 🚧 and truthful in prose; its computed backing is SR-35035
  FND-001.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | TC-016 step 2 requires confirming "the runtime exposes no separate reason carrier", and FR-006-AC-1 ends "The runtime consumes these kernel types directly under FR-275". The new FR-006/TC-016 notes and the ✅ matrix cell cite only the Boolean-evaluator test, which shows the kernel `Outcome` on that one entry point. Nothing records how the whole-runtime "no separate carrier" clause was confirmed (for example, that `Evaluation.outcome` and `Body` use `quire_exact::Outcome` and `InputRefusal`/`EvaluationRefusal` are pre-evaluation input refusals under AC-2). Add one sentence naming that inspection so the ✅ does not read as covering more than the test shows. | spec/exact/matrix/TC-016-exact-outcome-envelope.md:34; spec/exact/matrix/tests.md:13 |

## Verdict

The spec edits are truthful and narrow no criterion. TC-016 stays open for AC-3. One low
finding: the ✅ for AC-1 should say how its "consumes directly / no separate reason carrier"
clause was confirmed beyond the single entry point the test drives. Not blocking on its own.

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Repair head 86275e32 adds to spec/exact/matrix/tests.md (Evidence notes) an inspection sentence: the Boolean evaluator, `Body` and `Evaluation.outcome` use `quire_exact::Outcome` and its `Refusal` reasons directly, and `CheckRefusal`, `InputRefusal` and `EvaluationRefusal` report checking or pre-evaluation input rejection. Verified against `src/scalar/boolean.rs` and `src/exact/expression.rs` (`CheckRefusal` is returned only by `PackageDeclarations::check`). The line 13 status is now 🚧, so it no longer reads as complete. |
