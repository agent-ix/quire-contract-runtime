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
| FR-007-AC-1 | Integer division and modulus agree with QSpec ix://agent-ix/quire-specification/TC-192 in value, outcome kind and charges; the arithmetic amount `max(bits(a), bits(b))` is charged before the quotient exists. | Test (TC-018) |
| FR-007-AC-2 | Decimal arithmetic, rounding and ordering agree with QSpec ix://agent-ix/quire-specification/TC-185, including D20–D21 ordering charges and D22–D23 result-retain upscale charges; every operand-derived amount is exact at its limit and denied one under before allocation. | Test (TC-019) |
| FR-007-AC-3 | IEEE profile operations agree with QSpec `ix://agent-ix/quire-specification/TC-193`. | Test (TC-020) |
| FR-007-AC-4 | Text and enum operations agree with QSpec `ix://agent-ix/quire-specification/TC-186`. | Test (TC-021) |
| FR-007-AC-5 | Quantity and unit-graph operations agree with QSpec `ix://agent-ix/quire-specification/TC-187`. | Test (TC-022) |
| FR-007-AC-6 | Every evaluated shared-corpus vector is executed on both the runtime and the quire-spec-language authority with equal Debug renderings; admission-only vectors and charges not yet metered by the authority are listed by name. | Test (TC-018, TC-019, TC-020, TC-021, TC-022) |
| FR-007-AC-7 | Integer arithmetic, rational arithmetic, ordering and Boolean connectives match an independent `i128` oracle and the QSpec ix://agent-ix/quire-specification/TC-191 P11 and ix://agent-ix/quire-specification/TC-190 Q11 atom charges, with denial behavior at every kernel point; operand-derived arithmetic and normalize amounts are charged before any intermediate or result is allocated. | Test (TC-023) |
| FR-007-AC-8 | The six rounding spellings are `exact`, `toward-zero`, `toward-positive`, `toward-negative`, `nearest-even` and `nearest-away`, corresponding to the six named `RoundingMode` variants. For positive and negative exact ties, `NearestEven` selects the neighbour with the even coefficient, `NearestAway` selects the neighbour with greater absolute coefficient, `TowardZero` selects the neighbour nearer zero, `TowardPositive` selects the ceiling and `TowardNegative` selects the floor. An omitted spelling is `Exact` and refuses a discarded nonzero digit. | Test (TC-034) |
| FR-007-AC-9 | IEEE NaN propagation is leftmost-wins with sign and payload preserved and the result quieted; `invalid` is raised when any operand is signaling; an unrepresentable NaN payload is refused, never truncated; `-0.0` and `+0.0` convert to the same exact value with `discarded_negative_zero` reported; `total_order_key` totally orders every bit pattern including both zeros and NaNs. | Test (TC-034) |
| FR-007-AC-10 | `Rational` membership admits exactly the reduced pairs inside both intervals — including refusing a value whose numeric magnitude is inside the numerator interval but whose reduced denominator is outside the denominator interval — an absent domain decides no membership and retains, and every exposed rational is in canonical form with zero as `0/1`. | Test (TC-034) |
| FR-007-AC-11 | Retained decimals `(110, 2)` and `(11, 1)` compare equal in value and both normalize to `(11, 1)` without changing their retained representations. On separate fresh meters, `(110, 2) < (11, 1)` and `(11, 1) < (11, 1)` both return false; their ordering size counters (`integer_bits`, `decimal_digits`, `scale_expansion`) are respectively `8/3/1` and `4/2/0`, while both comparisons consume `value_occurrences/work_units/result_units = 2/3/1`, including one Boolean `ordering.result-retain` result unit. | Test (TC-034) |
| FR-007-AC-12 | Quantity `IllTyped` causes appear in the stated per-operation order with zero charges, and `Multiply`/`Divide`/`Power` raise no dimension fault. | Test (TC-034) |
| FR-007-AC-13 | Every hand-written `Debug` impl for a boxed value or type struct (`Rational`, `RationalDomain`, `Decimal`, `DecimalType`, `Quantity`, `Text`, `EnumValue`, `ObjectReference`, `CompoundUnit`) renders exactly the fields its `*Fields` struct declares, in declaration order, against a fixed expected string. | Test (TC-035) |
| FR-007-AC-15 | Decimal division with a target coefficient interval `[-100,100]` and scale `0` under `TowardZero`: `(-1, 0) / (-3, 0)` yields coefficient `0` from exact quotient `+1/3`; `(1, 0) / (-3, 0)` yields coefficient `0` with exact value `-1/3` in its loss. Under `NearestAway`, `(-1, 0) / (-2, 0)` has exact `+1/2` and yields coefficient `+1`. | Test (TC-034) |
| FR-007-AC-16 | `Power` on `1 celsius` with exponent `0` and with exponent `-1` returns `IllTyped` with cause `AffineUnitArithmetic` and zero charges in both cases. | Test (TC-034) |
| FR-007-AC-17 | In a unit graph with canonical root `meter` and non-affine `kilometer = 1000 meters` with zero offset, `Multiply` and `Divide` on `2 kilometers` and `3 meters` raise neither `IncompatibleDimensions` nor `DistinctUnits`. Their canonical results are `6000 meters²` and `2000/3` dimensionless, respectively. | Test (TC-034) |
| FR-007-AC-18 | With a domain admitting residues `1` and `2`, `7 mod 3 = 1`, `-7 mod 3 = 2`, `7 mod -3 = 1` and `-7 mod -3 = 2`, each satisfying `0 <= r < abs(b)` regardless of the selected `div`/`rem` law; `mod` takes no law argument. An out-of-domain `mod` remainder is refused without a value as `ModuloOutOfDomain`. | Test (TC-034) |
| FR-007-AC-19 | Under every `div`/`rem` profile, `div` checks only its exposed quotient and `rem` checks only its exposed remainder against the consumer domain; neither refuses because the unexposed member is outside. `10 div 5` returns `2` in `Int[1,10]` although its remainder is `0`, and `-10 rem -1` returns `0` in `Int[-10,5]` although its quotient is `10`. `-10 div -1` in `Int[-10,5]` refuses without a value as `DivisionOutOfDomain` naming the Quotient member. `9 rem 10` in `Int[0,5]` refuses without a value as `DivisionOutOfDomain` naming the Remainder member although its quotient is `0`. | Test (TC-034) |
| FR-007-AC-20 | `convert_quantity` with `QuantityTarget::Exact` between two units of equal dimension returns a completed `Conversion` whose `value()` is the `ConvertedValue::Exact` variant holding `inverse(target_path)(source_path(v))` over exact rationals and whose `unit()` is the target unit: `250 cm` to `m` is `5/2` (QSpec TC-187 U01), `0 degC` and `32 degF` to `K` are both `5463/20` (U04), compound `4 m^2` to `m2` is `4` (U22), compound `kg*m^2*s^-2` to `J` is `1` (U25 second step), and `1 u2` to `u1` is `-9` (U26); and for every numerator and denominator in an `i128` sweep of scale and offset fractions the magnitude equals an independent `i128` rational oracle. | Test (TC-201) |
| FR-007-AC-21 | `convert_quantity` with `Exact` between declared units on distinct dimension nodes, or from or to a compound unit whose normalized base-dimension map differs, returns `Err(IllTyped)` with cause `IncompatibleDimensions` before any `unit.identity-read` or other charge, leaving the meter unchanged: `1 m` to `s` (U03-style), a base-dimension name and shape equal to `L` under a different semantic-node key (U09), `1 N_m` to `J` (U22), and compound `4 m^2` to `m`. | Test (TC-201) |
| FR-007-AC-22 | `convert_quantity` with `Exact` admits an affine source or target unit and converts it exactly: `0 degC` to `K` returns `5463/20` and `1 u2` to `u1` returns `-9`, neither refused as `AffineUnitArithmetic` (U04, U26). | Test (TC-201) |
| FR-007-AC-23 | `convert_quantity` with `Exact` under a limit tuple that admits every charge charges one `unit.identity-read`, then all `unit.edge` charges for the source-to-root path followed by the root-to-target path with no shortcut, then two `unit.rational-arithmetic` events per edge, then `unit.target-domain` with no size amount, then `unit.result-retain`, at the consumed amounts the vectors state: `100 cm` to `m` under the exact U10 tuple returns `1` with largest amount `8` at the multiply event; `1 in` to `cm` under the first U15 tuple returns `127/50` after one read, two edges and four rational events of amounts 14, 15, 15 and 14; compound `4 m^2` to `cm2` under the first U22 tuple returns `40000` after one read, one reverse edge and two rational events; compound `4 m^2` to `m2` makes one read, target admission and retention with no edge; `1 N_m` to compound `kg*m^2*s^-2` and then that result to `J` under the U25 tuple each make one read, target admission and retention with no edge, the first step returning exact `1` in the compound unit; and `1 u2` to `u1` under the first U26 tuple returns `-9` after one read, three edges and six rational events. | Test (TC-201) |
| FR-007-AC-24 | `evaluate_quantity` `Add`, `Subtract`, `Multiply` and `Divide` of `1 degC` with `2 degC`, and `Power` of `1 degC` to the exponent `2`, each return `IllTyped` with cause `AffineUnitArithmetic` with zero charges (QSpec TC-187 U05), as do `1 u1 + 2 u1` and `1 u3 * 1 m` under `L(0,0,0,0,0,0)` (U20), while `1 u2 + 2 u2` under `L(4,0,0,2,5,1)` is not affine and returns exact `3 u2`. | Test (TC-201) |
| FR-007-AC-25 | When a charge of `convert_quantity` with `Exact` is denied, the result is `Incomplete` carrying the denied limit kind, limit, consumed amount, next charge and charge point, with no partial quantity: the first U10 tuple with `result_units` lowered to `0` is `Incomplete { limit_kind: result_units, limit: 0, consumed: 0, next_charge: 1, charge_point: unit.result-retain }`; `1 in` to `cm` under the second U15 tuple is `Incomplete { limit_kind: unit_edges, limit: 1, consumed: 1, next_charge: 2, charge_point: unit.edge }`, because both edges are charged before the first rational event; compound `4 m^2` to `cm2` under the second U22 tuple is `Incomplete { limit_kind: integer_bits, limit: 16, consumed: 5, next_charge: 17, charge_point: unit.rational-arithmetic }` at the divide event; and `1 u2` to `u1` under the second U26 tuple is `Incomplete { limit_kind: unit_edges, limit: 2, consumed: 2, next_charge: 3, charge_point: unit.edge }`, because no common-ancestor shortcut applies. | Test (TC-201) |

## Verification notes

FR-007-AC-20 through FR-007-AC-25 are checked by RT integration tests of QSV's public
`convert_quantity` and `evaluate_quantity` (TC-201, `tests/exact_quantity_conversion.rs`) under
`--features exact` (IR-818). They cite QSpec [quire-specification FR-142-AC-1](ix://agent-ix/quire-specification/FR-142), `FR-142-AC-2`,
`FR-142-AC-4`, `FR-142-AC-5`, `FR-142-AC-7` (exact-bound accounting; denial is incomplete with no
partial quantity) and `FR-142-AC-10`, and QSpec TC-187 rows U01, U03, U04, U05, U09, U10, U15, U20, U22,
U25 and U26 by id; they do not copy those bytes. `FR-142-AC-3` (lossy conversion) is out of this
slice. The U10 denying tuple (`result_units: 0`) and its `Incomplete` fields in AC-25 are this
spec's own derivation from the charge schedule, not a QSpec row: a differing executed value is a
finding against the criterion or the code, never a reason to change the test to fit.
`Decimal` and `Integer` targets and any `RoundingMode` are not specified here. Shared-corpus agreement stays IR-669 (TC-022).

The six literal rounding spellings, their mapping to the consumed `RoundingMode` variants,
the consumed enum's `Exact` default, directed-mode ties and explicit `Exact` refusal in
FR-007-AC-8, AC-15, AC-18 and AC-19 are backed by RT-local
`tests/exact_semantics.rs` tests; AC-16 and AC-17 use QSV directly in
`tests/exact_quantity_conversion.rs` under `--features exact` (TC-034). A compiler package
that omits the rounding spelling must still be tested to select `Exact`; the RT test
asserts only the consumed enum's default, not package compilation.

Use the stated AC-15 through AC-19 vectors as TC-034 expected results. If execution differs,
investigate the implementation and criterion together before changing either expected result.

## Kernel ownership

The runtime shall consume kernel scalar operations directly from `quire-exact`, without a
local implementation or re-export ([FR-275](./FR-275-single-exact-kernel.md)). The criteria
remain obligations on consumed behavior; they do not require RT to retest the kernel.

For FR-007-AC-7, [quire-exact FR-362-AC-1](ix://agent-ix/quire-exact/FR-362) through `FR-362-AC-9` and
`FR-362-AC-11` through `FR-362-AC-18` own the tested integer/rational values, atom charges,
stops and already-decided Boolean truth tables. `FR-362-AC-10` is planned and untagged:
the rational-normalize bit amount cannot be observed independently through the public meter;
IR-667 tracks that owner evidence gap. `FR-363-AC-1` through `FR-363-AC-7` own tested decimal
ordering subsets, and `FR-361-AC-1` through `FR-361-AC-9` own specified denied-work allocation
subsets. These owner tests do not execute the QSpec P11/Q11 expression workloads or compare RT
against QSL; IR-669 tracks two-sided agreement. `FR-362` leaves expression evaluation and the
decision to skip a right operand to the caller. RT therefore retains
`evaluate_boolean_short_circuit` and `ShortCircuitConnective`, with lazy invocation and stop
propagation required by FR-011-AC-3 and FR-011-AC-8 and evidenced by TC-032, not by the
kernel's already-decided Boolean tests.

FR-007-AC-1's large floor-division subset maps to [quire-exact FR-361-AC-7](ix://agent-ix/quire-exact/FR-361)
and `FR-361-AC-8` at `integer-division.operands`; its injected division/modulus subset maps
to `FR-361-AC-3`. None of these tests establishes AC-1's exact and one-under
`max(bits(a), bits(b))` charge at `integer-division.arithmetic` or
`integer-modulus.arithmetic`; quire-exact IR-678 tracks that owner evidence gap.
FR-007-AC-2's decimal allocation subset maps to `FR-361-AC-4` through `FR-361-AC-6` and
`FR-361-AC-9`, and its metered ordering subset to `FR-363-AC-1` through `FR-363-AC-7`.
These owner contracts do not prove QSpec/QSL shared-corpus agreement or the whole decimal
family. `FR-360` concerns
`Integer::abs(i64::MIN)` only and is not evidence for those broader claims.
For FR-007-AC-11, [quire-exact FR-363-AC-6](ix://agent-ix/quire-exact/FR-363) and `FR-363-AC-7` specify the
normalized-value and retained-operand-charge distinction; the kernel's
`tests/ir673_decimal.rs` tests the equal values, retained representations and exact
`8/3/1` versus `4/2/0` size counters with one result unit in each comparison.
The RT exact-feature obligation remains planned in TC-034 until RT-local evidence binds it.

[The exact matrix](../matrix/tests.md) separates current RT evidence, future kernel-owned
evidence, IR-669 agreement work and RT evaluation residue under IR-349 step 2 and IR-583. This spec amendment removes
no code or tests and does not claim IR-349's implementation is complete.

## Dependencies

- **Upstream**: [FR-006](./FR-006-exact-outcomes-and-accounting.md);
  `ix://agent-ix/quire-specification`; quire-spec-language.
