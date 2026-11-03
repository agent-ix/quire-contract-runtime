---
id: interface_005
title: "Ordinary Boolean operations in monitor entries"
type: interface
relationships:
  - target: ix://agent-ix/quire-contract-runtime/interface_002
    type: references
  - target: ix://agent-ix/quire-contract-runtime/interface_003
    type: references
---
# [interface-005] Ordinary Boolean operations in monitor entries

## Contract

```yaml
name: MonitorBooleanOperations
ownership: quire-contract-runtime
feature: exact
associated_types: [Operation, ProgramParts, Program]
operations:
  - name: prepare_boolean_operations
    inputs: [parts]
    output: Program
    semantics: check ordinary Boolean operands and retain original expression locations
invariants:
  - only canonical Boolean values are accepted
  - binary operations evaluate the left operand first
  - unselected right operands are not evaluated
  - these are ordinary value operations and not temporal formula tags
```

## Operations

### prepare_boolean_operations

The existing Operation enum gains BooleanNot with one `value: ExpressionId`, and
BooleanAnd/BooleanOr with `left: ExpressionId` and `right: ExpressionId`. The
existing Program constructor requires Boolean result and operand annotations;
wrong annotations refuse before execution. No truthy conversion is available.

BooleanNot returns false for true and true for false. BooleanAnd evaluates right
only when left is true. BooleanOr evaluates right only when left is false. A
right-hand call, resource stop or invariant failure is therefore observable only
when selected. Each evaluated value is a canonical shared Boolean. The machine
can address its two immutable canonical Boolean constants for a computed result;
there is no alternate Value, Outcome or type vocabulary.

Execution uses explicit continuations and the same work, stack and scratch
budgets as the surrounding entry. With the required native scalar budget
(interface-008), each decided connective result delegates its
`boolean.result-retain` charge to shared Exact `retain_boolean`. A short circuit
retains its decided result once and makes no right-operand charge. Native
shortages retain their original typed cause as operating stops. It introduces no
callback, temporal opcode,
source identity, State field, root binding or public carrier. Each expression's
existing original key/body path identifies its operation. These capabilities
serve Boolean expressions in the admitted ordinary QSL entry/helper closure;
passing their controls does not qualify a temporal monitor producer or result.

| Param | Type | Multiplicity | Constraints |
| --- | --- | --- | --- |
| parts | ProgramParts | 1..1 | |

Returns: Program[1..1]

## Features

| Feature | Kind |
| --- | --- |
| prepare_boolean_operations | operation |

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| interface-005-AC-1 | Every BooleanNot truth-table row returns the opposite Boolean and both binary truth tables return their independently expected value. | Test |
| interface-005-AC-2 | False And and true Or skip a recursive right operand, while true And and false Or reach its typed operating stop. | Test |
| interface-005-AC-3 | Non-Boolean operand or result annotations refuse construction. Selected work/stack/scratch exhaustion retains its operating-stop channel and does not become a Boolean result. | Test |
