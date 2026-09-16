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
Unicode 17 text, enums, unit graphs and quantities, and the 7d7943a metered integer arithmetic,
rational arithmetic, ordering and Boolean connectives. Every arithmetic, normalize, rounding,
retain-upscale, unit-event and target-domain amount is derived from operands and charged before any
intermediate or result is allocated.

## Completion Evidence

TC-023 in `tests/exact_arithmetic.rs`; the family agreement tests of Task-003.
