---
id: SR-13425
title: "IR-882 RT #145 spec-review/base review"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-runtime; spec/exact/functional/; spec/exact/matrix/"
review_set: subset
---

## Summary

Ticket: IR-882. Reviewed PR #145 at a30ee447ca136c950136b3dae0640db6b723ae7b. The changed FR and TC documents retain TC-024, TC-025 and TC-026 and accurately mark their remaining procedures planned. FR-277 AC-1 through AC-6 have consumer tests; AC-7 remains planned and AC-8 requires inspection.

## Verdict

**PASS** — No finding in this method.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Reviewed units

- FR-008-AC-5: `plan_equality` forms the occurrence-pair plan with no charge, refuses a cross-universe reference pair as `ForeignReference`, and distinguishes unlike collection kinds (`CollectionKindMismatch`) from unlike value kinds (`ValueKindMismatch`); `check_equality` refuses before any charge, including `operator-ineligible` when either operand type bears an IEEE value at any depth; `CheckedEquality::evaluate` charges the selected schedule and its conversions in operand order, and each of the six checked operand failures carries its ix://agent-ix/quire-exact/FR-369 cause, retaining `IllTypedCause` for a rejected quantity conversion. After IR-882, different-declaration composites compare unequal rather than refusing, enums compare by variant alone rather than refusing for declaration mismatch, and membership's `ForeignReference` names the collection's retained member as `required` and the probed candidate as `supplied`.
- FR-275-AC-14: The kernel move deletes no requirement, acceptance criterion or test case.
- FR-277-AC-1: On the bounded depth-four `Sequence` input with equal `quire_exact::Value` operands, the QSV-owned `CheckedEquality::evaluate` path consumed by RT completes `true` and admits exactly 17 `EqualityPair` charges in occurrence-pair order after `EqualityPlanForm` and `EqualityPlan` and before `EqualityResultRetain`.
- FR-277-AC-7: The IR-241 Kani claim remains `plan_equality(left, right).pair_events() == count_of_EqualityPair_charges_from_the_same_real_CheckedEquality::evaluate(left, right, meter)` for both equal and first-head-unequal bounded operands, with the existing exact operand bound and proof ceiling. A plan-only equality or duplicated QSL walker does not satisfy this criterion.
- FR-277-AC-8: Inspection of the locked QSV `CheckedEquality::evaluate` path shows a direct call to `quire_exact::planned_equality_with_plan` on the evaluated operands and use of that call's Boolean relation; the same call admits the complete 17-event pair schedule through its meter, and RT defines no local `CheckedEquality` or pair walker. A separate kernel call does not establish this criterion.
