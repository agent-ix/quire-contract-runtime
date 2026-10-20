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

Check QSV `TypeEnvironment` admission, shared composite construction,
`ValueGraph`/`build` containment over `quire_exact::Value`, and selected accounting
vocabulary. IR-882 removed RT's duplicate implementation and its former
`tests/exact_composite.rs` controls. Quire-exact FR-368-AC-1/2 owns the full
vocabulary census. The remaining procedures below require owner evidence and
RT consumer controls to be mapped before they receive acceptance credit.

## Test Procedure

1. Locally check that `ChargePoint::ALL` has exactly 63 members, ends with `temporal.visit`,
   and contains `function.call` and `collection.visit`. Check these three spellings and their
   `from_code` round trips; owner FR-368-AC-1/2 covers the full vocabulary round trips. Check
   that `function.call` and `collection.visit` do not appear in this construction test's charge
   sequence. Function application charges `function.call` in TC-194; do not infer a global
   absence from this test.
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
   Have one deferred field return a value outside its declared type and assert
   `CheckedInvariant { cause: DeferredResultNotAdmitted }`.
5. Build a `ValueGraph` where two slots name the same node and check the built value is shared with
   no object identity created; check a graph naming two nodes with one id refuses as `DuplicateNode`
   and a slot or root naming no node refuses as `UnknownNode`; build a graph whose containment closes
   a cycle and check the refusal names the closing node as `ContainmentCycle`; build a graph nested
   past typical host recursion limits and check `build` completes with no stack overflow.
6. Inject a denial at `composite.result-retain`; check the `Incomplete` record and that no counter
   changed.
7. On a bounded small-stack thread and a default-stack child process, construct an under-source-cap
   deeply nested `ValueType` through `Option` and `Collection`, then clone, compare, format and
   drop it. Check a shallow value's compact and alternate `Debug` output against exact literals,
   and distinguish unequal leaves, collection kinds and bounds.

## Expected Results

Every declaration and construction refusal names its typed cause and originating component; deferred
evaluation follows declaration order and stops at the first non-completing member; containment
graphs build bottom-up with sharing and no recursion-depth failure; the injected denial fires with no
partial value, and `CheckedInvariant` is unreachable from every admitted vector in this test's corpus.
The deliberately invalid deferred field carries `DeferredResultNotAdmitted`.
Deep type metadata survives cloning, comparison, formatting and freeing without host-stack recursion.

## Status

PLANNED: retained under FR-275-AC-14 after removal of RT's duplicate implementation.
No deleted RT test is counted as evidence for this procedure.
