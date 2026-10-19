---
id: FR-278
title: "Emit typed checked-invariant causes at runtime producer sites"
type: FR
relationships:
  - target: ix://agent-ix/quire-exact/FR-369
    type: depends_on
---
# FR-278: Emit typed checked-invariant causes at runtime producer sites

## Description

Every runtime checked-invariant producer SHALL emit the precise shared quire-exact `CheckedInvariantCause` assigned to its condition, retaining nested `IllTypedCause` payloads.

## Inputs

- Checked package, function, collection, composite, equality and quantity operations in runtime.

## Outputs

- Typed refusal or incomplete outcomes at the existing stop boundary.

## Behavior

The runtime SHALL use the quire-exact FR-369 cause table for the producer sites it owns. No producer SHALL flatten a typed nested cause to a message. Fuel exhaustion SHALL remain an `Incomplete` outcome.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-278-AC-1 | Every RT checked-package/function invariant producer returns its table cause: unknown name is `UnknownCheckedFunction`, a foreign expression is `ForeignCheckedExpression`, and each meter reborrow is `MeterBorrowConflict`. Runtime-managed calls use explicit frames and `work_units` fuel under RT FR-273; the final carrier has no `CallDepthExceeded` variant, and fuel exhaustion is `Incomplete` with `limit_kind: work_units`, never `CheckedInvariant`. | Test |
| FR-278-AC-2 | Every RT collection, composite, equality and quantity producer returns its table cause, including all six `operand_value` failures and the original kernel-owned `IllTypedCause` when a scheduled comparison or checked equality quantity conversion refuses; no site flattens a comparison or conversion error to a message. | Test |

## Dependencies

- [quire-exact FR-369](ix://agent-ix/quire-exact/FR-369) owns the shared cause vocabulary.
- [IR-878](https://linear.app/agent-ix/issue/IR-878) tracks the owner allocation.

## Status

PLANNED: runtime producer tests have not landed.
