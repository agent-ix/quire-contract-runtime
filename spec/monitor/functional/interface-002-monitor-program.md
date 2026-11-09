---
id: interface_002
title: "Closed monitor program construction"
type: interface
object: interface
relationships:
  - target: ix://agent-ix/quire-contract-runtime/FR-276
    type: references
---
# [interface-002] Closed monitor program construction

## Contract

```yaml
name: MonitorProgramConstruction
ownership: quire-contract-runtime
features:
  exact: [alloc]
associated_types:
  - ProgramParts
  - Program
  - Layout
  - Entries
  - Buffer
  - TypeBinding
  - Function
  - Expression
  - Operation
  - FunctionId
  - ExpressionId
  - ProgramRefusal
  - ProgramRefusalCode
operations:
  - name: construct_monitor_program
    inputs: [parts]
    output: Program
    semantics: move and validate the executable arenas without executing any entry
  - name: inspect_monitor_layout
    inputs: [program]
    output: Layout
    semantics: borrow retained root associations without rebinding or mutation
  - name: inspect_monitor_types
    inputs: [program]
    output: TypeEnvironment
    semantics: borrow the shared declaration environment from the retained owner
  - name: resolve_monitor_function
    inputs: [program, function]
    output: Function
    semantics: checked arena lookup; source identity remains the canonical NodeKey
  - name: resolve_monitor_expression
    inputs: [program, expression]
    output: Expression
    semantics: checked arena lookup; inline provenance retains the source NodeKey and body path
invariants:
  - IR and driver lower once into the runtime-owned representation
  - RT depends on neither IR nor QSL
  - no host callbacks, temporal formula reconstruction, mirror value types or second plan wire
  - source keys and exact values/types are the shared leaf types
  - Rc sharing retains only immutable program data
```

## Operations

### construct_monitor_program

`monitor::Program::try_new` consumes `ProgramParts`, checks executable integrity and
returns `Rc<Program>`. Parts move into the single owner; temporary driver preparation
storage is not borrowed. The owner keeps the selection context, complete source keys,
all supplied functions/expressions, shared declarations and exact premises. It runs no
initializer, creates no monitor state and emits no event or verdict.

| Param | Type | Multiplicity | Constraints |
| --- | --- | --- | --- |
| parts | ProgramParts | 1..1 | |

Returns: Program[1..1]

The Rust result is `Result<Rc<Program>, ProgramRefusal>`: failure takes the typed
ProgramRefusal channel and returns no Program. The table describes the successful value.

The source-language reader and driver own semantic admission and independent selection.
Construction checks arena addresses, duplicate source locations within their validated
execution contexts, expression cycles, argument ownership, shared type membership and
the four entry signatures. It does not
replace source admission or prove that generated bodies implement temporal meaning.
Generic call/result conformance remains the shared value-admission boundary when the
executor applies a function; this interface does not introduce a second type checker.

The four entry roles are `init`, `on_event`, `on_watermark`, `on_gap` of QSpec FR-487
at #595 (75508820b5003a342b2af8782dc400bdc9080403). They retain the actual source function keys. Compatible roles may resolve to
one function; no distinct-function premise is invented. Expression arena addresses are
not semantic IDs. Inline terms can share one source node key and differ by their
body-child paths. No runtime code mints a NodeKey or substitutes a digest for a key.

An expression's source location is its exact admitted `(key, body_path)` pair.
Construction derives its binding owner from `Argument`, `Local`, `Let` and fold, map and filter
bindings and from their transitive operands. When that derivation identifies exactly
one function without mixed ownership, its `FunctionId` qualifies the expression's
execution context for duplicate-location validation only. Copies of one admitted
parameter-dependent node may therefore keep the same source location in two
different function contexts. The context does not alter either source field, mint a
source identity, or become a substitute provenance path.

An expression with no binding owner has one global source location. One such
expression may be shared by arena address across function bodies; two neutral
copies at the same location do not become distinct merely because callers differ.
Two expressions in one function context, or a neutral and a bound expression, may
not claim the same source location. Mixed binding ownership does not qualify for
the cross-function exception. A reachable foreign argument or local still returns
its typed ownership refusal rather than being admitted by a duplicate-location
rule. These checks also apply to expressions outside an entry body's reachable
closure where the existing construction checks already inspect the full arena.

`Layout.tick_unit` refers to the unit retained in the shared TypeEnvironment's unit
table. Its Time dimension and scale are read there; no copied unit definition or
second tick-scale field exists. Rate and uncertainty are exact shared Rationals.
Buffer capacities and the seventeenth binding, activation_capacity, stay shared exact
Integers. Zero activation capacity is valid; no host-width cap is imposed on retained
premises. Target storage representability is a build/resource concern, not source
malformation. The retained State declaration is a closed record: now and history_start are
required Integers, followed by exactly one required buffer_<n> Sequence per buffer,
then required activations (a Sequence), then required event_position (an Integer).
No missing, extra, mistyped, optional or misordered outer field is accepted. The shared
declaration/value representation retains event_position; Layout has no duplicate host
counter. Activation status remains the ordinary admitted type, without an RT-invented
status enum. Source admission still owns the full activation/result schemas.

The event position counts admitted events, independently of unwrapped ticks. Init
returns 0; on_event adds 1 per admitted event, including the baseline on_event(0).
On_watermark, on_gap and a clock poll leave it unchanged (QSpec FR-487-AC-11).
Construction retains this field and its type but does not run or prove an entry body;
these transition observations remain executor qualification.
Tick intervals remain derived from the source clause and unit scales, not extra plan
bindings. The root's content key and independently selected lock/subject/occurrence
context remain owned; a display name or arena index cannot replace that context.

The initial closed operation set is literals, owning-function arguments, ordinary
calls and lazy conditionals. This is a construction slice, not a claim that the
complete real producer closure can already execute. Further capabilities must follow
actual admitted producer expressions through the same representation; no callback,
append opcode or temporal opcode supplies an absent capability.

Construction returns a stable ProgramRefusalCode and the source key when available.
A stopped shared membership walk retains its native EnvironmentFailure separately
from ordinary type mismatch and validation scratch reservation failure. There is no
promise to recover from arbitrary allocator exhaustion.

### inspect_monitor_layout

`Program::layout` returns an immutable view into the retained root associations.
No mutable owner view or rebind operation is exposed.

| Param | Type | Multiplicity | Constraints |
| --- | --- | --- | --- |
| program | Program | 1..1 | |

Returns: Layout[1..1]

### inspect_monitor_types

`Program::types` returns an immutable borrow of the retained shared environment.

| Param | Type | Multiplicity | Constraints |
| --- | --- | --- | --- |
| program | Program | 1..1 | |

Returns: TypeEnvironment[1..1]

### resolve_monitor_function

`Program::function` resolves only the supplied arena address. Its result retains the
exact source key, ordered parameter types, result type and body root.

| Param | Type | Multiplicity | Constraints |
| --- | --- | --- | --- |
| program | Program | 1..1 | |
| function | FunctionId | 1..1 | |

Returns: Function[0..1]

### resolve_monitor_expression

`Program::expression` resolves only the supplied arena address and retains the exact
source node key, body path, shared result type and ordinary operation.

| Param | Type | Multiplicity | Constraints |
| --- | --- | --- | --- |
| program | Program | 1..1 | |
| expression | ExpressionId | 1..1 | |

Returns: Expression[0..1]

## Features

| Feature | Kind |
| --- | --- |
| construct_monitor_program | operation |
| inspect_monitor_layout | operation |
| inspect_monitor_types | operation |
| resolve_monitor_function | operation |
| resolve_monitor_expression | operation |

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| interface-002-AC-1 | After construction consumes the preparation parts and one Rc owner drops, another owner retains the complete root key with nested operand keys, selection bytes, all four entry associations, literal bodies and shared nominal declarations. Invalid arena lookups return absent. No monitor execution credit is granted by this construction control. | Test (TC-209) |
| interface-002-AC-2 | Missing, extra, optional, mistyped or misordered outer State fields (including final required Integer event_position), missing or mistyped on_gap, invalid counter width, a gap outside the declared width, negative uncertainty and a tick unit under another dimension yield the typed construction refusal before any entry executes; width64 with maximum gap u64::MAX remains representable. Activation capacity zero and a positive value above u64::MAX remain retained exactly, while a negative value refuses without a host-width truncation. | Test (TC-209) |
| interface-002-AC-3 | Duplicate function keys and duplicate expression source locations in one validated execution context refuse `DuplicateKey`; invalid expression edges, direct expression cycles including unreachable arena nodes, and a reachable argument of another function refuse with their distinct typed codes. A finite shared neutral subexpression and distinct inline body paths under the same source node remain admissible; a 20,000-level shared inline arena validates on a 512 KiB native stack without recursive graph descent. | Test (TC-209) |
| interface-002-AC-4 | Two compatible entry roles may reference the same retained function; construction does not add a distinct-function premise to FR-487. | Test (TC-209) |
| interface-002-AC-5 | Two functions using the same admitted State parameter node, delta parameter node and parameter-dependent IntegerAdd node admit distinct expression arena addresses when each copy's transitive binding owner is its own `FunctionId`. Both copies retain the original, equal `key` and `body_path` through `Program::expression`; the functions retain their distinct original keys and operand associations. | Test (TC-209) |
| interface-002-AC-6 | Two copies of one source location with the same derived function owner, two neutral copies, or a neutral and a bound copy refuse `DuplicateKey`. Mixed binding owners do not gain a duplicate-location exception; a reachable foreign argument refuses `ForeignArgument`, and a reachable foreign local or mixed local binding refuses `InvalidBinding`, without changing the source key or body path. | Test (TC-209) |

## Delivery boundary

This interface is the RT-first construction contract used by IR-721. FR-276 and
TC-200's build/init, event/watermark/gap execution, resource-stop behavior and genuine
QSL producer/reference differential remain required on IR-722. These criteria neither
replace nor narrow FR-276-AC-1 through AC-10. Construction-only tests are tagged to
this interface, not claimed as qualification of the complete monitor.
