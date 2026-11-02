---
id: interface_009
title: "Canonical composite construction in monitor entries"
type: interface
relationships:
  - target: ix://agent-ix/quire-contract-runtime/interface_002
    type: references
  - target: ix://agent-ix/quire-contract-runtime/interface_003
    type: references
  - target: ix://agent-ix/quire-contract-runtime/interface_008
    type: references
---
# [interface-009] Canonical composite construction in monitor entries

## Contract

```yaml
name: MonitorCompositeConstruction
ownership: quire-contract-runtime
feature: exact
associated_types: [Operation, ProgramParts, Program]
operations:
  - name: construct_composite
    inputs: [program, budget]
    output: Value
    semantics: construct native canonical record or tuple slots from checked ordinary operands
invariants:
  - the result annotation retains the original admitted nominal declaration
  - named members retain their original shared identities
  - a resolved storage position is not a member identity
  - no alternate record or State value model is introduced
```

## Operations

### construct_composite

`Record { fields: Vec<(quire_exact::MemberId, ExpressionId)> }` constructs a
record whose required fields are all present. The result annotation identifies
the original admitted record declaration. Every supplied member must be owned by
that declaration and cover one declared required field exactly once. Missing,
duplicate, foreign or unknown members, optional declarations and nonrecord
result annotations refuse preparation. Optional-presence construction is a
separate capability and is not inferred from omission.

Record operands execute in declaration order, irrespective of supplied field
order. Preparation resolves only the requested declaration and retains private
operand addresses for that order. `Tuple { positions: Vec<ExpressionId> }`
constructs the tuple identified by its result annotation, requires its exact
arity, and evaluates operands in declared position order.

After each operand, shared equality facts avoid redundant membership walks;
nonidentical type boundaries use native bounded supplied membership before a
later operand executes. A stopped operand or conformance check prevents all
later operands from executing. The original native failure and source evidence
remain available.

Construction materializes already-checked slots through the shared canonical
constructor and delegates the composite retention charge to the shared exact
`retain_composite`. Native shortages remain operating stops with their original
cause. Slot storage and canonical result addresses consume explicit scratch;
storage refusal or exhaustion exposes no partial composite and commits no state.
Continuation work is iterative and proportional to selected operands, not a
package census or a recursive subtree copy. Arc-backed canonical children retain
their original immutable owners. No property verdict is synthesized.

| Param | Type | Multiplicity | Constraints |
| --- | --- | --- | --- |
| program | Program | 1..1 | |
| budget | StepBudget | 1..1 | |

Returns: Value[1..1]

## Features

| Feature | Kind |
| --- | --- |
| construct_composite | operation |

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| interface-009-AC-1 | Canonical record and tuple construction preserve declaration order and original member/value owners; reversed source member order does not change execution order. | Test |
| interface-009-AC-2 | Missing, duplicate, foreign or unknown required members, optional record declarations, wrong result shapes and wrong tuple arity refuse before operand execution. | Test |
| interface-009-AC-3 | An earlier stopped operand or bounded membership check prevents later execution; slot storage and native composite retention denials retain their typed operating channels without a partial value or state commit. | Test |
