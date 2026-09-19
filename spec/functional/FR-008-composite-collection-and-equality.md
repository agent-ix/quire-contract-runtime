---
id: FR-008
title: "Evaluate composite, collection and equality families under the same outcome and accounting contract"
type: FR
relationships:
  - target: ix://agent-ix/quire-contract-runtime/FR-006
    type: depends_on
  - target: ix://agent-ix/quire-contract-runtime/FR-007
    type: depends_on
---
# FR-008: Evaluate composite, collection and equality families under the same outcome and accounting contract

## Description

When the `exact` feature is enabled, the runtime shall construct composite and collection values,
compare values for equality, build finite values from a containment graph, and carry terminal object
references, with values and outcome kinds equal to the pinned quire-spec-language authority
(d01371b9) on every shared-corpus vector, and charges equal to the QSpec 7d7943a accounting
schedule. As with FR-006 and FR-007, the runtime is an implementation of that definition, not a
second semantic authority: `quire_spec_language::value` decides every construction, comparison and
canonical-order question; every type, field and variant this requirement adds keeps the authority's
name and order; and no operator here decides anything the authority does not.

## Inputs

- A `TypeEnvironment` admitting a checked package's closed record, tuple and object-type
  declarations, and the `ValueType` (`Boolean`, `Integer`, `Int`, `Rational`, `Decimal`, `Float`,
  `Quantity`, `Text`, `Enum`, `Option`, `Composite`, `Collection`, `Reference`) named outside a
  declaration.
- Deferred field, position and collection-element expressions (`Deferred`, `FieldExpression`) that
  run only when construction reaches them, and completed occurrences for the non-deferred
  constructors.
- A `CollectionType` (`kind<element>[bound]`) and its `CardinalityBound`; two `EqualityOperand`s and
  an `EqualityOperator`; a `ValueGraph` of named `GraphNode`s for containment construction; and a
  `ScalarLimits` tuple with the ten `ScalarLimitsV1` counters.
- Optionally, one injected denial naming a charge point and a 1-based occurrence.

## Outputs

- An FR-006 `Outcome<Value>` for each composite or collection construction and each containment
  graph build, and an FR-006 `Outcome<bool>` for each equality evaluation, plus the admitted charge
  sequence and consumed counters on the `Meter`.
- Static refusals reported beside the outcome: `ConstructionRefusal` (with its `Component` and
  `ConstructionCause`), `InvalidDeclaration` (with its `DeclarationCause` and `RecursionEdges`),
  `GraphRefusal` (with its `GraphCause`), and `IllTyped` from `TypeEnvironment::check_type` and
  `check_equality`.

## Behavior

- A `TypeEnvironment` admits its composites and object types only after `check_member_types` and two
  `check_recursion` passes both succeed; it refuses a `Reference<T>` whose target is not a model
  object type, and a set, bag or ordered-set element type that bears an IEEE value at any depth
  (`OperatorIneligible`), through `check_type` and `contains_ieee`. `record` and `tuple` construct
  from already-completed fields or positions; `evaluate_record` and `evaluate_tuple` run deferred
  field and position expressions in declaration order under the first-stopped rule, then charge
  `composite.result-retain` with the exact `occ` of the completed value before exposing it.
- `construct_collection` charges `collection.element` and then runs the deferred element; the first
  element that does not complete becomes the outcome and no later element runs. `form_collection`
  refuses an occurrence outside the declared element type before any charge. Formation then charges
  one `collection.member-walk` and one `collection.member-test` per membership comparison against
  the members retained so far, in retention order, stopping at the first equal member; charges
  `collection.bound` before the uncharged bound violation check; and charges
  `collection.result-retain` with the exact `occ` of the retained value. A set or bag stores its
  members in ascending canonical-key order, a bag listing each occurrence, which is both its
  canonical representation and its visiting order.
- Every type that admits `=` has a total, type-owned canonical key: two values of that type have
  equal keys exactly when they are FR-149 equal. The key fixes set and bag canonical and visiting
  order; it is not a general ordering operator and is not exposed for any type that does not admit
  `=`. Absent orders before null, which orders before a present value
  (`Absent < Null < Present`). Comparison is iterative over an explicit task stack, so value depth
  never reaches the host stack.
- `TypeEnvironment::check_equality` admits each `convert<T>(e)` operand only through the closed
  `admits_equality_conversion` table, refuses `=` on any type that bears an IEEE value in either
  operand at any depth as `operator-ineligible`, and selects one `EqualitySchedule` from the common
  comparison type — before any charge. `plan_equality` forms the occurrence-pair plan of two
  completed operands with no charge; a reference pair whose universes differ refuses as
  `ForeignReference` at plan time, never as a charged comparison. `CheckedEquality::evaluate` then
  runs the left and right conversion charges in operand order and the selected schedule: the FR-141
  text, FR-141 enum or FR-142 quantity schedule for those top-level types, and otherwise
  `equality.plan-form`, `equality.plan`, one `equality.pair` per planned occurrence-path pair and
  `equality.result-retain`.
- `TypeEnvironment::build` constructs the finite value rooted at one `GraphNodeId` of a `ValueGraph`
  bottom-up: a node reached through more than one slot becomes one shared immutable value, and a
  node that contains itself through value containment refuses as `ContainmentCycle` at the node that
  closes the cycle. Sharing a contained value never creates object identity.
- A `Reference<T>` value is terminal: its identity is the supplied `(UniverseIdentity, object-type
  key, ObjectIdentity)` triple, equality never inspects referenced state, and no source form
  constructs one from a component value.
- `Value` carries no structural equality; the FR-149 relation reached through `plan_equality` and
  `CheckedEquality::evaluate` is the only equality, because a structural comparison would be a second,
  unmetered relation and would be wrong for signed zero, NaN-bearing values, decimal values retained
  at differing scales, and reference pairs from different universes.
- Occurrence counts (`occ`), planned pair counts and cardinality counts beyond `u64::MAX` are
  arbitrary-precision mathematical integers; only the `Meter`'s own counters are fixed-width `u64`.
- Every operation that walks a value's occurrence tree — canonical-key comparison, equality-plan
  formation and pairing, and containment-graph construction — is iterative over an explicit
  worklist, never host-stack recursive, so no admitted value depth can exhaust the call stack.
- `Value`'s `Debug` and `Drop` are hand-written and iterative over an explicit worklist, not
  derived: on the governed `thumbv7em-none-eabi` target a host-stack overflow is silent memory
  corruption, not a panic, so a value nested past a recursive walk's stack limit must still format
  and free without recursing, in time and allocation proportional to the value's size. The
  hand-written `Debug` output is pinned by an exact literal string, in both compact and alternate
  form, on small fixed composite and collection values.
- The runtime reports a closed compiler or evaluator refusal vocabulary (`InvalidDeclaration`,
  `ConstructionRefusal`, `GraphRefusal`, `IllTyped`, the extended `Refusal` and `Undefined`) exactly
  as the authority names it; it never re-derives, renames or infers a refusal the authority does not
  already carry.
- The `Undefined` and `Refusal` vocabularies are extended to their FR-149/FR-144 complete authority
  shape: `Undefined::EmptyReduction` and `Undefined::NoneValue` (reachable only from direct kernel
  evaluation of an unlinked expression, which this crate does not yet expose — see Out of Scope);
  `Refusal::ForeignReference`; `Refusal::CardinalityOutOfBound { violation, kind, bound, count }`
  with its `BoundViolation::{BelowMinimum, AboveMaximum}`; and `Refusal::CheckedInvariant`, reachable
  only when a checked-program invariant fails and unreachable from any admitted vector. `Refusal`
  gains a `cause()` method carrying the closed FR-272 cause tag beside `code()`, populated for
  `CardinalityOutOfBound` and empty for every other variant.
- The `quire.value.accounting/v1` charge vocabulary gains twelve points ordered
  `equality.plan-form`, `equality.plan`, `equality.pair`, `equality.result-retain`, `function.call`,
  `collection.element`, `collection.visit`, `collection.member-walk`, `collection.member-test`,
  `collection.bound`, `collection.result-retain`, `composite.result-retain`, taking `ChargePoint::ALL`
  to 52. `ScalarLimitsV1` is unchanged; composite and collection construction bill
  `value_occurrences`, `work_units` and `result_units` only, exactly as FR-006's charge-before-work
  and high-water/cumulative rules already require.
- The expression machine and total pure functions (`Expression`, `FunctionDeclaration`,
  `PackageDeclarations::check`, `CheckedPackage::{check_expression, call, evaluate}`,
  `CheckingLimits`, `CheckRefusal`, `Evaluation`), and object environments and attribute dereference
  (`ObjectEnvironment`, `ObjectEnvironmentCause`, `ObjectEnvironmentRefusal`), are outside this
  requirement; [FR-273](./FR-273-exact-function-application.md) admits the application surface and the
  object environment it validates reference arguments against. `function.call` and `collection.visit`
  are ported as closed accounting vocabulary here, ahead of the operator that charges them. Temporal and
  protocol encodings and replay are out of scope (agent-ix/quire-spec-language#121). Library
  resolution and package identity (`resolve_libraries`, `PackageId::of_preimage`, `check_migration`)
  are not this crate's work at all: recomputing a package id from a preimage is compiler work, and
  this runtime never recomputes a key it was given — a permanent boundary, not a deferral.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-008-AC-1 | A `TypeEnvironment` admits a closed set of record, tuple and object-type declarations only after member-type and both recursion-rule checks succeed, and refuses at the originating declaration with a typed `DeclarationCause` otherwise; `record`, `tuple`, `evaluate_record` and `evaluate_tuple` construct in declaration order under the first-stopped rule and charge `composite.result-retain` with the exact retained `occ`. | Test (TC-024) |
| FR-008-AC-2 | `TypeEnvironment::build` constructs a finite value bottom-up from a `ValueGraph`, sharing a node reached from more than one slot as one immutable value with no object identity created, and refuses the first containment cycle as `ContainmentCycle` at the node that closes it. | Test (TC-024) |
| FR-008-AC-3 | `construct_collection` charges `collection.element` before running each deferred element and stops at the first non-completing element with no later element run; `form_collection` refuses a type-mismatched occurrence before any charge; formation charges membership comparisons, `collection.bound` and `collection.result-retain` in that order, and a set or bag is stored in ascending canonical-key order. | Test (TC-025) |
| FR-008-AC-4 | The type-owned canonical key totally orders every keyed type, ranks `Absent < Null < Present`, fixes set and bag canonical and visiting order, and is evaluated iteratively at a depth that would overflow a recursive host-stack walk. | Test (TC-025) |
| FR-008-AC-5 | `plan_equality` forms the occurrence-pair plan with no charge and refuses a cross-universe reference pair as `ForeignReference`; `check_equality` refuses before any charge, including `operator-ineligible` when either operand type bears an IEEE value at any depth; `CheckedEquality::evaluate` charges the selected schedule and its conversions in operand order. | Test (TC-026) |
| FR-008-AC-6 | A `Reference<T>` value carries only its supplied `(universe, object-type, identity)` triple, is constructed from no source form, and compares equal only within one universe. | Test (TC-026) |
| FR-008-AC-7 | The extended `Undefined` and `Refusal` vocabularies, `BoundViolation` and `Refusal::cause()` are closed, typed and distinct from every FR-006 variant; both `CardinalityOutOfBound` directions are reachable and report their `code()` and `cause()`; `CheckedInvariant` is unreachable from any admitted vector in the shared corpus. | Test (TC-024, TC-025, TC-026) |
| FR-008-AC-8 | Every one of the twelve added charge points round-trips its QSpec 7d7943a spelling, `ChargePoint::ALL` has exactly 52 members, and an injected denial at each of the twelve yields `Incomplete` on `work_units` naming that point with every counter left unchanged. | Test (TC-024, TC-025, TC-026) |
| FR-008-AC-9 | `Value`'s `Debug` and `Drop` are hand-written and iterative: formatting or dropping a value nested past a recursive walk's host-stack limit does not overflow the stack and completes in time and allocation proportional to the value's size, and the hand-written `Debug` output is an exact literal string, in both compact and alternate form, on a small fixed value. | Test (TC-024, TC-025) |

## Dependencies

- **Upstream**: [FR-006](./FR-006-exact-outcomes-and-accounting.md); [FR-007](./FR-007-exact-scalar-families.md);
  `ix://agent-ix/quire-specification` at `7d7943a` (`value-accounting.md`); quire-spec-language at
  `d01371b9`.
