---
id: TC-026
title: "Evaluate the equality matrix and terminal references"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-runtime/FR-008
    type: verifies
---
# TC-026: Evaluate the equality matrix and terminal references

## Description

Check `check_equality`, `plan_equality`, `CheckedEquality::evaluate` and terminal `Reference<T>`
identity. Evidence: `tests/exact_equality.rs` (`--features exact`). A Kani proof of the
plan/evaluate pair-count agreement property (Test Procedure step 6) is tracked as IR-241 (Linear).

## Test Procedure

1. Check `check_equality` refuses before any charge: an unadmitted `convert<T>(e)` conversion as
   `type-mismatch`, an IEEE-bearing operand type at any depth as `operator-ineligible`, distinct text
   profiles, distinct enum declarations and incompatible or mismatched quantity units each with their
   named `IllTypedCause`; and check it selects `EqualitySchedule::{Text, Enum, Quantity, Plan}` from
   the common comparison type.
2. Call `plan_equality` on two completed operands and check no charge is admitted; call it on a pair
   of references from different universes and check it refuses as `ForeignReference` at plan time,
   never as a charged comparison.
3. Evaluate `CheckedEquality::evaluate` over a composite and a collection operand pair with at least
   one nested occurrence pair; check the left and right conversion charges run in operand order, then
   `equality.plan-form`, `equality.plan`, one `equality.pair` per planned pair and
   `equality.result-retain`, all before the Boolean result is exposed. Evaluate a top-level text,
   enum and quantity pair and check each runs its own schedule instead.
4. Check an `ObjectReference` carries exactly its supplied `(universe, object-type, identity)`
   triple, that equality never reads or requires the referenced object's attributes, and that no
   record, tuple or collection constructor accepts a component value in place of a supplied
   reference.
5. Inject a denial at each of `equality.plan-form`, `equality.plan`, `equality.pair` and
   `equality.result-retain`; check each `Incomplete` record and that no counter changed.
6. For generated `Boolean`, `Integer`, `Option`, bounded `Sequence` and tuple-record composite
   operands, check that the planned pair count equals an independently computed occurrence-pair
   count and the number of admitted `equality.pair` charges.

## Expected Results

Every equality refusal is decided before any charge; the selected schedule and its conversions are
charged in the declared order; a cross-universe reference pair refuses by name, never by silent
inequality; references carry no inspected state; every injected denial fires with no partial result,
and `CheckedInvariant` is unreachable from every vector in this test's corpus. Over generated
composite values, `plan_equality`'s predicted pair count, an independently computed occurrence-pair
count, and the number of `equality.pair` charges `CheckedEquality::evaluate` admits always agree.
