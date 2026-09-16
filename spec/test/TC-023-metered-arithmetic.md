---
id: TC-023
title: "Meter integer, rational, ordering and Boolean operations"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-runtime/FR-007
    type: verifies
---
# TC-023: Meter integer, rational, ordering and Boolean operations

## Description

The pinned authority does not meter these families yet (quire-spec-language#119), so charges are
checked against QSpec 7d7943a and values against an independent `i128` oracle. Evidence:
`tests/exact_arithmetic.rs` and `tests/exact_allocation.rs` (`--features exact`).

## Test Procedure

1. TC-191 P11 atoms: `2 > 0` and `2 - 1` each charge three points at `integer_bits` 2; the scalar
   atoms of `down(2)` cost 15 work and 5 result units and stop at `0 > 0` retention under 14 work.
2. `3/2` charges four rational points at `integer_bits` 2; TC-190 Q11 `acc + x` charges
   `integer-arithmetic.*` per step.
3. `implies`: left true costs 7 work and 3 results; left false skips the right operand and costs 4
   and 2. A stopped right operand is returned unchanged without `boolean.result-retain`.
4. Operand-derived rational amounts (`×` and `÷` sum cross parts, `+`/`-` add one to the larger
   cross product), zero divisors (undefined after operands only), domain refusal before retention;
   rational `cross_bits` and retained-decimal `sbits`/`sdigits` ordering amounts, including an
   aligned coefficient beyond `u64`.
5. Exact and one-under limits for every arithmetic and normalize charge (`2/3 × 3/2`, `3/4 ÷ 5/7`,
   `5/7 - 4/7`, `1000 - 999`, `255 × 255`, negation at the operand amount); amounts one below and
   at a power of two and under total cancellation equal the operand formula, not the result size;
   squaring, `(2^k-1)(2^k+1)`, `2^k - (2^k-1)` and `2^k × 1/2^k` denied at the operand-derived
   amount make no allocation request larger than an eighth of one operand.
6. Generated sweeps: 12² integer pairs × add/subtract/multiply/negate/four orderings, and 10²
   fraction pairs × five operations and four orderings, with every named denial.

## Expected Results

Every value equals the oracle, every charge schedule equals QSpec 7d7943a, and every denial
names its point with no result unit.
