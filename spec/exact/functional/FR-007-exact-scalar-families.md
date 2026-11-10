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

When the `exact` feature is enabled, the runtime shall consume `quire-exact` for kernel-owned
operations in the scalar families of
complete V1 in agent-ix/quire-specification with values and outcome kinds equal to the
quire-spec-language authority on every shared-corpus vector, and charges equal to the QSpec
accounting schedule.

## Inputs

- Operands of one family and its declared type: integer division profiles, `Decimal[..]`,
  IEEE 754-2019 profiles, `Text[..]` under a Unicode 17 profile, enum declarations, unit graphs
  and quantities, `Integer`/`Int[..]`, `Rational[..]` and `Boolean`.

## Outputs

- An FR-006 outcome for each operation.

## Behavior

- Integer division and modulus follow the truncating, floor and Euclidean definitions. The three
  laws are selectable for `div`/`rem` only. `mod` takes no law and is **always** the Euclidean
  remainder (`0 <= r < abs(b)`), charged only at the four `integer-modulus.*` points. A `mod`
  remainder outside the consumer domain is refused without a value as `ModuloOutOfDomain`.
  Compiler admission refuses a package selecting a non-Euclidean `mod` law with
  `invalid_package` / `incompatible-definition`, as specified by
  [quire-specification FR-147-AC-5](ix://agent-ix/quire-specification/FR-147) and QSpec TC-192 DIV-03. RT carries the
  refusal vocabulary but neither produces nor tests this admission refusal. `div`
  checks only its exposed quotient against the consumer domain; `rem` checks only its exposed
  remainder. An out-of-domain exposed member is refused without a value, while the unexposed
  member is never checked and cannot cause a refusal, as in QSpec
  `ix://agent-ix/quire-specification/FR-147`.
- Decimal arithmetic, rounding and ordering follow `Decimal[..]`. Two representations are
  distinguished and each governs a different thing. *Value* comparison — `Decimal::compare` and
  `numerically_equal` — is mathematical and is computed on the normalized representation, so
  `1.10` and `1.1` are equal and neither orders before the other; `Decimal` deliberately has no
  structural `PartialEq`. *Ordering size charges* use the retained operand representations,
  never their normalized forms, so an equal-value pair can have different ordering size counters.
  The Boolean `ordering.result-retain` charge remains one result unit for either pair. Decimal
  arithmetic's result-retain upscale charges are a separate operation (AC-2). The retained
  representation is kept as provenance and is never silently normalized away.
- Decimal rounding has the six spellings `Exact`, `TowardZero`, `TowardPositive`,
  `TowardNegative`, `NearestEven` and `NearestAway`, as in QSpec
  `ix://agent-ix/quire-specification/FR-140`. An omitted spelling is strict `Exact`, which
  refuses any discarded nonzero digit. On an exact tie, `NearestEven` takes the neighbour with
  the even coefficient and `NearestAway` takes the neighbour away from zero. `TowardZero` and the
  `NearestAway` tie use the sign of the exact value being rounded (for division, the exact
  quotient), not the sign of the retained coefficient. The retained coefficient is the integer
  coefficient of the result at the target scale. `TowardNegative` always takes the floor and
  `TowardPositive` always takes the ceiling,
  independently of sign. No mode consults the host's floating point.
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
- Exact quantity-to-quantity conversion (IR-818) uses `quire-semantic-value`'s
  `convert_quantity` over `UnitQuantity` with `QuantityTarget::Exact` at the revision in RT's
  `Cargo.lock`. There is no RT symbol or wrapper (FR-275-AC-18). The converted magnitude
  is the `ConvertedValue::Exact` rational in the `Conversion` and its unit is the target unit.
  There is no rounding parameter because [quire-contract-ir FR-038-AC-213](ix://agent-ix/quire-contract-ir/FR-038) admits
  only the `exact` mode for `quire.op.quantity.convert`; `Decimal` and `Integer` targets are out
  of this slice. A direct conversion needs equal dimensions: two declared units must share one
  dimension node, so distinct nodes with equal base-dimension maps are refused, while a compound
  side compares normalized base-dimension maps. A compound unit as the **target** is admitted by
  the same rule and has an empty root path, so it returns the canonical value in that unit with no
  edge charge (QSpec TC-187 U25's first step). Affine units are convertible; only `Add`, `Subtract`,
  `Multiply`, `Divide` and `Power` refuse them. `IncompatibleDimensions` is `IllTyped` reported
  as `Err` before the identity read, so it charges nothing. The charge order is one
  `unit.identity-read`; one `unit.edge` per edge of the source-to-root path and then of the
  root-to-target path, all before the first rational event and with no common-ancestor shortcut;
  two `unit.rational-arithmetic` events per edge (multiply then add forward, subtract then divide
  in reverse); then `unit.target-domain` with no size amount; then `unit.result-retain`. AD-004
  step 4 removed RT's `convert_quantity` with the rest of `quantity`; consumers use QSV directly.
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
  QSpec `integer-arithmetic.*`, `rational-arithmetic.*`, `ordering.*` and
  `boolean.result-retain` points; a connective evaluates its right operand only when the left
  does not decide it.
- Every arithmetic, normalize, rounding, retain-upscale, unit-event and target-domain amount is
  derived from operand bit lengths and scales only and is charged before any intermediate or result
  is allocated; no charge is sized from a computed result.
- Model domains are out of scope (agent-ix/quire-spec-language#120), as is replay (#121).

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-007-AC-8 | The six rounding spellings are `exact`, `toward-zero`, `toward-positive`, `toward-negative`, `nearest-even` and `nearest-away`, corresponding to the six named `RoundingMode` variants. For positive and negative exact ties, `NearestEven` selects the neighbour with the even coefficient, `NearestAway` selects the neighbour with greater absolute coefficient, `TowardZero` selects the neighbour nearer zero, `TowardPositive` selects the ceiling and `TowardNegative` selects the floor. An omitted spelling is `Exact` and refuses a discarded nonzero digit. | Test (TC-034) |
| FR-007-AC-13 | The runtime's consumed QSV `CompoundUnit` Debug rendering matches a fixed expected string listing every field declared by `CompoundUnitFields`, in declaration order. | Test (TC-035) |
| FR-007-AC-15 | Decimal division with a target coefficient interval `[-100,100]` and scale `0` under `TowardZero`: `(-1, 0) / (-3, 0)` yields coefficient `0` from exact quotient `+1/3`; `(1, 0) / (-3, 0)` yields coefficient `0` with exact value `-1/3` in its loss. Under `NearestAway`, `(-1, 0) / (-2, 0)` has exact `+1/2` and yields coefficient `+1`. | Test (TC-034) |
| FR-007-AC-18 | With a domain admitting residues `1` and `2`, `7 mod 3 = 1`, `-7 mod 3 = 2`, `7 mod -3 = 1` and `-7 mod -3 = 2`, each satisfying `0 <= r < abs(b)` regardless of the selected `div`/`rem` law; `mod` takes no law argument. An out-of-domain `mod` remainder is refused without a value as `ModuloOutOfDomain`. | Test (TC-034) |
| FR-007-AC-19 | Under every `div`/`rem` profile, `div` checks only its exposed quotient and `rem` checks only its exposed remainder against the consumer domain; neither refuses because the unexposed member is outside. `10 div 5` returns `2` in `Int[1,10]` although its remainder is `0`, and `-10 rem -1` returns `0` in `Int[-10,5]` although its quotient is `10`. `-10 div -1` in `Int[-10,5]` refuses without a value as `DivisionOutOfDomain` naming the Quotient member. `9 rem 10` in `Int[0,5]` refuses without a value as `DivisionOutOfDomain` naming the Remainder member although its quotient is `0`. | Test (TC-034) |

## Verification notes

The RT-local tests for retained rounding, Euclidean `mod` and exposed division-member behavior
are listed in TC-034. The runtime consumes `quire-exact` for exact-kernel behavior; its
specification is linked from the subsystem registry.

## Kernel ownership

The runtime consumes the exact kernel and retains only the runtime-specific obligations in this
requirement. The authoritative kernel specification is linked from the subsystem registry.

## Dependencies

- **Upstream**: [FR-006](./FR-006-exact-outcomes-and-accounting.md);
  `ix://agent-ix/quire-specification`; quire-spec-language.
