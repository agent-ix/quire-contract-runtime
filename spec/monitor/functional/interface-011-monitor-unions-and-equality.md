---
id: interface_011
title: "Verified union operations and shared checked equality"
type: interface
relationships:
  - target: ix://agent-ix/quire-contract-runtime/interface_002
    type: references
  - target: ix://agent-ix/quire-contract-runtime/interface_008
    type: references
---
# [interface-011] Verified union operations and shared checked equality

## Contract

```yaml
name: MonitorUnionsAndEquality
ownership: quire-contract-runtime
feature: exact
associated_types: [Operation, ProgramParts, Program]
operations:
  - name: execute_union_or_equality
    inputs: [program, budget]
    output: Value
    semantics: retain verified native union members and delegate equality to its original checked native plan
invariants:
  - original native VariantId and UnionMember bindings are retained
  - no tag, status enum or member identity is manufactured
  - native CheckedEquality owns comparison semantics and its selected metadata
  - outcomes remain ordinary canonical values without a kind allow-list
```

## Operations

### execute_union_or_equality

`Union { variant: quire_exact::VariantId, positions: Vec<ExpressionId> }`
constructs the admitted union named by its result annotation. The native type
environment must already seal that exact member to its verified original
UnionMember; an unresolved or foreign binding refuses rather than synthesizing a
member from a label. Payload arity is checked before execution. Positions execute
in declaration order, with native bounded membership on nonidentical boundaries
before a later position runs. Shared canonical union materialization and native
composite retention preserve the original member and payload owners.

`UnionIs { value, variant }` compares the original full native variant identity
and returns a canonical Boolean. `UnionPayload { value, variant, position }`
reads the original member's positional payload without copying a subtree. The
source admission proves its selected branch; a mismatched evaluated member is
an executor invariant failure, never a default payload or property verdict.

`Equality { left, right, checked: quire_semantic_value::declaration::CheckedEquality }`
retains the original shared checked equality, including its operator, conversions,
selected unit facts and filtered enum-member metadata. Operands evaluate left
before right, and evaluation calls that plan against the retained native type
environment and cumulative native meter. The compiler supplies the same admitted
environment that checked it. RT does not reconstruct its schedule, scan a package
enum inventory or infer one from runtime values. Native outcomes and resource
stops retain their typed cause and original source evidence.

Union payloads, including ordinary monitor Outcome and opaque activation-status
values, are not parsed into a second RT enum. Temporal limit names such as
`atom_projection` remain data of the admitted program; they are not native scalar
budget kinds. All canonical owners survive preparation drop and invocation
cleanup. A stopped operation exposes no partial value and commits no state.

| Param | Type | Multiplicity | Constraints |
| --- | --- | --- | --- |
| program | Program | 1..1 | |
| budget | StepBudget | 1..1 | |

Returns: Value[1..1]

## Features

| Feature | Kind |
| --- | --- |
| execute_union_or_equality | operation |

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| interface-011-AC-1 | Verified union construction, full-identity case tests and payload reads preserve member/child ownership; foreign, unsealed or malformed bindings refuse and an inactive payload read cannot invent a value. | Test |
| interface-011-AC-2 | Shared checked equality produces independently expected equal/unequal results for selected ordinary values and retains the original conversion/enum/unit semantics without a replacement status or comparison model. | Test |
| interface-011-AC-3 | Earlier stopped payloads prevent later execution; native retention/equality shortages and scratch denial retain exact typed evidence without a partial result or state commit. | Test |
