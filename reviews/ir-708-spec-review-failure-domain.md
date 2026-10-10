---
id: SR-5134
title: "IR-708 PR 113 failure-domain review"
type: SpecReview
analysis: failure-domain
scope: "agent-ix/quire-contract-runtime@fbdb317cdaca352b1e9a1f84fc88d015b8b2fe9a; Cargo.lock; src/exact/{collection,composite,equality,expression,quantity}.rs; tests/exact_{collection,composite,equality,function_application,meter_state}.rs; spec/exact/functional/{FR-008,FR-273}; spec/exact/matrix/{TC-024,TC-025,TC-026,TC-194}"
review_set: subset
---

## Summary

The changed failure domains identify foreign package expressions, missing checked functions, meter borrow conflicts, admission failures, and equality conversions without an additional omitted mode in this diff.

## Verdict

**PASS** — No findings in the reviewed PR diff.

## Examined criteria

- `FR-008-AC-1` (spec/exact/functional/FR-008-composite-collection-and-equality.md): A `TypeEnvironment` admits a closed set of record, tuple and object-type declarations only after member-type and both recursion-rule checks succeed, and refuses at the originating declaration with a typed `DeclarationCause` otherwise; `record`, `tuple`, `evaluate_record` and `evaluate_tuple` construct in declaration order under the first-stopped rule and charge `composite.result-retain` with the exact retained `occ`; a deferred result outside its declared type carries `CheckedInvariant { cause: DeferredResultNotAdmitted }`.
- `FR-008-AC-3` (spec/exact/functional/FR-008-composite-collection-and-equality.md): `construct_collection` charges `collection.element` before running each deferred element and stops at the first non-completing element with no later element run; a completed element outside its declared type returns `CheckedInvariant { cause: CollectionElementNotAdmitted }`; `form_collection` refuses a type-mismatched occurrence before any charge; formation charges membership comparisons, `collection.bound` and `collection.result-retain` in that order, and a set or bag is stored in ascending canonical-key order.
- `FR-008-AC-5` (spec/exact/functional/FR-008-composite-collection-and-equality.md): `plan_equality` forms the occurrence-pair plan with no charge, refuses a cross-universe reference pair as `ForeignReference`, and distinguishes unlike collection kinds (`CollectionKindMismatch`) from unlike value kinds (`ValueKindMismatch`); `check_equality` refuses before any charge, including `operator-ineligible` when either operand type bears an IEEE value at any depth; `CheckedEquality::evaluate` charges the selected schedule and its conversions in operand order, and each of the six checked operand failures carries its FR-369 cause, retaining `IllTypedCause` for a rejected quantity conversion.
- `FR-008-AC-7` (spec/exact/functional/FR-008-composite-collection-and-equality.md): The extended `Undefined` and `Refusal` vocabularies, `BoundViolation` and `Refusal::cause()` are closed, typed and distinct from every FR-006 variant; both `CardinalityOutOfBound` directions are reachable and report their `code()` and `cause()`; `CheckedInvariant` is unreachable from any admitted vector in the shared corpus.
- `FR-273-AC-7` (spec/exact/functional/FR-273-exact-function-application.md): Re-entry into a checked package through `CheckedPackage::call`, `CheckedPackage::evaluate` or `Frame::call` is bounded by the runtime's own `CheckingLimits::depth`, not an authority checker limit (at most `MAX_CALL_DEPTH`) by one budget shared across all three entry paths, and exceeding it refuses as `Refusal::CheckedInvariant { cause: CallDepthExceeded }` before any charge. The bound is per-`CheckedPackage`, not universal.
- `FR-273-AC-8` (spec/exact/functional/FR-273-exact-function-application.md): A foreign checked expression, an absent function requested within a checked body, and a re-entrant meter borrow carry `ForeignCheckedExpression`, `UnknownCheckedFunction`, and `MeterBorrowConflict` respectively, without changing their charge behavior or treating them as ordinary input refusals.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |
