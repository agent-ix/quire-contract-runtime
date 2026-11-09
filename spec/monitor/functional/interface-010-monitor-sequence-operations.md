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
    semantics: execute admitted sequence construction, size, flattening, a scoped left fold, or a scoped map or filter
invariants:
  - native CollectionType and canonical Value remain the only sequence model
  - sequence occurrence order is retained without sorting
  - fold, map and filter item slots are lexical addresses in the original function
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

`SequenceMap { sequence, item, body }` and `SequenceFilter { sequence, item,
predicate }` are the scoped, order-preserving element operations that the QSL
`quire.op.collection.map` and `quire.op.collection.filter` lowerings use. Each
carries the sequence ExpressionId, one current-function item LocalId and one
scoped ExpressionId (`body` or `predicate`). The sequence expression evaluates
once, before any visit. The item slot has the sequence's admitted element type,
belongs to the current function, is declared once among that function's lexical
declarations (the same compact unique addressing as a fold slot) and is active
only while the body or predicate runs; it creates no helper key or semantic
identity. The body or predicate sees ordinary lexical captures, an inner scope
that reuses the same slot (a nested map, filter or fold over the item, or a
recursive invocation) restores the outer binding on completion, and a recursive
invocation cannot read another invocation's slot.

Occurrences are visited once each in source order. Before each body or
predicate, the native `collection.visit` is charged and the item is bound to
that occurrence. `SequenceMap` evaluates its body for every occurrence, in
order, and its result is the Sequence of the body results in the same order and
with the same length; duplicates are kept, and the result's element type is the
body's type (the result annotation is a Sequence whose element type is the body's
type; any other result annotation is a TypeMismatch refusal at construction).
`SequenceFilter` evaluates its predicate for every occurrence, in order, and its
result is the Sequence of those original occurrences whose predicate is true, in
their original relative order, with the same element type as the operand; the
predicate must be Boolean (a non-Boolean predicate type is a TypeMismatch
refusal at construction), is never short-circuited, and its value never changes
which later occurrences are visited. An empty sequence yields an empty result
without executing the body or predicate. Map and filter results are formed
through shared Exact `form` from the retained occurrences, so a declared
cardinality bound on the result type and the native element/retain charges apply
as for `Sequence` and `SequenceFlatten`.

A non-Sequence operand refuses at construction with `InvalidBinding`, like a
fold. Missing, duplicate, sparse or foreign item addresses refuse with
`InvalidBinding`. The result storage for at most one value per source occurrence
is reserved once from the same StepBudget scratch element and byte accounting as
other native sequence formation, before the first visit, and two scratch
elements stay charged to the suspended operation (the source owner and the result
owner) until it completes; the per-occurrence item address and each completed
body or predicate value count against scratch like fold's. A work, stack, scratch
or native scalar stop during the traversal retains its existing typed channel,
exposes no partial result, commits no monitor state and leaves later invocation
state clean. No new refusal or failure codes are introduced.

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
| interface-010-AC-4 | Map returns the body results and filter returns the true-predicate occurrences, each in occurrence order with duplicates kept, empty input yields an empty result, the item scope nests and restores, and each result equals the equivalent left fold. | Test |
| interface-010-AC-5 | A non-Sequence operand, non-Boolean predicate, mistyped result annotation and invalid item address refuse at construction, and a StepBudget stop during map or filter exposes no partial value. | Test |
