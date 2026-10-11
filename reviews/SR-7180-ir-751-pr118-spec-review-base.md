---
id: SR-7180
title: "IR-751 exact meter spec review"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-runtime@875d7aa7b325e8cd891a084e63a1f8585d2c8ce0; spec/exact/functional/FR-006-exact-outcomes-and-accounting.md, spec/exact/functional/FR-011-meter-state-at-a-stop.md, spec/exact/matrix/TC-032-meter-state-at-a-stop.md, spec/exact/matrix/tests.md"
review_set: subset
---

## Summary

Ticket: IR-751. Reviewed the four changed Markdown files against quire-exact FR-359, QSpec TC-187 U13, and the retained RT quantity implementation and test. The amended obligations and matrix accurately distinguish the target behavior from partial current evidence.

## Reviewed scope

| Unit | Role | Path | Excerpt |
| --- | --- | --- | --- |
| FR-006 stop behavior | examined | spec/exact/functional/FR-006-exact-outcomes-and-accounting.md | What the meter holds at an `Undefined` or `Refused` stop, the atomicity of one charge, the field-order scan over a charge's own size vector, the value and truncation behavior of `CHARGE_LOG_CAPACITY`, and exact derived charge requests are FR-011. |
| FR-011 charge behavior | examined | spec/exact/functional/FR-011-meter-state-at-a-stop.md | `work_units` and `result_units` are `u64` counters. An addition that exceeds a configured limit or `u64::MAX` is denied before either counter changes. Amounts derived from operand shape remain mathematical integers in the charge request, even above `u64::MAX`. |
| FR-011-AC-6 | examined | spec/exact/functional/FR-011-meter-state-at-a-stop.md | With a cumulative limit of `u64::MAX`, a counter at `u64::MAX - 1` admits one unit to reach the limit and denies the next unit without wrapping or changing any meter state. A quantity power whose `unit.rational-arithmetic` request exceeds `u64::MAX` returns `Incomplete` before computing the power; `next_charge` equals the exact request. |
| FR-011-AC-7 | examined | spec/exact/functional/FR-011-meter-state-at-a-stop.md | `Meter::consumed` answers without panic for every member of `LimitKind::ALL`, returning zero for untouched counters, admitted high-water values for semantic sizes and admitted cumulative totals for work/results. |
| TC-032 step 8 | examined | spec/exact/matrix/TC-032-meter-state-at-a-stop.md | Under a cumulative limit of `u64::MAX`, admit one unit from `u64::MAX - 1` and deny the next unit without changing meter state. Request a quantity power charge past `u64::MAX`; check `Incomplete::next_charge` equals the exact requested amount. |
| Exact matrix FR-011-AC-6 | examined | spec/exact/matrix/tests.md | Partly evidenced: retained RT test checks a quantity-power denial before computation, `next_charge > u64::MAX` and the admitted prefix, but does not assert the exact amount or every counter. |
| Kernel FR-359-AC-1/3/4/7 | context_only | quire-exact/spec/functional/FR-359-cumulative-meter-boundary.md | Cumulative boundary and atomic refusal; exact over-`u64` work/result requests; `consumed` over every `LimitKind::ALL` member. |
| QSpec TC-187 U13 | context_only | quire-specification/spec/test-cases/TC-187-quantity-and-unit-conversion.md | Quantity power expects `next_charge: 36893488147419103232` at `unit.rational-arithmetic` before power computation. |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Verdict

PASS. Exact request arithmetic, atomic cumulative refusal and the closed-domain consumed reader are stated consistently with the upstream contracts. TC-032 states falsifiable expected behavior. The matrix discloses the retained RT test's narrower assertions, so this review makes no new implementation claim. Scoped grammar validation passed for both changed FR files.
