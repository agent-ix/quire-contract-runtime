---
id: interface_004
title: "Lexical bindings in the retained monitor program"
type: interface
relationships:
  - target: ix://agent-ix/quire-contract-runtime/interface_002
    type: references
  - target: ix://agent-ix/quire-contract-runtime/interface_003
    type: references
  - target: ix://agent-ix/quire-contract-runtime/FR-276
    type: references
---
# [interface-004] Lexical bindings in the retained monitor program

## Contract

```yaml
name: MonitorLexicalBindings
ownership: quire-contract-runtime
feature: exact
associated_types:
  - LocalId
  - Operation
  - ProgramParts
  - Program
operations:
  - name: prepare_lexical_program
    inputs: [parts]
    output: Program
    semantics: validate compact function-qualified binding addresses in the retained closure
invariants:
  - local addresses are not semantic identities
  - operands retain their canonical values and original source keys
  - every invocation has its own active bindings
  - an unselected branch creates no active binding
  - a resource stop commits no monitor state
```

## Operations

### prepare_lexical_program

The existing `Program::try_new(parts)` accepts two additional ordinary operations:

- `Let { binding: LocalId, value: ExpressionId, body: ExpressionId }` evaluates
  value once, activates its operand address while evaluating body, and returns
  body's result. The previous activation of the same slot is restored afterward.
- `Local(LocalId)` reads the binding active in this exact function invocation.

`LocalId { function: FunctionId, position: usize }` is a compact arena address,
not a source key, parameter number, event position or host semantic limit. A
function's distinct binding declarations use positions 0 through N-1. Each
position has exactly one Let declaration; repeated execution and recursion reuse
that declaration with separate invocation activations. A local reference needs
that declaration in its owning function. Missing, duplicate, sparse or foreign
binding declarations return a typed construction refusal. An inactive local
encountered during execution is an executor invariant failure, not a value.

The canonical type of a binding comes from its value expression. A local read
with a different result annotation reaches shared bounded membership admission;
there is no second type checker or mirrored value representation. Let returns
its body's annotated type through the existing shared conformance boundary.

Binding index preparation is linear in the retained expressions and functions. Execution
uses direct slot lookup, an invocation token from already charged work, and
explicit bind/restore continuations. It does not scan all functions or all slots
on each step. Cleanup visits only slots activated by that invocation chain;
untouched slots in other functions do not add per-step cleanup work. Active
binding operands count toward the existing scratch-element ceiling, including
outer operands retained while a recursive call shadows a slot. No new budget
field or semantic capacity is introduced.

The program keeps the same ProgramParts, Layout, Function and Expression fields.
There is no synthetic helper declaration, new source identity, callback, closure
DTO or source parser. This is the lexical portion of the ordinary operations
emitted by QSL FR-251; it does not qualify temporal entry semantics or implement
a temporal operator. The remaining constructor, arithmetic, sequence and fold
capabilities and the gap path retain their separate Phase 2 work.

| Param | Type | Multiplicity | Constraints |
| --- | --- | --- | --- |
| parts | ProgramParts | 1..1 | |

Returns: Program[1..1]

## Features

| Feature | Kind |
| --- | --- |
| prepare_lexical_program | operation |

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| interface-004-AC-1 | Missing, duplicate, sparse and foreign local declarations refuse construction, while compact function-qualified bindings retain their original source locations. | Test |
| interface-004-AC-2 | A selected Let evaluates its value once and returns its scoped body's result; a lazy unselected binding is not activated. A differently annotated local uses shared admission and preserves its native failure. | Test |
| interface-004-AC-3 | Nested calls and recursive invocations cannot read an outer invocation's local as their own. A bind restores the prior invocation's slot on return, and an inactive read is a typed invariant failure. | Test |
| interface-004-AC-4 | Binding operands respect the scratch ceiling, including a shadowed outer operand. A stopped invocation clears only touched slots and cannot leak a binding into a subsequent invocation; deep binding chains use the explicit stack. | Test |
