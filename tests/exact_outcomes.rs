//! FR-006 typed exact outcomes and `quire.value.accounting/v1` metering, through
//! the public `exact` surface.
#![cfg(feature = "exact")]

use std::collections::BTreeSet;
use std::fs;
use std::path::Path;

use quire_contract_runtime::exact::{
    admit_text, compare_text, evaluate_integer, evaluate_not, ChargePoint, ComparisonOperator,
    IllTyped, IllTypedCause, Incomplete, InjectedDenial, Integer, IntegerDomain, IntegerInterval,
    IntegerOperation, LimitKind, Meter, Outcome, Refusal, ScalarLimits, TextPayload, TextProfile,
    TextType,
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
        evaluate_not(true, &mut Meter::new(UNLIMITED)),
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
    assert_eq!(ChargePoint::from_code("equality.plan-form"), None);
    assert_eq!(ChargePoint::from_code("ordering"), None);
    // The QSpec 5d88578 scalar families, in definition-row order.
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
    let outcome = evaluate_integer(
        IntegerOperation::Multiply(&two_64, &two_64),
        &IntegerDomain::Mathematical,
        &mut meter,
    );
    assert_eq!(
        outcome,
        Outcome::Incomplete(Incomplete {
            limit_kind: LimitKind::IntegerBits,
            limit: 128,
            consumed: 65,
            next_charge: int(129),
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
    let run = evaluate_integer(
        IntegerOperation::Add(&one, &three),
        &IntegerDomain::Mathematical,
        &mut meter,
    );
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
    let outcome = evaluate_integer(
        IntegerOperation::Add(&a, &b),
        &IntegerDomain::Mathematical,
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

    let domain = IntegerDomain::Bounded(IntegerInterval::new(int(0), int(10)).unwrap());
    let mut meter = Meter::new(UNLIMITED);
    let refused = evaluate_integer(IntegerOperation::Add(&int(7), &int(5)), &domain, &mut meter);
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
        let outcome = evaluate_integer(
            IntegerOperation::Negate(&int(-9)),
            &IntegerDomain::Mathematical,
            &mut meter,
        );
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
    assert_eq!(evaluate_not(false, &mut meter), Outcome::Completed(true));
    assert!(
        matches!(evaluate_not(false, &mut meter), Outcome::Incomplete(ref record) if record.limit == 1)
    );
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
    assert_eq!(sources.len(), 16);
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
                !(token.chars().next().unwrap().is_alphanumeric() && word(before))
                    && !(token.chars().last().unwrap().is_alphanumeric() && word(after))
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
    assert_eq!(declared.len(), 15);
    assert_eq!(public, exported);
}
