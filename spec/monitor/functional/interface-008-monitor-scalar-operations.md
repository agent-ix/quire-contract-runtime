---
id: interface_008
title: "Exact scalar operations and lossless operating stops"
type: interface
relationships:
  - target: ix://agent-ix/quire-contract-runtime/interface_003
    type: references
  - target: ix://agent-ix/quire-contract-runtime/FR-006
    type: references
---
# [interface-008] Exact scalar operations and lossless operating stops

## Contract

```yaml
name: MonitorScalarOperations
ownership: quire-contract-runtime
feature: exact
associated_types: [Operation, StepBudget, StepResourceStop]
operations:
  - name: execute_integer_operation
    inputs: [program, budget]
    output: Value
    semantics: execute the shared exact integer operation under explicit native operating limits
invariants:
  - integer values are mathematical integers rather than host-width counters
  - the shared leaf owns arithmetic, ordering and native charge schedules
  - a native operating shortage is never a semantic Incomplete outcome
  - an unavailable exact charge is retained without narrowing
```

## Operations

### execute_integer_operation

The closed ordinary capabilities `IntegerAdd { left, right }`,
`IntegerSubtract { left, right }`, `IntegerMultiply { left, right }`,
`IntegerNegate { value }`, and `IntegerOrder { operator, left, right }` carry
resolved expression addresses. `operator` is the shared exact `OrderingOperator`.
Binary operands evaluate left before right; unary operands evaluate once.
Arithmetic delegates to `evaluate_integer_arithmetic` with the declared result
interval, if any. Ordering delegates to `order_numbers`. No host-width arithmetic
or independent numeric evaluator substitutes for either operation.

Each `StepBudget` supplies the native `ScalarLimits` as its required `scalar`
member, alongside explicit interpreter work/stack/scratch limits. There is no
implicit default or semantic maximum integer size. A native meter is fresh per
invocation; its cumulative charges cover all selected scalar operations of that
invocation, rather than resetting at each expression. Inactive branches perform
no scalar operation or charge.

`StepResource::Scalar` retains the original shared `LimitKind`.
`StepResourceStop.requested` is an exact shared Integer for every resource,
including charges larger than a host integer. Native scalar stops additionally
retain their original shared `Incomplete` record, including charge point; the
record is operating evidence, never an emitted temporal Outcome. Non-completed
native results cannot become values or property verdicts. Native refusal or
undefined results retain their original typed cause and source site as terminal
executor invariant evidence. No partial arithmetic value is exposed.

Computed values use canonical shared `Value` storage, not a second scalar model.
Their live addresses consume scratch. A resource stop releases invocation
scratch and commits no state or input; later invocation can reuse the machine.
Program semantic capacities, State schema, root bindings, static function
identity and temporal outcome kinds are unchanged.

| Param | Type | Multiplicity | Constraints |
| --- | --- | --- | --- |
| program | Program | 1..1 | |
| budget | StepBudget | 1..1 | |

Returns: Value[1..1]

## Features

| Feature | Kind |
| --- | --- |
| execute_integer_operation | operation |

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| interface-008-AC-1 | Arithmetic and all four order relations agree with independent exact expectations, including integers beyond host width and bounded result refusal. | Test |
| interface-008-AC-2 | Explicit native limits apply cumulatively across selected operations; inactive branches do not charge, and denial retains its exact request, native cause and source evidence without a value or temporal verdict. | Test |
| interface-008-AC-3 | Computed values consume scratch; ordinary interpreter stops retain their exact units, and a stopped invocation cannot commit state or contaminate later execution. | Test |
