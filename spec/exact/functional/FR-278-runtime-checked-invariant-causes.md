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

The runtime SHALL use the quire-exact FR-369 cause table for the producer sites it owns. No producer SHALL flatten a typed nested cause to a message. Fuel exhaustion SHALL remain an `Incomplete` outcome. FR-278-AC-1 overlaps the already tagged FR-273-AC-8/TC-194 function producer tests. The six former runtime `operand_value` assertions concern equality and quantity, not collection producers; quire-semantic-value FR-122-AC-1 owns the five reached refusal cases under TC-906, and FR-122-AC-4 owns inspection of non-exact placement. The no-`CallDepthExceeded` carrier shape is owned by quire-exact FR-369-AC-1 and gated on the IR-497 cutover, while RT FR-273 owns explicit frames and fuel.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-278-AC-1 | Every RT checked-package/function invariant producer returns its table cause: unknown name is `UnknownCheckedFunction`, a foreign expression is `ForeignCheckedExpression`, and each meter reborrow is `MeterBorrowConflict`. Runtime-managed calls use explicit frames and `work_units` fuel under RT FR-273; the final carrier has no `CallDepthExceeded` variant, and fuel exhaustion is `Incomplete` with `limit_kind: work_units`, never `CheckedInvariant`. | Test |

## Dependencies

- [quire-exact FR-369](ix://agent-ix/quire-exact/FR-369) owns the shared cause vocabulary.
- [quire-semantic-value FR-122](ix://agent-ix/quire-semantic-value/FR-122) owns checked equality producer causes under AC-1 and AC-4.
- [IR-878](https://linear.app/agent-ix/issue/IR-878) tracks the owner allocation.

## Status

PLANNED for FR-278-AC-1 tags: RT tests already assert overlapping function producer paths under FR-273-AC-8/TC-194. Rebind those tests when the typed carrier lands. The kernel-owned no-`CallDepthExceeded` carrier gate remains quire-exact FR-369-AC-1 at IR-497.
