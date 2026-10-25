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

## Dispositions

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | fixed | `tests/exact_panic_gate.rs::exact_probe_retains_required_rt_calls_without_panic_relocations` invokes the actual `make exact-panic-probe`, asserts its successful result and RT population, required-call and zero-panic reports; Quoin now binds AC-8 to this test. |
| FND-002 | fixed | `tests/exact_panic_gate.rs::injected_rt_panic_is_rejected_by_the_same_gate` invokes the same real probe, which runs the scratch injected control, and asserts its success marker; Quoin now binds AC-9 to this test. |

## Follow-up verification

At d6d27b8e5f357a664a37b70bfbdcd2c9c98a5c76, the probe gate also requires `CheckedPackage>::check_expression`, closing the retained exact expression call list under NFR-002-AC-8 and TC-206. The tagged AC-8 test invokes that same gate, and focused `make exact-panic-probe` passed with the new required call reported. The prior dispositions remain fixed; no new trace gap arose from this delta.

At 075929af662d7767857080283fee39a388195b42, the AC-8 tagged gate test asserts that the malformed-report control ran. The control modifies a legitimate positive `objdump` report by inserting a malformed RT-owned relocation row and requires the scanner's specific rejection under AC-9. Focused `make exact-panic-probe` passed both the positive and negative cases. The prior dispositions remain fixed; no new gap arose from this delta.
