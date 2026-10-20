---
id: SR-14101
title: "code-review review of RT PR 151 quantity and unit owner cutover"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-runtime; Cargo.lock, src/exact, tests/exact_*, spec/assurance/AD-004, spec/exact/functional, spec/exact/matrix"
review_set: subset
---

## Summary

Reviewed the Rust owner cutover and replacement integration tests. The implementation removal is direct, but one regression weakens the conversion oracle.

## Examined units

| ID | Role | Path | Excerpt |
| --- | --- | --- | --- |
| FR-007-AC-20 | examined | spec/exact/functional/FR-007-exact-scalar-families.md | `convert_quantity` with `QuantityTarget::Exact` between two units of equal dimension returns a completed `Conversion` whose `value()` is the `ConvertedValue::Exact` variant holding `inverse(target_path)(source_path(v))` over exact rationals and whose `unit()` is the target unit: `250 cm` to `m` is `5/2` (QSpec TC-187 U01), `0 degC` and `32 degF` to `K` are both `5463/20` (U04), compound `4 m^2` to `m2` is `4` (U22), compound `kg*m^2*s^-2` to `J` is `1` (U25 second step), and `1 u2` to `u1` is `-9` (U26); and for every numerator and denominator in an `i128` sweep of scale and offset fractions t |
| FR-007-AC-21 | examined | spec/exact/functional/FR-007-exact-scalar-families.md | `convert_quantity` with `Exact` between declared units on distinct dimension nodes, or from or to a compound unit whose normalized base-dimension map differs, returns `Err(IllTyped)` with cause `IncompatibleDimensions` before any `unit.identity-read` or other charge, leaving the meter unchanged: `1 m` to `s` (U03-style), a base-dimension name and shape equal to `L` under a different semantic-node key (U09), `1 N_m` to `J` (U22), and compound `4 m^2` to `m`. |
| FR-007-AC-22 | examined | spec/exact/functional/FR-007-exact-scalar-families.md | `convert_quantity` with `Exact` admits an affine source or target unit and converts it exactly: `0 degC` to `K` returns `5463/20` and `1 u2` to `u1` returns `-9`, neither refused as `AffineUnitArithmetic` (U04, U26). |
| FR-007-AC-23 | examined | spec/exact/functional/FR-007-exact-scalar-families.md | `convert_quantity` with `Exact` under a limit tuple that admits every charge charges one `unit.identity-read`, then all `unit.edge` charges for the source-to-root path followed by the root-to-target path with no shortcut, then two `unit.rational-arithmetic` events per edge, then `unit.target-domain` with no size amount, then `unit.result-retain`, at the consumed amounts the vectors state: `100 cm` to `m` under the exact U10 tuple returns `1` with largest amount `8` at the multiply event; `1 in` to `cm` under the first U15 tuple returns `127/50` after one read, two edges and four rational event |
| FR-007-AC-24 | examined | spec/exact/functional/FR-007-exact-scalar-families.md | `evaluate_quantity` `Add`, `Subtract`, `Multiply` and `Divide` of `1 degC` with `2 degC`, and `Power` of `1 degC` to the exponent `2`, each return `IllTyped` with cause `AffineUnitArithmetic` with zero charges (QSpec TC-187 U05), as do `1 u1 + 2 u1` and `1 u3 * 1 m` under `L(0,0,0,0,0,0)` (U20), while `1 u2 + 2 u2` under `L(4,0,0,2,5,1)` is not affine and returns exact `3 u2`. |
| FR-007-AC-25 | examined | spec/exact/functional/FR-007-exact-scalar-families.md | When a charge of `convert_quantity` with `Exact` is denied, the result is `Incomplete` carrying the denied limit kind, limit, consumed amount, next charge and charge point, with no partial quantity: the first U10 tuple with `result_units` lowered to `0` is `Incomplete { limit_kind: result_units, limit: 0, consumed: 0, next_charge: 1, charge_point: unit.result-retain }`; `1 in` to `cm` under the second U15 tuple is `Incomplete { limit_kind: unit_edges, limit: 1, consumed: 1, next_charge: 2, charge_point: unit.edge }`, because both edges are charged before the first rational event; compound `4 m |
| FR-011-AC-2 | examined | spec/exact/functional/FR-011-meter-state-at-a-stop.md | A quantity `power` with zero base and negative exponent is `Undefined::DivisionByZero`, and a divide by zero is reported in preference to it when both hold. |
| FR-011-AC-4 | examined | spec/exact/functional/FR-011-meter-state-at-a-stop.md | A charge whose second-scanned counter is short writes no counter, appends no log entry and advances no occurrence counter; and a charge presented with its size vector in either order reports the same first short counter in `ScalarLimitsV1` field order. |
| FR-011-AC-6 | examined | spec/exact/functional/FR-011-meter-state-at-a-stop.md | With a cumulative limit of `u64::MAX`, a counter at `u64::MAX - 1` admits one unit to reach the limit and denies the next unit without wrapping or changing any meter state. A quantity power whose `unit.rational-arithmetic` request exceeds `u64::MAX` returns `Incomplete` before computing the power; `next_charge` equals the exact request, and the denied charge changes no counter, admission count or admitted-charge log. |
| FR-011-AC-7 | examined | spec/exact/functional/FR-011-meter-state-at-a-stop.md | `Meter::consumed` answers without panic for every member of `LimitKind::ALL`, returning zero for untouched counters, admitted high-water values for semantic sizes and admitted cumulative totals for work/results; IEEE flag iteration and QSV dimension and compound-unit term iteration are stable for every accepted graph-node insertion order, while QSV's checked-node and `UnitGraph::from_checked_nodes` boundaries refuse malformed order with the stated first cause. |
| FR-012-AC-4 | examined | spec/exact/functional/FR-012-carried-compiler-vocabulary.md | Each of the twelve graph causes reachable through QSV's `DimensionNode::checked`, `UnitNode::checked` or `UnitGraph::from_checked_nodes` is raised by a malformed input; `DuplicateNode` is unrepresentable because the graph accepts key-unique `BTreeMap`s, rather than a list with repeated keys. `UndeclaredCase` arises by reading a case against its declaration; no graph input raises any of the four compiler-admission causes. `EnumDeclaration::new` raises `NonCanonicalPreimage` and `UnsortedUnorderedMembers` for the malformed member lists named above; `OwnerNotSelected`, `StaleKey`, `ForeignDeclara |
| FR-012-AC-5 | examined | spec/exact/functional/FR-012-carried-compiler-vocabulary.md | QSV's `Dimension` and `CompoundUnit` hold no zero exponent, iterate ascending by node key, and compare equal exactly when they denote the same unit, for every insertion order of the same admitted graph nodes; compound term input is ascending, and an unsorted term input is refused. |
| FR-275-AC-23 | examined | spec/exact/functional/FR-275-single-exact-kernel.md | After the step-4 value-bearing cutover, QSV's `Conversion::identified_quantity` returns a `quire_exact::Quantity` with the exact magnitude and the target's compound-arm `UnitId`, paired with QSV's `IdentifiedUnit` for that same target; a non-exact conversion returns `HandoffRefusal::NotExact`, and target identity refusal returns `HandoffRefusal::Identity` with neither value nor identified unit. Retaining the identified unit with `UnitScope::form` permits `UnitScope::resolve(&quantity)` to supply the same normalized `QuantityUnit::Compound` to the next conversion. Declared and compound ID arms  |

## Verdict

**CONDITIONAL** — findings require a same-PR fix.

## Rust review

The diff deletes RT's duplicate quantity and unit modules and their exports without adding a production wrapper or alias. The remaining Rust changes are integration tests using QSV's typed owner APIs. I checked import ownership, panic sites (test assertions only), direct enum-arm identity assertions, meter borrowing, and trace tags against the changed criteria. No additional Rust idiom, unsafe, integer-conversion, or resource-boundary finding arose in the changed code.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The new i128-typed sweep uses only scales 1/2, 2/3 and 3, offsets -1/2, 0 and 1/3, and one edge on each side. The retired oracle exercised a negative scale, a numerator above i64::MAX, prime/power-of-ten denominators, and noncommuting multi-edge paths. The unchanged AC-20 requires an i128 scale/offset sweep and path composition; retain discriminating samples through QSV. | tests/exact_quantity_conversion.rs:183 |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | bbdae1f adds a negative-scale and prime-denominator sweep, an i128 value above i64::MAX, and bidirectional noncommuting two-edge conversion with exact rational and edge-count assertions; both focused tests pass. |
