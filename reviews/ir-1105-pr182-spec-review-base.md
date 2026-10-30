---
id: SR-568102
title: "IR-1105 RT PR 182 base specification review"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-runtime; spec/exact/functional/FR-012-carried-compiler-vocabulary.md, spec/exact/matrix/TC-033-carried-compiler-vocabulary.md, spec/exact/matrix/tests.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-runtime/FR-012
    type: reviews
  - target: ix://agent-ix/quire-contract-runtime/TC-033
    type: reviews
---

## Summary

Ticket: IR-1105. Reviewed PR #182 at 4196e88 for unchanged AC scope, executable evidence, and honest open status. TC-033 gives the producer-origin test to quire-integration and leaves the RT fixture migration open.

## Verdict

FAIL pending FND-001. The test-lane move is coherent, but FR-012-AC-2 must retain its universal admitted-identity antecedent. The QSL producer and reader path belong in the verification procedure, not the criterion's precondition.

## Examined scope

- FR-012-AC-2 (examined): declaration, foreign-universe, and declared/compound identity comparison outcomes.
- FR-012-AC-8 (examined): no semantic identity construction, fixed oracle, copied fixture, or replacement pin in RT; producer-origin evidence remains open.
- FR-012-AC-1, AC-3 through AC-7 (context only): retired parser and unchanged vocabulary, graph and inspection obligations.
- TC-033 steps 1 through 6 and expected results (examined): RT inspection and QSV checks plus planned quire-integration producer-origin behavior.
- `spec/exact/matrix/tests.md` FR-012 and TC-033 rows (examined): open residuals and lane classification.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | AC-2 now predicates RT behavior on QSL-produced values in a named test lane, excluding other valid admitted identities | FR-012-AC-2, TC-033 step 2 |

## Detail

The prior criterion began `Given admitted checked-package identities`. The edited criterion begins `Given identities obtained from QSL-produced, IR-admitted checked-package values in the quire-integration test lane`. Thus an RT consumer failure for an otherwise admitted identity from another conforming producer no longer falsifies AC-2. Preserve the original antecedent and all three comparison outcomes; specify the QSL-origin witness and quire-integration location in TC-033 and the Verification cell. No change to the migration's blocked status is needed.

## Checks

Targeted `quire validate --scope .` passed on the three edited spec files. Existing RT test fixtures still construct digest-shaped keys; the matrix keeps AC-8 open.

## Dispositions

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | fixed | At e1bff83, AC-2's behavior text exactly matches the baseline universal admitted-identity criterion; QSL and quire-integration details occur only in its Verification cell and TC-033. |

### After excerpt

> Given admitted checked-package identities, RT consumers retain the declaration identity, distinguish an identity from a foreign reference universe, and distinguish declared from compound unit identities regardless of equal underlying bytes. The corresponding foreign and cross-domain comparisons retain their inequality or typed refusal outcomes without an RT reminting step.
