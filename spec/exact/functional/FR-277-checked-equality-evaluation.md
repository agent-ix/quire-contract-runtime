---
id: FR-277
title: "Exercise checked equality through the runtime evaluator and kernel meter"
type: FR
relationships:
  - target: ix://agent-ix/quire-exact/FR-375
    type: depends_on
---
# FR-277: Exercise checked equality through the runtime evaluator and kernel meter

## Description

The runtime SHALL consume `quire_semantic_value::declaration::CheckedEquality::evaluate` over `quire_exact::Value` and define no equality evaluator or pair walker of its own. The QSV-owned evaluator SHALL call `quire_exact::planned_equality_with_plan` for its planned schedule, consuming one kernel plan for its relation and `Meter` effects. This requirement places the end-to-end consumer controls formerly in quire-exact FR-375-AC-1 through AC-5 and AC-8 at the runtime's dependency boundary; QSV owns the executable evaluator.

## Inputs

- Two separately constructed, admitted depth-four `quire_exact::Value` sequences (`CollectionKind::Sequence`) with the same checked composite declaration identity, 16 visited value occurrences per operand and a 17-pair kernel plan; the unequal control differs at its first head. RT's integration tests construct these values through kernel constructors and do not import QSL `e21_operands`.
- A runtime meter with the stated charge limits or injected denial.

## Outputs

- The existing typed equality outcome and admitted meter charges.

## Behavior

RT SHALL consume QSV's checked equality and `quire_exact::Value` directly, delete its distinct `Value`, `CheckedEquality`, `planned_equality` and `plan_pairs`, and define no adapter or re-export for them. QSV's evaluator SHALL preserve the kernel plan's relation and complete pair schedule through its direct `planned_equality_with_plan` call. The meter SHALL admit only successful charges, in the order plan-form, plan, each pair, and result retention. QSV checked construction SHALL reject unlike static composite declarations before evaluation. The remaining runtime criteria are the Kani proof claim in FR-277-AC-7, owned by titan:kani:plan, and the ownership inspection in FR-277-AC-8.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-277-AC-7 | The IR-241 Kani claim remains `plan_equality(left, right).pair_events() == count_of_EqualityPair_charges_from_the_same_real_CheckedEquality::evaluate(left, right, meter)` for both equal and first-head-unequal bounded operands, with the existing exact operand bound and proof ceiling. A plan-only equality or duplicated QSL walker does not satisfy this criterion. | Test |
| FR-277-AC-8 | Inspection of the locked QSV `CheckedEquality::evaluate` path shows a direct call to `quire_exact::planned_equality_with_plan` on the evaluated operands and use of that call's Boolean relation; the same call admits the complete 17-event pair schedule through its meter, and RT defines no local `CheckedEquality` or pair walker. A separate kernel call does not establish this criterion. | Inspection (TC-203) |

## Dependencies

- [quire-exact FR-375](ix://agent-ix/quire-exact/FR-375) owns equality planning and metering semantics.
- [IR-878](https://linear.app/agent-ix/issue/IR-878) tracks the owner allocation. FR-275 residue removal and kernel routing must land before FR-277 tests are tagged.

## Status

PARTIAL for RT: the locked QSV evaluator consumes the result-bearing kernel seam,
RT has deleted its local equality evaluator, pair walker and `Value`, and
`tests/ir882_equality_owner.rs` exercises AC-1 through AC-6 on that consumed
path. AC-8 additionally requires inspection of the locked QSV source and RT's
absent owner modules. AC-7 remains PLANNED in titan:kani:plan; the RT consumer
tests grant no Kani proof credit.
