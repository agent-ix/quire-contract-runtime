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

The local kernel tests check charges against QSpec and values against an independent `i128`
oracle. They do not establish quire-spec-language shared-corpus agreement (Linear IR-669, open quire-integration owner; IR-430 removal is Done). Evidence:
`tests/exact_arithmetic.rs` and `tests/exact_allocation.rs` (`--features exact`).

## Ownership and evidence

Kernel arithmetic, ordering, atom charges and already-decided Boolean retention leave RT
in IR-349: `ix://agent-ix/quire-exact/FR-362-AC-1` through `FR-362-AC-9` and
`FR-362-AC-11` through `FR-362-AC-18` own tested scalar subsets; decimal ordering belongs to
`FR-363-AC-1` through `FR-363-AC-7`, and specified denied-work allocation bounds to
`FR-361-AC-1` through `FR-361-AC-9`. `FR-362-AC-10` remains partial and untagged because
the public meter masks the rational-normalize bit amount; IR-667 owns that gap. These are
owner references, not copied tests or a claim
that the P11/Q11 expression workloads have been evaluated upstream. Lazy operand evaluation
is RT-owned and remains in TC-032; step 3's caller simulation in this file is not its retained
evidence. Shared-authority agreement remains separate, with open quire-integration ticket IR-669; IR-430 removal is Done.

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

Every value equals the oracle, every charge schedule equals QSpec, and every denial
names its point with no result unit.
