---
id: FR-007
title: "Evaluate the complete-V1 exact scalar operator families"
type: FR
relationships:
  - target: ix://agent-ix/quire-contract-runtime/FR-006
    type: depends_on
---
# FR-007: Evaluate the complete-V1 exact scalar operator families

## Description

When the `exact` feature is enabled, the runtime shall evaluate the scalar operator families of
complete V1 at agent-ix/quire-specification@7d7943a with values and outcome kinds equal to the
pinned quire-spec-language authority (d9d5273) on every shared-corpus vector, and charges equal to
the QSpec 7d7943a accounting schedule.

## Inputs

- Operands of one family and its declared type: integer division profiles, `Decimal[..]`,
  IEEE 754-2019 profiles, `Text[..]` under a Unicode 17 profile, enum declarations, unit graphs
  and quantities, `Integer`/`Int[..]`, `Rational[..]` and `Boolean`.

## Outputs

- An FR-006 outcome for each operation.

## Behavior

- Integer division and modulus follow the truncating, floor and Euclidean definitions.
- Decimal arithmetic, rounding and ordering follow `Decimal[..]`; ordering is measured on the
  retained representation, never a normalized one.
- IEEE operations follow the default profile's exact-intermediate and rounding rules.
- Text admission, comparison and normalization follow `value-text-unicode-17.md`; enum
  comparison is by declaration identity and ordering only for ordered enums.
- Quantities convert, combine and compare through the declared unit graph; topology faults are
  typed refusals.
- Integer arithmetic, rational arithmetic, numeric ordering and Boolean connectives charge the
  7d7943a `integer-arithmetic.*`, `rational-arithmetic.*`, `ordering.*` and
  `boolean.result-retain` points; a connective evaluates its right operand only when the left
  does not decide it.
- Every arithmetic, normalize, rounding, retain-upscale, unit-event and target-domain amount is
  derived from operand bit lengths and scales only and is charged before any intermediate or result
  is allocated; no charge is sized from a computed result.
- Model domains are out of scope (agent-ix/quire-spec-language#120), as is replay (#121).

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-007-AC-1 | Integer division and modulus agree with QSpec TC-192 in value, outcome kind and charges; the arithmetic amount `max(bits(a), bits(b))` is charged before the quotient exists. | Test (TC-018) |
| FR-007-AC-2 | Decimal arithmetic, rounding and ordering agree with QSpec TC-185, including D20–D21 ordering charges and D22–D23 result-retain upscale charges; every operand-derived amount is exact at its limit and denied one under before allocation. | Test (TC-019) |
| FR-007-AC-3 | IEEE profile operations agree with QSpec TC-193. | Test (TC-020) |
| FR-007-AC-4 | Text and enum operations agree with QSpec TC-186. | Test (TC-021) |
| FR-007-AC-5 | Quantity and unit-graph operations agree with QSpec TC-187. | Test (TC-022) |
| FR-007-AC-6 | Every evaluated shared-corpus vector is executed on both the runtime and the pinned authority with equal Debug renderings; admission-only vectors and charges not yet metered by the authority are listed by name. | Test (TC-018, TC-019, TC-020, TC-021, TC-022) |
| FR-007-AC-7 | Integer arithmetic, rational arithmetic, ordering and Boolean connectives match an independent `i128` oracle and the QSpec TC-191 P11 and TC-190 Q11 atom charges, with short-circuit and denial behavior at every point; operand-derived arithmetic and normalize amounts are charged before any intermediate or result is allocated. | Test (TC-023) |

## Dependencies

- **Upstream**: [FR-006](./FR-006-exact-outcomes-and-accounting.md);
  `ix://agent-ix/quire-specification` at `7d7943a`; quire-spec-language at `d9d5273`.
