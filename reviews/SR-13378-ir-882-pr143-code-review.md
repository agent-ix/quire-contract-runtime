---
id: SR-13378
title: "Code review of IR-882 checked-function owner-value cutover"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-runtime; src/exact/expression.rs, tests/exact_function_application.rs, tests/exact_outcomes.rs, tests/ir882_direct_kernel_value.rs; FR-273-AC-1, FR-273-AC-2, FR-273-AC-3, FR-273-AC-8, FR-275-AC-2"
review_set: subset
---

## Summary

Ticket: IR-882. Reviewed PR #143 at aba5eb8 against the changed function boundary and its acceptance criteria. The owner types reach `PackageDeclarations`, `plan_call`, `plan_evaluation`, `CheckedPackage`, and `Frame` directly. The Rust lane and evidence analysis are recorded in their own artifacts for this review dispatch.

## Verdict

**PASS** — No code defect found in the changed boundary. The new union branch lacks a focused acceptance test; see SR-13380.

## Examined criteria

- FR-273-AC-1, examined: `CheckedPackage::call` and `CheckedPackage::evaluate` are called only against a package `PackageDeclarations::check` admitted under `CheckMode::Linked`; a package this crate's `PackageDeclarations::check` rejects is never applied.
- FR-273-AC-2, examined: Declared arity is decided before any per-argument check, each argument's value kind and carried references are validated in parameter order, and all of that precedes the `function.call` charge, which itself precedes the function's body on every `call`.
- FR-273-AC-3, examined: Arity, value-kind and dangling-reference input mismatches, and an unknown function name, each refuse as the matching `InputRefusal` variant before any charge; no `Meter` observes a refused call.
- FR-273-AC-8, context only: A foreign checked expression, an absent function requested within a checked body, and a re-entrant meter borrow carry distinct checked-invariant causes without changing their charge behavior.
- FR-275-AC-2, context only: The final value-bearing cutover removes RT-defined duplicate value and walker definitions; this PR is an intermediate step, and the matrix still marks the criterion planned.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Checks

- The PR changes no CI workflow or gate configuration.
- `cargo fmt --check`, the repository's `make lint`, `cargo deny check`, and 32 focused tests passed.
- The generic workspace-wide Clippy command does not apply to the no-std footprint crate on the host target; the repository's two-target `make lint` passed.
