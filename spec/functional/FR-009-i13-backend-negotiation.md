---
id: FR-009
title: "Dispose each I13 backend negotiation item independently"
type: FR
relationships:
  - target: ix://agent-ix/quire-contract-runtime/FR-006
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-147
    type: implements
  - target: ix://agent-ix/quire-specification/FR-148
    type: implements
---
# FR-009: Dispose each I13 backend negotiation item independently

## Description

When a backend asks whether it may execute a package's integer-division or IEEE items,
the runtime shall return one disposition per item, decided from the item's declared requirement and
the backend's declared capabilities alone.
The runtime shall neither evaluate the item nor alter the item, the package or any meter.

## Inputs

- For integer division: a slice of `IntegerDivisionConsumer`, each either `Mathematical` or
  `Finite(IntegerDivisionBounds)` with optional `operand`, `intermediate` and `result` bounds.
- For IEEE: a slice of `IeeeItemRequirement` (width, operation, rounding, whether finite execution
  needs a resource proof) and one `IeeeBackendCapabilities` (supported widths, operations and
  rounding directions, whether the profile's NaN and flag policy is implemented, whether a finite
  resource proof can be discharged).

## Outputs

- A `Vec<IntegerDivisionDisposition>` or `Vec<IeeeDisposition>` with exactly one element per input
  item, in input order.
- `IeeeDisposition::Unsupported` carries the typed `IeeeUnsupportedCause` naming what the backend
  lacks.

## Behavior

- Negotiation is a static per-item decision. It never evaluates an item, never narrows
  mathematical integers, never selects a substitute evaluator, never changes package admission and
  charges nothing: no `Meter` is an input or an output.
- The result sequence is positional. The disposition at index `i` is the disposition of the item at
  index `i`, and the length of the result equals the length of the input, including for the empty
  input. No item's disposition depends on another item's.
- `requires-bound` and `unsupported` are provider dispositions, not evaluator outcomes. They have
  no `Outcome` variant and are never returned beside one.
- An `IntegerDivisionConsumer::Mathematical` item is `Supported`: unbounded mathematical integers
  need no bound. A `Finite` item is `Supported` only when all three of `operand`, `intermediate`
  and `result` are present, and `RequiresBound` when any one of them is absent. Which bound is
  absent does not change the disposition.
- An IEEE item is checked against the backend in one fixed cause order, first failure wins:
  unsupported width, then unsupported operation, then — for rounding operations only — unsupported
  rounding direction, then an unimplemented exceptional policy. A non-rounding operation's
  `rounding` field is not read and never causes `Unsupported(Rounding(..))`.
- Only after every `unsupported` cause is excluded is `requires_finite_proof` consulted: an item
  that needs a finite resource proof a backend cannot discharge is `RequiresBound`. An item whose
  backend lacks both a capability and the finite proof is `Unsupported` with the capability cause,
  never `RequiresBound`.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-009-AC-1 | For every input slice, both negotiators return exactly one disposition per item in input order, including for the empty slice, and permuting the input permutes the result identically. | Test (TC-030) |
| FR-009-AC-2 | A `Mathematical` consumer is `Supported`; a `Finite` consumer is `Supported` exactly when `operand`, `intermediate` and `result` are all present, and `RequiresBound` for each of the seven incomplete bound subsets. | Test (TC-030) |
| FR-009-AC-3 | IEEE causes are decided in the order width, operation, rounding, exceptional policy, first failure wins: an item failing two checks reports the earlier cause, and a non-rounding operation with an unsupported rounding direction is not `Unsupported(Rounding(..))`. | Test (TC-030) |
| FR-009-AC-4 | `requires_finite_proof` yields `RequiresBound` only when no `unsupported` cause applies; an item that both lacks a capability and needs an undischargeable proof reports the capability cause. | Test (TC-030) |
| FR-009-AC-5 | Neither negotiator takes or mutates a `Meter`, and no disposition is convertible into an `Outcome` variant. | Inspection (TC-030) |

## Dependencies

- **Upstream**: [FR-006](./FR-006-exact-outcomes-and-accounting.md);
  `ix://agent-ix/quire-specification` at `7d7943a`.
