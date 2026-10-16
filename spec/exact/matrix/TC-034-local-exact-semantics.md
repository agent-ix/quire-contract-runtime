---
id: TC-034
title: "Pin the exact semantics the agreement corpus does not reach"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-runtime/FR-007
    type: verifies
---
# TC-034: Pin the exact semantics the agreement corpus does not reach

## Description

Check all six rounding modes, quotient-sign rounding, IEEE exceptional semantics, rational
membership and canonical form, decimal normalized-versus-retained behavior, single-member
division exposure, Euclidean `mod`, affine power, same-dimension quantity arithmetic and
quantity type-fault order directly against the runtime, independently of the shared-corpus
agreement suites. Evidence:
`tests/exact_semantics.rs` (`--features exact`).

## Ownership and evidence

`ix://agent-ix/quire-exact/FR-365-AC-1` tests all six modes on both signs of a tie, and
`FR-365-AC-2` tests the default;
`FR-366-AC-1` through `FR-366-AC-4` test IEEE NaNs, signed zero and total order;
`FR-362-AC-12` and `FR-362-AC-13` test rational canonical form and membership; and
`FR-363-AC-6` and `FR-363-AC-7`, traced by kernel `tests/ir673_decimal.rs`, test decimal
normalized values versus retained ordering size charges and the unchanged Boolean result unit.
`ix://agent-ix/quire-exact/FR-357-AC-2` (`only_the_exposed_member_must_be_in_domain`) tests
single-member admission, `FR-357-AC-3` tests exposed-member domain refusal, and
`FR-357-AC-7` tests all four `mod` sign quadrants independently of the division profile.
Those are kernel subsets only. The RT quantity type-fault ordering and any RT/QSL agreement
remain separate obligations. The kernel tests left with the copied implementation; RT's
quantity cases remain in `tests/exact_semantics.rs`.

## Test Procedure

1. Round exact ties in every one of the six spellings, for a positive and a negative value.
   Check `NearestEven` chooses the even coefficient, `NearestAway` the greater magnitude,
   `TowardZero` the zero neighbour, `TowardPositive` the ceiling and `TowardNegative` the floor.
   Check the default `Exact` spelling refuses a discarded nonzero digit.
2. Combine quiet and signaling NaNs in both operand positions; check leftmost-wins, sign and
   payload retention, quieting and the `invalid` flag.
3. Convert a NaN payload too wide for the target; check the refusal rather than a truncation.
4. Convert `-0.0` and `+0.0` exactly; check the equal result and `discarded_negative_zero`.
5. Sort every distinguished bit pattern by `total_order_key`, including both zeros and NaNs.
6. Evaluate `1/3` under a domain whose numerator interval contains it and whose denominator
   interval excludes `3`; evaluate the same operation with no domain.
7. Construct rationals from unreduced, negative-denominator and zero numerators; check canonical
   form and that zero is `0/1`.
8. Compare retained `(110, 2)` against `(11, 1)` and order each of `(110, 2) < (11, 1)` and
   `(11, 1) < (11, 1)` on a fresh meter. Check equal normalized values and false ordering;
   check `integer_bits/decimal_digits/scale_expansion` of `8/3/1` versus `4/2/0`, with
   `value_occurrences/work_units/result_units = 2/3/1` for both comparisons. The Boolean
   `ordering.result-retain` point consumes one result unit in each case.
9. With a domain admitting `1` and `2`, evaluate `7 mod 3`, `-7 mod 3`, `7 mod -3` and
   `-7 mod -3`; expect `1`, `2`, `1` and `2` under each `div`/`rem` law, with no law parameter
   passed to `mod`. Refuse `-7 mod 3` under `Int[0,1]` as `ModuloOutOfDomain` without a value.
10. For every `div`/`rem` profile, check `10 div 5` returns `2` in `Int[1,10]` even though
    its unexposed remainder is `0`, and `-10 rem -1` returns `0` in `Int[-10,5]` even though
    its unexposed quotient is `10`. Refuse only when the exposed member is outside the domain,
    naming that member in `DivisionOutOfDomain` and returning no value.
11. Type-check each quantity operation against inputs failing more than one cause.
12. With decimal target coefficient bounds `[-100,100]` and scale `0`, divide `(-1,0)` by
    `(-3,0)` and `(1,0)` by `(-3,0)` under `TowardZero`; expect coefficient `0` and loss exact
    values `+1/3` and `-1/3`. Divide `(-1,0)` by `(-2,0)` under `NearestAway`; expect the
    exact `+1/2` tie to yield coefficient `+1`.
13. Raise `1 celsius` to exponents `0` and `-1`; expect `IllTyped` with
    `AffineUnitArithmetic` and zero charges for both.
14. In the graph rooted at `meter` with non-affine `kilometer = 1000 meters` and zero offset,
    multiply and divide `2 kilometers` by `3 meters`; expect `6000 meters²` and `2000/3`
    dimensionless, with neither `IncompatibleDimensions` nor `DistinctUnits`.

Compiler admission of a non-Euclidean `mod` selection belongs to QSpec FR-147-AC-5 and
TC-192 DIV-03. RT does not execute that admission check (FR-007-AC-19).

## Expected Results

Every semantics above is the one FR-007 states, and each
check fails if the stated decision changes.
