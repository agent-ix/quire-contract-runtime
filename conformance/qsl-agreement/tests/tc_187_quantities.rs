// SPDX-License-Identifier: AGPL-3.0-or-later
//! QSpec TC-187 quantities and unit conversion: runtime versus authority.
//!
//! Admission-only vectors are compiler work and are not evaluated here: the
//! owner-selection and stale-key checks of U11 run only where node keys are
//! recomputed from preimages. U11's topology refusals (two roots, no root, a
//! target cycle, an unknown target, a cross-dimension target) and U06's zero
//! scale are evaluated through both graph admissions.

#[macro_use]
mod support;

use support::rt_side::*;
use support::{unit, GraphSpec};

/// Vectors evaluated through both boundaries.
const EVALUATED: [&str; 30] = [
    "U01", "U02", "U03", "U04", "U05", "U06", "U07", "U08", "U09", "U09b", "U10", "U11", "U12",
    "U13", "U14", "U15", "U16", "U17", "U18", "U19", "U20", "U21", "U22", "U23", "U24", "U25",
    "U26", "U27", "U28", "U29",
];
/// Compiler-owned admission vectors.
const ADMISSION_ONLY: [&str; 1] = ["U11 owner selection and stale keys"];

/// Trace: TC-022, FR-007-AC-5, FR-007-AC-6
#[test]
fn tc_022_every_tc187_vector_is_evaluated_or_admission_only() {
    let mut expected: Vec<String> = (1..=29).map(|n| format!("U{n:02}")).collect();
    expected.insert(9, "U09b".into());
    assert_eq!(EVALUATED.to_vec(), expected);
    assert!(ADMISSION_ONLY[0].starts_with("U11"));
    println!("TC-187 agreement: {} evaluated, {} admission-only", EVALUATED.len(), ADMISSION_ONLY.len());
}

const READ: ChargePoint = ChargePoint::UnitIdentityRead;
const EDGE: ChargePoint = ChargePoint::UnitEdge;
const EVENT: ChargePoint = ChargePoint::UnitRationalArithmetic;
const TARGET: ChargePoint = ChargePoint::UnitTargetDomain;
const RETAIN: ChargePoint = ChargePoint::UnitResultRetain;

type Metered<T> = (Result<Outcome<T>, IllTyped>, Vec<ChargePoint>, Vec<u64>);

fn ill(cause: IllTypedCause) -> IllTyped {
    IllTyped { cause }
}

/// The converted exact rational of a completed conversion.
fn exact(outcome: &Result<Outcome<Conversion>, IllTyped>) -> Rational {
    match outcome {
        Ok(Outcome::Completed(conversion)) => match conversion.value() {
            ConvertedValue::Exact(value) => value.clone(),
            other => panic!("not exact: {other:?}"),
        },
        other => panic!("not completed: {other:?}"),
    }
}

/// Trace: TC-022, FR-007-AC-5
#[test]
fn tc_022_u01_u04_u09b_exact_affine_conversion_through_the_root() {
    let f = fixture();
    let (u01, sum, u04, u09b) = agree! {{
        let f = fixture();
        let convert = |q: Quantity, to: &str| convert_quantity(&q, &f.unit(to), &QuantityTarget::Exact, &mut Meter::new(UNLIMITED));
        let u01 = convert(f.qi(250, "cm"), "m");
        let metres = f.q(ratio(5, 2), "m");
        let sum = evaluate_quantity(QuantityOperation::Add(&f.qi(1, "m"), &metres), &mut Meter::new(UNLIMITED));
        let u04 = [convert(f.qi(0, "degC"), "K"), convert(f.qi(32, "degF"), "K")];
        let u09b = convert(f.qi(1, "m_alias"), "m");
        (u01, sum, u04, u09b)
    }};
    assert_eq!(exact(&u01), ratio(5, 2));
    let conversion = u01.unwrap().completed().unwrap();
    assert_eq!(conversion.source(), &f.qi(250, "cm"));
    assert_eq!(conversion.canonical(), &ratio(5, 2));
    assert_eq!(conversion.unit(), &f.unit("m"));
    assert_eq!(conversion.unit().dimension(), f.graph.dimension(f.key("L")).unwrap());
    assert_eq!(sum, Ok(Outcome::Completed(f.q(ratio(7, 2), "m"))));
    for kelvin in &u04 {
        assert_eq!(exact(kelvin), ratio(5463, 20));
    }
    assert_eq!(exact(&u09b), whole(1));
    let alias = u09b.unwrap().completed().unwrap();
    assert_ne!(alias.source().unit(), alias.unit());
    assert_ne!(f.key("m_alias"), f.key("m"));
}

/// Trace: TC-022, FR-007-AC-5
#[test]
fn tc_022_u02_dimension_maps_normalize_and_drop_zero_exponents() {
    let (cancelled, zero) = agree! {{
        let f = fixture();
        let (length, time) = (f.unit("m").dimension().clone(), f.unit("s").dimension().clone());
        (length.multiply(&time).divide(&time), length.power(&int(0)))
    }};
    let f = fixture();
    assert_eq!(&cancelled, f.unit("m").dimension());
    assert_eq!(cancelled.exponents().count(), 1);
    assert!(zero.is_dimensionless());
    assert_eq!(zero, Dimension::dimensionless());
}

/// Trace: TC-022, FR-007-AC-5, FR-006-AC-2
#[test]
fn tc_022_u03_u05_u09_u14_u20_u22_u23_ill_typed_before_any_charge() {
    let (operations, conversions, comparisons) = agree! {{
        let f = fixture();
        let zero = || Meter::new(limits([0; 10]));
        let mut meters: Vec<Meter> = Vec::new();
        let mut operation = |op: QuantityOperation<'_>| {
            let mut meter = zero();
            let outcome = evaluate_quantity(op, &mut meter).map(|_| ());
            meters.push(meter);
            outcome
        };
        let (m, s, deg_c, deg_f, cm, u1, u3, m_other) =
            (f.qi(1, "m"), f.qi(1, "s"), f.qi(1, "degC"), f.qi(1, "degF"), f.qi(1, "cm"), f.qi(1, "u1"), f.qi(1, "u3"), f.qi(1, "m_other"));
        let two = int(2);
        let operations = vec![
            operation(QuantityOperation::Add(&m, &s)),
            operation(QuantityOperation::Add(&deg_c, &deg_c)),
            operation(QuantityOperation::Subtract(&deg_f, &deg_f)),
            operation(QuantityOperation::Multiply(&deg_c, &m)),
            operation(QuantityOperation::Divide(&deg_f, &m)),
            operation(QuantityOperation::Power(&deg_c, &two)),
            operation(QuantityOperation::Add(&m, &m_other)),
            operation(QuantityOperation::Add(&deg_c, &s)),
            operation(QuantityOperation::Divide(&deg_c, &f.qi(0, "degC"))),
            operation(QuantityOperation::Add(&cm, &m)),
            operation(QuantityOperation::Add(&u1, &f.qi(2, "u1"))),
            operation(QuantityOperation::Multiply(&u3, &m)),
        ];
        let charged = meters.iter().map(|meter| meter.admitted_charges().len()).sum::<usize>();
        let mut meter = zero();
        let conversions = vec![
            convert_quantity(&m_other, &f.unit("m"), &QuantityTarget::Exact, &mut meter).map(|_| ()),
            convert_quantity(&f.qi(1, "N_m"), &f.unit("J"), &QuantityTarget::Exact, &mut meter).map(|_| ()),
        ];
        let comparisons = vec![
            compare_quantity(ComparisonOperator::Less, &f.qi(0, "degC"), &f.qi(32, "degF"), &mut meter).map(|_| ()),
            compare_quantity(ComparisonOperator::Equal, &f.qi(0, "degC"), &f.qi(32, "degF"), &mut meter).map(|_| ()),
        ];
        (operations, conversions, (comparisons, charged + meter.admitted_charges().len()))
    }};
    use IllTypedCause::{AffineUnitArithmetic as Affine, DistinctUnits, IncompatibleDimensions as Dimensions};
    let causes = [Dimensions, Affine, Affine, Affine, Affine, Affine, Dimensions, Dimensions, Affine, DistinctUnits, Affine, Affine];
    assert_eq!(operations, causes.map(|cause| Err(ill(cause))).to_vec());
    assert_eq!(conversions, [Err(ill(Dimensions)), Err(ill(Dimensions))]);
    assert_eq!(comparisons.0, [Err(ill(DistinctUnits)), Err(ill(DistinctUnits))]);
    assert_eq!(comparisons.1, 0);
}

/// Trace: TC-022, FR-007-AC-5
#[test]
fn tc_022_u06_u11_graph_topology_refuses_before_any_quantity() {
    let alias = |name, dimension, target, key_as| support::UnitSpec { key_as: Some(key_as), ..unit(name, dimension, Some(target), (1, 1), (0, 1)) };
    let graphs = [
        GraphSpec::tc187().with(unit("zero", "L", Some("m"), (0, 1), (0, 1))),
        GraphSpec::tc187().with(unit("root2", "L", None, (1, 1), (0, 1))),
        GraphSpec::tc187().without(&["m", "cm", "in", "rev", "m_alias"]).with(unit("lonely", "L", Some("s"), (1, 1), (0, 1))),
        GraphSpec::tc187()
            .with(unit("c2_old", "L", Some("m"), (1, 2), (0, 1)))
            .with(unit("c1", "L", Some("c2_old"), (1, 1), (0, 1)))
            .with(alias("c2", "L", "c1", "c2_old"))
            .without(&["c2_old"]),
        GraphSpec::tc187()
            .with(unit("gone", "L", Some("m"), (1, 3), (0, 1)))
            .with(unit("orphan", "L", Some("gone"), (1, 1), (0, 1)))
            .without(&["gone"]),
        GraphSpec::tc187().with(unit("sideways", "L", Some("s"), (1, 1), (0, 1))),
    ];
    let mut causes = Vec::new();
    for graph in graphs {
        causes.push(agree! { admit_graph(&graph).map(|_| ()) });
    }
    use SemanticGraphCause::*;
    let expected = [ZeroScale, DuplicateRoot, CrossDimensionTarget, TargetCycle, UnknownTarget, CrossDimensionTarget];
    assert_eq!(causes, expected.map(|cause| Err(InvalidSemanticGraph { cause })).to_vec());

    // No root at all: every L unit targets another L unit.
    let rootless = GraphSpec::tc187()
        .without(&["m", "cm", "in", "rev", "m_alias"])
        .with(unit("a_old", "L", Some("s"), (1, 1), (0, 1)))
        .with(unit("b", "L", Some("a_old"), (1, 1), (0, 1)))
        .with(alias("a", "L", "b", "a_old"))
        .without(&["a_old"]);
    let missing = agree! { admit_graph(&rootless).map(|_| ()) };
    assert_eq!(missing, Err(InvalidSemanticGraph { cause: MissingRoot }));
    assert!(agree! { admit_graph(&GraphSpec::tc187()).map(|_| ()) }.is_ok());
}

/// Trace: TC-022, FR-007-AC-5, FR-007-AC-2
#[test]
fn tc_022_u07_u08_u16_u17_u18_explicit_decimal_targets() {
    let f = fixture();
    let run = |value: i64, from: &'static str, lo: i64, hi: i64, smin: u64, smax: u64, mode: usize, tuple: [u64; 6]| -> Metered<Conversion> {
        agree! {{
            let f = fixture();
            let target = QuantityTarget::Decimal(decimal_type(lo, hi, smin, smax, RoundingMode::ALL[mode]));
            metered(unit_tuple(tuple), |m| convert_quantity(&f.qi(value, from), &f.unit("m"), &target, m))
        }}
    };
    const EXACT: usize = 0;
    const NEAREST_EVEN: usize = 4;
    let wide = [u64::MAX, u64::MAX, u64::MAX, u64::MAX, u64::MAX, u64::MAX];

    // U07
    assert_eq!(run(1, "in", -1000, 1000, 2, 2, EXACT, wide).0, Ok(Outcome::Refused(Refusal::InexactDecimal)));
    let rounded = run(1, "in", -1000, 1000, 2, 2, NEAREST_EVEN, wide).0.unwrap().completed().unwrap();
    let ConvertedValue::Decimal(result) = rounded.value() else { panic!("not decimal") };
    assert_eq!(result.value().representation(), &DecimalRepresentation::new(int(3), 2));
    let loss = result.loss().unwrap();
    assert_eq!(
        (loss.exact_numerator(), loss.exact_denominator(), loss.rounded_coefficient(), loss.rounded_scale(), loss.mode()),
        (&int(127), int(5000), &int(3), 2, RoundingMode::NearestEven)
    );
    assert_eq!(rounded.canonical(), &ratio(127, 5000));
    assert_eq!(rounded.source(), &f.qi(1, "in"));

    // U08
    for (value, admitted) in [(-2, true), (2, true), (-3, false), (3, false)] {
        let outcome = run(value, "m", -2, 2, 0, 0, EXACT, wide).0.unwrap();
        match outcome {
            Outcome::Completed(conversion) => {
                assert!(admitted);
                assert_eq!(conversion.value(), &ConvertedValue::Decimal(decimal_result(value)));
            }
            other => {
                assert!(!admitted);
                assert_eq!(other, Outcome::Refused(Refusal::DecimalOutOfDomain));
            }
        }
    }

    // U16
    let exact_run = run(1, "in", -1000, 1000, 2, 2, EXACT, [13, 1, 1, 1, 6, 1]);
    assert_eq!(exact_run.0, Ok(Outcome::Refused(Refusal::InexactDecimal)));
    assert_eq!(exact_run.1, [READ, EDGE, EVENT, EVENT]);
    assert_eq!(run(1, "in", -1000, 1000, 2, 2, NEAREST_EVEN, [13, 1, 1, 1, 6, 1]).0.map(|o| o.completed().is_some()), Ok(true));
    assert_eq!(run(1, "in", -1000, 1000, 2, 2, EXACT, [13, 1, 1, 1, 4, 1]).0, Ok(Outcome::Refused(Refusal::InexactDecimal)));
    assert_eq!(
        run(1, "in", -1000, 1000, 2, 2, NEAREST_EVEN, [13, 1, 1, 1, 4, 1]).0,
        Ok(Outcome::Incomplete(incomplete(LimitKind::WorkUnits, 4, 4, int(1), TARGET)))
    );

    // U17
    let u17 = run(3, "m", -2, 2, 0, 0, EXACT, [2, 1, 0, 1, 2, 0]);
    assert_eq!(u17.0, Ok(Outcome::Refused(Refusal::DecimalOutOfDomain)));
    assert_eq!(u17.1, [READ, TARGET]);
    assert_eq!(
        run(3, "m", -2, 2, 0, 0, EXACT, [2, 1, 0, 1, 1, 0]).0,
        Ok(Outcome::Incomplete(incomplete(LimitKind::WorkUnits, 1, 1, int(1), TARGET)))
    );

    // U18
    let u18 = run(1, "m", 0, 1, 0, 4_294_967_295, EXACT, [u64::MAX, 64, 0, 1, 2, 1]);
    assert_eq!(
        u18.0,
        Ok(Outcome::Incomplete(incomplete(LimitKind::DecimalDigits, 64, 0, big("4294967296"), TARGET)))
    );
}

/// The completed decimal of an integer placed at scale zero with no loss.
fn decimal_result(value: i64) -> DecimalResult {
    let target = decimal_type(-2, 2, 0, 0, RoundingMode::Exact);
    let f = fixture();
    let converted = convert_quantity(&f.qi(value, "m"), &f.unit("m"), &QuantityTarget::Decimal(target), &mut Meter::new(UNLIMITED));
    match converted.unwrap().completed().unwrap().value() {
        ConvertedValue::Decimal(result) => {
            assert_eq!(result.value().representation(), &DecimalRepresentation::new(int(value.into()), 0));
            assert!(result.loss().is_none());
            result.clone()
        }
        other => panic!("not decimal: {other:?}"),
    }
}

/// Trace: TC-022, FR-007-AC-5, FR-006-AC-3, FR-006-AC-4
#[test]
fn tc_022_u10_u15_u26_edge_schedules_and_named_denials() {
    let run = |value: i64, from: &'static str, to: &'static str, tuple: [u64; 6]| -> Metered<Conversion> {
        agree! {{
            let f = fixture();
            metered(unit_tuple(tuple), |m| convert_quantity(&f.qi(value, from), &f.unit(to), &QuantityTarget::Exact, m))
        }}
    };
    // U10
    let u10 = [7, 0, 1, 1, 6, 1];
    let exact_run = run(100, "cm", "m", u10);
    assert_eq!(exact(&exact_run.0), whole(1));
    assert_eq!(exact_run.1, [READ, EDGE, EVENT, EVENT, TARGET, RETAIN]);
    assert_eq!(exact_run.2, [7, 0, 0, 0, 0, 0, 1, 1, 6, 1]);
    let denied = agree! {{
        let f = fixture();
        denials(unit_tuple(u10), |m| convert_quantity(&f.qi(100, "cm"), &f.unit("m"), &QuantityTarget::Exact, m))
    }};
    assert_eq!(denied.len(), 6);
    for (work, (point, _, outcome, results)) in (0_u64..).zip(denied) {
        assert_eq!(outcome, Ok(Outcome::Incomplete(work_denied(work, point))));
        assert_eq!(results, 0);
    }

    // U15
    let u15 = run(1, "in", "cm", [13, 0, 2, 1, 9, 1]);
    assert_eq!(exact(&u15.0), ratio(127, 50));
    assert_eq!(u15.1, [READ, EDGE, EDGE, EVENT, EVENT, EVENT, EVENT, TARGET, RETAIN]);
    assert_eq!(
        run(1, "in", "cm", [7, 0, 1, 1, 9, 1]).0,
        Ok(Outcome::Incomplete(incomplete(LimitKind::UnitEdges, 1, 1, int(2), EDGE)))
    );

    // U26
    let u26 = run(1, "u2", "u1", [4, 0, 3, 1, 12, 1]);
    assert_eq!(exact(&u26.0), whole(-9));
    let mut schedule = vec![READ, EDGE, EDGE, EDGE];
    schedule.extend([EVENT; 6]);
    schedule.extend([TARGET, RETAIN]);
    assert_eq!(u26.1, schedule);
    assert_eq!(
        run(1, "u2", "u1", [4, 0, 2, 1, 12, 1]).0,
        Ok(Outcome::Incomplete(incomplete(LimitKind::UnitEdges, 2, 2, int(3), EDGE)))
    );
}

/// Trace: TC-022, FR-007-AC-5, FR-006-AC-3
#[test]
fn tc_022_u12_u13_u21_u27_u28_compound_arithmetic_and_powers() {
    let f = fixture();
    // `a op b` or `a ^ exponent`, with operands `(value, unit)`.
    let run = |op: usize, a: (i64, &'static str), b: (i64, &'static str), exponent: &'static str, tuple: [u64; 6]| -> Metered<Quantity> {
        agree! {{
            let f = fixture();
            let (a, b, n) = (f.qi(a.0, a.1), f.qi(b.0, b.1), big(exponent));
            metered(unit_tuple(tuple), |m| {
                let operation = match op {
                    0 => QuantityOperation::Multiply(&a, &b),
                    1 => QuantityOperation::Divide(&a, &b),
                    _ => QuantityOperation::Power(&a, &n),
                };
                evaluate_quantity(operation, m)
            })
        }}
    };
    const MUL: usize = 0;
    const DIV: usize = 1;
    const POW: usize = 2;
    let wide = [u64::MAX; 6];
    let completed = |run: Metered<Quantity>| run.0.unwrap().completed().unwrap();

    // U12
    assert_eq!(completed(run(MUL, (2, "m"), (3, "s"), "0", wide)), Quantity::new(whole(6), f.compound(&[("m", 1), ("s", 1)])));
    assert_eq!(completed(run(DIV, (6, "m"), (2, "s"), "0", wide)), Quantity::new(whole(3), f.compound(&[("m", 1), ("s", -1)])));
    assert_eq!(completed(run(POW, (2, "m"), (0, "m"), "2", wide)), Quantity::new(whole(4), f.compound(&[("m", 2)])));
    let one = completed(run(DIV, (5, "m"), (5, "m"), "0", wide));
    assert_eq!(one, Quantity::new(whole(1), QuantityUnit::Compound(CompoundUnit::dimensionless())));
    assert!(one.unit().dimension().is_dimensionless());

    // U13
    let u13 = run(POW, (2, "m"), (0, "m"), "18446744073709551616", [64, 0, 0, 1, 5, 1]);
    assert_eq!(
        u13.0,
        Ok(Outcome::Incomplete(incomplete(LimitKind::IntegerBits, 64, 2, big("18446744073709551617"), EVENT)))
    );
    assert_eq!(u13.1, [READ]);

    // U21
    let u21 = run(POW, (0, "m"), (0, "m"), "0", [1, 0, 0, 1, 4, 1]);
    assert_eq!(u21.1, [READ, EVENT, TARGET, RETAIN]);
    assert_eq!(completed(u21), Quantity::new(whole(1), QuantityUnit::Compound(CompoundUnit::dimensionless())));
    let undefined = run(POW, (0, "m"), (0, "m"), "-1", [1, 0, 0, 1, 1, 0]);
    assert_eq!(undefined.0, Ok(Outcome::Undefined(Undefined::DivisionByZero)));
    assert_eq!(undefined.1, [READ]);
    assert_eq!(
        run(POW, (0, "m"), (0, "m"), "-1", [1, 0, 0, 1, 0, 0]).0,
        Ok(Outcome::Incomplete(incomplete(LimitKind::WorkUnits, 0, 0, int(1), READ)))
    );

    // U27
    let u27 = run(DIV, (1, "cm"), (0, "cm"), "0", [1, 0, 0, 2, 2, 0]);
    assert_eq!(u27.0, Ok(Outcome::Undefined(Undefined::DivisionByZero)));
    assert_eq!(u27.1, [READ, READ]);
    assert_eq!(
        run(DIV, (1, "cm"), (0, "cm"), "0", [1, 0, 0, 2, 1, 0]).0,
        Ok(Outcome::Incomplete(incomplete(LimitKind::WorkUnits, 1, 1, int(1), READ)))
    );

    // U28
    let u28 = run(MUL, (1, "cm"), (1, "in"), "0", [19, 0, 2, 2, 11, 1]);
    assert_eq!(u28.1, [READ, READ, EDGE, EDGE, EVENT, EVENT, EVENT, EVENT, EVENT, TARGET, RETAIN]);
    assert_eq!(completed(u28), Quantity::new(ratio(127, 500_000), f.compound(&[("m", 2)])));
    assert_eq!(
        run(MUL, (1, "cm"), (1, "in"), "0", [12, 0, 2, 2, 11, 1]).0,
        Ok(Outcome::Incomplete(incomplete(LimitKind::IntegerBits, 12, 7, int(13), EVENT)))
    );
    assert_eq!(
        run(MUL, (1, "cm"), (1, "in"), "0", [18, 0, 2, 2, 11, 1]).0,
        Ok(Outcome::Incomplete(incomplete(LimitKind::IntegerBits, 18, 13, int(19), EVENT)))
    );
}

/// Trace: TC-022, FR-007-AC-5, FR-006-AC-3
#[test]
fn tc_022_u19_u20_u24_same_unit_sums_charge_no_edge() {
    let f = fixture();
    let run = |a: i64, b: i64, name: &'static str, tuple: [u64; 6]| -> Metered<Quantity> {
        agree! {{
            let f = fixture();
            metered(unit_tuple(tuple), |m| evaluate_quantity(QuantityOperation::Add(&f.qi(a, name), &f.qi(b, name)), m))
        }}
    };
    for (a, b, name, tuple, sum) in [(2, 3, "m", [3, 0, 0, 2, 5, 1], 5), (1, 2, "u2", [2, 0, 0, 2, 5, 1], 3), (1, 2, "cm", [2, 0, 0, 2, 5, 1], 3)] {
        let exact_run = run(a, b, name, tuple);
        assert_eq!(exact_run.0, Ok(Outcome::Completed(f.qi(sum, name))));
        assert_eq!(exact_run.1, [READ, READ, EVENT, TARGET, RETAIN]);
    }
    assert_eq!(
        run(2, 3, "m", [3, 0, 0, 2, 4, 1]).0,
        Ok(Outcome::Incomplete(incomplete(LimitKind::WorkUnits, 4, 4, int(1), RETAIN)))
    );
}

/// Trace: TC-022, FR-007-AC-5, FR-006-AC-3
#[test]
fn tc_022_u22_u25_compound_conversions() {
    let f = fixture();
    let run = |terms: &'static [(&'static str, i64)], to: &'static str, tuple: [u64; 6]| -> Metered<Conversion> {
        agree! {{
            let f = fixture();
            metered(unit_tuple(tuple), |m| convert_quantity(&Quantity::new(whole(4), f.compound(terms)), &f.unit(to), &QuantityTarget::Exact, m))
        }}
    };
    let m2 = run(&[("m", 2)], "m2", [3, 0, 0, 1, 3, 1]);
    assert_eq!(exact(&m2.0), whole(4));
    assert_eq!(m2.1, [READ, TARGET, RETAIN]);
    let cm2 = run(&[("m", 2)], "cm2", [16, 0, 1, 1, 6, 1]);
    assert_eq!(exact(&cm2.0), whole(40_000));
    assert_eq!(cm2.1, [READ, EDGE, EVENT, EVENT, TARGET, RETAIN]);
    assert_eq!(
        run(&[("m", 2)], "cm2", [15, 0, 1, 1, 6, 1]).0,
        Ok(Outcome::Incomplete(incomplete(LimitKind::IntegerBits, 15, 3, int(16), EVENT)))
    );

    // U25
    let (to_compound, to_joule) = agree! {{
        let f = fixture();
        let tuple = unit_limits(1, 0, 0, 1, 3, 1);
        let compound = f.compound(&[("kg", 1), ("m", 2), ("s", -2)]);
        let first = metered(tuple, |m| convert_quantity(&f.qi(1, "N_m"), &compound, &QuantityTarget::Exact, m));
        let step = match &first.0 {
            Ok(Outcome::Completed(conversion)) => match conversion.value() {
                ConvertedValue::Exact(value) => Quantity::new(value.clone(), compound.clone()),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        };
        let second = metered(tuple, |m| convert_quantity(&step, &f.unit("J"), &QuantityTarget::Exact, m));
        (first, second)
    }};
    for step in [&to_compound, &to_joule] {
        assert_eq!(exact(&step.0), whole(1));
        assert_eq!(step.1, [READ, TARGET, RETAIN]);
    }
    assert_eq!(to_joule.0.unwrap().completed().unwrap().unit(), &f.unit("J"));
}

/// Trace: TC-022, FR-007-AC-5, FR-006-AC-3
#[test]
fn tc_022_u23_comparisons_order_by_root_value() {
    let run = |op: usize, a: i64, b: i64, name: &'static str, tuple: [u64; 6]| -> Metered<bool> {
        agree! {{
            let f = fixture();
            metered(unit_tuple(tuple), |m| compare_quantity(ComparisonOperator::ALL[op], &f.qi(a, name), &f.qi(b, name), m))
        }}
    };
    const EQUAL: usize = 0;
    const LESS: usize = 2;
    let ordered = run(LESS, 1, 2, "degC", [13, 0, 2, 2, 9, 1]);
    assert_eq!(ordered.0, Ok(Outcome::Completed(true)));
    assert_eq!(ordered.1, [READ, READ, EDGE, EDGE, EVENT, EVENT, EVENT, EVENT, RETAIN]);
    assert_eq!(
        run(LESS, 1, 2, "degC", [13, 0, 2, 2, 8, 1]).0,
        Ok(Outcome::Incomplete(incomplete(LimitKind::WorkUnits, 8, 8, int(1), RETAIN)))
    );
    assert_eq!(run(LESS, 1, 2, "rev", [u64::MAX; 6]).0, Ok(Outcome::Completed(false)));
    assert_eq!(run(EQUAL, 1, 1, "degC", [u64::MAX; 6]).0, Ok(Outcome::Completed(true)));
}

/// Trace: TC-022, FR-007-AC-5, FR-006-AC-3
#[test]
fn tc_022_u29_integer_targets() {
    let run = |value: i64, from: &'static str, mode: usize, tuple: [u64; 6]| -> Metered<Conversion> {
        agree! {{
            let f = fixture();
            let target = QuantityTarget::Integer { domain: IntegerInterval::new(int(-2), int(2)).unwrap(), rounding: RoundingMode::ALL[mode] };
            metered(unit_tuple(tuple), |m| convert_quantity(&f.qi(value, from), &f.unit("m"), &target, m))
        }}
    };
    const EXACT: usize = 0;
    const NEAREST_EVEN: usize = 4;
    let strict = run(1, "in", EXACT, [13, 0, 1, 1, 4, 1]);
    assert_eq!(strict.0, Ok(Outcome::Refused(Refusal::InexactDecimal)));
    assert_eq!(strict.1, [READ, EDGE, EVENT, EVENT]);
    assert_eq!(
        run(1, "in", NEAREST_EVEN, [13, 0, 1, 1, 4, 1]).0,
        Ok(Outcome::Incomplete(incomplete(LimitKind::WorkUnits, 4, 4, int(1), TARGET)))
    );
    let rounded = run(1, "in", NEAREST_EVEN, [u64::MAX; 6]).0.unwrap().completed().unwrap();
    let ConvertedValue::Integer { value, loss } = rounded.value() else { panic!("not integer") };
    assert_eq!(value.value(), &int(0));
    assert_eq!(loss.as_ref().map(DecimalLoss::rounded_coefficient), Some(&int(0)));
    let out = run(3, "m", EXACT, [2, 0, 0, 1, 2, 0]);
    assert_eq!(out.0, Ok(Outcome::Refused(Refusal::IntegerOutOfDomain)));
    assert_eq!(out.1, [READ, TARGET]);
    assert_eq!(
        run(3, "m", EXACT, [2, 0, 0, 1, 1, 0]).0,
        Ok(Outcome::Incomplete(incomplete(LimitKind::WorkUnits, 1, 1, int(1), TARGET)))
    );
}

/// An `i128` fraction oracle, independent of both implementations.
#[derive(Clone, Copy, Debug)]
struct Fraction(i128, i128);

impl Fraction {
    fn add(self, other: Self) -> Self {
        Self(self.0 * other.1 + other.0 * self.1, self.1 * other.1)
    }
    fn mul(self, other: Self) -> Self {
        Self(self.0 * other.0, self.1 * other.1)
    }
    fn inverse(self) -> Self {
        Self(self.1, self.0)
    }
    fn neg(self) -> Self {
        Self(-self.0, self.1)
    }
    fn rational(self) -> Rational {
        let (n, d) = (i64::try_from(self.0).unwrap(), i64::try_from(self.1).unwrap());
        ratio(n, d)
    }
}

/// Compose a declared unit's affine map to its root from the fixture spec.
fn to_root(spec: &GraphSpec, name: &str, value: Fraction) -> Fraction {
    let unit = spec.units.iter().find(|u| u.name == name).unwrap();
    match unit.target {
        None => value,
        Some(target) => {
            let (s, o) = (Fraction(unit.scale.0.into(), unit.scale.1.into()), Fraction(unit.offset.0.into(), unit.offset.1.into()));
            to_root(spec, target, value.mul(s).add(o))
        }
    }
}

fn from_root(spec: &GraphSpec, name: &str, root: Fraction) -> Fraction {
    let unit = spec.units.iter().find(|u| u.name == name).unwrap();
    match unit.target {
        None => root,
        Some(target) => {
            let (s, o) = (Fraction(unit.scale.0.into(), unit.scale.1.into()), Fraction(unit.offset.0.into(), unit.offset.1.into()));
            from_root(spec, target, root).add(o.neg()).mul(s.inverse())
        }
    }
}

/// Trace: TC-022, FR-007-AC-5, FR-006-AC-4
#[test]
fn tc_022_generated_conversions_arithmetic_and_denials_agree() {
    let spec = GraphSpec::tc187();
    let families: [&[&str]; 2] = [&["m", "cm", "in", "rev", "m_alias"], &["K", "degC", "degF", "u1", "u2", "u3"]];
    let values = [Fraction(0, 1), Fraction(1, 1), Fraction(-7, 3), Fraction(250, 1), Fraction(5463, 20)];
    let mut vectors = 0_u32;
    for family in families {
        for from in family {
            for to in family {
                for value in values {
                    let (n, d) = (i64::try_from(value.0).unwrap(), i64::try_from(value.1).unwrap());
                    let (converted, denied) = agree! {{
                        let f = fixture();
                        let run = |m: &mut Meter| convert_quantity(&f.q(ratio(n, d), from), &f.unit(to), &QuantityTarget::Exact, m);
                        (metered(UNLIMITED, run), denials(UNLIMITED, run))
                    }};
                    let expected = from_root(&spec, to, to_root(&spec, from, value)).rational();
                    assert_eq!(exact(&converted.0), expected, "{value:?} {from} -> {to}");
                    assert_eq!(denied.len(), converted.1.len());
                    for (work, (point, _, outcome, results)) in (0_u64..).zip(denied) {
                        assert_eq!(outcome, Ok(Outcome::Incomplete(work_denied(work, point))));
                        assert_eq!(results, 0);
                    }
                    vectors += 1;
                }
            }
        }
    }
    assert_eq!(vectors, (5 * 5 + 6 * 6) * 5);

    let mut operations = 0_u32;
    for name in ["m", "cm", "in", "rev", "u2"] {
        for left in values {
            for right in values {
                let (ln, ld, rn, rd) = (left.0 as i64, left.1 as i64, right.0 as i64, right.1 as i64);
                let outcomes = agree! {{
                    let f = fixture();
                    let (a, b) = (f.q(ratio(ln, ld), name), f.q(ratio(rn, rd), name));
                    let unlimited = || Meter::new(UNLIMITED);
                    (
                        evaluate_quantity(QuantityOperation::Add(&a, &b), &mut unlimited()),
                        evaluate_quantity(QuantityOperation::Subtract(&a, &b), &mut unlimited()),
                        evaluate_quantity(QuantityOperation::Multiply(&a, &b), &mut unlimited()),
                        evaluate_quantity(QuantityOperation::Divide(&a, &b), &mut unlimited()),
                        ComparisonOperator::ALL.map(|op| compare_quantity(op, &a, &b, &mut unlimited())),
                    )
                }};
                let sum = outcomes.0.unwrap().completed().unwrap();
                assert_eq!(sum.value(), &left.add(right).rational());
                let difference = outcomes.1.unwrap().completed().unwrap();
                assert_eq!(difference.value(), &left.add(right.neg()).rational());
                let (lr, rr) = (to_root(&spec, name, left), to_root(&spec, name, right));
                let product = outcomes.2.unwrap().completed().unwrap();
                assert_eq!(product.value(), &lr.mul(rr).rational());
                match outcomes.3.unwrap() {
                    Outcome::Completed(quotient) => assert_eq!(quotient.value(), &lr.mul(rr.inverse()).rational()),
                    Outcome::Undefined(Undefined::DivisionByZero) => assert_eq!(right.0, 0),
                    other => panic!("unexpected {other:?}"),
                }
                let ordering = lr.rational().cmp(&rr.rational());
                let expected = [ordering.is_eq(), ordering.is_ne(), ordering.is_lt(), ordering.is_le(), ordering.is_gt(), ordering.is_ge()];
                assert_eq!(outcomes.4.map(|o| o.unwrap().completed().unwrap()), expected);
                operations += 1;
            }
        }
    }
    assert_eq!(operations, 5 * 25);
}
