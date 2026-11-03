---
id: interface_006
title: "Canonical tuple projection in monitor entries"
type: interface
relationships:
  - target: ix://agent-ix/quire-contract-runtime/interface_002
    type: references
  - target: ix://agent-ix/quire-contract-runtime/interface_003
    type: references
---
# [interface-006] Canonical tuple projection in monitor entries

## Contract

```yaml
name: MonitorTupleProjection
ownership: quire-contract-runtime
feature: exact
associated_types: [Operation, ProgramParts, Program]
operations:
  - name: prepare_tuple_projection
    inputs: [parts]
    output: Program
    semantics: resolve a canonical tuple position without a replacement tuple or Value model
invariants:
  - the source tuple's admitted declaration gives each position its meaning
  - an arena address is never a semantic member identity
  - projected values remain in canonical immutable ownership
  - no projected subtree is copied or recursively traversed merely to select it
```

## Operations

### prepare_tuple_projection

`TupleElement { value: ExpressionId, position: usize }` projects one required
position of an ordinary canonical tuple. The value annotation identifies the
admitted tuple declaration; its ordered position type must match the result
annotation. Unknown addresses, out-of-range positions, record receivers and
wrong result annotations refuse construction.

The receiver evaluates once. The machine retains the original canonical tuple
owner and a storage address; it neither reconstructs the tuple nor clones the
selected subtree. A conforming dynamic tuple uses the same positional meaning.
Shared type equality and native nominal conformance preserve already-admitted
slot type facts; any remaining nonidentical conformance boundary uses the shared
bounded membership walker. No second type checker is introduced.

Retained projection addresses count toward scratch until the invocation ends.
The root entry's canonical composite result is retained before temporary owners
are released. This allows projection to return a State or Step value after driver
preparation objects have dropped, without a dangling borrow or a new Value DTO.
Work/stack/scratch stops retain their existing channel and commit no state.
Tuple positions are not record member identities; named record projection is a
separate ordinary capability carrying the admitted shared member identity.

| Param | Type | Multiplicity | Constraints |
| --- | --- | --- | --- |
| parts | ProgramParts | 1..1 | |

Returns: Program[1..1]

## Features

| Feature | Kind |
| --- | --- |
| prepare_tuple_projection | operation |

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| interface-006-AC-1 | A projected canonical State remains valid after temporary projection owners and preparation objects drop; tuple position order determines the selected value. | Test |
| interface-006-AC-2 | Record receivers, missing positions and mismatched result annotations refuse before entry execution. | Test |
| interface-006-AC-3 | Retained projection addresses are charged to scratch; deep projections use the explicit machine, and source keys/body paths remain available without copying a subtree. | Test |
