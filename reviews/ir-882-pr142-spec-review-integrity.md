---
id: SR-13375
title: "IR-882 PR 142 spec-review/integrity review"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-contract-runtime; spec/assurance/AD-004-runtime-crate-layout.md, spec/exact/functional/FR-275-single-exact-kernel.md, spec/exact/functional/FR-277-checked-equality-evaluation.md, spec/exact/matrix/TC-026-equality-matrix.md, spec/exact/matrix/TC-203-checked-equality-evaluation.md, spec/exact/matrix/tests.md"
review_set: subset
---

## Summary

Ticket IR-882; frozen head 41db64f297e8b5bf46e4816b1ff2624a1e3daf09. Reviewed the six changed spec files, with FR-278 and TC-204 as context. The QSV owner allocation and planned RT consumer coverage are internally coherent except for the stale FR-278/TC-204 evidence path.

## Examined units

- AD (examined; spec/assurance/AD-004-runtime-crate-layout.md): RT consumes QSV's `CheckedEquality::evaluate` over `quire_exact::Value`; QSV's plan schedule calls `quire_exact::planned_equality_with_plan` and owns the result-bearing evaluator.
- FR-275 (examined; spec/exact/functional/FR-275-single-exact-kernel.md): No module, alias or feature of the runtime keeps a removed kernel definition reachable; after the IR-882 value-bearing cutover, inspection finds no RT-defined second `Value`, `CheckedEquality` evaluator, pair walker or membership walker, `UnitId` resolver, `EnumShape` registry, `EffectiveId` registry, copy of `UnitTable`, `TypeEnvironment`, `EnumMemberIndex` or `EnumDeclaration`, conversion layer, adapter or old-`Value` re-export. Quantity, enum, reference and checked-equality paths use the QSV owners directly.
- FR-277 (examined; spec/exact/functional/FR-277-checked-equality-evaluation.md): Inspection of the locked QSV `CheckedEquality::evaluate` path shows a direct call to `quire_exact::planned_equality_with_plan` on the evaluated operands and use of that call's Boolean relation; the same call admits the complete 17-event pair schedule through its meter, and RT defines no local `CheckedEquality` or pair walker. A separate kernel call does not establish this criterion.
- TC-026 (examined; spec/exact/matrix/TC-026-equality-matrix.md): Check QSV's `check_equality`, quire-exact's `plan_equality`, QSV's `CheckedEquality::evaluate` over `quire_exact::Value`, and terminal `Reference<T>` identity through RT consumer tests.
- TC-203 (examined; spec/exact/matrix/TC-203-checked-equality-evaluation.md): Verify FR-277-AC-1 through AC-6 by RT consumer tests and AC-8 by inspection only after QSV's `CheckedEquality::evaluate` routes through `quire_exact::planned_equality_with_plan` and RT removes its duplicate.
- tests.md (examined; spec/exact/matrix/tests.md): | FR-277 | FR-277-AC-1, FR-277-AC-2, FR-277-AC-3, FR-277-AC-4, FR-277-AC-5, FR-277-AC-6, FR-277-AC-7, FR-277-AC-8 | TC-203 | 🚧 Planned: IR-882 migrates to `quire_exact::Value`, deletes RT's duplicate evaluator and pair walker, and consumes QSV's result-bearing kernel route; no AC-1 tag until that QSV path runs; AC-8 inspects QSV and RT source; AC-7 Kani lane owner titan:kani:plan |
- FR-278 (context_only; spec/exact/functional/FR-278-runtime-checked-invariant-causes.md): Every RT collection, composite, equality and quantity producer returns its table cause, including all six `operand_value` failures and the original kernel-owned `IllTypedCause` when a scheduled comparison or checked equality quantity conversion refuses; no site flattens a comparison or conversion error to a message.
- TC-204 (context_only; spec/exact/matrix/TC-204-runtime-checked-invariant-causes.md): Reuse the six `operand_value` cause assertions in the `src/exact/equality.rs` unit tests tagged FR-008-AC-5 and check that scheduled comparison and checked equality quantity conversion preserve their original `IllTypedCause` payloads.
- FR-277-AC-1 (examined; spec/exact/functional/FR-277-checked-equality-evaluation.md): On the bounded depth-four `Sequence` input with equal `quire_exact::Value` operands, the QSV-owned `CheckedEquality::evaluate` path consumed by RT completes `true` and admits exactly 17 `EqualityPair` charges in occurrence-pair order after `EqualityPlanForm` and `EqualityPlan` and before `EqualityResultRetain`.
- FR-277-AC-2 (examined; spec/exact/functional/FR-277-checked-equality-evaluation.md): On the same bounded shape with the first head unequal, the actual `CheckedEquality::evaluate` path completes `false` and still admits the complete 17-pair schedule and result-retain charge; equality does not short-circuit its occurrence-pair traversal or charge schedule.
- FR-277-AC-3 (examined; spec/exact/functional/FR-277-checked-equality-evaluation.md): If a test meter denies the 17th `EqualityPair`, evaluation returns the existing typed `Incomplete` outcome at `EqualityPair`; only the preceding 16 pair charges are admitted, and `EqualityResultRetain` is not admitted. A denial of `EqualityResultRetain` occurs only after all 17 pair charges have been admitted.
- FR-277-AC-4 (examined; spec/exact/functional/FR-277-checked-equality-evaluation.md): With equal-shaped operands whose declared (static) composite types differ, checked construction refuses with `IllTypedCause::TypeMismatch` before `CheckedEquality::evaluate`; the evaluator is not called and no equality pair charge is admitted.
- FR-277-AC-5 (examined; spec/exact/functional/FR-277-checked-equality-evaluation.md): With `work_units = 50` and the bounded equal pair, the actual checked evaluation admits plan-form work of 32, then refuses `equality.plan` because its atomic reservation requires 19 remaining work units but only 18 remain. It returns the existing typed `Incomplete` at `WorkUnits` with limit 50, consumed 32 and `next_charge = 19`; no plan, pair, or result-retain charge is admitted and no equality result is returned.
- FR-277-AC-6 (examined; spec/exact/functional/FR-277-checked-equality-evaluation.md): With `result_units = 0` and sufficient work and occurrence limits, the actual checked evaluation admits `equality.plan-form`, then refuses `equality.plan` atomically with typed `Incomplete { limit_kind: ResultUnits, limit: 0, consumed: 0, next_charge: 1 }`. No plan, pair, or result-retain charge is admitted and no equality result is returned. A version that checks result capacity only after pair charges fails this control.
- FR-277-AC-7 (examined; spec/exact/functional/FR-277-checked-equality-evaluation.md): The IR-241 Kani claim remains `plan_equality(left, right).pair_events() == count_of_EqualityPair_charges_from_the_same_real_CheckedEquality::evaluate(left, right, meter)` for both equal and first-head-unequal bounded operands, with the existing exact operand bound and proof ceiling. A plan-only equality or duplicated QSL walker does not satisfy this criterion.
- FR-277-AC-8 (examined; spec/exact/functional/FR-277-checked-equality-evaluation.md): Inspection of the locked QSV `CheckedEquality::evaluate` path shows a direct call to `quire_exact::planned_equality_with_plan` on the evaluated operands and use of that call's Boolean relation; the same call admits the complete 17-event pair schedule through its meter, and RT defines no local `CheckedEquality` or pair walker. A separate kernel call does not establish this criterion.

## Verdict

**CONDITIONAL** — The stale TC-204 evidence reference needs correction before the value cutover.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | TC-026 now plans QSV consumer tests after RT removes src/exact/equality.rs, but TC-204 and FR-278-AC-2 still require reusing six operand_value assertions in that deleted RT file. The planned FR-278 evidence becomes impossible at the value cutover. Update FR-278 and TC-204 to allocate those cause checks to QSV and specify RT consumer controls, or explicitly defer their reallocation with an owner and criterion before deleting the RT evaluator. | spec/exact/matrix/TC-026-equality-matrix.md:12-17,63-67 |

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-002 | high | The fix deletes FR-278-AC-2 and TC-204, contradicting FR-275-AC-14 and TC-197 step 5, which require the kernel move to delete no acceptance criterion or test case. QSV FR-122-AC-1 and AC-4 correctly own the equality/quantity cases, but the removed AC also covered RT collection and composite producers. Retain and update FR-278-AC-2 and TC-204 for the remaining RT producer obligations, recording the QSV transfer and planned evidence, or revise the governing no-deletion requirement and its inspection procedure through an explicit spec decision. | spec/exact/functional/FR-275-single-exact-kernel.md:180; spec/exact/matrix/TC-197-single-kernel-ownership.md:42-43; spec/exact/functional/FR-278-runtime-checked-invariant-causes.md:30-33 |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | At fix head c4f87854ff816011cc7db068b5e083da8a44b347, FR-278 no longer directs reuse of RT equality tests, and TC-204 is removed. QSV main FR-122-AC-1 and TC-906 own the five reached equality/quantity refusal cases; FR-122-AC-4 owns Inspection of the unreachable non-exact placement arm. |
| FND-002 | still-open | The same fix deletes FR-278-AC-2 and TC-204 while FR-275-AC-14 and TC-197 step 5 still forbid deleting any acceptance criterion or test case during this move. |

## Disposition verdict

**FAIL; not mergeable at c4f87854ff816011cc7db068b5e083da8a44b347.** FND-001 is fixed, but new high FND-002 remains open. The rebased branch retains PR #141's AD-004 compound-unit section, FR-275-AC-23, TC-197/TC-201 updates and planned matrix row. The seven reviewer-held SR-13371 through SR-13377 artifacts are outside the branch and await custody by the lead or fix-round coder.

## Dispositions (round 2)

| FND | outcome | reason |
| --- | --- | --- |
| FND-002 | fixed | Fix head 73b4773d22993e6b6a5af5c5b7877f72e9d496e4 restores FR-278-AC-2 and TC-204 with their original IDs. They now assign the five reached equality/quantity causes to QSV FR-122-AC-1 and the non-exact placement inspection to AC-4, while retaining distinct, planned RT collection/composite cause controls. The matrix restores both IDs and marks those RT controls unbacked. FR-275-AC-14 and TC-197 step 5 are no longer contradicted. |

## Disposition verdict (round 2)

**PASS at 73b4773d22993e6b6a5af5c5b7877f72e9d496e4; mergeable after review-artifact custody and the required gates.** FND-001 and FND-002 are both fixed. The branch still contains PR #141's compound-unit identity section, FR-275-AC-23, TC-197/TC-201 updates and planned matrix row. Quire validates 66 of 66 spec documents; the computed matrix leaves FR-278-AC-2 untagged, consistent with its planned status. The seven reviewer-held SR-13371 through SR-13377 artifacts remain outside the branch.
