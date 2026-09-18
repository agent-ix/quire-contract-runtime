---
id: FR-002
title: "Evaluate safe oracle operators"
type: FR
relationships:
  - target: ix://agent-ix/quire-contract-runtime/FR-001
    type: depends_on
  - target: ix://agent-ix/quire-contract-runtime/interface-001
    type: implements
---
# FR-002: Evaluate safe oracle operators

## Description

When generated code evaluates contract expressions, the runtime shall expose separately named
short-circuit and total Boolean operators plus definedness-safe access, arithmetic, and division.

## Inputs

- Boolean values or lazy Boolean operands.
- Options, slices, integer operands, and divisors.

## Outputs

- Boolean results or `Option` values that encode undefined operations.

## Behavior

- Short-circuit operators shall skip an operand when Boolean semantics permit it.
- Total operators shall evaluate each operand once, from left to right.
- Invalid indexing, overflow, underflow, zero division, signed minimum divided by negative one, and
  absent options shall return `None` rather than panic.
- The runtime shall implement `CheckedInteger` for exactly the twelve Rust built-in integer types — `u8`,
  `u16`, `u32`, `u64`, `u128`, `usize`, `i8`, `i16`, `i32`, `i64`, `i128`, `isize` — and each
  member shall agree with the primitive's own `checked_*` operation on every input.
- `CheckedInteger` shall remain sealed, so generated code cannot supply a panicking implementation
  of a definedness-safe operator.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-002-AC-1 | Truth-table tests cover all short-circuit and total operators. | Test (TC-002) |
| FR-002-AC-2 | Evaluation-count tests prove the distinct operand evaluation contracts. | Test (TC-002) |
| FR-002-AC-3 | Boundary tests and Kani harnesses cover every definedness helper family. | Test (TC-003) |
| FR-002-AC-4 | `CheckedInteger` is implemented for exactly the twelve built-in integer types, each member agrees with the primitive `checked_*` operation, and the trait cannot be implemented outside the crate. | Test (TC-003) |

## Dependencies

- **Upstream**: [FR-001](./FR-001-verdict-observation.md).
