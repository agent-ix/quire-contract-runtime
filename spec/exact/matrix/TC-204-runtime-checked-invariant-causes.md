---
id: TC-204
title: "Retag runtime checked-invariant producer controls"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-runtime/FR-278
    type: verifies
---
# TC-204: Retag runtime checked-invariant producer controls

## Description

Bind the existing RT producer controls to FR-278 after the IR-497 typed carrier cutover. TC-194 already exercises FR-273-AC-8 function producer cases, and TC-026 exercises FR-008-AC-5 equality operand cases.

## Test Procedure

1. Reuse the existing foreign expression, unknown function and meter reborrow assertions in TC-194, checking their precise typed causes and prior admitted charges.
2. Reuse TC-026's six `operand_value` causes and check that scheduled comparison and checked equality quantity conversion preserve their original `IllTypedCause` payloads.
3. Inspect the quire-exact FR-369-AC-1 carrier gate for absence of `CallDepthExceeded`; verify RT FR-273 fuel exhaustion remains `Incomplete`.

## Expected Results

Each reached RT producer reports its assigned table cause without message flattening. Existing charge boundaries remain unchanged, while the kernel carrier and RT fuel follow their owning criteria.

## Status

PLANNED for FR-278 tags and any missing controls; overlapping tests already exist under TC-194 and TC-026.
