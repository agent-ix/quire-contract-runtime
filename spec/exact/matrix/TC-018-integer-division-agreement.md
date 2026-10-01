---
id: TC-018
title: "Agree with the authority on integer division vectors"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-runtime/FR-007
    type: verifies
---
# TC-018: Agree with the authority on integer division vectors

## Description

Execute every QSpec TC-192 vector on the runtime and on quire-spec-language.
Evidence: The QSL agreement oracle is removed from this repository; recreating it in agent-ix/quire-integration is planned under Linear IR-430. Steps 4-5 had evidence in the
arithmetic allocation bound in `tests/exact_allocation.rs`; that file left in IR-349 part 1, slice 1,
because integer division is the `quire-exact` kernel's now, so no test here evidences any step
(the matrix row is planned, with that reason).

## Test Procedure

1. Check the evaluated and admission-only lists together equal the TC-192 vector census.
2. For each evaluated vector, evaluate both sides under the same limits and compare Debug renderings
   of value, outcome kind, charge sequence and consumed counters.
3. Deny every admitted charge and compare the `Incomplete` records.
4. Sweep truncating, floor and Euclidean division and modulus over generated operands.
5. Charge division and modulus arithmetic at `max(bits(a), bits(b))` exact and one under, and deny
   it by injection with no quotient or remainder allocation.

## Expected Results

12 vectors agree exactly; DIV-03 and DIV-09 are admission-only and named.
