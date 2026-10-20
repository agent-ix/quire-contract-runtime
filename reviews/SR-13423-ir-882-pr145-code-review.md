---
id: SR-13423
title: "IR-882 RT #145 code-review review"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-runtime; src/exact/; tests/ir882_*; Cargo.lock"
review_set: subset
---

## Summary

Ticket: IR-882. Reviewed PR #145 at a30ee447ca136c950136b3dae0640db6b723ae7b. Rust/code review found no implementation defect in the owner cutover. The Rust lane checked the deleted local modules, direct QSV/quire-exact imports, traced tests, panic surface, and CI workflow diff (none).

## Verdict

**PASS** — No finding in this method.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Reviewed units

- FR-277-AC-1: On the bounded depth-four `Sequence` input with equal `quire_exact::Value` operands, the QSV-owned `CheckedEquality::evaluate` path consumed by RT completes `true` and admits exactly 17 `EqualityPair` charges in occurrence-pair order after `EqualityPlanForm` and `EqualityPlan` and before `EqualityResultRetain`.
- FR-277-AC-2: On the same bounded shape with the first head unequal, the actual `CheckedEquality::evaluate` path completes `false` and still admits the complete 17-pair schedule and result-retain charge; equality does not short-circuit its occurrence-pair traversal or charge schedule.
- FR-277-AC-3: If a test meter denies the 17th `EqualityPair`, evaluation returns the existing typed `Incomplete` outcome at `EqualityPair`; only the preceding 16 pair charges are admitted, and `EqualityResultRetain` is not admitted. A denial of `EqualityResultRetain` occurs only after all 17 pair charges have been admitted.
- FR-277-AC-4: With equal-shaped operands whose declared (static) composite types differ, checked construction refuses with `IllTypedCause::TypeMismatch` before `CheckedEquality::evaluate`; the evaluator is not called and no equality pair charge is admitted.
- FR-277-AC-5: With `work_units = 50` and the bounded equal pair, the actual checked evaluation admits plan-form work of 32, then refuses `equality.plan` because its atomic reservation requires 19 remaining work units but only 18 remain. It returns the existing typed `Incomplete` at `WorkUnits` with limit 50, consumed 32 and `next_charge = 19`; no plan, pair, or result-retain charge is admitted and no equality result is returned.
- FR-277-AC-6: With `result_units = 0` and sufficient work and occurrence limits, the actual checked evaluation admits `equality.plan-form`, then refuses `equality.plan` atomically with typed `Incomplete { limit_kind: ResultUnits, limit: 0, consumed: 0, next_charge: 1 }`. No plan, pair, or result-retain charge is admitted and no equality result is returned. A version that checks result capacity only after pair charges fails this control.
- FR-277-AC-8: Inspection of the locked QSV `CheckedEquality::evaluate` path shows a direct call to `quire_exact::planned_equality_with_plan` on the evaluated operands and use of that call's Boolean relation; the same call admits the complete 17-event pair schedule through its meter, and RT defines no local `CheckedEquality` or pair walker. A separate kernel call does not establish this criterion.
- FR-008-AC-5: `plan_equality` forms the occurrence-pair plan with no charge, refuses a cross-universe reference pair as `ForeignReference`, and distinguishes unlike collection kinds (`CollectionKindMismatch`) from unlike value kinds (`ValueKindMismatch`); `check_equality` refuses before any charge, including `operator-ineligible` when either operand type bears an IEEE value at any depth; `CheckedEquality::evaluate` charges the selected schedule and its conversions in operand order, and each of the six checked operand failures carries its ix://agent-ix/quire-exact/FR-369 cause, retaining `IllTypedCause` for a rejected quantity conversion. After IR-882, different-declaration composites compare unequal rather than refusing, enums compare by variant alone rather than refusing for declaration mismatch, and membership's `ForeignReference` names the collection's retained member as `required` and the probed candidate as `supplied`.
- FR-275-AC-14: The kernel move deletes no requirement, acceptance criterion or test case.
