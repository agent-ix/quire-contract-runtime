---
id: interface_010
title: "Canonical sequence operations and scoped folds"
type: interface
relationships:
  - target: ix://agent-ix/quire-contract-runtime/interface_003
    type: references
  - target: ix://agent-ix/quire-contract-runtime/interface_004
    type: references
  - target: ix://agent-ix/quire-contract-runtime/interface_008
    type: references
---
# [interface-010] Canonical sequence operations and scoped folds

## Contract

```yaml
name: MonitorSequenceOperations
ownership: quire-contract-runtime
feature: exact
associated_types: [Operation, LocalId, Program]
operations:
  - name: execute_sequence_operation
    inputs: [program, budget]
    output: Value
    semantics: execute admitted sequence construction, size, flattening or a scoped left fold
invariants:
  - native CollectionType and canonical Value remain the only sequence model
  - sequence occurrence order is retained without sorting
  - fold slots are lexical addresses in the original function
  - no append opcode or source callback is introduced
```

## Operations

### execute_sequence_operation

`Sequence { elements: Vec<ExpressionId> }` uses the result's shared sequence
CollectionType, evaluates elements in order, checks each nonidentical element
boundary with native bounded membership before the next element, and forms the
result through shared Exact `form`. Each element makes its native
`collection.element` charge before evaluation. This capability admits sequences
only; it does not silently reinterpret another collection kind.

`SequenceSize { value: ExpressionId }` returns the exact native Integer count
with the FR-145 scalar collection retain. `SequenceFlatten { value:
ExpressionId }` traverses outer then inner sequence occurrence order, makes the
FR-145 native visits and uses shared formation for the result. No package census,
key sort or recursive copying of canonical children is performed.

`SequenceFold { sequence, initial, accumulator, item, body }` carries three
ExpressionIds and two distinct LocalIds. The sequence expression evaluates once,
then the identity expression evaluates once, before any visit. The accumulator
slot has the result's admitted type; the item slot has the sequence's admitted
element type. Both slots belong to the current function and are active only
while its body runs. Their declaration positions are compact and unique among
that function's lexical declarations, with accumulator before item in the fold's
binder roles. They do not create a helper key or semantic identity.

Occurrences are visited in source order. Before each body, the native
`collection.visit` is charged, then the current accumulator and that occurrence
are bound. The completed body replaces the accumulator. The body sees ordinary
lexical captures, and recursive invocations cannot read another invocation's
slots. On completion, the original outer slots are restored and the native
collection accumulator-retain charge applies. An empty sequence returns only its
declared identity. These are the sequence semantics and charge schedules of
QSpec FR-145; no set/bag commutativity rule is guessed.

Canonical owners and operand addresses survive suspended continuations. Work,
stack, native scalar and scratch stops retain their existing typed channels,
expose no partial result and commit no monitor state. Invocation cleanup releases
only visited slot/temporary state; it does not sweep an unrelated package.

| Param | Type | Multiplicity | Constraints |
| --- | --- | --- | --- |
| program | Program | 1..1 | |
| budget | StepBudget | 1..1 | |

Returns: Value[1..1]

## Features

| Feature | Kind |
| --- | --- |
| execute_sequence_operation | operation |

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| interface-010-AC-1 | Construction, exact size and flattening preserve duplicates, occurrence order and canonical child ownership; wrong kinds, shapes and lexical slots refuse. | Test |
| interface-010-AC-2 | A noncommutative left fold has an independent expected result, empty input returns its declared identity, lexical captures remain scoped and recursive invocations restore outer bindings. | Test |
| interface-010-AC-3 | Native element/visit/formation/retain charges and explicit work/stack/scratch limits stop before later selected work, retain exact typed evidence, expose no partial value and leave later invocation state clean. | Test |
