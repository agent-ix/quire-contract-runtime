---
id: SR-13427
title: "IR-882 RT #145 spec-review/scope-boundary review"
type: SpecReview
analysis: scope-boundary
scope: "agent-ix/quire-contract-runtime; spec/assurance/AD-004-runtime-crate-layout.md; spec/exact/functional/FR-275-single-exact-kernel.md; spec/exact/functional/FR-277-checked-equality-evaluation.md"
review_set: subset
---

## Summary

Ticket: IR-882. Reviewed PR #145 at a30ee447ca136c950136b3dae0640db6b723ae7b. The boundary assigns value and pair planning to quire-exact, checked equality to QSV, and function application plus remaining quantity/enum residue to RT. The lock resolves the QSV implementation that calls the kernel directly.

## Verdict

**PASS** — No finding in this method.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Reviewed units

- FR-008-AC-5: `plan_equality` forms the occurrence-pair plan with no charge, refuses a cross-universe reference pair as `ForeignReference`, and distinguishes unlike collection kinds (`CollectionKindMismatch`) from unlike value kinds (`ValueKindMismatch`); `check_equality` refuses before any charge, including `operator-ineligible` when either operand type bears an IEEE value at any depth; `CheckedEquality::evaluate` charges the selected schedule and its conversions in operand order, and each of the six checked operand failures carries its ix://agent-ix/quire-exact/FR-369 cause, retaining `IllTypedCause` for a rejected quantity conversion. After IR-882, different-declaration composites compare unequal rather than refusing, enums compare by variant alone rather than refusing for declaration mismatch, and membership's `ForeignReference` names the collection's retained member as `required` and the probed candidate as `supplied`.
- FR-275-AC-2: No module, alias or feature of the runtime keeps a removed kernel definition reachable; after the IR-882 value-bearing cutover, inspection finds no RT-defined second `Value`, `CheckedEquality` evaluator, pair walker or membership walker, `UnitId` resolver, `EnumShape` registry, `EffectiveId` registry, copy of `UnitTable`, `TypeEnvironment`, `EnumMemberIndex` or `EnumDeclaration`, conversion layer, adapter or old-`Value` re-export. Quantity, enum, reference and checked-equality paths use the QSV owners directly.
- FR-275-AC-14: The kernel move deletes no requirement, acceptance criterion or test case.
- FR-277-AC-1: On the bounded depth-four `Sequence` input with equal `quire_exact::Value` operands, the QSV-owned `CheckedEquality::evaluate` path consumed by RT completes `true` and admits exactly 17 `EqualityPair` charges in occurrence-pair order after `EqualityPlanForm` and `EqualityPlan` and before `EqualityResultRetain`.
- FR-277-AC-8: Inspection of the locked QSV `CheckedEquality::evaluate` path shows a direct call to `quire_exact::planned_equality_with_plan` on the evaluated operands and use of that call's Boolean relation; the same call admits the complete 17-event pair schedule through its meter, and RT defines no local `CheckedEquality` or pair walker. A separate kernel call does not establish this criterion.
