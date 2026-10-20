---
id: SR-13380
title: "Gap analysis of IR-882 checked-function owner-value cutover"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-runtime; spec/exact/functional/FR-273-exact-function-application.md, spec/exact/functional/FR-275-single-exact-kernel.md, src/exact/expression.rs, tests/exact_function_application.rs, tests/exact_outcomes.rs, tests/ir882_direct_kernel_value.rs; FR-273-AC-2, FR-273-AC-3, FR-275-AC-2"
review_set: subset
---

## Summary

Ticket: IR-882. The computed matrix binds the changed function tests to FR-273-AC-2 and AC-3. The new exact-union traversal branch is not exercised by any test in this repository, leaving the carried-reference refusal on that new value form unverified.

## Verdict

**CONDITIONAL** — One medium evidence gap in this PR's new branch. The repository-wide pre-existing matrix debt is disclosed below and is not attributed to this diff.

## Examined criteria

- FR-273-AC-2, examined: Declared arity is decided before any per-argument check, each argument's value kind and carried references are validated in parameter order, and all of that precedes the `function.call` charge, which itself precedes the function's body on every `call`.
- FR-273-AC-3, examined: Arity, value-kind and dangling-reference input mismatches, and an unknown function name, each refuse as the matching `InputRefusal` variant before any charge; no `Meter` observes a refused call.
- FR-275-AC-2, context only: The final value-bearing cutover removes RT-defined duplicate value and walker definitions; this PR is an intermediate step, and the matrix still marks the criterion planned.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | No test exercises a dangling reference nested in the newly handled exact union payload before `function.call` charges | src/exact/expression.rs:571, tests/exact_function_application.rs:176 |

## Coverage

- Reconciliation: `quoin matrix --repo . --json` using Quoin 0.28.3 and Quire 0.36.2; no run evidence was bound. FR-273-AC-2 and AC-3 are tagged. The repository-wide matrix has 68 tagged, 63 untagged, and 39 method-without-symbol criteria, including pre-existing planned work. The changed tests introduce no new untagged criterion.
- Plan completion: not assessed
- Changed behavior inventory: one owner-value function boundary and one newly reachable union payload walk; no new public function or source stub.
- Semantic review: skipped; no opt-in was supplied.
- A union can hold a `Value::Reference` in its payload; `ValueType::admits` accepts an admitted union as a composite. Current TC-194 tests cover a direct reference only. A regression that skips `Value::Union` traversal would leave all current changed tests green while allowing a dangling reference into the body.

## Dispositions

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | fixed | Commit 92fa0f2 adds a TC-194 test with a QSV-admitted union carrying a dangling `ObjectReference`; it asserts `DanglingReference { parameter: 0 }` and zero admitted charges. Removing the production union traversal makes that test fail, so the test detects this branch's regression. |
