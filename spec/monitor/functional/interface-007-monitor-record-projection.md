---
id: interface_007
title: "Required record member reads in monitor entries"
type: interface
relationships:
  - target: ix://agent-ix/quire-contract-runtime/interface_002
    type: references
  - target: ix://agent-ix/quire-contract-runtime/interface_003
    type: references
---
# [interface-007] Required record member reads in monitor entries

## Contract

```yaml
name: MonitorRecordProjection
ownership: quire-contract-runtime
feature: exact
associated_types: [Operation, ProgramParts, Program]
operations:
  - name: prepare_record_projection
    inputs: [parts]
    output: Program
    semantics: retain a checked required record member and resolve its storage address
invariants:
  - a shared admitted member identity is retained without replacement
  - a physical slot position is private storage metadata
  - projection retains the original immutable canonical owner
```

## Operations

### prepare_record_projection

`RecordField { value: ExpressionId, member: quire_exact::MemberId }` reads a
required field of the receiver's admitted record declaration. This capability
carries the original node-owned member identity. Its owner must be the receiver
annotation's record declaration; effective-model attributes and optional-field
presence are separate ordinary capabilities. They are not inferred here.

Construction refuses a foreign owner, nonrecord receiver, absent or optional
field, or result type unequal to the declared field type. Relevant record
requests are grouped before scanning their declarations; reads of the annotated declaration do not
repeat a package census or a field scan during execution. The retained private
slot position never replaces the public member identity.

The receiver evaluates once. Execution retains its canonical owner and a slot
address, using the shared nominal conformance and bounded membership authorities
at nonidentical type boundaries. A conforming dynamic record resolves the same
field identifier in its own declaration order, charging each inspected field to
work; it never reuses a supertype storage ordinal. It does not copy the selected subtree or
reconstruct a record. Temporary projection ownership and scratch charging follow
the tuple projection contract. No state is committed on a resource stop.

| Param | Type | Multiplicity | Constraints |
| --- | --- | --- | --- |
| parts | ProgramParts | 1..1 | |

Returns: Program[1..1]

## Features

| Feature | Kind |
| --- | --- |
| prepare_record_projection | operation |

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| interface-007-AC-1 | Repeated reads use the declared member identity and retain the selected canonical value after temporary owners drop. | Test |
| interface-007-AC-2 | Foreign owners, tuples, absent or optional fields, and mismatched result types refuse before execution. | Test |
| interface-007-AC-3 | Projection addresses consume scratch and explicit continuations preserve source evidence without subtree copies or repeated declaration scans. | Test |
