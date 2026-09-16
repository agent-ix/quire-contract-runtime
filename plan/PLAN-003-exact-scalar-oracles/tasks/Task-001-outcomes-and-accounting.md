---
id: Task-001
title: "Outcome envelope, meter and vocabularies"
type: Task
status: done
track: A
priority: P0
relationships:
  - target: ix://agent-ix/quire-contract-runtime/FR-006
    type: references
---
# Task-001: Outcome envelope, meter and vocabularies

## Scope

Add the `exact` feature (`alloc`, `num-bigint`, `num-integer`, `num-traits`,
`unicode-normalization`, all exact-pinned) with private modules re-exported from `exact`:
`Outcome`, closed `Refusal`/`Undefined` reasons, `IllTyped`, `ScalarLimits`, `LimitKind`,
`ChargePoint` spellings from QSpec 7d7943a, `Incomplete`, `InjectedDenial` and `Meter`.

## Completion Evidence

TC-016 and TC-017 in `tests/exact_outcomes.rs`; `tests/release_contract.rs` asserts the feature
line and keeps `src/exact/` out of the campaign-accounting census.
