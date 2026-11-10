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

Check retained rounding, quotient-sign rounding, single-member division exposure and Euclidean
`mod` directly against the runtime. Evidence: `tests/exact_semantics.rs` (`--features exact`).

## Ownership and evidence

RT-local `tests/exact_semantics.rs` backs the consumed rounding behavior in AC-8, quotient-sign
rounding and exact loss in AC-15, Euclidean `mod` in AC-18, and division exposure in AC-19.
The exact kernel's authoritative behavior is specified by quire-exact.

## Test Procedure

1. Round exact ties in every one of the six spellings, for a positive and a negative value.
   Check `NearestEven` chooses the even coefficient, `NearestAway` the greater magnitude,
   `TowardZero` the neighbour nearer zero, `TowardPositive` the ceiling and `TowardNegative` the floor.
   Check the default `Exact` spelling refuses a discarded nonzero digit.
2. With decimal target coefficient bounds `[-100,100]` and scale `0`, divide `(-1,0)` by
   `(-3,0)` and `(1,0)` by `(-3,0)` under `TowardZero`; expect coefficient `0` and loss exact
   values `+1/3` and `-1/3`. Divide `(-1,0)` by `(-2,0)` under `NearestAway`; expect the
   exact `+1/2` tie to yield coefficient `+1`.
3. With a domain admitting `1` and `2`, evaluate `7 mod 3`, `-7 mod 3`, `7 mod -3` and
   `-7 mod -3`; expect `1`, `2`, `1` and `2` under each `div`/`rem` law, with no law parameter
   passed to `mod`. Refuse `-7 mod 3` under `Int[0,1]` as `ModuloOutOfDomain` without a value.
4. For every `div`/`rem` profile, check `10 div 5` returns `2` in `Int[1,10]` even though
   its unexposed remainder is `0`, and `-10 rem -1` returns `0` in `Int[-10,5]` even though
   its unexposed quotient is `10`. Check `-10 div -1` in `Int[-10,5]` refuses without a value
   as `DivisionOutOfDomain` naming the Quotient member. Check `9 rem 10` in `Int[0,5]`
   refuses without a value as `DivisionOutOfDomain` naming the Remainder member although
   its quotient is `0`. The test author executes this vector against the kernel and corrects
   FR-007-AC-19 if the executed value differs; never change the test to fit the criterion.

Compiler admission of a non-Euclidean `mod` selection belongs to
[quire-specification FR-147-AC-5](ix://agent-ix/quire-specification/FR-147) and QSpec TC-192 DIV-03. RT neither produces
nor tests that admission refusal.

## Expected Results

Every retained semantics above is the one FR-007 states, and each check fails if the stated decision changes.
