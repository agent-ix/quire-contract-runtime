//! FR-006 typed exact outcomes and `quire.value.accounting/v1` metering, through
//! the public `exact` surface.
#![cfg(feature = "exact")]

use std::collections::BTreeSet;
use std::fs;
use std::path::Path;

use quire_contract_runtime::exact::{
    admit_text, compare_enum, compare_text, divide, evaluate_boolean, evaluate_decimal,
    evaluate_ieee, evaluate_integer_arithmetic, evaluate_quantity, evaluate_rational_arithmetic,
    modulo, order_numbers, BooleanConnective, ChargePoint, ComparisonOperator, Decimal,
    DecimalOperation, DecimalType, DivisionProfile, EnumDeclaration, IeeeFlags, IeeeOperation,
    IeeeValue, IllTyped, IllTypedCause, Incomplete, InjectedDenial, Integer, IntegerArithmetic,
    IntegerDomain, IntegerInterval, LimitKind, Meter, NodeKey, OrderedOperands, OrderingOperator,
    Outcome, Quantity, QuantityOperation, QuantityUnit, Rational, RationalArithmetic, Refusal,
    RoundingMode, ScalarLimits, TextPayload, TextProfile, TextType, UnitDeclaration, UnitGraph,
};

const UNLIMITED: ScalarLimits = limits([u64::MAX; 10]);

const fn limits(v: [u64; 10]) -> ScalarLimits {
    ScalarLimits {
        integer_bits: v[0],
        decimal_digits: v[1],
        scale_expansion: v[2],
        text_input_bytes: v[3],
        text_scalars: v[4],
        normalized_scalars: v[5],
        unit_edges: v[6],
        value_occurrences: v[7],
        work_units: v[8],
        result_units: v[9],
    }
}

fn consumed(meter: &Meter) -> Vec<u64> {
    LimitKind::ALL
        .iter()
        .map(|kind| meter.consumed(*kind))
        .collect()
}

fn int(value: i128) -> Integer {
    Integer::from(value)
}

/// Trace: TC-016, FR-006-AC-1
#[test]
fn tc_016_outcomes_are_four_distinct_dispositions_and_false_is_a_value() {
    let incomplete = Incomplete {
        limit_kind: LimitKind::WorkUnits,
        limit: 0,
        consumed: 0,
        next_charge: int(1),
        charge_point: ChargePoint::BooleanResultRetain,
    };
    let outcomes: [Outcome<bool>; 5] = [
        Outcome::Completed(false),
        Outcome::Completed(true),
        Outcome::Undefined(quire_contract_runtime::exact::Undefined::DivisionByZero),
        Outcome::Refused(Refusal::IntegerOutOfDomain),
        Outcome::Incomplete(incomplete.clone()),
    ];
    let completed: Vec<Option<bool>> = outcomes.iter().cloned().map(Outcome::completed).collect();
    assert_eq!(completed, [Some(false), Some(true), None, None, None]);
    for (i, left) in outcomes.iter().enumerate() {
        for (j, right) in outcomes.iter().enumerate() {
            assert_eq!(i == j, left == right);
        }
    }

    // A false result is completed, never a refusal.
    assert_eq!(
        evaluate_boolean(BooleanConnective::Not(true), &mut Meter::new(UNLIMITED)),
        Outcome::Completed(false)
    );

    // Ill-typed precedes evaluation and is returned beside the outcome.
    let admit = |text: &str, profile| {
        admit_text(
            &TextPayload::from_utf8(text.as_bytes()).unwrap(),
            &TextType::new(0, 8, profile).unwrap(),
            &mut Meter::new(UNLIMITED),
        )
        .completed()
        .unwrap()
    };
    let mut meter = Meter::new(limits([0; 10]));
    let ill = compare_text(
        ComparisonOperator::Equal,
        &admit("a", TextProfile::Nfc),
        &admit("a", TextProfile::BinaryUtf8),
        &mut meter,
    );
    assert_eq!(
        ill,
        Err(IllTyped {
            cause: IllTypedCause::DistinctTextProfiles
        })
    );
    assert_eq!(IllTyped::CODE, "ill_typed");
    assert!(meter.admitted_charges().is_empty());
}

/// Trace: TC-016, FR-006-AC-1
#[test]
fn tc_016_refusal_codes_are_closed() {
    let codes: Vec<Option<&str>> = [
        Refusal::InexactDecimal,
        Refusal::DecimalOutOfDomain,
        Refusal::DivisionPairOutOfDomain {
            quotient_admitted: true,
            remainder_admitted: false,
        },
        Refusal::ModuloOutOfDomain,
        Refusal::TextLengthOutOfDomain,
        Refusal::IntegerOutOfDomain,
        Refusal::IeeeNanPayloadNotRepresentable,
        Refusal::IeeeRationalOutOfDomain,
        Refusal::RationalOutOfDomain,
    ]
    .into_iter()
    .map(Refusal::code)
    .collect();
    assert_eq!(
        codes,
        [
            None,
            None,
            None,
            None,
            None,
            None,
            Some("ieee_nan_payload_not_representable"),
            Some("ieee_rational_out_of_domain"),
            None,
        ]
    );
}

/// Trace: TC-016, FR-006-AC-3
#[test]
fn tc_016_charge_point_and_limit_vocabularies_round_trip() {
    let spellings: Vec<&str> = ChargePoint::ALL
        .iter()
        .map(|point| point.as_str())
        .collect();
    assert_eq!(
        spellings.iter().collect::<BTreeSet<_>>().len(),
        ChargePoint::ALL.len()
    );
    for point in ChargePoint::ALL {
        assert_eq!(ChargePoint::from_code(point.as_str()), Some(point));
    }
    assert_eq!(ChargePoint::from_code("equality.not-a-real-point"), None);
    assert_eq!(ChargePoint::from_code("ordering"), None);
    // The QSpec 7d7943a scalar families, in definition-row order.
    assert_eq!(
        spellings[spellings.len() - 11..],
        [
            "integer-arithmetic.operands",
            "integer-arithmetic.arithmetic",
            "integer-arithmetic.result-retain",
            "rational-arithmetic.operands",
            "rational-arithmetic.arithmetic",
            "rational-arithmetic.normalize",
            "rational-arithmetic.result-retain",
            "ordering.operands",
            "ordering.arithmetic",
            "ordering.result-retain",
            "boolean.result-retain",
        ]
    );
    let fields: Vec<&str> = LimitKind::ALL.iter().map(|kind| kind.as_str()).collect();
    assert_eq!(
        fields,
        [
            "integer_bits",
            "decimal_digits",
            "scale_expansion",
            "text_input_bytes",
            "text_scalars",
            "normalized_scalars",
            "unit_edges",
            "value_occurrences",
            "work_units",
            "result_units",
        ]
    );
    let cumulative: Vec<bool> = LimitKind::ALL
        .iter()
        .map(|kind| kind.is_cumulative())
        .collect();
    assert_eq!(
        cumulative,
        [false, false, false, false, false, false, false, false, true, true]
    );
}

/// Trace: TC-017, FR-006-AC-3
#[test]
fn tc_017_charges_precede_work_and_a_denied_charge_consumes_nothing() {
    let two_64 = Integer::from(1_i128 << 64);
    let mut tuple = [u64::MAX; 10];
    tuple[0] = 128;
    let mut meter = Meter::new(limits(tuple));
    let outcome = evaluate_integer_arithmetic(
        IntegerArithmetic::Multiply(&two_64, &two_64),
        None,
        &mut meter,
    );
    assert_eq!(
        outcome,
        Outcome::Incomplete(Incomplete {
            limit_kind: LimitKind::IntegerBits,
            limit: 128,
            consumed: 65,
            // `bits(2^64) + bits(2^64) = 130`, never the square's 129.
            next_charge: int(130),
            charge_point: ChargePoint::IntegerArithmeticArithmetic,
        })
    );
    assert_eq!(
        meter.admitted_charges(),
        [ChargePoint::IntegerArithmeticOperands]
    );
    let mut expected = vec![65, 0, 0, 0, 0, 0, 0, 2, 1, 0];
    assert_eq!(consumed(&meter), expected);

    // Size counters are high-water marks; work and result units accumulate.
    let (one, three) = (int(1), int(3));
    let run = evaluate_integer_arithmetic(IntegerArithmetic::Add(&one, &three), None, &mut meter);
    assert_eq!(run, Outcome::Completed(int(4)));
    expected[8] = 4;
    expected[9] = 1;
    assert_eq!(consumed(&meter), expected);
}

/// Trace: TC-017, FR-006-AC-3
#[test]
fn tc_017_first_short_counter_in_field_order_and_domain_refusal_before_retention() {
    // Both integer_bits and work_units are short: integer_bits is reported.
    let (a, b) = (int(255), int(1));
    let mut short = [u64::MAX; 10];
    short[0] = 7;
    short[8] = 0;
    let outcome = evaluate_integer_arithmetic(
        IntegerArithmetic::Add(&a, &b),
        None,
        &mut Meter::new(limits(short)),
    );
    assert_eq!(
        outcome,
        Outcome::Incomplete(Incomplete {
            limit_kind: LimitKind::IntegerBits,
            limit: 7,
            consumed: 0,
            next_charge: int(8),
            charge_point: ChargePoint::IntegerArithmeticOperands,
        })
    );

    let bound = IntegerInterval::new(int(0), int(10)).unwrap();
    let mut meter = Meter::new(UNLIMITED);
    let refused = evaluate_integer_arithmetic(
        IntegerArithmetic::Add(&int(7), &int(5)),
        Some(&bound),
        &mut meter,
    );
    assert_eq!(refused, Outcome::Refused(Refusal::IntegerOutOfDomain));
    assert_eq!(
        meter.admitted_charges(),
        [
            ChargePoint::IntegerArithmeticOperands,
            ChargePoint::IntegerArithmeticArithmetic
        ]
    );
    assert_eq!(meter.consumed(LimitKind::ResultUnits), 0);
}

/// Trace: TC-017, FR-006-AC-4
#[test]
fn tc_017_injected_denial_names_the_point_and_leaves_counters_unchanged() {
    let points = [
        ChargePoint::IntegerArithmeticOperands,
        ChargePoint::IntegerArithmeticArithmetic,
        ChargePoint::IntegerArithmeticResultRetain,
    ];
    for (work, point) in (0_u64..).zip(points) {
        let mut meter = Meter::new(UNLIMITED).with_injected_denial(InjectedDenial {
            point,
            occurrence: 1,
        });
        let outcome =
            evaluate_integer_arithmetic(IntegerArithmetic::Negate(&int(-9)), None, &mut meter);
        assert_eq!(
            outcome,
            Outcome::Incomplete(Incomplete {
                limit_kind: LimitKind::WorkUnits,
                limit: work,
                consumed: work,
                next_charge: int(1),
                charge_point: point,
            })
        );
        assert_eq!(meter.consumed(LimitKind::WorkUnits), work);
        assert_eq!(meter.consumed(LimitKind::ResultUnits), 0);
        assert_eq!(
            meter.admitted_charges().len(),
            usize::try_from(work).unwrap()
        );
    }

    // A second occurrence is denied only on its second charge.
    let mut meter = Meter::new(UNLIMITED).with_injected_denial(InjectedDenial {
        point: ChargePoint::BooleanResultRetain,
        occurrence: 2,
    });
    assert_eq!(
        evaluate_boolean(BooleanConnective::Not(false), &mut meter),
        Outcome::Completed(true)
    );
    assert!(matches!(
        evaluate_boolean(BooleanConnective::Not(false), &mut meter),
        Outcome::Incomplete(ref record) if record.limit == 1
    ));
}

/// Trace: TC-016, FR-006-AC-5
#[test]
fn tc_016_exact_sources_have_no_host_float_std_panic_or_unsafe_path() {
    let root = include_str!("../src/lib.rs");
    for attribute in [
        "#![no_std]",
        "#![forbid(unsafe_code)]",
        "#![deny(clippy::arithmetic_side_effects, clippy::indexing_slicing)]",
    ] {
        assert!(root.contains(attribute), "{attribute}");
    }
    assert!(root.contains("#[cfg(feature = \"exact\")]\npub mod exact;"));

    let directory = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/exact");
    let mut sources = Vec::new();
    for entry in fs::read_dir(&directory).unwrap() {
        let path = entry.unwrap().path();
        sources.push((
            path.file_name().unwrap().to_string_lossy().into_owned(),
            fs::read_to_string(&path).unwrap(),
        ));
    }
    sources.sort();
    assert_eq!(sources.len(), 22);
    let forbidden = [
        "f32",
        "f64",
        "std::",
        "panic!",
        "unreachable!",
        "todo!",
        "unimplemented!",
        ".unwrap(",
        ".expect(",
        "unsafe",
        "assert!",
        "SystemTime",
        "Instant",
        "thread_rng",
        "getrandom",
    ];
    for (name, source) in &sources {
        let code: String = source
            .lines()
            .filter(|line| !line.trim_start().starts_with("//"))
            .collect::<Vec<_>>()
            .join("\n");
        for token in forbidden {
            let hit = code.match_indices(token).any(|(at, _)| {
                let before = code[..at].chars().next_back();
                let after = code[at + token.len()..].chars().next();
                let word = |c: Option<char>| c.is_some_and(|c| c.is_alphanumeric() || c == '_');
                let starts_with_word = token.chars().next().is_some_and(char::is_alphanumeric);
                let ends_with_word = token.chars().last().is_some_and(char::is_alphanumeric);
                let joined_before = starts_with_word && word(before);
                let joined_after = ends_with_word && word(after);
                !joined_before && !joined_after
            });
            assert!(!hit, "{name} contains {token}");
        }
    }
}

/// Trace: TC-016, FR-006-AC-5
#[test]
fn tc_016_exact_surface_is_reexported_from_private_modules() {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/exact");
    let module = fs::read_to_string(directory.join("mod.rs")).unwrap();
    let mut declared = BTreeSet::new();
    for line in module.lines() {
        assert!(
            !line.starts_with("pub mod"),
            "exact submodules are private: {line}"
        );
        if let Some(name) = line
            .strip_prefix("mod ")
            .and_then(|rest| rest.strip_suffix(';'))
        {
            declared.insert(name.to_owned());
        }
    }
    let exported: BTreeSet<String> = module
        .split("pub use ")
        .skip(1)
        .flat_map(|item| {
            let item = item.split(';').next().unwrap();
            let (module, names) = item.split_once("::").unwrap();
            names
                .trim_matches(|c| c == '{' || c == '}')
                .split(',')
                .map(str::trim)
                .filter(|name| !name.is_empty())
                .map(|name| format!("{module}::{name}"))
                .collect::<Vec<_>>()
        })
        .collect();
    let mut public = BTreeSet::new();
    for name in &declared {
        let source = fs::read_to_string(directory.join(format!("{name}.rs"))).unwrap();
        for line in source.lines() {
            for kind in [
                "pub struct ",
                "pub enum ",
                "pub fn ",
                "pub const ",
                "pub trait ",
                "pub type ",
            ] {
                if let Some(rest) = line.strip_prefix(kind) {
                    let item: String = rest
                        .chars()
                        .take_while(|c| c.is_alphanumeric() || *c == '_')
                        .collect();
                    public.insert(format!("{name}::{item}"));
                }
            }
        }
    }
    assert_eq!(declared.len(), 21);
    assert_eq!(public, exported);
}

// ---- TC-031 / FR-010: the injected-charge-denial seam ---------------------

fn limits_with_work(work_units: u64) -> ScalarLimits {
    let mut tuple = [u64::MAX; 10];
    tuple[8] = work_units;
    limits(tuple)
}

/// Unwrap an [`Outcome`] known to be [`Outcome::Incomplete`]; panics with the
/// disposition otherwise. `T` need not be `Debug`: only the non-`Incomplete`
/// dispositions, which are `Debug` regardless of `T`, are ever formatted.
fn expect_incomplete<T>(outcome: Outcome<T>) -> Incomplete {
    match outcome {
        Outcome::Incomplete(record) => record,
        Outcome::Completed(_) => panic!("expected Outcome::Incomplete, got Completed"),
        Outcome::Undefined(reason) => panic!("expected Outcome::Incomplete, got {reason:?}"),
        Outcome::Refused(reason) => panic!("expected Outcome::Incomplete, got {reason:?}"),
    }
}

// One driver per admitted charge point family, run under `UNLIMITED` so every
// charge before the point under test is genuinely admitted. Each returns the
// `Incomplete` an injected denial at that call's first matching point
// produces.

fn drive_integer_arithmetic(meter: &mut Meter) -> Incomplete {
    expect_incomplete(evaluate_integer(
        IntegerOperation::Negate(&int(-9)),
        &IntegerDomain::Mathematical,
        meter,
    ))
}

fn drive_rational_arithmetic(meter: &mut Meter) -> Incomplete {
    let operand = Rational::from_integer(int(5));
    expect_incomplete(evaluate_rational(
        RationalOperation::Negate(&operand),
        None,
        meter,
    ))
}

fn drive_ordering(meter: &mut Meter) -> Incomplete {
    expect_incomplete(evaluate_ordering(
        OrderingOperator::Less,
        OrderingOperands::Integer(&int(1), &int(2)),
        meter,
    ))
}

fn drive_boolean(meter: &mut Meter) -> Incomplete {
    expect_incomplete(evaluate_not(false, meter))
}

fn drive_decimal_basic(meter: &mut Meter) -> Incomplete {
    let target = DecimalType::new(int(-1000), int(1000), 0, 10, RoundingMode::Exact).unwrap();
    expect_incomplete(evaluate_decimal(
        DecimalOperation::Negate(&Decimal::new(int(5), 0)),
        &target,
        meter,
    ))
}

/// A `Divide` whose quotient (`1/3`) is not exactly representable, so it
/// reaches `decimal.rounding` before `decimal.result-retain`.
fn drive_decimal_rounding(meter: &mut Meter) -> Incomplete {
    let target = DecimalType::new(int(-1000), int(1000), 0, 2, RoundingMode::TowardZero).unwrap();
    let (one, three) = (Decimal::new(int(1), 0), Decimal::new(int(3), 0));
    expect_incomplete(evaluate_decimal(
        DecimalOperation::Divide(&one, &three),
        &target,
        meter,
    ))
}

/// `binary-utf8` never normalizes, so this reaches `text.result-retain`
/// without any `text.normalize-*` charge.
fn drive_text_basic(meter: &mut Meter) -> Incomplete {
    let payload = TextPayload::from_utf8(b"ab").unwrap();
    let text_type = TextType::new(0, 10, TextProfile::BinaryUtf8).unwrap();
    expect_incomplete(admit_text(&payload, &text_type, meter))
}

/// `nfc` emits one normalized scalar, charging both `text.normalize-input`
/// and `text.normalize-output`.
fn drive_text_normalize(meter: &mut Meter) -> Incomplete {
    let payload = TextPayload::from_utf8(b"a").unwrap();
    let text_type = TextType::new(0, 10, TextProfile::Nfc).unwrap();
    expect_incomplete(admit_text(&payload, &text_type, meter))
}

fn drive_enum(meter: &mut Meter) -> Incomplete {
    let declaration =
        EnumDeclaration::new(NodeKey::from_bytes([9; 32]), true, &["a", "b"]).unwrap();
    let a = declaration
        .member("a", NodeKey::from_bytes([10; 32]))
        .unwrap();
    let b = declaration
        .member("b", NodeKey::from_bytes([11; 32]))
        .unwrap();
    expect_incomplete(compare_enum(ComparisonOperator::Equal, &a, &b, meter).unwrap())
}

/// `a ^ 2` of a quantity in a non-canonical (one-edge) unit: charges
/// `unit.identity-read`, `unit.edge`, `unit.rational-arithmetic` (from the
/// edge traversal and the power itself), `unit.target-domain` and
/// `unit.result-retain`, in that order.
fn drive_quantity(meter: &mut Meter) -> Incomplete {
    let dimension = NodeKey::from_bytes([1; 32]);
    let root = NodeKey::from_bytes([2; 32]);
    let derived = NodeKey::from_bytes([3; 32]);
    let graph = UnitGraph::admit(
        [(dimension, Vec::new())],
        [
            (
                root,
                UnitDeclaration {
                    dimension,
                    target: None,
                    scale: Rational::from_integer(int(1)),
                    offset: Rational::from_integer(int(0)),
                },
            ),
            (
                derived,
                UnitDeclaration {
                    dimension,
                    target: Some(root),
                    scale: Rational::new(int(1000), int(1)).unwrap(),
                    offset: Rational::from_integer(int(0)),
                },
            ),
        ],
    )
    .unwrap();
    let unit = graph.unit(derived).unwrap().clone();
    let quantity = Quantity::new(
        Rational::from_integer(int(5)),
        QuantityUnit::Declared(Box::new(unit)),
    );
    expect_incomplete(
        evaluate_quantity(QuantityOperation::Power(&quantity, &int(2)), meter).unwrap(),
    )
}

fn drive_integer_division(meter: &mut Meter) -> Incomplete {
    expect_incomplete(divide(
        DivisionProfile::Truncating,
        &int(7),
        &int(2),
        &IntegerDomain::Mathematical,
        meter,
    ))
}

fn drive_integer_modulus(meter: &mut Meter) -> Incomplete {
    expect_incomplete(modulo(
        &int(7),
        &int(2),
        &IntegerDomain::Mathematical,
        meter,
    ))
}

fn drive_ieee(meter: &mut Meter) -> Incomplete {
    let operand = IeeeValue::binary32(0x3F80_0000);
    expect_incomplete(
        evaluate_ieee(
            IeeeOperation::Add(operand, operand),
            RoundingMode::NearestEven,
            meter,
        )
        .unwrap(),
    )
}

/// One `(point, driver)` entry of the AC-1 sweep below.
type Driver = fn(&mut Meter) -> Incomplete;

/// Trace: TC-031, FR-010-AC-1
///
/// `equality.plan`, `equality.pair` and `equality.result-retain` are declared
/// in `ChargePoint::ALL` (quire.value.accounting/v1) but no public `exact`
/// operator charges them: no comparison function in this crate emits an
/// `Equality*` charge point (`compare_text`, `compare_enum` and
/// `compare_quantity` all retain through their own family's
/// `*.result-retain`). They are therefore excluded from this AC-1 sweep as
/// `unreachable`, not silently dropped: this is a gap between the declared
/// `ChargePoint` vocabulary and what the runtime actually charges, not a gap
/// in this test.
#[test]
fn tc_031_injected_denial_at_occurrence_one_names_every_admitted_charge_point() {
    let unreachable = [
        ChargePoint::EqualityPlan,
        ChargePoint::EqualityPair,
        ChargePoint::EqualityResultRetain,
    ];
    let drivers: Vec<(ChargePoint, Driver)> = vec![
        (
            ChargePoint::IntegerArithmeticOperands,
            drive_integer_arithmetic,
        ),
        (
            ChargePoint::IntegerArithmeticArithmetic,
            drive_integer_arithmetic,
        ),
        (
            ChargePoint::IntegerArithmeticResultRetain,
            drive_integer_arithmetic,
        ),
        (
            ChargePoint::RationalArithmeticOperands,
            drive_rational_arithmetic,
        ),
        (
            ChargePoint::RationalArithmeticArithmetic,
            drive_rational_arithmetic,
        ),
        (
            ChargePoint::RationalArithmeticNormalize,
            drive_rational_arithmetic,
        ),
        (
            ChargePoint::RationalArithmeticResultRetain,
            drive_rational_arithmetic,
        ),
        (ChargePoint::OrderingOperands, drive_ordering),
        (ChargePoint::OrderingArithmetic, drive_ordering),
        (ChargePoint::OrderingResultRetain, drive_ordering),
        (ChargePoint::BooleanResultRetain, drive_boolean),
        (ChargePoint::DecimalOperands, drive_decimal_basic),
        (ChargePoint::DecimalScaleExpansion, drive_decimal_basic),
        (ChargePoint::DecimalArithmetic, drive_decimal_basic),
        (ChargePoint::DecimalRounding, drive_decimal_rounding),
        (ChargePoint::DecimalResultRetain, drive_decimal_basic),
        (ChargePoint::TextInputBytes, drive_text_basic),
        (ChargePoint::TextDecodeScalars, drive_text_basic),
        (ChargePoint::TextNormalizeInput, drive_text_normalize),
        (ChargePoint::TextNormalizeOutput, drive_text_normalize),
        (ChargePoint::TextResultRetain, drive_text_basic),
        (ChargePoint::EnumIdentityRead, drive_enum),
        (ChargePoint::EnumResultRetain, drive_enum),
        (ChargePoint::UnitIdentityRead, drive_quantity),
        (ChargePoint::UnitEdge, drive_quantity),
        (ChargePoint::UnitRationalArithmetic, drive_quantity),
        (ChargePoint::UnitTargetDomain, drive_quantity),
        (ChargePoint::UnitResultRetain, drive_quantity),
        (ChargePoint::IntegerDivisionOperands, drive_integer_division),
        (
            ChargePoint::IntegerDivisionArithmetic,
            drive_integer_division,
        ),
        (
            ChargePoint::IntegerDivisionDomainPair,
            drive_integer_division,
        ),
        (
            ChargePoint::IntegerDivisionResultPair,
            drive_integer_division,
        ),
        (ChargePoint::IntegerModulusOperands, drive_integer_modulus),
        (ChargePoint::IntegerModulusArithmetic, drive_integer_modulus),
        (ChargePoint::IntegerModulusDomain, drive_integer_modulus),
        (
            ChargePoint::IntegerModulusResultRetain,
            drive_integer_modulus,
        ),
        (ChargePoint::IeeeOperands, drive_ieee),
        (ChargePoint::IeeeExactIntermediate, drive_ieee),
        (ChargePoint::IeeeRound, drive_ieee),
        (ChargePoint::IeeeResultRetain, drive_ieee),
    ];

    // Every declared charge point is either driven here or named as
    // known-unreachable, with no duplicates and no omissions.
    let covered: BTreeSet<ChargePoint> = drivers.iter().map(|(point, _)| *point).collect();
    assert_eq!(
        covered.len(),
        drivers.len(),
        "a charge point is driven twice"
    );
    assert_eq!(covered.len() + unreachable.len(), ChargePoint::ALL.len());
    for point in ChargePoint::ALL {
        assert!(
            covered.contains(&point) || unreachable.contains(&point),
            "{point:?} is neither driven nor declared unreachable"
        );
    }

    for (point, drive) in drivers {
        let mut meter = Meter::new(UNLIMITED).with_injected_denial(InjectedDenial {
            point,
            occurrence: 1,
        });
        let record = drive(&mut meter);
        assert_eq!(record.limit_kind, LimitKind::WorkUnits, "{point:?}");
        assert_eq!(record.charge_point, point);
        assert_eq!(record.limit, record.consumed, "{point:?}");
        assert_eq!(record.next_charge, int(1), "{point:?}");
        // The denied charge itself changed nothing: `work_units` after the
        // call is exactly the charges admitted before it (one work unit
        // each), no result unit was retained, and the point was not logged.
        assert_eq!(
            meter.consumed(LimitKind::WorkUnits),
            record.consumed,
            "{point:?}"
        );
        assert_eq!(meter.consumed(LimitKind::ResultUnits), 0, "{point:?}");
        assert_eq!(
            meter.admitted_charges().len(),
            usize::try_from(record.consumed).unwrap(),
            "{point:?}"
        );
        assert!(!meter.admitted_charges().contains(&point), "{point:?}");
    }
}

/// Trace: TC-031, FR-010-AC-2
#[test]
fn tc_031_injected_record_is_independent_of_the_configured_work_units_limit() {
    let point = ChargePoint::IntegerArithmeticResultRetain;
    let denial = InjectedDenial {
        point,
        occurrence: 1,
    };
    let mut low = Meter::new(limits_with_work(5)).with_injected_denial(denial);
    let mut high = Meter::new(limits_with_work(500)).with_injected_denial(denial);
    let drive = |meter: &mut Meter| {
        expect_incomplete(evaluate_integer(
            IntegerOperation::Negate(&int(-9)),
            &IntegerDomain::Mathematical,
            meter,
        ))
    };
    let record_low = drive(&mut low);
    let record_high = drive(&mut high);

    // Same record under two different configured `work_units` limits: the
    // report is the work already spent, not either configured limit.
    assert_eq!(record_low, record_high);
    assert_eq!(record_low.limit, record_low.consumed);
    // `integer-arithmetic.operands` then `.arithmetic` precede
    // `.result-retain`: two admitted charges.
    assert_eq!(record_low.consumed, 2);
    assert_eq!(record_low.next_charge, int(1));
    assert_eq!(record_low.limit_kind, LimitKind::WorkUnits);
    assert_ne!(record_low.limit, low.limits().work_units);
    assert_ne!(record_high.limit, high.limits().work_units);
    assert_eq!(low.consumed(LimitKind::WorkUnits), 2);
    assert_eq!(high.consumed(LimitKind::WorkUnits), 2);
}

/// Trace: TC-031, FR-010-AC-3
#[test]
fn tc_031_injected_denial_takes_precedence_over_a_genuinely_short_counter() {
    let mut short = [u64::MAX; 10];
    short[0] = 1; // integer_bits: far too small for `magnitude_bits(9) = 4`.
    let short = limits(short);

    // Without injection, the real short counter denies this exact charge.
    let real = evaluate_integer(
        IntegerOperation::Negate(&int(-9)),
        &IntegerDomain::Mathematical,
        &mut Meter::new(short),
    );
    assert_eq!(
        real,
        Outcome::Incomplete(Incomplete {
            limit_kind: LimitKind::IntegerBits,
            limit: 1,
            consumed: 0,
            next_charge: int(4),
            charge_point: ChargePoint::IntegerArithmeticOperands,
        })
    );

    // With the injection at that same point and occurrence, the injected
    // record wins: `WorkUnits`, never the real `IntegerBits` shortfall.
    let mut meter = Meter::new(short).with_injected_denial(InjectedDenial {
        point: ChargePoint::IntegerArithmeticOperands,
        occurrence: 1,
    });
    let injected = expect_incomplete(evaluate_integer(
        IntegerOperation::Negate(&int(-9)),
        &IntegerDomain::Mathematical,
        &mut meter,
    ));
    assert_eq!(
        injected,
        Incomplete {
            limit_kind: LimitKind::WorkUnits,
            limit: 0,
            consumed: 0,
            next_charge: int(1),
            charge_point: ChargePoint::IntegerArithmeticOperands,
        }
    );
    assert!(meter.admitted_charges().is_empty());
}

/// Trace: TC-031, FR-010-AC-4
#[test]
fn tc_031_occurrence_counts_only_admitted_charges_at_the_injected_point() {
    let mut tuple = [u64::MAX; 10];
    tuple[0] = 10; // integer_bits: room for a 4-bit operand, not a 20-bit one.
    let mut meter = Meter::new(limits(tuple)).with_injected_denial(InjectedDenial {
        point: ChargePoint::IntegerArithmeticOperands,
        occurrence: 2,
    });

    // A charge at another point never advances the injected point's
    // occurrence counter.
    assert_eq!(evaluate_not(false, &mut meter), Outcome::Completed(true));

    // A charge at the injected point that a genuinely short counter denies
    // (too many bits) is not admitted, so it does not count as an occurrence.
    let oversized = int(1_i128 << 19);
    assert_eq!(
        evaluate_integer(
            IntegerOperation::Negate(&oversized),
            &IntegerDomain::Mathematical,
            &mut meter,
        ),
        Outcome::Incomplete(Incomplete {
            limit_kind: LimitKind::IntegerBits,
            limit: 10,
            consumed: 0,
            next_charge: int(20),
            charge_point: ChargePoint::IntegerArithmeticOperands,
        })
    );

    // The first ADMITTED charge at the injected point: occurrence one, not
    // denied (the injection names occurrence two).
    let small = int(-9);
    assert_eq!(
        evaluate_integer(
            IntegerOperation::Negate(&small),
            &IntegerDomain::Mathematical,
            &mut meter,
        ),
        Outcome::Completed(int(9))
    );

    // Another charge at a different point, again uncounted.
    assert_eq!(evaluate_not(true, &mut meter), Outcome::Completed(false));

    // The second ADMITTED charge at the injected point: occurrence two, so
    // the injected denial fires here.
    let record = expect_incomplete(evaluate_integer(
        IntegerOperation::Negate(&small),
        &IntegerDomain::Mathematical,
        &mut meter,
    ));
    assert_eq!(record.limit_kind, LimitKind::WorkUnits);
    assert_eq!(record.charge_point, ChargePoint::IntegerArithmeticOperands);
    assert_eq!(record.limit, record.consumed);
    assert_eq!(record.next_charge, int(1));
}

/// Trace: TC-031, FR-010-AC-5
///
/// FR-010's Behavior section: "Exactly one charge is denied per meter... Once
/// the injected denial has fired, the meter's subsequent charges are metered
/// normally against the configured limits." FR-010-AC-5 restates this as "no
/// second charge is injected-denied."
///
/// This test encodes that requirement as written. It currently FAILS:
/// `Meter::charge`'s `check_injected` (`src/exact/accounting.rs`) matches on
/// `denial_point_seen.checked_add(1) == Some(denial.occurrence)` alone, and
/// nothing marks the denial as spent or advances `denial_point_seen` on the
/// denial path. So every later charge at the same injected point matches the
/// same condition again and is denied again, indefinitely, rather than
/// exactly once. See the disagreement reported alongside this test.
#[test]
fn tc_031_further_charges_after_the_injected_denial_meter_normally() {
    let mut meter = Meter::new(UNLIMITED).with_injected_denial(InjectedDenial {
        point: ChargePoint::BooleanResultRetain,
        occurrence: 1,
    });
    let fired = evaluate_not(false, &mut meter);
    assert_eq!(
        fired,
        Outcome::Incomplete(Incomplete {
            limit_kind: LimitKind::WorkUnits,
            limit: 0,
            consumed: 0,
            next_charge: int(1),
            charge_point: ChargePoint::BooleanResultRetain,
        })
    );

    // A further charge at the SAME point, under the same generous limits,
    // must meter normally now that the one injected denial has fired.
    assert_eq!(evaluate_not(true, &mut meter), Outcome::Completed(false));
    assert_eq!(evaluate_not(false, &mut meter), Outcome::Completed(true));
}

/// Trace: TC-016, FR-006-AC-6
#[test]
fn tc_016_refusal_code_is_some_for_exactly_two_ieee_variants() {
    let variants = [
        Refusal::InexactDecimal,
        Refusal::DecimalOutOfDomain,
        Refusal::DivisionPairOutOfDomain {
            quotient_admitted: true,
            remainder_admitted: false,
        },
        Refusal::ModuloOutOfDomain,
        Refusal::TextLengthOutOfDomain,
        Refusal::IntegerOutOfDomain,
        Refusal::IeeeNotExact {
            would_be: IeeeFlags::EMPTY,
        },
        Refusal::IeeeNanPayloadNotRepresentable,
        Refusal::IeeeRationalOutOfDomain,
        Refusal::RationalOutOfDomain,
    ];
    let codes: Vec<Option<&str>> = variants.into_iter().map(Refusal::code).collect();
    assert_eq!(
        codes,
        [
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            Some("ieee_nan_payload_not_representable"),
            Some("ieee_rational_out_of_domain"),
            None,
        ]
    );
}
