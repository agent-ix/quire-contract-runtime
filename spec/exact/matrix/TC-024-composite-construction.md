---
id: TC-024
title: "Construct composite values and their declaration environment"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-runtime/FR-008
    type: verifies
---
# TC-024: Construct composite values and their declaration environment

## Description

Check `TypeEnvironment` admission, `record`/`tuple`/`evaluate_record`/`evaluate_tuple` construction,
`ValueGraph`/`build` containment, and the accounting vocabulary at 52 points. Evidence:
`tests/exact_composite.rs` (`--features exact`).

## Test Procedure

1. Round-trip all 52 `ChargePoint` spellings; check `function.call` and `collection.visit` are
   present in the vocabulary but appear in no charge sequence this slice admits, since no evaluator
   this crate exposes yet charges them.
2. Admit a declaration set with a duplicate key, a duplicate field/attribute name, an unnamed-edge
   cycle and a non-escaping-edge cycle; check each refuses at its originating declaration with the
   named `DeclarationCause` and, for the two recursion cases, the exact offending cycle.
3. Check `check_type` refuses a `Reference<T>` to a non-object-type declaration as `type-mismatch`
   and a set/bag/ordered-set element type bearing an IEEE value as `operator-ineligible`, and
   `contains_ieee` finds a `Float32`/`Float64` at any depth through nested record and tuple
   declarations.
4. Build records and tuples with a missing required field, a `null` for a required field, an
   undeclared or duplicated field, and a wrong tuple arity; check each `ConstructionRefusal` names
   its `Component` and `ConstructionCause`. Run `evaluate_record`/`evaluate_tuple` with deferred
   field expressions and check they run in declaration order under the first-stopped rule, and that
   a completed value charges `composite.result-retain` with its exact `occ` before exposure.
5. Build a `ValueGraph` where two slots name the same node and check the built value is shared with
   no object identity created; check a graph naming two nodes with one id refuses as `DuplicateNode`
   and a slot or root naming no node refuses as `UnknownNode`; build a graph whose containment closes
   a cycle and check the refusal names the closing node as `ContainmentCycle`; build a graph nested
   past typical host recursion limits and check `build` completes with no stack overflow.
6. Inject a denial at `composite.result-retain`; check the `Incomplete` record and that no counter
   changed.

## Expected Results

Every declaration and construction refusal names its typed cause and originating component; deferred
evaluation follows declaration order and stops at the first non-completing member; containment
graphs build bottom-up with sharing and no recursion-depth failure; the injected denial fires with no
partial value, and `CheckedInvariant` is unreachable from every vector in this test's corpus.
