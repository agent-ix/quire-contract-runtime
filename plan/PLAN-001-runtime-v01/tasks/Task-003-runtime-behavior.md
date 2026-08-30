---
id: Task-003
title: "Operators adapters and accounting"
type: Task
status: done
track: A
priority: P0
relationships:
  - target: ix://agent-ix/quire-contract-runtime/FR-002
    type: references
  - target: ix://agent-ix/quire-contract-runtime/FR-003
    type: references
  - target: ix://agent-ix/quire-contract-runtime/FR-004
    type: references
---
# Task-003: Operators, adapters, and accounting

## Scope

Implement safe Boolean and definedness operators, proptest adaptation with optional census recording,
and opaque saturating campaign accounting.

## Deliverables

- Safe operator and optional adapter modules.
- Typed identity mismatch and immutable external accounting state.

## Completion Evidence

TC-002, TC-003, TC-004, and TC-006 pass across the supported feature matrix and Rust 1.75 core check.
