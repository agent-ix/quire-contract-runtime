---
id: SR-13424
title: "IR-882 RT #145 gap-analysis review"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-runtime; spec/exact/functional/FR-008-composite-collection-and-equality.md; tests/ir882_kernel_pair_results.rs"
review_set: subset
---

## Summary

Ticket: IR-882. Reviewed PR #145 at a30ee447ca136c950136b3dae0640db6b723ae7b. FR-008-AC-5 is tagged by two new kernel tests that cover only three changed pair results. The criterion also requires type and operator refusals, six operand failures, conversion order, and schedule behavior; TC-026 marks these planned.

## Verdict

**CONDITIONAL** — The partial FR-008-AC-5 binding must be corrected or completed before acceptance credit.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-008-AC-5 is marked tagged by selected pair-result tests while the remaining equality matrix is unverified | tests/ir882_kernel_pair_results.rs:24 |

## Reviewed units

- FR-008-AC-1: A `TypeEnvironment` admits a closed set of record, tuple and object-type declarations only after member-type and both recursion-rule checks succeed, and refuses at the originating declaration with a typed `DeclarationCause` otherwise; `record`, `tuple`, `evaluate_record` and `evaluate_tuple` construct in declaration order under the first-stopped rule and charge `composite.result-retain` with the exact retained `occ`; a deferred result outside its declared type carries `CheckedInvariant { cause: DeferredResultNotAdmitted }`.
- FR-008-AC-2: `TypeEnvironment::build` constructs a finite value bottom-up from a `ValueGraph`, sharing a node reached from more than one slot as one immutable value with no object identity created, and refuses the first containment cycle as `ContainmentCycle` at the node that closes it.
- FR-008-AC-3: `construct_collection` charges `collection.element` before running each deferred element and stops at the first non-completing element with no later element run; a completed element outside its declared type returns `CheckedInvariant { cause: CollectionElementNotAdmitted }`; `form_collection` refuses a type-mismatched occurrence before any charge; formation charges membership comparisons, `collection.bound` and `collection.result-retain` in that order, and a set or bag is stored in ascending canonical-key order.
- FR-008-AC-4: The type-owned canonical key totally orders every keyed type, ranks `Absent < Null < Present`, fixes set and bag canonical and visiting order, and is evaluated iteratively at a depth that would overflow a recursive host-stack walk.
- FR-008-AC-5: `plan_equality` forms the occurrence-pair plan with no charge, refuses a cross-universe reference pair as `ForeignReference`, and distinguishes unlike collection kinds (`CollectionKindMismatch`) from unlike value kinds (`ValueKindMismatch`); `check_equality` refuses before any charge, including `operator-ineligible` when either operand type bears an IEEE value at any depth; `CheckedEquality::evaluate` charges the selected schedule and its conversions in operand order, and each of the six checked operand failures carries its ix://agent-ix/quire-exact/FR-369 cause, retaining `IllTypedCause` for a rejected quantity conversion. After IR-882, different-declaration composites compare unequal rather than refusing, enums compare by variant alone rather than refusing for declaration mismatch, and membership's `ForeignReference` names the collection's retained member as `required` and the probed candidate as `supplied`.
- FR-008-AC-6: A `Reference<T>` value carries only its supplied `(universe, object-type, identity)` triple, is constructed from no source form, and compares equal only within one universe.
- FR-008-AC-7: The extended `Undefined` and `Refusal` vocabularies, `BoundViolation` and `Refusal::cause()` are closed, typed and distinct from every FR-006 variant; both `CardinalityOutOfBound` directions are reachable and report their `code()` and `cause()`; `CheckedInvariant` is unreachable from any admitted vector in the shared corpus.
- FR-008-AC-8: Every one of the twelve added charge points round-trips its QSpec spelling, `ChargePoint::ALL` has exactly 63 members including `temporal.visit`, and an injected denial at each of the twelve yields `Incomplete` on `work_units` naming that point with every counter left unchanged.
- FR-008-AC-9: The consumed kernel `Value` has iterative `Debug` and nested-node `Drop`: formatting or dropping a value nested past a recursive walk's host-stack limit does not overflow the stack and completes in time and allocation proportional to the value's size, and its `Debug` output matches an exact literal string in both compact and alternate form on a small fixed value.
- FR-008-AC-10: When formation coalesces occurrences, the runtime charges one `collection.member-walk` and one `collection.member-test` per membership comparison for a set, bag or ordered set, and charges neither point for a sequence.
- FR-008-AC-11: When a collection is retained, the runtime stores a sequence in supplied occurrence order, an ordered set as the first occurrence of each distinct member in first-occurrence order, a set as one occurrence of each distinct member in ascending canonical-key order, and a bag as every occurrence in ascending canonical-key order by multiplicity: equal occurrences are one member whose multiplicity is their number, and each is represented by that member's first-supplied value, as in ix://agent-ix/quire-specification/FR-144 and FR-144-AC-21.
- FR-008-AC-12: When a retained collection is visited by equality planning or by a further formation, the runtime visits its elements in the order it stored them, for every one of the four kinds.
- FR-008-AC-13: For an `Option`/`Collection`-nested `ValueType` whose depth exceeds a recursive host-stack walk under the admitted source-size cap, `Clone`, `PartialEq`, `Debug` and `Drop` complete on a small-stack thread and a default-stack child process without host-stack overflow; unequal leaf, collection kind and bound remain unequal, and the shallow compact and alternate `Debug` output matches the derived representation exactly.
- FR-277-AC-1: On the bounded depth-four `Sequence` input with equal `quire_exact::Value` operands, the QSV-owned `CheckedEquality::evaluate` path consumed by RT completes `true` and admits exactly 17 `EqualityPair` charges in occurrence-pair order after `EqualityPlanForm` and `EqualityPlan` and before `EqualityResultRetain`.
- FR-277-AC-2: On the same bounded shape with the first head unequal, the actual `CheckedEquality::evaluate` path completes `false` and still admits the complete 17-pair schedule and result-retain charge; equality does not short-circuit its occurrence-pair traversal or charge schedule.
- FR-277-AC-3: If a test meter denies the 17th `EqualityPair`, evaluation returns the existing typed `Incomplete` outcome at `EqualityPair`; only the preceding 16 pair charges are admitted, and `EqualityResultRetain` is not admitted. A denial of `EqualityResultRetain` occurs only after all 17 pair charges have been admitted.
- FR-277-AC-4: With equal-shaped operands whose declared (static) composite types differ, checked construction refuses with `IllTypedCause::TypeMismatch` before `CheckedEquality::evaluate`; the evaluator is not called and no equality pair charge is admitted.
- FR-277-AC-5: With `work_units = 50` and the bounded equal pair, the actual checked evaluation admits plan-form work of 32, then refuses `equality.plan` because its atomic reservation requires 19 remaining work units but only 18 remain. It returns the existing typed `Incomplete` at `WorkUnits` with limit 50, consumed 32 and `next_charge = 19`; no plan, pair, or result-retain charge is admitted and no equality result is returned.
- FR-277-AC-6: With `result_units = 0` and sufficient work and occurrence limits, the actual checked evaluation admits `equality.plan-form`, then refuses `equality.plan` atomically with typed `Incomplete { limit_kind: ResultUnits, limit: 0, consumed: 0, next_charge: 1 }`. No plan, pair, or result-retain charge is admitted and no equality result is returned. A version that checks result capacity only after pair charges fails this control.
- FR-277-AC-7: The IR-241 Kani claim remains `plan_equality(left, right).pair_events() == count_of_EqualityPair_charges_from_the_same_real_CheckedEquality::evaluate(left, right, meter)` for both equal and first-head-unequal bounded operands, with the existing exact operand bound and proof ceiling. A plan-only equality or duplicated QSL walker does not satisfy this criterion.
- FR-277-AC-8: Inspection of the locked QSV `CheckedEquality::evaluate` path shows a direct call to `quire_exact::planned_equality_with_plan` on the evaluated operands and use of that call's Boolean relation; the same call admits the complete 17-event pair schedule through its meter, and RT defines no local `CheckedEquality` or pair walker. A separate kernel call does not establish this criterion.
- FR-275-AC-14: The kernel move deletes no requirement, acceptance criterion or test case.

## Coverage

Plan completion: not assessed. Computed matrix: FR-008-AC-5 tagged by only the two new kernel pair-result tests; FR-008-AC-1/2/3 remain untagged as planned. The baseline has known unbacked criteria.

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | At 829bffe, both partial pair-result tests retain TC-025/TC-026 tags but remove FR-008-AC-5; the computed matrix now reports FR-008-AC-5 untagged, consistent with the planned remainder in TC-026. The two focused tests pass. |

