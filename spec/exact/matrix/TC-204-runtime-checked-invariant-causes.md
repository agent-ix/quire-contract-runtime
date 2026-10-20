---
id: TC-204
title: "Runtime checked-invariant producer controls"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-runtime/FR-278
    type: verifies
---
# TC-204: Runtime checked-invariant producer controls

## Description

Bind typed-cause evidence to the producer sites the runtime still owns. Preserve the test case during the kernel move under FR-275-AC-14. The six former `operand_value` assertions cover equality and quantity, not collection or composite producers; quire-semantic-value FR-122-AC-1 and AC-4 own those controls after the cutover.

## Test Procedure

1. Reuse TC-194's foreign expression, unknown function and meter reborrow assertions and check their precise typed causes and prior admitted charges.
2. While RT owns collection and composite producers, exercise each reached checked-invariant cause and check that nested typed causes are retained. Record missing producer controls as planned rather than citing the six equality/quantity assertions as evidence.
3. Inspect quire-semantic-value FR-122-AC-1 and TC-906 for the five reached former `operand_value` causes, and FR-122-AC-4 for non-exact placement; inspect RT for absence of duplicate equality and quantity producers after the move.
4. Inspect quire-exact FR-369-AC-1 for absence of `CallDepthExceeded`; verify RT FR-273 fuel exhaustion remains `Incomplete`.

## Expected Results

Each reached RT producer reports its assigned typed cause without message flattening. Producer controls transferred out of RT have named owner evidence and no duplicate runtime implementation. Existing charge boundaries remain unchanged.

## Status

PLANNED: TC-194 overlaps function cases. RT collection and composite producer controls require distinct evidence until those producers move. Equality and quantity controls are owned by quire-semantic-value FR-122-AC-1 and AC-4.
