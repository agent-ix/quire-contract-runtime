---
id: SR-14115
title: "IR-641 RT consumer gap-analysis"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-runtime; IR-641; Makefile, tests/deny_one_copy.rs, docs and full repository matrix"
review_set: subset
---

## Summary

Independent review of RT PR153. FAIL. Existing repository debt remains; no repository-complete or green strict-coverage claim.

## Verdict

FAIL. Existing repository debt remains; no repository-complete or green strict-coverage claim.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | 67 inherited acceptance criteria have no computed test binding; 107 inherited strict coverage rows remain unbacked | spec/core/functional/interface-001-runtime-api.md:- |
| FND-002 | medium | Three inherited IR-512 trace tags target a ticket rather than an acceptance criterion | tests/exact_function_application.rs:1093,1108,1267 |
| FND-003 | high | Inherited unauthorized exact residue remains in src/exact despite the required empty residue end state | src/exact/expression.rs:162-184 |

## Coverage

Candidate e8cc22f301a556bce8a1ad20c8936502e05538b9, exact base 3d2d8201dfda896fde0e960a05cfc26d44c5542f. No applicable AssuranceProfile exists under spec. No Cargo/native/fmt/backend/provision/aggregate command was run by this reviewer. Actual author focused05 raw log has six tests passed, no ignored tests, exit0, 16GiB/swap0, peak75100160B, zero OOM events and vanished scope. Existing final Clippy/fmt evidence is read, not rerun. Root retains normal aggregate gates. Cargo.toml, Cargo.lock, runtime/kernel sources and CI are unchanged. Deleted AWK logic is consumed via the real distributed CLI; no local algorithm, executable, shim or revision allowance was introduced. CLI check_lock source requires one or more explicit arguments; zero locks therefore fail rather than silently skip. Missing executable/malformed input cannot satisfy duplicate/split controls, whose full fragments and diagnostic assertions were inspected. Source inventory covers src/, scripts/, verification/ and tests/; candidate adds no orphan runtime API or stub. Exact Makefile substitution was independently observed with shell printf only: tracked space dir/Cargo.lock became two arguments, while é/Cargo.lock became literal Git quote/escape text. This observation did not replace or run checker logic. Real filename-boundary regression tests must use the same genuine CLI. Scope records include all reviewed units, including clean bindings. Plan completion: not assessed. Full computed matrix176:68tagged/67untagged/41method-without-symbol. Base176:63/72/41. Five Test criteria newly bound are AC5/24/25/26/28; AC27 Inspection gains an actual source-inspection binder without changing its verification method. Normal strict rows112→107, zero contradicted on both; exact removed five row identities are AC24..28 and no added identities. Unique row identities99→94 differ from raw row counts due duplicate reference rows; these are not interchangeable metrics. Optional full semantic review was not requested and was not run. Existing source residue is expressly unauthorized in FR275 and remains owned by IR349/IR590 migration; no exception is granted. IR878 is actually In Progress; live owner handoffs fad2588d and6a87b61b preserve inherited strict/matrix debt. All findings require explicit scope dispositions before merge.

## Examined Units

```yaml
scope:
- id: FR-004-AC-1
  path: spec/accounting/functional/FR-004-campaign-accounting.md
  role: examined
  excerpt: Mixed outcome sequences produce the specified four counters.
- id: FR-004-AC-2
  path: spec/accounting/functional/FR-004-campaign-accounting.md
  role: examined
  excerpt: Counter overflow saturates and never panics or wraps.
- id: FR-004-AC-4
  path: spec/accounting/functional/FR-004-campaign-accounting.md
  role: examined
  excerpt: Allocation-free immutable snapshots retain exact identity and all counters
    after subsequent report recording; imported snapshots cannot resume a mutable
    report.
- id: FR-004-AC-5
  path: spec/accounting/functional/FR-004-campaign-accounting.md
  role: examined
  excerpt: JSON transport round-trips real mixed reports and refuses independently
    authored malformed, incomplete, duplicate, unsupported or inconsistent data.
- id: FR-004-AC-6
  path: spec/accounting/functional/FR-004-campaign-accounting.md
  role: examined
  excerpt: Conservative at-limit inspection distinguishes representable values from
    potentially saturated counts/totals without inventing overflow history.
- id: FR-004-AC-7
  path: spec/accounting/functional/FR-004-campaign-accounting.md
  role: examined
  excerpt: Byte/identity/depth limits fail boundedly without partial output; allocator/process
    failure is not mislabeled as semantic refusal.
- id: FR-004-AC-8
  path: spec/accounting/functional/FR-004-campaign-accounting.md
  role: examined
  excerpt: A verdict whose requirement or revision differs from the report's increments
    no counter, leaves a prior snapshot unchanged, and returns an `IdentityMismatch`
    carrying both identities; a report that recorded a mismatch has the same counters
    as one that never saw the verdict.
- id: FR-004-AC-9
  path: spec/accounting/functional/FR-004-campaign-accounting.md
  role: examined
  excerpt: Encoded snapshots emit `schemaVersion`, `requirement`, `revision`, `counterSemantics`,
    `counts` in that order, with `accepted`, `rejected`, `failed`, `discarded` in
    that order inside `counts`; byte-for-byte comparison of encoded output observes
    this order.
- id: FR-004-AC-10
  path: spec/accounting/functional/FR-004-campaign-accounting.md
  role: examined
  excerpt: An unknown top-level or `counts` member is refused at its key before its
    value is read. In `src/snapshot_json.rs`, the unknown-key arm of `FieldVisitor::visit_str`
    must return an error before `SnapshotSeed` or `CountsSeed` can call `next_value`
    or `next_value_seed` for that member; neither visitor may consume an unknown value
    with `IgnoredAny` or an equivalent skip/decode path.
- id: FR-004-AC-11
  path: spec/accounting/functional/FR-004-campaign-accounting.md
  role: examined
  excerpt: The decoder validates each member without materializing an intermediate
    generic JSON value tree.
- id: FR-004-AC-12
  path: spec/accounting/functional/FR-004-campaign-accounting.md
  role: examined
  excerpt: Parser scratch on the `decode_campaign_snapshot` path remains bounded proportionally
    to the input cap of `MAX_BYTES` (65536 bytes), including escaped-string scratch
    and owned decoded identities; no input can cause scratch allocation to grow without
    a bound derived from that cap.
- id: FR-001-AC-1
  path: spec/core/functional/FR-001-verdict-observation.md
  role: examined
  excerpt: The three verdicts remain distinct through construction and pattern matching.
- id: FR-001-AC-2
  path: spec/core/functional/FR-001-verdict-observation.md
  role: examined
  excerpt: Every verdict carries requirement, revision, execution-point, and observation
    identity.
- id: FR-001-AC-4
  path: spec/core/functional/FR-001-verdict-observation.md
  role: examined
  excerpt: "The five clause outcomes \u2014 passed, failed, rejected, not-evaluated,\
    \ undefined \u2014 are distinct through construction and pattern matching, and\
    \ a failed or rejected clause retains its typed code, clause identity and optional\
    \ borrowed detail."
- id: FR-002-AC-1
  path: spec/core/functional/FR-002-safe-operators.md
  role: examined
  excerpt: Truth-table tests cover all short-circuit and total operators.
- id: FR-002-AC-2
  path: spec/core/functional/FR-002-safe-operators.md
  role: examined
  excerpt: Evaluation-count tests prove the distinct operand evaluation contracts.
- id: FR-002-AC-3
  path: spec/core/functional/FR-002-safe-operators.md
  role: examined
  excerpt: Boundary tests and Kani harnesses cover every definedness helper family.
- id: FR-002-AC-4
  path: spec/core/functional/FR-002-safe-operators.md
  role: examined
  excerpt: '`CheckedInteger` is implemented for exactly the twelve built-in integer
    types, each member agrees with the primitive `checked_*` operation, and the trait
    cannot be implemented outside the crate.'
- id: interface-001-AC-1
  path: spec/core/functional/interface-001-runtime-api.md
  role: examined
  excerpt: The runtime shall define no `quire_contract_runtime::exact` module.
- id: interface-001-AC-2
  path: spec/core/functional/interface-001-runtime-api.md
  role: examined
  excerpt: The runtime shall not re-export, by `pub use`, alias, wrapper or feature,
    any item that `quire-exact`, `quire-semantic-value` or the evaluation leaf crate
    exports.
- id: interface-001-AC-3
  path: spec/core/functional/interface-001-runtime-api.md
  role: examined
  excerpt: Where the upstream `quire-exact` crate exports an item name, the runtime
    shall contain no public item definition with that name.
- id: interface-001-AC-4
  path: spec/core/functional/interface-001-runtime-api.md
  role: examined
  excerpt: The runtime shall expose no item named in the `kernel.consumed` lists of
    the `exact` contract block at any `quire_contract_runtime` path, whether defined
    or re-exported there.
- id: interface-001-AC-5
  path: spec/core/functional/interface-001-runtime-api.md
  role: examined
  excerpt: 'The runtime shall define none of `Value`, `ValueType`, `Outcome`, `Refusal`,
    `Undefined`, `BoundViolation`, `Meter`, `ChargePoint`, `Incomplete`, `ScalarLimits`,
    `NodeKey`, `Origin` or `Location` (the `named_kernel_types` list): an explicit
    per-type instance of interface-001-AC-3.'
- id: interface-001-AC-6
  path: spec/core/functional/interface-001-runtime-api.md
  role: examined
  excerpt: The runtime's library source shall not call a `NodeKey` constructor.
- id: interface-001-AC-7
  path: spec/core/functional/interface-001-runtime-api.md
  role: examined
  excerpt: The runtime shall not define `Frame`, `Body`, `CheckedPackage`, `Evaluation`
    or `plan_call` in its own source (FR-275; no exception, no expiry).
- id: interface-001-AC-8
  path: spec/core/functional/interface-001-runtime-api.md
  role: examined
  excerpt: 'The runtime shall define the `negotiate_*` predicates AD-016 WP7 selects
    (today: `negotiate_integer_division`, `negotiate_ieee`) and the requirement, capability
    and disposition types they take and return in its own source.'
- id: interface-001-AC-9
  path: spec/core/functional/interface-001-runtime-api.md
  role: examined
  excerpt: While a residue `exact` operation exists in the runtime, an operation the
    correspondence table above maps to a `quire-exact` operation shall call the `quire-exact`
    operation the table maps it to.
- id: interface-001-AC-10
  path: spec/core/functional/interface-001-runtime-api.md
  role: examined
  excerpt: The runtime shall define every item the `core_items` list names in its
    `verdict`, `observation` and `accounting` modules.
- id: interface-001-AC-11
  path: spec/core/functional/interface-001-runtime-api.md
  role: examined
  excerpt: The runtime shall depend on `quire-exact` and `quire-semantic-value` only
    through the `exact` feature.
- id: interface-001-AC-13
  path: spec/core/functional/interface-001-runtime-api.md
  role: examined
  excerpt: While the `exact` feature is enabled and the `std` feature is disabled,
    the runtime shall build for `thumbv7em-none-eabi` at the Rust version that `compatibility.msrv`
    declares.
- id: interface-001-AC-14
  path: spec/core/functional/interface-001-runtime-api.md
  role: examined
  excerpt: The runtime's normal, build and dev dependency graphs shall include no
    crate from the `agent-ix/quire-spec-language` Git repository.
- id: NFR-001-AC-1
  path: spec/core/non-functional/NFR-001-no-std-footprint.md
  role: examined
  excerpt: The default feature profile compiles without `std` or an allocator.
- id: NFR-001-AC-2
  path: spec/core/non-functional/NFR-001-no-std-footprint.md
  role: examined
  excerpt: The library compiles with no `unsafe` code.
- id: NFR-001-AC-3
  path: spec/core/non-functional/NFR-001-no-std-footprint.md
  role: examined
  excerpt: On Rust 1.98.1 for `thumbv7em-none-eabi`, the fixed-population static-library
    consumer in `measurement/footprint/` has linked `.text` plus `.rodata` between
    500 bytes and 4 KiB and its runtime/harness objects retain no panic-path reference.
- id: NFR-001-M-1
  path: spec/core/non-functional/NFR-001-no-std-footprint.md
  role: examined
  excerpt: Unsafe blocks
- id: NFR-001-M-2
  path: spec/core/non-functional/NFR-001-no-std-footprint.md
  role: examined
  excerpt: Linked `.text` + `.rodata`
- id: NFR-002-AC-1
  path: spec/core/non-functional/NFR-002-panic-compatibility-license.md
  role: examined
  excerpt: Valid public evaluation and accounting inputs encounter no intentional
    panic path.
- id: NFR-002-AC-3
  path: spec/core/non-functional/NFR-002-panic-compatibility-license.md
  role: examined
  excerpt: Public data enums whose evolution affects downstream matching remain non-exhaustive,
    as the `compile_fail` doctests in `src/observation.rs` and `src/verdict.rs` show.
- id: NFR-002-AC-5
  path: spec/core/non-functional/NFR-002-panic-compatibility-license.md
  role: examined
  excerpt: The crate manifest declares `license = "AGPL-3.0-or-later"`.
- id: NFR-002-AC-6
  path: spec/core/non-functional/NFR-002-panic-compatibility-license.md
  role: examined
  excerpt: The crate manifest declares `publish = false`, disabling registry publication
    for this package.
- id: NFR-002-AC-7
  path: spec/core/non-functional/NFR-002-panic-compatibility-license.md
  role: examined
  excerpt: '`make deny` passes its license-policy check for the resolved dependency
    set.'
- id: NFR-002-M-1
  path: spec/core/non-functional/NFR-002-panic-compatibility-license.md
  role: examined
  excerpt: Intentional panic sites in library code
- id: NFR-002-M-2
  path: spec/core/non-functional/NFR-002-panic-compatibility-license.md
  role: examined
  excerpt: License policy violations
- id: NFR-002-M-3
  path: spec/core/non-functional/NFR-002-panic-compatibility-license.md
  role: examined
  excerpt: Registry publication enabled
- id: FR-006-AC-1
  path: spec/exact/functional/FR-006-exact-outcomes-and-accounting.md
  role: examined
  excerpt: The four outcome dispositions are distinct; completed `false` is a value.
    `Refusal` and `Undefined` are closed enums; `Incomplete` is a typed record whose
    reason uses the closed `LimitKind` and `ChargePoint` enums. The runtime consumes
    these kernel types directly under FR-275.
- id: FR-006-AC-2
  path: spec/exact/functional/FR-006-exact-outcomes-and-accounting.md
  role: examined
  excerpt: Ill-typed operand combinations are reported before evaluation with zero
    charges, and provenance-bearing refusals (invalid UTF-8 offset, stale identity)
    are typed.
- id: FR-006-AC-3
  path: spec/exact/functional/FR-006-exact-outcomes-and-accounting.md
  role: examined
  excerpt: Every charge point and limit kind round-trips its QSpec spelling; charges
    precede work; size counters are high-water, work/result cumulative; the first
    short counter in field order is reported with the exact denied amount.
- id: FR-006-AC-4
  path: spec/exact/functional/FR-006-exact-outcomes-and-accounting.md
  role: examined
  excerpt: An injected denial at any admitted charge point yields `Incomplete` on
    `work_units` naming that point, with no result units and no partial value.
- id: FR-006-AC-6
  path: spec/exact/functional/FR-006-exact-outcomes-and-accounting.md
  role: examined
  excerpt: Inspection identifies [quire-exact FR-096-AC-8](ix://agent-ix/quire-exact/FR-096)
    as the owner of `Refusal::code()` and `Refusal::cause()` for the twelve kernel
    refusal causes and `CheckedInvariant`. Under FR-275, RT consumes the typed kernel
    `Outcome::Refused(Refusal)` directly, with no RT record or code/cause mapping.
- id: FR-006-AC-7
  path: spec/exact/functional/FR-006-exact-outcomes-and-accounting.md
  role: examined
  excerpt: Inspection of `src/exact/` finds no host floating-point types, fields,
    casts or operations in the exact surface, including `f32` and `f64` uses.
- id: FR-006-AC-8
  path: spec/exact/functional/FR-006-exact-outcomes-and-accounting.md
  role: examined
  excerpt: With `exact` enabled and `std` disabled, the runtime builds for `thumbv7em-none-eabi`
    on the supported MSRV through the `build-exact-no-std-msrv` row of `make test-features`.
- id: FR-006-AC-9
  path: spec/exact/functional/FR-006-exact-outcomes-and-accounting.md
  role: examined
  excerpt: Every public item exposed through `quire_contract_runtime::exact` is re-exported
    by `src/exact/mod.rs` from a private child module; child module paths are inaccessible
    to consumers.
- id: FR-006-AC-10
  path: spec/exact/functional/FR-006-exact-outcomes-and-accounting.md
  role: examined
  excerpt: Non-test code in `src/exact/` has no panic site.
- id: FR-007-AC-1
  path: spec/exact/functional/FR-007-exact-scalar-families.md
  role: examined
  excerpt: Integer division and modulus agree with QSpec ix://agent-ix/quire-specification/TC-192
    in value, outcome kind and charges; the arithmetic amount `max(bits(a), bits(b))`
    is charged before the quotient exists.
- id: FR-007-AC-2
  path: spec/exact/functional/FR-007-exact-scalar-families.md
  role: examined
  excerpt: "Decimal arithmetic, rounding and ordering agree with QSpec ix://agent-ix/quire-specification/TC-185,\
    \ including D20\u2013D21 ordering charges and D22\u2013D23 result-retain upscale\
    \ charges; every operand-derived amount is exact at its limit and denied one under\
    \ before allocation."
- id: FR-007-AC-3
  path: spec/exact/functional/FR-007-exact-scalar-families.md
  role: examined
  excerpt: IEEE profile operations agree with QSpec `ix://agent-ix/quire-specification/TC-193`.
- id: FR-007-AC-4
  path: spec/exact/functional/FR-007-exact-scalar-families.md
  role: examined
  excerpt: Text and enum operations agree with QSpec `ix://agent-ix/quire-specification/TC-186`.
- id: FR-007-AC-5
  path: spec/exact/functional/FR-007-exact-scalar-families.md
  role: examined
  excerpt: Quantity and unit-graph operations agree with QSpec `ix://agent-ix/quire-specification/TC-187`.
- id: FR-007-AC-6
  path: spec/exact/functional/FR-007-exact-scalar-families.md
  role: examined
  excerpt: Every evaluated shared-corpus vector is executed on both the runtime and
    the quire-spec-language authority with equal Debug renderings; admission-only
    vectors and charges not yet metered by the authority are listed by name.
- id: FR-007-AC-7
  path: spec/exact/functional/FR-007-exact-scalar-families.md
  role: examined
  excerpt: Integer arithmetic, rational arithmetic, ordering and Boolean connectives
    match an independent `i128` oracle and the QSpec ix://agent-ix/quire-specification/TC-191
    P11 and ix://agent-ix/quire-specification/TC-190 Q11 atom charges, with denial
    behavior at every kernel point; operand-derived arithmetic and normalize amounts
    are charged before any intermediate or result is allocated.
- id: FR-007-AC-8
  path: spec/exact/functional/FR-007-exact-scalar-families.md
  role: examined
  excerpt: The six rounding spellings are `exact`, `toward-zero`, `toward-positive`,
    `toward-negative`, `nearest-even` and `nearest-away`, corresponding to the six
    named `RoundingMode` variants. For positive and negative exact ties, `NearestEven`
    selects the neighbour with the even coefficient, `NearestAway` selects the neighbour
    with greater absolute coefficient, `TowardZero` selects the neighbour nearer zero,
    `TowardPositive` selects the ceiling and `TowardNegative` selects the floor. An
    omitted spelling is `Exact` and refuses a discarded nonzero digit.
- id: FR-007-AC-9
  path: spec/exact/functional/FR-007-exact-scalar-families.md
  role: examined
  excerpt: IEEE NaN propagation is leftmost-wins with sign and payload preserved and
    the result quieted; `invalid` is raised when any operand is signaling; an unrepresentable
    NaN payload is refused, never truncated; `-0.0` and `+0.0` convert to the same
    exact value with `discarded_negative_zero` reported; `total_order_key` totally
    orders every bit pattern including both zeros and NaNs.
- id: FR-007-AC-10
  path: spec/exact/functional/FR-007-exact-scalar-families.md
  role: examined
  excerpt: "`Rational` membership admits exactly the reduced pairs inside both intervals\
    \ \u2014 including refusing a value whose numeric magnitude is inside the numerator\
    \ interval but whose reduced denominator is outside the denominator interval \u2014\
    \ an absent domain decides no membership and retains, and every exposed rational\
    \ is in canonical form with zero as `0/1`."
- id: FR-007-AC-11
  path: spec/exact/functional/FR-007-exact-scalar-families.md
  role: examined
  excerpt: Retained decimals `(110, 2)` and `(11, 1)` compare equal in value and both
    normalize to `(11, 1)` without changing their retained representations. On separate
    fresh meters, `(110, 2) < (11, 1)` and `(11, 1) < (11, 1)` both return false;
    their ordering size counters (`integer_bits`, `decimal_digits`, `scale_expansion`)
    are respectively `8/3/1` and `4/2/0`, while both comparisons consume `value_occurrences/work_units/result_units
    = 2/3/1`, including one Boolean `ordering.result-retain` result unit.
- id: FR-007-AC-12
  path: spec/exact/functional/FR-007-exact-scalar-families.md
  role: examined
  excerpt: Quantity `IllTyped` causes appear in the stated per-operation order with
    zero charges, and `Multiply`/`Divide`/`Power` raise no dimension fault.
- id: FR-007-AC-13
  path: spec/exact/functional/FR-007-exact-scalar-families.md
  role: examined
  excerpt: Every hand-written `Debug` impl for a boxed value or type struct (`Rational`,
    `RationalDomain`, `Decimal`, `DecimalType`, `Quantity`, `Text`, `EnumValue`, `ObjectReference`,
    `CompoundUnit`) renders exactly the fields its `*Fields` struct declares, in declaration
    order, against a fixed expected string.
- id: FR-007-AC-15
  path: spec/exact/functional/FR-007-exact-scalar-families.md
  role: examined
  excerpt: 'Decimal division with a target coefficient interval `[-100,100]` and scale
    `0` under `TowardZero`: `(-1, 0) / (-3, 0)` yields coefficient `0` from exact
    quotient `+1/3`; `(1, 0) / (-3, 0)` yields coefficient `0` with exact value `-1/3`
    in its loss. Under `NearestAway`, `(-1, 0) / (-2, 0)` has exact `+1/2` and yields
    coefficient `+1`.'
- id: FR-007-AC-16
  path: spec/exact/functional/FR-007-exact-scalar-families.md
  role: examined
  excerpt: '`Power` on `1 celsius` with exponent `0` and with exponent `-1` returns
    `IllTyped` with cause `AffineUnitArithmetic` and zero charges in both cases.'
- id: FR-007-AC-17
  path: spec/exact/functional/FR-007-exact-scalar-families.md
  role: examined
  excerpt: "In a unit graph with canonical root `meter` and non-affine `kilometer\
    \ = 1000 meters` with zero offset, `Multiply` and `Divide` on `2 kilometers` and\
    \ `3 meters` raise neither `IncompatibleDimensions` nor `DistinctUnits`. Their\
    \ canonical results are `6000 meters\xB2` and `2000/3` dimensionless, respectively."
- id: FR-007-AC-18
  path: spec/exact/functional/FR-007-exact-scalar-families.md
  role: examined
  excerpt: With a domain admitting residues `1` and `2`, `7 mod 3 = 1`, `-7 mod 3
    = 2`, `7 mod -3 = 1` and `-7 mod -3 = 2`, each satisfying `0 <= r < abs(b)` regardless
    of the selected `div`/`rem` law; `mod` takes no law argument. An out-of-domain
    `mod` remainder is refused without a value as `ModuloOutOfDomain`.
- id: FR-007-AC-19
  path: spec/exact/functional/FR-007-exact-scalar-families.md
  role: examined
  excerpt: Under every `div`/`rem` profile, `div` checks only its exposed quotient
    and `rem` checks only its exposed remainder against the consumer domain; neither
    refuses because the unexposed member is outside. `10 div 5` returns `2` in `Int[1,10]`
    although its remainder is `0`, and `-10 rem -1` returns `0` in `Int[-10,5]` although
    its quotient is `10`. `-10 div -1` in `Int[-10,5]` refuses without a value as
    `DivisionOutOfDomain` naming the Quotient member. `9 rem 10` in `Int[0,5]` refuses
    without a value as `DivisionOutOfDomain` naming the Remainder member although
    its quotient is `0`.
- id: FR-007-AC-20
  path: spec/exact/functional/FR-007-exact-scalar-families.md
  role: examined
  excerpt: '`convert_quantity` with `QuantityTarget::Exact` between two units of equal
    dimension returns a completed `Conversion` whose `value()` is the `ConvertedValue::Exact`
    variant holding `inverse(target_path)(source_path(v))` over exact rationals and
    whose `unit()` is the target unit: `250 cm` to `m` is `5/2` (QSpec TC-187 U01),
    `0 degC` and `32 degF` to `K` are both `5463/20` (U04), compound `4 m^2` to `m2`
    is `4` (U22), compound `kg*m^2*s^-2` to `J` is `1` (U25 second step), and `1 u2`
    to `u1` is `-9` (U26); and for every numerator and denominator in an `i128` sweep
    of scale and offset fractions t'
- id: FR-007-AC-21
  path: spec/exact/functional/FR-007-exact-scalar-families.md
  role: examined
  excerpt: '`convert_quantity` with `Exact` between declared units on distinct dimension
    nodes, or from or to a compound unit whose normalized base-dimension map differs,
    returns `Err(IllTyped)` with cause `IncompatibleDimensions` before any `unit.identity-read`
    or other charge, leaving the meter unchanged: `1 m` to `s` (U03-style), a base-dimension
    name and shape equal to `L` under a different semantic-node key (U09), `1 N_m`
    to `J` (U22), and compound `4 m^2` to `m`.'
- id: FR-007-AC-22
  path: spec/exact/functional/FR-007-exact-scalar-families.md
  role: examined
  excerpt: '`convert_quantity` with `Exact` admits an affine source or target unit
    and converts it exactly: `0 degC` to `K` returns `5463/20` and `1 u2` to `u1`
    returns `-9`, neither refused as `AffineUnitArithmetic` (U04, U26).'
- id: FR-007-AC-23
  path: spec/exact/functional/FR-007-exact-scalar-families.md
  role: examined
  excerpt: '`convert_quantity` with `Exact` under a limit tuple that admits every
    charge charges one `unit.identity-read`, then all `unit.edge` charges for the
    source-to-root path followed by the root-to-target path with no shortcut, then
    two `unit.rational-arithmetic` events per edge, then `unit.target-domain` with
    no size amount, then `unit.result-retain`, at the consumed amounts the vectors
    state: `100 cm` to `m` under the exact U10 tuple returns `1` with largest amount
    `8` at the multiply event; `1 in` to `cm` under the first U15 tuple returns `127/50`
    after one read, two edges and four rational event'
- id: FR-007-AC-24
  path: spec/exact/functional/FR-007-exact-scalar-families.md
  role: examined
  excerpt: '`evaluate_quantity` `Add`, `Subtract`, `Multiply` and `Divide` of `1 degC`
    with `2 degC`, and `Power` of `1 degC` to the exponent `2`, each return `IllTyped`
    with cause `AffineUnitArithmetic` with zero charges (QSpec TC-187 U05), as do
    `1 u1 + 2 u1` and `1 u3 * 1 m` under `L(0,0,0,0,0,0)` (U20), while `1 u2 + 2 u2`
    under `L(4,0,0,2,5,1)` is not affine and returns exact `3 u2`.'
- id: FR-007-AC-25
  path: spec/exact/functional/FR-007-exact-scalar-families.md
  role: examined
  excerpt: 'When a charge of `convert_quantity` with `Exact` is denied, the result
    is `Incomplete` carrying the denied limit kind, limit, consumed amount, next charge
    and charge point, with no partial quantity: the first U10 tuple with `result_units`
    lowered to `0` is `Incomplete { limit_kind: result_units, limit: 0, consumed:
    0, next_charge: 1, charge_point: unit.result-retain }`; `1 in` to `cm` under the
    second U15 tuple is `Incomplete { limit_kind: unit_edges, limit: 1, consumed:
    1, next_charge: 2, charge_point: unit.edge }`, because both edges are charged
    before the first rational event; compound `4 m'
- id: FR-008-AC-1
  path: spec/exact/functional/FR-008-composite-collection-and-equality.md
  role: examined
  excerpt: 'A `TypeEnvironment` admits a closed set of record, tuple and object-type
    declarations only after member-type and both recursion-rule checks succeed, and
    refuses at the originating declaration with a typed `DeclarationCause` otherwise;
    `record`, `tuple`, `evaluate_record` and `evaluate_tuple` construct in declaration
    order under the first-stopped rule and charge `composite.result-retain` with the
    exact retained `occ`; a deferred result outside its declared type carries `CheckedInvariant
    { cause: DeferredResultNotAdmitted }`.'
- id: FR-008-AC-2
  path: spec/exact/functional/FR-008-composite-collection-and-equality.md
  role: examined
  excerpt: '`TypeEnvironment::build` constructs a finite value bottom-up from a `ValueGraph`,
    sharing a node reached from more than one slot as one immutable value with no
    object identity created, and refuses the first containment cycle as `ContainmentCycle`
    at the node that closes it.'
- id: FR-008-AC-3
  path: spec/exact/functional/FR-008-composite-collection-and-equality.md
  role: examined
  excerpt: '`construct_collection` charges `collection.element` before running each
    deferred element and stops at the first non-completing element with no later element
    run; a completed element outside its declared type returns `CheckedInvariant {
    cause: CollectionElementNotAdmitted }`; `form_collection` refuses a type-mismatched
    occurrence before any charge; formation charges membership comparisons, `collection.bound`
    and `collection.result-retain` in that order, and a set or bag is stored in ascending
    canonical-key order.'
- id: FR-008-AC-10
  path: spec/exact/functional/FR-008-composite-collection-and-equality.md
  role: examined
  excerpt: When formation coalesces occurrences, the runtime charges one `collection.member-walk`
    and one `collection.member-test` per membership comparison for a set, bag or ordered
    set, and charges neither point for a sequence.
- id: FR-008-AC-11
  path: spec/exact/functional/FR-008-composite-collection-and-equality.md
  role: examined
  excerpt: 'When a collection is retained, the runtime stores a sequence in supplied
    occurrence order, an ordered set as the first occurrence of each distinct member
    in first-occurrence order, a set as one occurrence of each distinct member in
    ascending canonical-key order, and a bag as every occurrence in ascending canonical-key
    order by multiplicity: equal occurrences are one member whose multiplicity is
    their number, and each is represented by that member''s first-supplied value,
    as in ix://agent-ix/quire-specification/FR-144 and FR-144-AC-21.'
- id: FR-008-AC-12
  path: spec/exact/functional/FR-008-composite-collection-and-equality.md
  role: examined
  excerpt: When a retained collection is visited by equality planning or by a further
    formation, the runtime visits its elements in the order it stored them, for every
    one of the four kinds.
- id: FR-008-AC-4
  path: spec/exact/functional/FR-008-composite-collection-and-equality.md
  role: examined
  excerpt: The type-owned canonical key totally orders every keyed type, ranks `Absent
    < Null < Present`, fixes set and bag canonical and visiting order, and is evaluated
    iteratively at a depth that would overflow a recursive host-stack walk.
- id: FR-008-AC-5
  path: spec/exact/functional/FR-008-composite-collection-and-equality.md
  role: examined
  excerpt: '`plan_equality` forms the occurrence-pair plan with no charge, refuses
    a cross-universe reference pair as `ForeignReference`, and distinguishes unlike
    collection kinds (`CollectionKindMismatch`) from unlike value kinds (`ValueKindMismatch`);
    `check_equality` refuses before any charge, including `operator-ineligible` when
    either operand type bears an IEEE value at any depth; `CheckedEquality::evaluate`
    charges the selected schedule and its conversions in operand order, and each of
    the six checked operand failures carries its ix://agent-ix/quire-exact/FR-369
    cause, retaining `IllTypedCause` for '
- id: FR-008-AC-6
  path: spec/exact/functional/FR-008-composite-collection-and-equality.md
  role: examined
  excerpt: A `Reference<T>` value carries only its supplied `(universe, object-type,
    identity)` triple, is constructed from no source form, and compares equal only
    within one universe.
- id: FR-008-AC-7
  path: spec/exact/functional/FR-008-composite-collection-and-equality.md
  role: examined
  excerpt: The extended `Undefined` and `Refusal` vocabularies, `BoundViolation` and
    `Refusal::cause()` are closed, typed and distinct from every FR-006 variant; both
    `CardinalityOutOfBound` directions are reachable and report their `code()` and
    `cause()`; `CheckedInvariant` is unreachable from any admitted vector in the shared
    corpus.
- id: FR-008-AC-8
  path: spec/exact/functional/FR-008-composite-collection-and-equality.md
  role: examined
  excerpt: Every one of the twelve added charge points round-trips its QSpec spelling,
    `ChargePoint::ALL` has exactly 63 members including `temporal.visit`, and an injected
    denial at each of the twelve yields `Incomplete` on `work_units` naming that point
    with every counter left unchanged.
- id: FR-008-AC-9
  path: spec/exact/functional/FR-008-composite-collection-and-equality.md
  role: examined
  excerpt: 'The consumed kernel `Value` has iterative `Debug` and nested-node `Drop`:
    formatting or dropping a value nested past a recursive walk''s host-stack limit
    does not overflow the stack and completes in time and allocation proportional
    to the value''s size, and its `Debug` output matches an exact literal string in
    both compact and alternate form on a small fixed value.'
- id: FR-008-AC-13
  path: spec/exact/functional/FR-008-composite-collection-and-equality.md
  role: examined
  excerpt: For an `Option`/`Collection`-nested `ValueType` whose depth exceeds a recursive
    host-stack walk under the admitted source-size cap, `Clone`, `PartialEq`, `Debug`
    and `Drop` complete on a small-stack thread and a default-stack child process
    without host-stack overflow; unequal leaf, collection kind and bound remain unequal,
    and the shallow compact and alternate `Debug` output matches the derived representation
    exactly.
- id: FR-009-AC-1
  path: spec/exact/functional/FR-009-i13-backend-negotiation.md
  role: examined
  excerpt: For every input slice, both negotiators return exactly one disposition
    per item in input order, including for the empty slice, and permuting the input
    permutes the result identically.
- id: FR-009-AC-2
  path: spec/exact/functional/FR-009-i13-backend-negotiation.md
  role: examined
  excerpt: A `Mathematical` consumer is `Supported`; a `Finite` consumer is `Supported`
    exactly when `operand`, `intermediate` and `result` are all present, and `RequiresBound`
    for each of the seven incomplete bound subsets.
- id: FR-009-AC-3
  path: spec/exact/functional/FR-009-i13-backend-negotiation.md
  role: examined
  excerpt: 'IEEE causes are decided in the order width, operation, rounding, exceptional
    policy, first failure wins: an item failing two checks reports the earlier cause,
    and a non-rounding operation with an unsupported rounding direction is not `Unsupported(Rounding(..))`.'
- id: FR-009-AC-4
  path: spec/exact/functional/FR-009-i13-backend-negotiation.md
  role: examined
  excerpt: '`requires_finite_proof` yields `RequiresBound` only when no `unsupported`
    cause applies; an item that both lacks a capability and needs an undischargeable
    proof reports the capability cause.'
- id: FR-009-AC-5
  path: spec/exact/functional/FR-009-i13-backend-negotiation.md
  role: examined
  excerpt: No `IeeeDisposition` is convertible into an `Outcome` variant, as the `compile_fail`
    doctest on `IeeeDisposition` (`src/exact/ieee.rs`) shows.
- id: FR-010-AC-1
  path: spec/exact/functional/FR-010-injected-charge-denial-seam.md
  role: examined
  excerpt: For every charge point that an RT-owned residue operation can issue through
    the `quire-exact` meter, an occurrence-1 injected denial returns `Incomplete`
    with `WorkUnits` and that point; the observed work consumption equals the record's
    `consumed`, result consumption is zero, and the denied point is absent from the
    admitted-charge log. [quire-exact FR-358-AC-1](ix://agent-ix/quire-exact/FR-358)
    owns full meter-state atomicity.
- id: FR-010-AC-2
  path: spec/exact/functional/FR-010-injected-charge-denial-seam.md
  role: examined
  excerpt: A denial reached through an RT exact-feature charge-point driver returns
    the `quire-exact` record specified and directly tested by [quire-exact FR-358-AC-12](ix://agent-ix/quire-exact/FR-358);
    RT adds no second record definition.
- id: FR-010-AC-3
  path: spec/exact/functional/FR-010-injected-charge-denial-seam.md
  role: examined
  excerpt: An RT exact-feature charge-point driver uses the `quire-exact` injection-versus-ordinary-limit
    precedence specified and directly tested by [quire-exact FR-358-AC-13](ix://agent-ix/quire-exact/FR-358);
    RT adds no second precedence rule.
- id: FR-010-AC-4
  path: spec/exact/functional/FR-010-injected-charge-denial-seam.md
  role: examined
  excerpt: An injection at occurrence `n` fires on the `n`th admitted charge at that
    point, counting no charge at any other point and no charge at that point that
    a short counter denied.
- id: FR-010-AC-5
  path: spec/exact/functional/FR-010-injected-charge-denial-seam.md
  role: examined
  excerpt: After the injected denial fires, further charges are metered against the
    configured limits and no second charge is injected-denied.
- id: FR-010-AC-6
  path: spec/exact/functional/FR-010-injected-charge-denial-seam.md
  role: examined
  excerpt: A 0-based `occurrence` cannot be constructed, so an injected denial can
    never silently match no charge and degrade into "no fault injected".
- id: FR-011-AC-1
  path: spec/exact/functional/FR-011-meter-state-at-a-stop.md
  role: examined
  excerpt: 'For each of `Undefined`, `Refused` and `Incomplete`, the meter after the
    stop holds exactly the charges admitted before it: an `Undefined` division by
    zero retains the operands charge and no arithmetic charge; a refused result retains
    the arithmetic charge and no result unit; a denied charge retains neither.'
- id: FR-011-AC-2
  path: spec/exact/functional/FR-011-meter-state-at-a-stop.md
  role: examined
  excerpt: A quantity `power` with zero base and negative exponent is `Undefined::DivisionByZero`,
    and a divide by zero is reported in preference to it when both hold.
- id: FR-011-AC-3
  path: spec/exact/functional/FR-011-meter-state-at-a-stop.md
  role: examined
  excerpt: A lazy connective skips its right operand when the left decides the result;
    for every connective kind and every decided operand pair, its completed result
    admits exactly one `boolean.result-retain` charge.
- id: FR-011-AC-4
  path: spec/exact/functional/FR-011-meter-state-at-a-stop.md
  role: examined
  excerpt: A charge whose second-scanned counter is short writes no counter, appends
    no log entry and advances no occurrence counter; and a charge presented with its
    size vector in either order reports the same first short counter in `ScalarLimitsV1`
    field order.
- id: FR-011-AC-5
  path: spec/exact/functional/FR-011-meter-state-at-a-stop.md
  role: examined
  excerpt: Past `CHARGE_LOG_CAPACITY` admitted charges the log holds exactly the first
    4096 points in admission order, `charge_log_truncated()` is true, and the counters
    are still exact and still enforced.
- id: FR-011-AC-6
  path: spec/exact/functional/FR-011-meter-state-at-a-stop.md
  role: examined
  excerpt: With a cumulative limit of `u64::MAX`, a counter at `u64::MAX - 1` admits
    one unit to reach the limit and denies the next unit without wrapping or changing
    any meter state. A quantity power whose `unit.rational-arithmetic` request exceeds
    `u64::MAX` returns `Incomplete` before computing the power; `next_charge` equals
    the exact request, and the denied charge changes no counter, admission count or
    admitted-charge log.
- id: FR-011-AC-7
  path: spec/exact/functional/FR-011-meter-state-at-a-stop.md
  role: examined
  excerpt: '`Meter::consumed` answers without panic for every member of `LimitKind::ALL`,
    returning zero for untouched counters, admitted high-water values for semantic
    sizes and admitted cumulative totals for work/results; IEEE flag iteration and
    QSV dimension and compound-unit term iteration are stable for every accepted graph-node
    insertion order, while QSV''s checked-node and `UnitGraph::from_checked_nodes`
    boundaries refuse malformed order with the stated first cause.'
- id: FR-011-AC-8
  path: spec/exact/functional/FR-011-meter-state-at-a-stop.md
  role: examined
  excerpt: A connective whose right operand stops returns that stop unchanged, admits
    no `boolean.result-retain` charge and consumes no result unit.
- id: FR-012-AC-1
  path: spec/exact/functional/FR-012-carried-compiler-vocabulary.md
  role: examined
  excerpt: '`NodeKey::from_hex` accepts exactly the 64-lowercase-hex-digit inputs
    and returns `None` for every wrong length, uppercase digit and non-hexadecimal
    byte; `from_bytes`/`as_bytes`/`from_hex`/`Display` round-trip every digest unchanged.
    Retired at IR-882 step 4; this ID remains reserved with its original criterion
    text.'
- id: FR-012-AC-2
  path: spec/exact/functional/FR-012-carried-compiler-vocabulary.md
  role: examined
  excerpt: '`NODE_KEY_DOMAIN` and `COMPOUND_UNIT_DOMAIN` equal their normative strings,
    and no code path constructs either from parts.'
- id: FR-012-AC-3
  path: spec/exact/functional/FR-012-carried-compiler-vocabulary.md
  role: examined
  excerpt: '`SelectionRefusalCode::ALL` is the eight codes in normative check order,
    `as_str` yields the lock spelling for each, `from_code` resolves exactly those
    eight spellings and nothing else, and `PackageRefusalCode::as_str` is `invalid_package`.'
- id: FR-012-AC-4
  path: spec/exact/functional/FR-012-carried-compiler-vocabulary.md
  role: examined
  excerpt: Each of the twelve graph causes reachable through QSV's `DimensionNode::checked`,
    `UnitNode::checked` or `UnitGraph::from_checked_nodes` is raised by a malformed
    input; `DuplicateNode` is unrepresentable because the graph accepts key-unique
    `BTreeMap`s, rather than a list with repeated keys. `UndeclaredCase` arises by
    reading a case against its declaration; no graph input raises any of the four
    compiler-admission causes. `EnumDeclaration::new` raises `NonCanonicalPreimage`
    and `UnsortedUnorderedMembers` for the malformed member lists named above; `OwnerNotSelected`,
    `StaleKey`, `ForeignDeclara
- id: FR-012-AC-5
  path: spec/exact/functional/FR-012-carried-compiler-vocabulary.md
  role: examined
  excerpt: QSV's `Dimension` and `CompoundUnit` hold no zero exponent, iterate ascending
    by node key, and compare equal exactly when they denote the same unit, for every
    insertion order of the same admitted graph nodes; compound term input is ascending,
    and an unsorted term input is refused.
- id: FR-012-AC-6
  path: spec/exact/functional/FR-012-carried-compiler-vocabulary.md
  role: examined
  excerpt: At the IR-882 step-4 cutover, inspection finds no RT-local `NodeKey`, no
    RT `from_bytes` or `from_hex` constructor, and no TC-033 parser or round-trip
    test of those local constructors. RT library code has no `NodeKey` construction
    or hex decoder.
- id: FR-012-AC-7
  path: spec/exact/functional/FR-012-carried-compiler-vocabulary.md
  role: examined
  excerpt: After the R-3 cutover, inspection finds no RT-local `PackageCause`, `PackageRefusal`
    or `PackageRefusalCode` definition, type alias, public re-export or adapter. The
    RT consumers of selection and package refusals use QSV's definition types directly;
    `RevisionMismatch`, `DigestDomainMismatch` and `ByteDigestMismatch` are absent
    from RT's `invalid_package` surface, while FR-012-AC-3's eight selection spellings
    and `invalid_package` code remain available from QSV.
- id: FR-273-AC-1
  path: spec/exact/functional/FR-273-exact-function-application.md
  role: examined
  excerpt: '`CheckedPackage::call` and `CheckedPackage::evaluate` are called only
    against a package `PackageDeclarations::check` admitted under `CheckMode::Linked`;
    a package this crate''s `PackageDeclarations::check` rejects is never applied.'
- id: FR-273-AC-2
  path: spec/exact/functional/FR-273-exact-function-application.md
  role: examined
  excerpt: Declared arity is decided before any per-argument check, each argument's
    value kind and carried references are validated in parameter order, and all of
    that precedes the `function.call` charge, which itself precedes the function's
    body on every `call`.
- id: FR-273-AC-3
  path: spec/exact/functional/FR-273-exact-function-application.md
  role: examined
  excerpt: Arity, value-kind and dangling-reference input mismatches, and an unknown
    function name, each refuse as the matching `InputRefusal` variant before any charge;
    no `Meter` observes a refused call.
- id: FR-273-AC-4
  path: spec/exact/functional/FR-273-exact-function-application.md
  role: examined
  excerpt: A function whose declared operator requirements no registered backend can
    discharge negotiates `unsupported`, naming the required capability, before any
    application; the disposition is never an `Outcome` variant, never an `InputRefusal`,
    and takes no `Meter`.
- id: FR-273-AC-5
  path: spec/exact/functional/FR-273-exact-function-application.md
  role: examined
  excerpt: Outcome, refusal and charge sequence agree with the quire-spec-language
    authority on every shared-corpus function-application vector.
- id: FR-273-AC-6
  path: spec/exact/functional/FR-273-exact-function-application.md
  role: examined
  excerpt: '`CheckMode::Kernel` application is out of scope: no test in this requirement''s
    corpus applies a package checked only under `CheckMode::Kernel`.'
- id: FR-273-AC-7
  path: spec/exact/functional/FR-273-exact-function-application.md
  role: examined
  excerpt: A checked, decreasing function completes 4,096 runtime-managed calls on
    a 64 KiB native stack when `work_units` suffices; across depths 1, 128 and 4,096,
    no two generated-body transitions are simultaneously active, and inspection shows
    the scheduler returns to one loop before invoking the next transition. No depth-specific
    refusal or incomplete outcome exists.
- id: FR-273-AC-8
  path: spec/exact/functional/FR-273-exact-function-application.md
  role: examined
  excerpt: A foreign checked expression, an absent function requested within a checked
    body, and a re-entrant meter borrow carry `ForeignCheckedExpression`, `UnknownCheckedFunction`,
    and `MeterBorrowConflict` respectively, without changing their charge behavior
    or treating them as ordinary input refusals.
- id: FR-273-AC-9
  path: spec/exact/functional/FR-273-exact-function-application.md
  role: examined
  excerpt: For an admitted chain of `N` calls whose bodies only request the next call
    or return a literal, the expected charge vector is `function.call` repeated `N`
    times and the expected `work_units` spend is `N`. A limit of `N` completes; a
    limit of `N - 1` returns `Incomplete` at the Nth `function.call`, naming `work_units`,
    with exactly `N - 1` calls in both the counter and log, and without changing the
    checked package's static totality verdict.
- id: FR-273-AC-10
  path: spec/exact/functional/FR-273-exact-function-application.md
  role: examined
  excerpt: 'In a nested call with two argument applications and an enclosing application,
    the charge vector is exactly three `function.call` entries in left argument, right
    argument, enclosing call order; the enclosing charge precedes parameter binding
    and body work. With `work_units: 2`, the enclosing charge is denied, the log and
    counter retain exactly the first two calls, and no enclosing body work occurs.'
- id: FR-273-AC-11
  path: spec/exact/functional/FR-273-exact-function-application.md
  role: examined
  excerpt: A resumable body transition returns a child-call request while its owned
    parent state retains live locals and the child-result destination; after the child
    completes, the runtime resumes that parent with the child's value and does not
    invoke a second body transition from within the first. A stopped child propagates
    without resuming the parent.
- id: FR-275-AC-1
  path: spec/exact/functional/FR-275-single-exact-kernel.md
  role: examined
  excerpt: With `exact` enabled, no public item the runtime defines (in `scalar` or
    any other module) has the name of an item that `quire-exact` or `quire-semantic-value`
    exports.
- id: FR-275-AC-2
  path: spec/exact/functional/FR-275-single-exact-kernel.md
  role: examined
  excerpt: No module, alias or feature of the runtime keeps a removed kernel definition
    reachable; after the IR-882 value-bearing cutover, inspection finds no RT-defined
    second `Value`, `CheckedEquality` evaluator, pair walker or membership walker,
    `UnitId` resolver, `EnumShape` registry, `EffectiveId` registry, copy of `UnitTable`,
    `TypeEnvironment`, `EnumMemberIndex` or `EnumDeclaration`, conversion layer, adapter
    or old-`Value` re-export. Quantity, enum, reference and checked-equality paths
    use the QSV owners directly.
- id: FR-275-AC-3
  path: spec/exact/functional/FR-275-single-exact-kernel.md
  role: examined
  excerpt: '`quire-exact` is an optional dependency of the runtime, enabled only by
    the `exact` feature.'
- id: FR-275-AC-4
  path: spec/exact/functional/FR-275-single-exact-kernel.md
  role: examined
  excerpt: The optional `quire-exact` and `quire-semantic-value` dependencies use
    their own `agent-ix` Git repositories with `branch = "main"` and no `rev`, `tag`,
    `path` or committed `[patch]`.
- id: FR-275-AC-5
  path: spec/exact/functional/FR-275-single-exact-kernel.md
  role: examined
  excerpt: '`Cargo.lock` holds exactly one entry each for `quire-exact`, `quire-semantic-value`,
    `quire-canonical` and `quire-canonical-derive`.'
- id: FR-275-AC-6
  path: spec/exact/functional/FR-275-single-exact-kernel.md
  role: examined
  excerpt: '`make deny` exits non-zero when `Cargo.lock` holds a second entry for
    any first-party Git crate.'
- id: FR-275-AC-7
  path: spec/exact/functional/FR-275-single-exact-kernel.md
  role: examined
  excerpt: '`deny.toml` admits only the `quire-exact`, `quire-semantic-value` and
    `quire-canonical` own-repository Git sources and the dev-only `ix-trace-rs` first-party
    tooling source, and keeps `unknown-git = "deny"`; it does not admit the QSL Git
    repository.'
- id: FR-275-AC-8
  path: spec/exact/functional/FR-275-single-exact-kernel.md
  role: examined
  excerpt: '`make deny` reports cargo-deny''s `source-not-allowed` error for the QSL
    Git repository, and exits non-zero, when a normal, build or dev dependency resolves
    from it; a licence failure alone does not satisfy this.'
- id: FR-275-AC-9
  path: spec/exact/functional/FR-275-single-exact-kernel.md
  role: examined
  excerpt: With `exact` enabled and `std` disabled, the runtime builds for `thumbv7em-none-eabi`.
    The row `build-exact-no-std-msrv` builds it on the 1.98.1 floor.
- id: FR-275-AC-10
  path: spec/exact/functional/FR-275-single-exact-kernel.md
  role: examined
  excerpt: The dependency graph of `quire-contract-runtime-footprint` for `thumbv7em-none-eabi`
    contains no `quire-exact`, `quire-semantic-value`, `quire-canonical` or `quire-canonical-derive`.
- id: FR-275-AC-11
  path: spec/exact/functional/FR-275-single-exact-kernel.md
  role: examined
  excerpt: '`make size` measures linked `.text` plus `.rodata` inside NFR-001-AC-3''s
    500 byte to 4 KiB band.'
- id: FR-275-AC-12
  path: spec/exact/functional/FR-275-single-exact-kernel.md
  role: examined
  excerpt: The runtime has no test that compares its output with a second implementation
    of the kernel or that depends on a crate from the QSL Git repository.
- id: FR-275-AC-13
  path: spec/exact/functional/FR-275-single-exact-kernel.md
  role: examined
  excerpt: 'The runtime contains no file copied from the QSL repository and no port
    of QSL code, where a port is code that keeps the QSL authority''s item names and
    order. The residue list is not an allowance: every item on it is a violation of
    this criterion, which stays unmet while the list is non-empty. The runtime-owned
    negotiators are not ports.'
- id: FR-275-AC-14
  path: spec/exact/functional/FR-275-single-exact-kernel.md
  role: examined
  excerpt: The kernel move deletes no requirement, acceptance criterion or test case.
- id: FR-275-AC-15
  path: spec/exact/functional/FR-275-single-exact-kernel.md
  role: examined
  excerpt: Every matrix row whose evidence leaves the runtime in the kernel move carries
    a planned status with a stated reason, because the status vocabulary has no "verified
    upstream" status (a planner question, no status is invented).
- id: FR-275-AC-16
  path: spec/exact/functional/FR-275-single-exact-kernel.md
  role: examined
  excerpt: The runtime has no `quire_contract_runtime::exact` module and no `pub use`
    of `quire-exact`, `quire-semantic-value` or the evaluation leaf crate; the only
    items it defines under the `exact` feature are the `scalar` items (the backend
    negotiators and their types, `evaluate_boolean_short_circuit` and `ShortCircuitConnective`),
    and no file under `src/exact` exists.
- id: FR-275-AC-17
  path: spec/exact/functional/FR-275-single-exact-kernel.md
  role: examined
  excerpt: Every item the runtime defines under the `exact` feature, other than the
    `scalar` items, is named in the residue list, and the list records no exception,
    expiry or approval.
- id: FR-275-AC-18
  path: spec/exact/functional/FR-275-single-exact-kernel.md
  role: examined
  excerpt: The residue list is empty.
- id: FR-275-AC-19
  path: spec/exact/functional/FR-275-single-exact-kernel.md
  role: examined
  excerpt: The runtime defines `negotiate_integer_division`, `negotiate_ieee` and
    their types in its own source and depends on no QSL crate for them.
- id: FR-275-AC-20
  path: spec/exact/functional/FR-275-single-exact-kernel.md
  role: examined
  excerpt: The runtime's declared `rust-version` is 1.98.1.
- id: FR-275-AC-21
  path: spec/exact/functional/FR-275-single-exact-kernel.md
  role: examined
  excerpt: '`make msrv` and `make size` build with Rust 1.98.1.'
- id: FR-275-AC-22
  path: spec/exact/functional/FR-275-single-exact-kernel.md
  role: examined
  excerpt: Inspection of runtime source finds no local definition or alias of QSV's
    `SemanticGraphCause`, `InvalidSemanticGraph`, or semantic-node term checker, including
    definitions emitted by macros.
- id: FR-275-AC-23
  path: spec/exact/functional/FR-275-single-exact-kernel.md
  role: examined
  excerpt: 'After the step-4 value-bearing cutover, QSV''s `Conversion::identified_quantity`
    returns a `quire_exact::Quantity` with the exact magnitude and the target''s compound-arm
    `UnitId`, paired with QSV''s `IdentifiedUnit` for that same target; a non-exact
    conversion returns `HandoffRefusal::NotExact`, and target identity refusal returns
    `HandoffRefusal::Identity` with neither value nor identified unit. Retaining the
    identified unit with `UnitScope::form` permits `UnitScope::resolve(&quantity)`
    to supply the same normalized `QuantityUnit::Compound` to the next conversion.
    Declared and compound ID arms '
- id: FR-275-AC-24
  path: spec/exact/functional/FR-275-single-exact-kernel.md
  role: examined
  excerpt: The normal `make deny` recipe invokes the released `quire check-lock Cargo.lock`
    command on this repository's committed lock. On a coherent scratch lock the real
    command succeeds and the recipe completes when its other policy checks pass; an
    invocation record identifies the executable and argument without substituting
    checker logic.
- id: FR-275-AC-25
  path: spec/exact/functional/FR-275-single-exact-kernel.md
  role: examined
  excerpt: Through the same normal `make deny` recipe, a scratch lock with duplicate
    first-party Git package names and a scratch lock with distinct crate names at
    split resolved revisions of one repository each cause a non-zero Make exit and
    the real command's coherence diagnostic. The aligned multi-crate control passes
    the command. Every agent-ix Git source in these coherence controls has a valid
    full resolved fragment admitted by CLI FR-027; a malformed-input diagnostic cannot
    satisfy a duplicate or split-revision control. These controls exercise CLI FR-027;
    they do not define a second normalization al
- id: FR-275-AC-26
  path: spec/exact/functional/FR-275-single-exact-kernel.md
  role: examined
  excerpt: A missing checker executable, malformed scratch lock, or checker refusal
    cannot produce successful `make deny`. The recipe propagates failure without an
    ignored exit, success fallback, or skipped check.
- id: FR-275-AC-27
  path: spec/exact/functional/FR-275-single-exact-kernel.md
  role: examined
  excerpt: The committed `scripts/check_one_copy.awk` file is absent, the normal recipe
    contains no reference to it, and no replacement local implementation, executable
    copy, compatibility shim, or revision allowance bypasses the owned command.
- id: FR-275-AC-28
  path: spec/exact/functional/FR-275-single-exact-kernel.md
  role: examined
  excerpt: Adoption leaves the committed Cargo manifests and lock byte-for-byte unchanged
    and adds no runtime dependency on developer tooling. Qualification runs the real
    command on the committed lock and reports any existing incoherence as a failing
    gate with an owning blocker; a scratch passing control or an unrelated gate failure
    cannot qualify that lock.
- id: FR-277-AC-1
  path: spec/exact/functional/FR-277-checked-equality-evaluation.md
  role: examined
  excerpt: On the bounded depth-four `Sequence` input with equal `quire_exact::Value`
    operands, the QSV-owned `CheckedEquality::evaluate` path consumed by RT completes
    `true` and admits exactly 17 `EqualityPair` charges in occurrence-pair order after
    `EqualityPlanForm` and `EqualityPlan` and before `EqualityResultRetain`.
- id: FR-277-AC-2
  path: spec/exact/functional/FR-277-checked-equality-evaluation.md
  role: examined
  excerpt: On the same bounded shape with the first head unequal, the actual `CheckedEquality::evaluate`
    path completes `false` and still admits the complete 17-pair schedule and result-retain
    charge; equality does not short-circuit its occurrence-pair traversal or charge
    schedule.
- id: FR-277-AC-3
  path: spec/exact/functional/FR-277-checked-equality-evaluation.md
  role: examined
  excerpt: If a test meter denies the 17th `EqualityPair`, evaluation returns the
    existing typed `Incomplete` outcome at `EqualityPair`; only the preceding 16 pair
    charges are admitted, and `EqualityResultRetain` is not admitted. A denial of
    `EqualityResultRetain` occurs only after all 17 pair charges have been admitted.
- id: FR-277-AC-4
  path: spec/exact/functional/FR-277-checked-equality-evaluation.md
  role: examined
  excerpt: With equal-shaped operands whose declared (static) composite types differ,
    checked construction refuses with `IllTypedCause::TypeMismatch` before `CheckedEquality::evaluate`;
    the evaluator is not called and no equality pair charge is admitted.
- id: FR-277-AC-5
  path: spec/exact/functional/FR-277-checked-equality-evaluation.md
  role: examined
  excerpt: With `work_units = 50` and the bounded equal pair, the actual checked evaluation
    admits plan-form work of 32, then refuses `equality.plan` because its atomic reservation
    requires 19 remaining work units but only 18 remain. It returns the existing typed
    `Incomplete` at `WorkUnits` with limit 50, consumed 32 and `next_charge = 19`;
    no plan, pair, or result-retain charge is admitted and no equality result is returned.
- id: FR-277-AC-6
  path: spec/exact/functional/FR-277-checked-equality-evaluation.md
  role: examined
  excerpt: 'With `result_units = 0` and sufficient work and occurrence limits, the
    actual checked evaluation admits `equality.plan-form`, then refuses `equality.plan`
    atomically with typed `Incomplete { limit_kind: ResultUnits, limit: 0, consumed:
    0, next_charge: 1 }`. No plan, pair, or result-retain charge is admitted and no
    equality result is returned. A version that checks result capacity only after
    pair charges fails this control.'
- id: FR-277-AC-7
  path: spec/exact/functional/FR-277-checked-equality-evaluation.md
  role: examined
  excerpt: The IR-241 Kani claim remains `plan_equality(left, right).pair_events()
    == count_of_EqualityPair_charges_from_the_same_real_CheckedEquality::evaluate(left,
    right, meter)` for both equal and first-head-unequal bounded operands, with the
    existing exact operand bound and proof ceiling. A plan-only equality or duplicated
    QSL walker does not satisfy this criterion.
- id: FR-277-AC-8
  path: spec/exact/functional/FR-277-checked-equality-evaluation.md
  role: examined
  excerpt: Inspection of the locked QSV `CheckedEquality::evaluate` path shows a direct
    call to `quire_exact::planned_equality_with_plan` on the evaluated operands and
    use of that call's Boolean relation; the same call admits the complete 17-event
    pair schedule through its meter, and RT defines no local `CheckedEquality` or
    pair walker. A separate kernel call does not establish this criterion.
- id: FR-278-AC-1
  path: spec/exact/functional/FR-278-runtime-checked-invariant-causes.md
  role: examined
  excerpt: 'Every RT checked-package/function invariant producer returns its table
    cause: unknown name is `UnknownCheckedFunction`, a foreign expression is `ForeignCheckedExpression`,
    and each meter reborrow is `MeterBorrowConflict`. Runtime-managed calls use explicit
    frames and `work_units` fuel under RT FR-273; the final carrier has no `CallDepthExceeded`
    variant, and fuel exhaustion is `Incomplete` with `limit_kind: work_units`, never
    `CheckedInvariant`.'
- id: FR-278-AC-2
  path: spec/exact/functional/FR-278-runtime-checked-invariant-causes.md
  role: examined
  excerpt: Every collection or composite producer still owned by RT returns its assigned
    typed cause without flattening a nested cause to a message. Once those producers
    move to their owning crates, RT retains no duplicate producer and records the
    owner evidence. Checked equality and quantity producer controls belong to quire-semantic-value
    FR-122-AC-1 and AC-4; their six former `operand_value` assertions are not RT collection
    or composite evidence.
- id: FR-276-AC-1
  path: spec/monitor/functional/FR-276-consume-timed-monitor-plan.md
  role: examined
  excerpt: "With a 20-bit 1 MHz counter and zero uncertainty, request at tick 0 and\
    \ first acknowledgement at 2998, 3000, or 3001 yield respectively definite true,\
    \ indeterminate, and definite false for the 3 ms obligation; with 2 \xB5s uncertainty\
    \ the 2998 case remains indeterminate. No boundary case is decided from a point\
    \ estimate."
- id: FR-276-AC-2
  path: spec/monitor/functional/FR-276-consume-timed-monitor-plan.md
  role: examined
  excerpt: The QSL plan's 21-slot buffer for `once[0 ms, 10 ms] holds(p)` at 2 events/ms
    admits 21 in-window events; the 22nd yields `Incomplete(LimitReached)` naming
    2 events/ms, without dropping that event or changing an unrelated obligation.
    The runtime uses the capacity supplied by QSL rather than deriving a second formula.
- id: FR-276-AC-3
  path: spec/monitor/functional/FR-276-consume-timed-monitor-plan.md
  role: examined
  excerpt: 'Under a 16-bit counter with maximum gap 100, readings 65530 then 4 advance
    by 10; readings 65530 then 65500 settle `Failed(ReadingGapExceeded{gap: 65506,
    max_reading_gap: 100})` with no temporal verdict. A plan whose maximum gap is
    65536 is refused before construction.'
- id: FR-276-AC-4
  path: spec/monitor/functional/FR-276-consume-timed-monitor-plan.md
  role: examined
  excerpt: Changing the counter identity, unit, width, profile or admitted order never
    reuses the first plan's monitor state or settles its obligation; equal timestamps
    without admitted order never acquire causality from arrival order.
- id: FR-276-AC-5
  path: spec/monitor/functional/FR-276-consume-timed-monitor-plan.md
  role: examined
  excerpt: The step API carries distinct definite, pending, indeterminate, incomplete
    and failed outcomes with the affected obligation identities; a limit or fault
    on one obligation does not turn a sibling's state into a verdict or erase its
    identity.
- id: FR-276-AC-6
  path: spec/monitor/functional/FR-276-consume-timed-monitor-plan.md
  role: examined
  excerpt: The complete admitted plan alone builds a monitor after driver preparation
    objects are dropped. Its checked subject, activation/captures, atom bodies, ordered
    topology, typed state initializers and executable event/watermark entries remain
    available. Build consumes no event and emits no verdict; a second build has independent
    state, and neither monitor can be rebound to another plan. No source parse, second
    clause, callback or RT operator translation completes it.
- id: FR-276-AC-7
  path: spec/monitor/functional/FR-276-consume-timed-monitor-plan.md
  role: examined
  excerpt: For each of a foreign atom/operand, missing transition body, changed initializer,
    interval/buffer owner, activation binding, uncertainty and maximum gap under the
    original selection, driver/IR admission refuses without RT initialization; equal
    numeric subformula IDs under different subjects do not alias. An unsupported executable
    operation yields `Unsupported`, and RT allocation or initialization failure returns
    typed build failure with no monitor.
- id: FR-276-AC-8
  path: spec/monitor/functional/FR-276-consume-timed-monitor-plan.md
  role: examined
  excerpt: Invalid event/progress inputs, including a missing required admitted value,
    leave monitor state unchanged with no verdict. An admitted progress assertion
    can settle a deadline, while a mere counter poll cannot. Executing the admitted
    program agrees with QSL's reference evaluator on boundary, reversed-operand, false-activation
    and open/closed-endpoint cases; an RT executor invariant failure returns `Failed`,
    terminates the monitor and rejects later input.
- id: FR-003-AC-1
  path: spec/proptest_adapter/functional/FR-003-proptest-adapter.md
  role: examined
  excerpt: Each verdict maps to the corresponding proptest outcome.
- id: FR-003-AC-2
  path: spec/proptest_adapter/functional/FR-003-proptest-adapter.md
  role: examined
  excerpt: The adapter is absent unless the opt-in feature is selected, as the feature-off
    compiler probe and `compile_fail` doctest show.
- id: FR-003-AC-3
  path: spec/proptest_adapter/functional/FR-003-proptest-adapter.md
  role: examined
  excerpt: '`adapt` records nothing; `adapt_recording` records a matching verdict
    exactly once and maps it as `adapt` does; a mismatched identity yields a proptest
    failure naming both identities and leaves the report''s counters unchanged.'
bindings: []
```

## Dispositions

Round1, Full gap remains FAIL; all three inherited findings explicitly deferred to their existing owners without exception or a green claim.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | deferred | Inherited 67-unbound-criterion/107-strict-row debt remains in IR-878 (live In Progress), with actual prior owner handoffs fad2588d and6a87b61b. User/root narrow Make/CLI adoption scope leaves this debt outside PR153; no new full matrix measurement or green coverage claim. |
| FND-002 | deferred | Three unchanged IR-512 ticket-only tags remain coverage-lane triage in IR-878, alongside the inherited runtime trace debt. No test or binding is weakened by this PR; this disposition does not assert that the tags are correct. |
| FND-003 | deferred | Inherited forbidden exact residue remains required removal under live IR-349 (In Progress), with usable evaluator dependency IR-590 (Backlog). User/root narrow Make/CLI adoption scope does not authorize changing that API or grant a copy/compatibility exception; the finding remains real and repository gap FAIL. |

## Narrow Verification (round 1)

Exact published candidate c8609adbba0ddfa000d11f310fe1be7d88153bd8; original same reviewer/run ae2ff59d-21b1-4119-a583-3a8435154c50. Read-only git/Linear/GitHub and preserved raw logs only; no new Cargo/native/Kani/full gate, lock change, rebase, branch write or merge. All six original typed/raw review files in candidate are byte-exact to the original reviewer custody. Relative to test-first75ed338b, production change is only the Make recipe and Rust change is regression-test formatting. The seventh real process control had actual RED101 on75ed338b (Git-quoted café path, verification retained) and all7 controls PASS0 after the fix using actual ownerCLI2005; no ignored tests or checker substitute. Four filename families are space, Unicode, newline and glob characters; the real incoherent lock is placed at each path independently. Existing duplicate/split/malformed/missing-checker cases remain. Raw fmt/lint exit files are0. The author reports16GiB/swap0/jobs2 scopes; focused log names its actual scope, but new-scope memory peak/caps are not independently remeasured by this source-only disposition. Root owns caps/final-gate verification. RT154 and RT155 are still OPEN at readback; RT153 final rebase/aggregate/acceptance/merge remain HELD pending RT154 thenRT155 ordering. No final mergeable claim is made against unrebased main.

## Narrow Verification (round 2)

Full-gap FAIL retained. All three latest outcomes remain deferred from round1: FND001/002 to IR878 coverage and inherited IR512 tags; FND003 to IR349/IR590 required residue removal. These are inherited, unchanged findings; no compatibility/copy exception, fixed assertion or fresh matrix/strict-green claim is made. No new findings or disposition rows are required this round.

Original reviewer/run ae2ff59d-21b1-4119-a583-3a8435154c50, published exact head 0c37ad083f2254b0588942920ad7ee194785e0c2, RT PR153 still OPEN. Relative c8609adb production delta only changes Makefile enumeration/completion handling; test delta adds one external Git-boundary fault control and passes PATH explicitly, with the genuine owner checker unchanged. All six prior round1 custody files match reviewer source bytes. No Cargo manifest/lock, runtime/kernel source or CI change. Reviewer ran only source/readback/Quire operations, no new Cargo/native/full gate/Kani/lock/rebase/merge. Raw producer-red.log independently shows the old recipe returned Make success after valid root Cargo.lock NUL prefix plus Git exit42, causing the new test RED101. The actual fixed recipe is exercised by all8controls PASS0, no ignored tests; producer-fmt and producer-clippy exit0. OwnerCLI2005 is the same real installed CLI; the only test double is the external Git producer, not the checker or lock algorithm. The test first demonstrates actual refusal of the incoherent tracked sibling without injection, then injects prefix/failure and requires nonzero Make, producer diagnostic, and no checker success report. Previous filename spaces/Unicode/newlines/globs and all real aligned/duplicate/split/malformed/missing-executable controls remain. Author-reported16GiB/swap0/jobs2 scope settings are not independently remeasured here; raw logs show actual scope invocation IDs and root owns cap/final-gate verification. RT154 remains OPEN at this readback; RT154 merge SHA and required lane ordering, followed by final rebase/gates/review, still hold RT153. No final mergeable claim is made on unrebased main.
