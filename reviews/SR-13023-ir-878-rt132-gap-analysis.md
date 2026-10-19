---
id: SR-13023
title: "IR-878 RT PR 132 gap analysis: FR-277/FR-278 criteria vs named code"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-runtime@300924158cacde4428d59de719497fdbd56b7b37; spec/exact/functional/FR-277-checked-equality-evaluation.md, spec/exact/functional/FR-278-runtime-checked-invariant-causes.md; code src/exact/equality.rs, src/exact/composite.rs, src/exact/expression.rs; tests tests/exact_equality.rs, tests/exact_function_application.rs; kernel agent-ix/quire-exact origin/main src/equality.rs"
review_set: subset
---

## Summary

Ticket: IR-878. Planless gap analysis of the eight new criteria against the code they name. Plan completion: not assessed. Ownership holds in part. `CheckedEquality`, `check_equality` and `evaluate` live in RT `src/exact/equality.rs`, and the meter charges (`charge`, `charge_plan`) are the kernel `Meter`, so AC-3..AC-6 can be tested in RT. FR-278's named producers exist in RT `src/exact/expression.rs` and `src/exact/equality.rs`.

There are two gaps. First, RT's checked evaluation never reaches the kernel equality seam: it plans over RT's own `Value` with a local copy of `plan_pairs`, so FR-277's kernel-relation claims have no code path in RT. Second, FR-278's "unbacked" status is a tagging gap rather than missing tests: tagged FR-273-AC-8 and TC-026 tests already assert those causes. `CallDepthExceeded` is still produced (expression.rs:800, 862, 931), so FR-278-AC-1 is correctly not yet met. No `/// Trace:` lines are added, and coverage shows 0 contradicted statuses.

## Examined criteria

- `FR-277-AC-1`..`FR-277-AC-6` against `src/exact/equality.rs:97-240` (check_equality, evaluate, run, planned_equality, plan_pairs) and kernel `quire-exact` `src/equality.rs:111-135` (`planned_equality`, `planned_equality_with_plan`).
- `FR-277-AC-4` against `check_equality`: differing static composite types fall to `_ => ill_typed(IllTypedCause::TypeMismatch)` before any charge (clean).
- `FR-277-AC-5`/`AC-6` against `planned_equality`: plan-form charge then kernel `meter.charge_plan(&plan.pairs)` (clean ownership; kernel meter semantics).
- `FR-278-AC-1` against `src/exact/expression.rs:800-980` and tests/exact_function_application.rs:687,1075,1114,1153.
- `FR-278-AC-2` against `src/exact/equality.rs` operand_value and scheduled-comparison producers and tests/exact_equality.rs TC-026 tests.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | RT `CheckedEquality::evaluate` calls RT-local `planned_equality` and `plan_pairs` over RT's own `Value` (composite.rs:262), never `quire_exact::planned_equality_with_plan`. The "result-bearing kernel seam" in FR-277-AC-1, and the Description's "consuming the quire-exact planned relation", have no RT code path. Meeting them needs an RT code change (FR-275 residue removal), not only the planned test PR | src/exact/equality.rs:160-240; src/exact/composite.rs:262; spec/exact/functional/FR-277-checked-equality-evaluation.md:15,32 |
| FND-002 | medium | FR-278's planned tests mostly exist. tests/exact_function_application.rs:687,1075,1114,1153 already assert `ForeignCheckedExpression`, `MeterBorrowConflict` and `UnknownCheckedFunction` (tagged FR-273-AC-8), and TC-026 asserts the six `operand_value` causes (FR-008-AC-5). The FR-278 rows are unbacked for lack of tags, which the status wording misstates | spec/exact/functional/FR-278-runtime-checked-invariant-causes.md:31-32,41; tests/exact_function_application.rs:687 |

## Verdict

**FAIL (changes requested)**: FND-001 shows that FR-277 as allocated implies an unstated code change. FND-002 shows that the FR-278 status overstates the work missing. Neither is a problem with the verbatim text itself.

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Round 1 (5312d30): FR-277 now states the RT code change (route through `planned_equality_with_plan`, remove the RT-local `planned_equality`/`plan_pairs`) as a requirement, and its status orders code before tests. The code itself is unchanged by design, since this is a spec-only PR and the code change is ticketed separately |
| FND-002 | fixed | Round 1: FR-278 Status now says the overlapping assertions exist and only the FR-278 tags and the missing controls are pending. TC-204 is a retag procedure |
