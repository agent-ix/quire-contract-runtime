---
id: Task-002
title: "Scalar operator families"
type: Task
status: done
track: A
priority: P0
relationships:
  - target: ix://agent-ix/quire-contract-runtime/FR-007
    type: references
---
# Task-002: Scalar operator families

## Scope

Implement integer division and modulus profiles, `Decimal[..]`, IEEE 754-2019 profiles,
Unicode 17 text, enums, unit graphs and quantities, and the 5d88578 metered integer arithmetic,
rational arithmetic, ordering and Boolean connectives. Every intermediate materialized before its
charge is bounded by operands already admitted.

## Completion Evidence

TC-023 in `tests/exact_arithmetic.rs`; the family agreement tests of Task-003.
