---
id: TC-201
title: "Convert a quantity exactly between units of one dimension"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-runtime/FR-007
    type: verifies
---
# TC-201: Convert a quantity exactly between units of one dimension

## Description

Check QSV's public `convert_quantity` with `QuantityTarget::Exact`, and the affine refusals of
QSV's public `evaluate_quantity`, through RT integration tests. No RT symbol is added
(FR-275-AC-18). The criteria cite QSpec [quire-specification FR-142-AC-1](ix://agent-ix/quire-specification/FR-142),
`FR-142-AC-2`, `FR-142-AC-4`, `FR-142-AC-5`, `FR-142-AC-7` and `FR-142-AC-10` and QSpec TC-187 rows U01,
U03, U04, U05, U09, U10, U15, U20, U22, U25 and U26 by id; their vectors are re-stated in
FR-007-AC-20 through AC-25, not copied from QSpec. Lossy conversion (`FR-142-AC-3`), `Decimal` and
`Integer` targets and rounding are out of scope except the FR-275-AC-23 refusal check. Evidence:
`tests/exact_quantity_conversion.rs` and `tests/exact_quantity_owner.rs` (`--features exact`). Shared-corpus agreement is IR-669
(TC-022).

## Test Procedure

1. Convert the FR-007-AC-20 vectors (U01, U04, U22 `m2`, U25 second step, U26) and compare each
   `ConvertedValue::Exact` magnitude and the `unit()` to the stated value.
2. Sweep scale and offset fractions over bounded `i128` families and compare each magnitude to an
   independent `i128` rational oracle.
3. Convert across distinct dimension nodes and across different base-dimension maps (`m` to `s`,
   U09, U22 `N_m` to `J`, compound `4 m^2` to `m`); expect `Err(IllTyped)` `IncompatibleDimensions`
   and a meter with no charge.
4. Convert `0 degC` to `K` and `1 u2` to `u1`; expect success, not `AffineUnitArithmetic`.
5. Run `evaluate_quantity` Add, Subtract, Multiply and Divide of `1 degC` with `2 degC`, and Power of `1 degC` to `2` (U05), then U20's
   `u1 + u1`, `u3 * m` and `u2 + u2`; expect `AffineUnitArithmetic` with zero charges, and `3 u2`.
6. Run U10, U15 first tuple, U22 `m2` and `cm2` first tuples, U25 both steps and U26 first tuple;
   check the recorded charge order and amounts.
7. Run the denying tuples of FR-007-AC-25 (U10 with `result_units: 0`, U15 and U22 and U26 second
   tuples); check each `Incomplete` field and that no partial quantity is returned. A differing executed
   value for the U10 tuple is a finding, never a reason to change the test to fit.
8. Run U25 through QSV: complete
   `convert_quantity` with `QuantityTarget::Exact`; invoke QSV-owned
   exact-result handoff on the completed `Conversion` to obtain a paired
   `quire_exact::Quantity` and `IdentifiedUnit`; retain the latter with
   `UnitScope::form`; and pass **the returned quantity**, not the `Conversion`,
   to `UnitScope::resolve` before converting it to `J`. QSV FR-124 defines its result and
   refusal semantics. Check that `Conversion::unit()` equals the identified
   target unit, the quantity's ID equals the retained and resolved compound-arm
   `UnitId`, the result is exact `1 J`, and the existing U25 charge
   order is unchanged. Exercise equal raw ID bytes in the declared and compound arms and
   expect distinct IDs. Exercise compound identity refusal and expect no returned quantity
   or retained unit. Check that a non-exact `Conversion` has no exact-quantity
   result and that successful handoff adds no meter charge (FR-275-AC-23).
   The RT test checks the exact and non-exact cases and the two-step handoff; QSV TC-923
   checks the large compound identity refusal.

## Expected Results

Every value, refusal and charge is the one FR-007-AC-20 through AC-25 state, and each check fails
if the stated decision changes.
