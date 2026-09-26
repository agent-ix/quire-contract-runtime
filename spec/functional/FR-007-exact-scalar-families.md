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

- Integer division and modulus follow the truncating, floor and Euclidean definitions. The three
  laws are selectable for `div`/`rem` only. `mod` is **always** the Euclidean remainder, whatever
  law a `div`/`rem` in the same package selected, and is charged only at the four
  `integer-modulus.*` points. A quotient/remainder pair is refused as a pair: when either member
  is outside the consumer domain the refusal names which members were admitted and exposes
  neither.
- Decimal arithmetic, rounding and ordering follow `Decimal[..]`. Two representations are
  distinguished and each governs a different thing. *Value* comparison — `Decimal::compare` and
  `numerically_equal` — is mathematical and is computed on the normalized representation, so
  `1.10` and `1.1` are equal and neither orders before the other; `Decimal` deliberately has no
  structural `PartialEq`. *Charges* are sized on the retained representation, never a normalized
  one, so a value retained at a larger scale costs more to order and to retain. The retained
  representation is kept as provenance and is never silently normalized away.
- Decimal rounding has six spellings, and an omitted spelling is strict `Exact`, which refuses any
  discarded nonzero digit rather than rounding. On an exact tie the tie-break is fixed per mode:
  `NearestEven` takes whichever of the two neighbours is even; `NearestAway` takes the neighbour
  away from zero, so the floor for a negative value and the ceiling otherwise; `TowardZero`,
  `Floor` and `Ceiling` are decided by the sign of the exact quotient, not by the sign of the
  retained coefficient. No mode consults the host's floating point.
- IEEE operations follow the default profile's exact-intermediate and rounding rules, and the
  profile's exceptional semantics are fixed rather than left to a backend:
  - NaN propagation is **leftmost-wins**: the result NaN is the first NaN operand in operand order,
    quieted, with its sign and payload preserved. A later NaN operand never displaces it.
  - The `invalid` flag is raised when **any** operand is a signaling NaN, whichever operand
    position it is in and whichever NaN is propagated.
  - A NaN payload that does not fit an explicit conversion's target width is refused as
    `IeeeNanPayloadNotRepresentable`; it is never truncated to fit.
  - Exact conversion out of IEEE loses the sign of a negative zero, because the exact value is
    zero and the exact families have no signed zero. The loss is reported as
    `discarded_negative_zero` rather than being silent, and `-0.0` and `+0.0` convert to the same
    exact value.
  - `total_order_key` provides the profile's total order over every bit pattern, including NaNs
    and both zeros, so an ordering is defined where IEEE comparison is not.
- Text admission, comparison and normalization follow `value-text-unicode-17.md`; enum
  comparison is by declaration identity and ordering only for ordered enums.
- Quantities convert, combine and compare through the declared unit graph; topology faults are
  typed refusals, decided before evaluation and in one fixed cause order per operation.
  `Add` and `Subtract` check incompatible dimensions, then affine-unit arithmetic, then distinct
  units, first failure wins. `Multiply`, `Divide` and `Power` check affine-unit arithmetic only:
  their dimensions combine rather than having to match, so a dimension difference is not a fault
  there. Every one of these is `IllTyped`, reported beside the outcome with zero charges.
- `Rational[lo..hi; dmin..dmax]` membership is componentwise on the **reduced** form: the reduced
  numerator must lie in the numerator interval and the reduced denominator in the denominator
  interval. It is therefore not a numeric interval — a domain whose numeric span plainly contains
  `1/3` still refuses it when the denominator interval excludes `3` — and a consumer that wants a
  numeric range must declare a denominator interval that admits every denominator it will meet.
  An evaluation given no result domain performs no membership decision at all and retains the
  result, so absent is not the same as unbounded-but-checked.
- The canonical form of a rational is fixed: the sign lives in the numerator, the denominator is
  strictly positive, the reduced form is unique, and zero is exactly `0/1` — never `-0/1` and
  never `0/n` for any other `n`. Every rational exposed by the runtime is in this form.
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
| FR-007-AC-8 | The six rounding spellings round every exact tie to the stated neighbour for both signs; an omitted spelling is `Exact` and refuses a discarded nonzero digit; no rounding path reads host floating point. | Test (TC-034) |
| FR-007-AC-9 | IEEE NaN propagation is leftmost-wins with sign and payload preserved and the result quieted; `invalid` is raised when any operand is signaling; an unrepresentable NaN payload is refused, never truncated; `-0.0` and `+0.0` convert to the same exact value with `discarded_negative_zero` reported; `total_order_key` totally orders every bit pattern including both zeros and NaNs. | Test (TC-034) |
| FR-007-AC-10 | `Rational` membership admits exactly the reduced pairs inside both intervals — including refusing a value whose numeric magnitude is inside the numerator interval but whose reduced denominator is outside the denominator interval — an absent domain decides no membership and retains, and every exposed rational is in canonical form with zero as `0/1`. | Test (TC-034) |
| FR-007-AC-11 | `Decimal` value comparison is on the normalized representation and charges are sized on the retained one, demonstrated by a pair equal in value whose ordering and retain charges differ; `Decimal` exposes no structural `PartialEq`. | Test (TC-034) |
| FR-007-AC-12 | `mod` returns the Euclidean remainder for every operand sign whatever `div`/`rem` law is selected; a quotient/remainder pair outside the consumer domain is refused as a pair naming which members were admitted, exposing neither; quantity `IllTyped` causes appear in the stated per-operation order with zero charges, and `Multiply`/`Divide`/`Power` raise no dimension fault. | Test (TC-034) |
| FR-007-AC-13 | Every hand-written `Debug` impl for a boxed value or type struct (`Rational`, `RationalDomain`, `Decimal`, `DecimalType`, `Quantity`, `Text`, `EnumValue`, `ObjectReference`, `CompoundUnit`) renders exactly the fields its `*Fields` struct declares, in declaration order, against a fixed pinned string. | Test (TC-035) |

## Dependencies

- **Upstream**: [FR-006](./FR-006-exact-outcomes-and-accounting.md);
  `ix://agent-ix/quire-specification` at `7d7943a`; quire-spec-language at `d9d5273`.
