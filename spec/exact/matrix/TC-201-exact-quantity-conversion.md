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

Check `convert_quantity_exact`, the exact-only conversion that returns a `Quantity`, directly
against the runtime. It cites QSpec `ix://agent-ix/quire-specification/FR-142-AC-1` through
`FR-142-AC-5` and `FR-142-AC-10` and TC-187 rows U01, U03, U04, U09, U09b, U10, U15, U20, U22,
U25 and U26 by id; their vectors are re-stated in FR-007-AC-20 through AC-23, not copied from
QSpec. Compound-unit conversion targets, `Decimal` and `Integer` targets and rounding are out of
scope, so U25's first step is not run. Evidence (planned, IR-818): `tests/exact_quantity_conversion.rs`
(`--features exact`). Shared-corpus agreement is IR-669 (TC-022).

## Test Procedure

1. Convert the FR-007-AC-20 vectors (U01, U04, U09b, U22 `m2`, U25 second step, U26) and compare each
   magnitude and unit identity to the stated value; check no `DecimalLoss` is reported.
2. Sweep scale and offset fractions over `i128` families and compare each magnitude to an
   independent `i128` rational oracle.
3. Convert across distinct dimension nodes and across different base-dimension maps (U03-style, U09,
   U22 `N_m` to `J`); expect `Err(IllTyped)` `IncompatibleDimensions` and a meter with no charge.
4. Convert from and to an affine unit (U04); then run Add, Subtract, Multiply, Divide and Power on
   affine units (U20); expect `AffineUnitArithmetic` with zero charges for the arithmetic.
5. Run U10, U15 (both tuples), U22 `cm2` (both tuples), U25 second step and U26 (both tuples) under
   their exact limit tuples; check the recorded charge order, the `Incomplete` fields and that a
   denied charge leaves no partial quantity.

## Expected Results

Every value, refusal and charge is the one FR-007-AC-20 through AC-23 state, and each check fails if the
stated decision changes.
