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

Check rounding tie-breaks, IEEE exceptional semantics, rational membership and canonical form,
decimal normalized-versus-retained, Euclidean `mod` and quantity type-fault order directly against
the runtime, independently of the shared-corpus agreement suites. Evidence:
`tests/exact_semantics.rs` (`--features exact`).

## Test Procedure

1. Round exact ties in every one of the six spellings, for a positive and a negative value, and
   check the default spelling refuses a discarded nonzero digit.
2. Combine quiet and signaling NaNs in both operand positions; check leftmost-wins, sign and
   payload retention, quieting and the `invalid` flag.
3. Convert a NaN payload too wide for the target; check the refusal rather than a truncation.
4. Convert `-0.0` and `+0.0` exactly; check the equal result and `discarded_negative_zero`.
5. Sort every distinguished bit pattern by `total_order_key`, including both zeros and NaNs.
6. Evaluate `1/3` under a domain whose numerator interval contains it and whose denominator
   interval excludes `3`; evaluate the same operation with no domain.
7. Construct rationals from unreduced, negative-denominator and zero numerators; check canonical
   form and that zero is `0/1`.
8. Compare and order `1.10` against `1.1`; check equality, ordering and that the ordering and
   retain charges differ.
9. Evaluate `mod` for all four operand-sign combinations under each `div`/`rem` law.
10. Refuse a quotient/remainder pair with each admitted-member combination.
11. Type-check each quantity operation against inputs failing more than one cause.

## Expected Results

Every semantics above is the one FR-007 states, decided without host floating point, and each
check fails if the stated decision changes.
