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

The runtime SHALL use the quire-exact FR-369 cause table for the producer sites it owns. No producer SHALL flatten a typed nested cause to a message. Fuel exhaustion SHALL remain an `Incomplete` outcome. FR-278-AC-1 overlaps the already tagged FR-273-AC-8/TC-194 function producer tests. FR-278-AC-2 remains planned for RT-owned collection and composite producers until those producers leave the runtime. Its six former `operand_value` assertions concern equality and quantity, not collection producers: quire-semantic-value FR-122-AC-1 owns the five reached refusal cases under TC-906, and FR-122-AC-4 owns inspection of non-exact placement. The no-`CallDepthExceeded` carrier shape is owned by quire-exact FR-369-AC-1 and gated on the IR-497 cutover, while RT FR-273 owns explicit frames and fuel.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-278-AC-1 | Every RT checked-package/function invariant producer returns its table cause: unknown name is `UnknownCheckedFunction`, a foreign expression is `ForeignCheckedExpression`, and each meter reborrow is `MeterBorrowConflict`. Runtime-managed calls use explicit frames and `work_units` fuel under RT FR-273; the final carrier has no `CallDepthExceeded` variant, and fuel exhaustion is `Incomplete` with `limit_kind: work_units`, never `CheckedInvariant`. | Test |
| FR-278-AC-2 | Every collection or composite producer still owned by RT returns its assigned typed cause without flattening a nested cause to a message. Once those producers move to their owning crates, RT retains no duplicate producer and records the owner evidence. Checked equality and quantity producer controls belong to quire-semantic-value FR-122-AC-1 and AC-4; their six former `operand_value` assertions are not RT collection or composite evidence. | Test |

## Dependencies

- [quire-exact FR-369](ix://agent-ix/quire-exact/FR-369) owns the shared cause vocabulary.
- [quire-semantic-value FR-122](ix://agent-ix/quire-semantic-value/FR-122) owns checked equality producer causes under AC-1 and AC-4.
- [IR-878](https://linear.app/agent-ix/issue/IR-878) tracks the owner allocation.

## Status

PARTIAL: FR-278-AC-1 has direct tags on the retained TC-194 function controls
for foreign expressions, missing children, meter reborrows, host re-entry,
invalid child arguments and explicit-frame work-fuel exhaustion. The kernel
API package adds a successful public-cause import and a failing
`CallDepthExceeded` import, executed by the existing MSRV doctest gate against
the same resolved kernel. Together these controls cover the function/fuel and
final-carrier clauses without replacing a native refusal with a vocabulary grep.

FR-278-AC-2 remains planned: moving a producer out of RT does not make the
former equality/quantity assertions into collection or composite evidence.
Its complete owner-evidence mapping is still required. No collection/composite
criterion or TC-204-wide completion is claimed here, and the deferred evaluator
cutover is unchanged.
