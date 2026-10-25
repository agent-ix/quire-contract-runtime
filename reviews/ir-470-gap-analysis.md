---
id: SR-35026
title: "Gap analysis of IR-470 exact panic gate"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-runtime; NFR-002-AC-8..10, TC-206, Makefile, measurement/exact-panic/, scripts/check_exact_panic*.py, tools/kernel-api-check/"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-runtime/NFR-002
    type: reviews
---

## Summary

Reviewed the changed exact gate against the computed repository Test Matrix and inspected its implementation. The new command controls execute, while the matrix has no binding test for two new Test criteria.

## Verdict

**FAIL** — NFR-002-AC-8 and AC-9 are untagged in the computed matrix.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | AC-8 has no binding test symbol in the computed Test Matrix | NFR-002-AC-8; Makefile:111 |
| FND-002 | high | AC-9 has no binding test symbol in the computed Test Matrix | NFR-002-AC-9; scripts/check_exact_panic_control.py:17 |

## Coverage

Plan completion: not assessed.

Reconciliation: `quoin matrix --repo . --json` (quoin 0.29.1) on the PR head. NFR-002-AC-8: untagged, zero binders; AC-9: untagged, zero binders; AC-10: tagged to `tools/kernel-api-check/src/lib.rs::tests::admitted_divisor_is_required_by_all_three_helpers`. No run evidence is bound in the evidence store. `quire coverage --scope . --json` reported no untracked symbols for these three criteria. Its unrelated module diagnostics concern missing `Inspections`/`SuiteRegistry` documents, one FR-009 production tag, and criterion-shape analysis.

The Python commands are wired into `make ci`, but the declared Test evidence model does not bind them as tests. A trace tag on an unrelated Rust test would not close the evidence gap. Reverse inspection found no new production API outside the NFR-002/TC-206 scope, and the gate is not a stub.
