---
id: SR-14105
title: "spec-review/scope-boundary review of RT PR 151 quantity and unit owner cutover"
type: SpecReview
analysis: scope-boundary
scope: "agent-ix/quire-contract-runtime; Cargo.lock, src/exact, tests/exact_*, spec/assurance/AD-004, spec/exact/functional, spec/exact/matrix"
review_set: subset
---

## Summary

Checked ownership between RT, QSV and the exact kernel after the unit cutover. One stale present-tense boundary sentence remains.

## Examined units

| ID | Role | Path | Excerpt |
| --- | --- | --- | --- |
| FR-011-AC-7 | examined | spec/exact/functional/FR-011-meter-state-at-a-stop.md | `Meter::consumed` answers without panic for every member of `LimitKind::ALL`, returning zero for untouched counters, admitted high-water values for semantic sizes and admitted cumulative totals for work/results; IEEE flag iteration and QSV dimension and compound-unit term iteration are stable for every accepted graph-node insertion order, while QSV's checked-node and `UnitGraph::from_checked_nodes` boundaries refuse malformed order with the stated first cause. |
| FR-012 | examined | spec/exact/functional/FR-012-carried-compiler-vocabulary.md | `src/exact/mod.rs` exports the remaining runtime unit vocabulary until its owner cutover. |
| FR-012-AC-4 | examined | spec/exact/functional/FR-012-carried-compiler-vocabulary.md | Each of the twelve graph causes reachable through QSV's `DimensionNode::checked`, `UnitNode::checked` or `UnitGraph::from_checked_nodes` is raised by a malformed input; `DuplicateNode` is unrepresentable because the graph accepts key-unique `BTreeMap`s, rather than a list with repeated keys. `UndeclaredCase` arises by reading a case against its declaration; no graph input raises any of the four compiler-admission causes. `EnumDeclaration::new` raises `NonCanonicalPreimage` and `UnsortedUnorderedMembers` for the malformed member lists named above; `OwnerNotSelected`, `StaleKey`, `ForeignDeclara |
| FR-012-AC-5 | examined | spec/exact/functional/FR-012-carried-compiler-vocabulary.md | QSV's `Dimension` and `CompoundUnit` hold no zero exponent, iterate ascending by node key, and compare equal exactly when they denote the same unit, for every insertion order of the same admitted graph nodes; compound term input is ascending, and an unsorted term input is refused. |
| FR-275-AC-23 | examined | spec/exact/functional/FR-275-single-exact-kernel.md | After the step-4 value-bearing cutover, QSV's `Conversion::identified_quantity` returns a `quire_exact::Quantity` with the exact magnitude and the target's compound-arm `UnitId`, paired with QSV's `IdentifiedUnit` for that same target; a non-exact conversion returns `HandoffRefusal::NotExact`, and target identity refusal returns `HandoffRefusal::Identity` with neither value nor identified unit. Retaining the identified unit with `UnitScope::form` permits `UnitScope::resolve(&quantity)` to supply the same normalized `QuantityUnit::Compound` to the next conversion. Declared and compound ID arms  |

## Verdict

**CONDITIONAL** — findings require a same-PR fix.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The introduction says src/exact/mod.rs exports remaining runtime unit vocabulary until its owner cutover, while this PR deletes those modules and the same document assigns the graph to QSV. Update the present-tense boundary description. | spec/exact/functional/FR-012-carried-compiler-vocabulary.md:24 |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | FR-012 now states that src/exact/mod.rs no longer exports the unit vocabulary and consumes QSV checked graphs and units directly; both changed spec documents validate. |
