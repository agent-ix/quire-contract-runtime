//! FR-007-AC-20 through AC-25: exact same-dimension quantity conversion through the existing
//! `convert_quantity` with `QuantityTarget::Exact`, and the affine refusals of the existing
//! `evaluate_quantity`, through the public `exact` surface (TC-201). No RT symbol is added
//! (FR-275-AC-18). Fixtures are built with RT's own unit-graph builders from the vectors the
//! criteria restate; no QSpec bytes are copied.
#![cfg(feature = "exact")]

use ix_trace_rs::trace;
use quire_contract_runtime::exact::{
    convert_quantity, evaluate_quantity, Conversion, ConvertedValue, NodeKey, Quantity,
    QuantityOperation, QuantityTarget, QuantityUnit, UnitDeclaration, UnitGraph,
};
use quire_exact::{
    ChargePoint, IllTyped, IllTypedCause, Incomplete, Integer, LimitKind, Meter, Outcome, Rational,
    ScalarLimits,
};

const fn key(byte: u8) -> NodeKey {
    NodeKey::from_bytes([byte; 32])
}

// Dimension nodes. `D_ALIKE` has the shape of `D_L` under a different semantic-node key.
const D_L: NodeKey = key(1);
const D_T: NodeKey = key(2);
const D_THETA: NodeKey = key(3);
const D_M: NodeKey = key(4);
const D_TORQUE: NodeKey = key(5);
const D_ENERGY: NodeKey = key(6);
const D_AREA: NodeKey = key(7);
const D_ALIKE: NodeKey = key(8);

// Unit nodes. The compound-unit terms must ascend by key: `KG < M < S`.
const KG: NodeKey = key(20);
const M: NodeKey = key(21);
const S: NodeKey = key(22);
const K: NodeKey = key(23);
const CM: NodeKey = key(24);
const IN: NodeKey = key(25);
const DEGC: NodeKey = key(26);
const DEGF: NodeKey = key(27);
const N_M: NodeKey = key(28);
const J: NodeKey = key(29);
const M2: NodeKey = key(30);
const U1: NodeKey = key(31);
const U2: NodeKey = key(32);
const U3: NodeKey = key(33);
const REV: NodeKey = key(34);
const CM2: NodeKey = key(35);
const ALIAS: NodeKey = key(36);
const ALIKE_ROOT: NodeKey = key(37);
// `X` targets `degF` and `W` targets `X`: chains whose edges do not commute.
const X: NodeKey = key(38);
const W: NodeKey = key(39);

fn int(value: i128) -> Integer {
    Integer::from(value)
}

fn ratio(numerator: i128, denominator: i128) -> Rational {
    Rational::new(int(numerator), int(denominator)).unwrap()
}

/// `ScalarLimitsV1` in the `L(i,d,e,o,w,r)` order the vectors use; the unused counters are zero.
const fn limits(i: u64, d: u64, e: u64, o: u64, w: u64, r: u64) -> ScalarLimits {
    ScalarLimits {
        integer_bits: i,
        decimal_digits: d,
        scale_expansion: 0,
        text_input_bytes: 0,
        text_scalars: 0,
        normalized_scalars: 0,
        unit_edges: e,
        value_occurrences: o,
        work_units: w,
        result_units: r,
    }
}

const UNLIMITED: ScalarLimits = limits(u64::MAX, u64::MAX, u64::MAX, u64::MAX, u64::MAX, u64::MAX);

fn root(dimension: NodeKey) -> UnitDeclaration {
    edge(dimension, None, ratio(1, 1), ratio(0, 1))
}

fn edge(
    dimension: NodeKey,
    target: Option<NodeKey>,
    scale: Rational,
    offset: Rational,
) -> UnitDeclaration {
    UnitDeclaration {
        dimension,
        target,
        scale,
        offset,
    }
}

fn to(dimension: NodeKey, target: NodeKey, scale: Rational, offset: Rational) -> UnitDeclaration {
    edge(dimension, Some(target), scale, offset)
}

/// The unit graph of the vectors: length, time, temperature, mass, two distinct derived
/// dimensions of one base-dimension map, area, and a base dimension shaped like length.
fn graph() -> UnitGraph {
    let terms = || vec![(D_L, int(2)), (D_T, int(-2)), (D_M, int(1))];
    UnitGraph::admit(
        [
            (D_L, vec![]),
            (D_T, vec![]),
            (D_THETA, vec![]),
            (D_M, vec![]),
            (D_TORQUE, terms()),
            (D_ENERGY, terms()),
            (D_AREA, vec![(D_L, int(2))]),
            (D_ALIKE, vec![]),
        ],
        [
            (M, root(D_L)),
            (S, root(D_T)),
            (K, root(D_THETA)),
            (KG, root(D_M)),
            (N_M, root(D_TORQUE)),
            (J, root(D_ENERGY)),
            (M2, root(D_AREA)),
            (ALIKE_ROOT, root(D_ALIKE)),
            (CM, to(D_L, M, ratio(1, 100), ratio(0, 1))),
            (IN, to(D_L, M, ratio(127, 5000), ratio(0, 1))),
            (ALIAS, to(D_L, M, ratio(1, 1), ratio(0, 1))),
            (REV, to(D_L, M, ratio(-1, 1), ratio(0, 1))),
            (DEGC, to(D_THETA, K, ratio(1, 1), ratio(5463, 20))),
            (DEGF, to(D_THETA, K, ratio(5, 9), ratio(45967, 180))),
            (U1, to(D_THETA, K, ratio(1, 1), ratio(10, 1))),
            (U2, to(D_THETA, U1, ratio(1, 1), ratio(-10, 1))),
            (U3, to(D_THETA, DEGC, ratio(1, 1), ratio(0, 1))),
            (CM2, to(D_AREA, M2, ratio(1, 10000), ratio(0, 1))),
            (X, to(D_THETA, DEGF, ratio(2, 1), ratio(3, 1))),
            (W, to(D_THETA, X, ratio(3, 1), ratio(-1, 1))),
        ],
    )
    .unwrap()
}

fn declared(graph: &UnitGraph, unit: NodeKey) -> QuantityUnit {
    QuantityUnit::Declared(Box::new(graph.unit(unit).unwrap().clone()))
}

fn quantity(graph: &UnitGraph, unit: NodeKey, numerator: i128, denominator: i128) -> Quantity {
    Quantity::new(ratio(numerator, denominator), declared(graph, unit))
}

/// A compound unit over canonical roots, e.g. `m^2`.
fn compound(graph: &UnitGraph, terms: &[(NodeKey, i128)]) -> QuantityUnit {
    let terms: Vec<_> = terms
        .iter()
        .map(|(unit, power)| (*unit, int(*power)))
        .collect();
    QuantityUnit::Compound(graph.compound_unit(&terms).unwrap())
}

/// Exact conversion of `source` into `unit` under `scalar_limits`, with the meter it ran on.
fn convert(
    source: &Quantity,
    unit: &QuantityUnit,
    scalar_limits: ScalarLimits,
) -> (Result<Outcome<Conversion>, IllTyped>, Meter) {
    let mut meter = Meter::new(scalar_limits);
    let result = convert_quantity(source, unit, &QuantityTarget::Exact, &mut meter);
    (result, meter)
}

/// The completed conversion, with the meter it ran on.
fn completed(
    source: &Quantity,
    unit: &QuantityUnit,
    scalar_limits: ScalarLimits,
) -> (Conversion, Meter) {
    let (result, meter) = convert(source, unit, scalar_limits);
    match result {
        Ok(Outcome::Completed(conversion)) => (conversion, meter),
        Ok(other) => panic!("expected a completed conversion, got {other:?}"),
        Err(refused) => panic!("expected a completed conversion, got {refused:?}"),
    }
}

/// The incomplete record, asserting that no partial quantity exists.
fn incomplete(
    source: &Quantity,
    unit: &QuantityUnit,
    scalar_limits: ScalarLimits,
) -> (Incomplete, Meter) {
    let (result, meter) = convert(source, unit, scalar_limits);
    match result {
        Ok(Outcome::Incomplete(record)) => (record, meter),
        Ok(other) => panic!("expected an incomplete conversion, got {other:?}"),
        Err(refused) => panic!("expected an incomplete conversion, got {refused:?}"),
    }
}

fn record(
    limit_kind: LimitKind,
    limit: u64,
    consumed: u64,
    next_charge: i128,
    charge_point: ChargePoint,
) -> Incomplete {
    Incomplete {
        limit_kind,
        limit,
        consumed,
        next_charge: int(next_charge),
        charge_point,
    }
}

fn exact_value(conversion: &Conversion) -> &Rational {
    match conversion.value() {
        ConvertedValue::Exact(value) => value,
        other => panic!("expected an exact converted value, got {other:?}"),
    }
}

/// The admitted charge schedule of an exact conversion: one read, `edges` edge charges,
/// `events` rational events, then target admission and retention.
fn schedule(edges: usize, events: usize) -> Vec<ChargePoint> {
    let mut points = vec![ChargePoint::UnitIdentityRead];
    points.extend(vec![ChargePoint::UnitEdge; edges]);
    points.extend(vec![ChargePoint::UnitRationalArithmetic; events]);
    points.push(ChargePoint::UnitTargetDomain);
    points.push(ChargePoint::UnitResultRetain);
    points
}

fn assert_consumed(meter: &Meter, expected: [(LimitKind, u64); 6]) {
    for (kind, amount) in expected {
        assert_eq!(meter.consumed(kind), amount, "consumed {kind:?}");
    }
}

fn assert_no_charge(meter: &Meter) {
    assert!(meter.admitted_charges().is_empty());
    assert_eq!(meter.admission_count(), 0);
    for kind in LimitKind::ALL {
        assert_eq!(meter.consumed(kind), 0, "consumed {kind:?}");
    }
}

#[trace("TC-201", "FR-007-AC-20")]
#[test]
fn tc_201_exact_conversion_returns_the_composed_rational_in_the_target_unit() {
    let graph = graph();
    let metre = declared(&graph, M);
    let kelvin = declared(&graph, K);

    // U01: 250 cm is 5/2 m.
    let source = quantity(&graph, CM, 250, 1);
    let (conversion, _) = completed(&source, &metre, UNLIMITED);
    assert_eq!(exact_value(&conversion), &ratio(5, 2));
    assert_eq!(conversion.unit(), &metre);
    assert_eq!(conversion.source(), &source);

    // U04: 0 degC and 32 degF are both 5463/20 K.
    for (unit, value) in [(DEGC, 0), (DEGF, 32)] {
        let (conversion, _) = completed(&quantity(&graph, unit, value, 1), &kelvin, UNLIMITED);
        assert_eq!(exact_value(&conversion), &ratio(5463, 20));
        assert_eq!(conversion.unit(), &kelvin);
    }

    // U22: compound 4 m^2 to the declared area unit m2 is 4.
    let area = declared(&graph, M2);
    let square = Quantity::new(ratio(4, 1), compound(&graph, &[(M, 2)]));
    let (conversion, _) = completed(&square, &area, UNLIMITED);
    assert_eq!(exact_value(&conversion), &ratio(4, 1));
    assert_eq!(conversion.unit(), &area);

    // U25 second step: 1 kg*m^2*s^-2 to J is 1.
    let joule = declared(&graph, J);
    let energy = Quantity::new(ratio(1, 1), compound(&graph, &[(KG, 1), (M, 2), (S, -2)]));
    let (conversion, _) = completed(&energy, &joule, UNLIMITED);
    assert_eq!(exact_value(&conversion), &ratio(1, 1));
    assert_eq!(conversion.unit(), &joule);

    // U26: 1 u2 to u1 is -9.
    let u1 = declared(&graph, U1);
    let (conversion, _) = completed(&quantity(&graph, U2, 1, 1), &u1, UNLIMITED);
    assert_eq!(exact_value(&conversion), &ratio(-9, 1));
    assert_eq!(conversion.unit(), &u1);

    // U09b: a distinct unit key under the same dimension converts exactly, and the source and
    // target unit identities stay distinct.
    let alias = quantity(&graph, ALIAS, 7, 3);
    let (conversion, _) = completed(&alias, &metre, UNLIMITED);
    assert_eq!(exact_value(&conversion), &ratio(7, 3));
    assert_eq!(conversion.unit(), &metre);
    assert_ne!(conversion.source().unit(), conversion.unit());

    // A negative composed scale converts exactly through the root: 1 rev is -1 m.
    let (conversion, _) = completed(&quantity(&graph, REV, 1, 1), &metre, UNLIMITED);
    assert_eq!(exact_value(&conversion), &ratio(-1, 1));
}

/// An exact `i128` fraction in lowest terms with a positive denominator, independent of the
/// runtime's `Rational`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Fraction {
    numerator: i128,
    denominator: i128,
}

impl Fraction {
    fn new(numerator: i128, denominator: i128) -> Self {
        let (mut a, mut b) = (numerator.abs(), denominator.abs());
        while b != 0 {
            (a, b) = (b, a % b);
        }
        let sign = if denominator < 0 { -1 } else { 1 };
        let divisor = a.max(1);
        Self {
            numerator: sign * numerator / divisor,
            denominator: sign * denominator / divisor,
        }
    }

    fn mul(self, other: Self) -> Self {
        Self::new(
            self.numerator.checked_mul(other.numerator).unwrap(),
            self.denominator.checked_mul(other.denominator).unwrap(),
        )
    }

    fn div(self, other: Self) -> Self {
        self.mul(Self::new(other.denominator, other.numerator))
    }

    fn add(self, other: Self) -> Self {
        Self::new(
            self.numerator
                .checked_mul(other.denominator)
                .unwrap()
                .checked_add(other.numerator.checked_mul(self.denominator).unwrap())
                .unwrap(),
            self.denominator.checked_mul(other.denominator).unwrap(),
        )
    }

    fn neg(self) -> Self {
        Self::new(-self.numerator, self.denominator)
    }

    fn rational(self) -> Rational {
        ratio(self.numerator, self.denominator)
    }
}

/// Every nonzero scale and every offset of the sweep: numerators spanning sign and a value past
/// `i64::MAX` (`10^20 + 3`), denominators one, a prime and a power of ten. Intermediate values
/// stay within `i128` (checked in the oracle).
fn sweep_edges() -> Vec<(Fraction, Fraction)> {
    const BIG: i128 = 100_000_000_000_000_000_003;
    let scales: Vec<_> = [-3, 1, 7, 1_000_003, BIG]
        .into_iter()
        .flat_map(|n| [1, 1000].map(|d| Fraction::new(n, d)))
        .collect();
    let offsets: Vec<_> = [-5, 0, 1_000_003, BIG]
        .into_iter()
        .flat_map(|n| [1, 7].map(|d| Fraction::new(n, d)))
        .collect();
    scales
        .iter()
        .flat_map(|scale| offsets.iter().map(|offset| (*scale, *offset)))
        .collect()
}

/// One root `R` and two units `S` and `T` targeting it with the given edges.
fn sweep_graph(source: (Fraction, Fraction), target: (Fraction, Fraction)) -> UnitGraph {
    UnitGraph::admit(
        [(D_L, vec![])],
        [
            (M, root(D_L)),
            (CM, to(D_L, M, source.0.rational(), source.1.rational())),
            (IN, to(D_L, M, target.0.rational(), target.1.rational())),
        ],
    )
    .unwrap()
}

#[trace("TC-201", "FR-007-AC-20")]
#[test]
fn tc_201_exact_conversion_matches_an_independent_i128_rational_oracle_over_edge_fractions() {
    let edges = sweep_edges();
    let values = [
        Fraction::new(0, 1),
        Fraction::new(7, 3),
        Fraction::new(-250, 1),
    ];
    let mut converted = 0_u32;
    for source_edge in &edges {
        for target_edge in &edges {
            let graph = sweep_graph(*source_edge, *target_edge);
            let target = declared(&graph, IN);
            for value in values {
                let source = Quantity::new(value.rational(), declared(&graph, CM));
                let (conversion, _) = completed(&source, &target, UNLIMITED);
                // target(v) = (scale_s * v + offset_s - offset_t) / scale_t
                let root_value = source_edge.0.mul(value).add(source_edge.1);
                let oracle = root_value.add(target_edge.1.neg()).div(target_edge.0);
                assert_eq!(
                    exact_value(&conversion),
                    &oracle.rational(),
                    "{value:?} through {source_edge:?} to {target_edge:?}"
                );
                assert_eq!(conversion.canonical(), &root_value.rational());
                converted += 1;
            }
        }
    }
    assert_eq!(edges.len(), 80);
    assert_eq!(converted, 80 * 80 * 3);
}

/// `target = scale * source + offset` along `path`, source-to-root, in the oracle's fractions.
fn apply_path(path: &[(Fraction, Fraction)], value: Fraction) -> Fraction {
    path.iter().fold(value, |current, (scale, offset)| {
        scale.mul(current).add(*offset)
    })
}

/// The inverse of [`apply_path`]: root-to-source, undoing the last edge first.
fn unapply_path(path: &[(Fraction, Fraction)], value: Fraction) -> Fraction {
    path.iter().rev().fold(value, |current, (scale, offset)| {
        current.add(offset.neg()).div(*scale)
    })
}

#[trace("TC-201", "FR-007-AC-20")]
#[test]
fn tc_201_exact_conversion_orders_the_edges_of_multi_edge_paths() {
    let graph = graph();
    let f = |n, d| Fraction::new(n, d);
    // Source-to-root edges of `X` (to degF, then to K) and of `W` (to X, to degF, to K); none of
    // the adjacent edges commute.
    let degf = (f(5, 9), f(45967, 180));
    let x_path = [(f(2, 1), f(3, 1)), degf];
    let w_path = [(f(3, 1), f(-1, 1)), (f(2, 1), f(3, 1)), degf];
    let u2_path = [(f(1, 1), f(-10, 1)), (f(1, 1), f(10, 1))];
    let cases = [
        (X, &x_path[..], K, &[][..]),
        (K, &[][..], X, &x_path[..]),
        (W, &w_path[..], K, &[][..]),
        (K, &[][..], W, &w_path[..]),
        (X, &x_path[..], W, &w_path[..]),
        (W, &w_path[..], X, &x_path[..]),
        (W, &w_path[..], U2, &u2_path[..]),
        (W, &w_path[..], DEGC, &[(f(1, 1), f(5463, 20))][..]),
    ];
    for (source_unit, source_path, target_unit, target_path) in cases {
        for value in [f(1, 1), f(-7, 3), f(0, 1)] {
            let source = Quantity::new(value.rational(), declared(&graph, source_unit));
            let (conversion, _) = completed(&source, &declared(&graph, target_unit), UNLIMITED);
            let root = apply_path(source_path, value);
            let expected = unapply_path(target_path, root);
            assert_eq!(conversion.canonical(), &root.rational());
            assert_eq!(
                exact_value(&conversion),
                &expected.rational(),
                "{value:?} from {source_unit:?} to {target_unit:?}"
            );
        }
    }
    // 1 X is 5163/20 K: 2 + 3 = 5 degF, then 5 * 5/9 + 45967/180.
    let (conversion, _) = completed(&quantity(&graph, X, 1, 1), &declared(&graph, K), UNLIMITED);
    assert_eq!(exact_value(&conversion), &ratio(5163, 20));
    let (conversion, _) = completed(
        &quantity(&graph, K, 5163, 20),
        &declared(&graph, X),
        UNLIMITED,
    );
    assert_eq!(exact_value(&conversion), &ratio(1, 1));
}

#[trace("TC-201", "FR-007-AC-21")]
#[test]
fn tc_201_exact_conversion_refuses_incompatible_dimensions_before_any_charge() {
    let graph = graph();
    let one = |unit| quantity(&graph, unit, 1, 1);
    let square = Quantity::new(ratio(4, 1), compound(&graph, &[(M, 2)]));
    let cases: [(Quantity, QuantityUnit); 7] = [
        // U03-style: length to time.
        (one(M), declared(&graph, S)),
        // U09: a base dimension shaped like length under a different semantic-node key.
        (one(M), declared(&graph, ALIKE_ROOT)),
        (one(ALIKE_ROOT), declared(&graph, M)),
        // U22: distinct dimension nodes with equal base-dimension maps.
        (one(N_M), declared(&graph, J)),
        (one(J), declared(&graph, N_M)),
        // Compound 4 m^2 to the length unit m, and 1 m to the compound m^2.
        (square, declared(&graph, M)),
        (one(M), compound(&graph, &[(M, 2)])),
    ];
    for (source, unit) in &cases {
        let (result, meter) = convert(source, unit, UNLIMITED);
        let error = result.expect_err("incompatible dimensions are ill-typed");
        assert_eq!(error.cause, IllTypedCause::IncompatibleDimensions);
        assert_no_charge(&meter);
    }
    // The refusal precedes the identity read even under a zero budget.
    let (result, meter) = convert(&one(M), &declared(&graph, S), limits(0, 0, 0, 0, 0, 0));
    assert_eq!(
        result.unwrap_err().cause,
        IllTypedCause::IncompatibleDimensions
    );
    assert_no_charge(&meter);
}

#[trace("TC-201", "FR-007-AC-22")]
#[test]
fn tc_201_exact_conversion_admits_affine_source_and_target_units() {
    let graph = graph();
    let (conversion, _) = completed(
        &quantity(&graph, DEGC, 0, 1),
        &declared(&graph, K),
        UNLIMITED,
    );
    assert_eq!(exact_value(&conversion), &ratio(5463, 20));
    let (conversion, _) = completed(
        &quantity(&graph, U2, 1, 1),
        &declared(&graph, U1),
        UNLIMITED,
    );
    assert_eq!(exact_value(&conversion), &ratio(-9, 1));
    // An affine target: 5463/20 K is 0 degC.
    let (conversion, _) = completed(
        &quantity(&graph, K, 5463, 20),
        &declared(&graph, DEGC),
        UNLIMITED,
    );
    assert_eq!(exact_value(&conversion), &ratio(0, 1));
    // An affine source and an affine target of the same dimension: 32 degF is 0 degC.
    let (conversion, _) = completed(
        &quantity(&graph, DEGF, 32, 1),
        &declared(&graph, DEGC),
        UNLIMITED,
    );
    assert_eq!(exact_value(&conversion), &ratio(0, 1));
}

#[trace("TC-201", "FR-007-AC-23")]
#[test]
fn tc_201_exact_conversion_charges_one_read_then_edges_then_events_then_target_then_retain() {
    let graph = graph();

    // U10: 100 cm to m under the exact tuple returns 1, largest amount 8 at the multiply event.
    let source = quantity(&graph, CM, 100, 1);
    let metre = declared(&graph, M);
    let (conversion, meter) = completed(&source, &metre, limits(8, 0, 1, 1, 6, 1));
    assert_eq!(exact_value(&conversion), &ratio(1, 1));
    assert_eq!(meter.admitted_charges(), schedule(1, 2));
    assert_consumed(
        &meter,
        [
            (LimitKind::IntegerBits, 8),
            (LimitKind::DecimalDigits, 0),
            (LimitKind::UnitEdges, 1),
            (LimitKind::ValueOccurrences, 1),
            (LimitKind::WorkUnits, 6),
            (LimitKind::ResultUnits, 1),
        ],
    );
    // One bit fewer denies the multiply event at amount 8 after the read of 7 bits.
    let (denied, _) = incomplete(&source, &metre, limits(7, 0, 1, 1, 6, 1));
    assert_eq!(
        denied,
        record(
            LimitKind::IntegerBits,
            7,
            7,
            8,
            ChargePoint::UnitRationalArithmetic
        )
    );

    // U15 first tuple: 1 in to cm is 127/50 after one read, two edges and four events whose
    // amounts are 14, 15, 15 and 14.
    let inch = quantity(&graph, IN, 1, 1);
    let centimetre = declared(&graph, CM);
    let (conversion, meter) = completed(&inch, &centimetre, limits(15, 0, 2, 1, 9, 1));
    assert_eq!(exact_value(&conversion), &ratio(127, 50));
    assert_eq!(meter.admitted_charges(), schedule(2, 4));
    assert_consumed(
        &meter,
        [
            (LimitKind::IntegerBits, 15),
            (LimitKind::DecimalDigits, 0),
            (LimitKind::UnitEdges, 2),
            (LimitKind::ValueOccurrences, 1),
            (LimitKind::WorkUnits, 9),
            (LimitKind::ResultUnits, 1),
        ],
    );
    // The first event is amount 14 and the second 15: the denials name each in turn.
    let (denied, _) = incomplete(&inch, &centimetre, limits(13, 0, 2, 1, 9, 1));
    assert_eq!(
        denied,
        record(
            LimitKind::IntegerBits,
            13,
            1,
            14,
            ChargePoint::UnitRationalArithmetic
        )
    );
    let (denied, _) = incomplete(&inch, &centimetre, limits(14, 0, 2, 1, 9, 1));
    assert_eq!(
        denied,
        record(
            LimitKind::IntegerBits,
            14,
            14,
            15,
            ChargePoint::UnitRationalArithmetic
        )
    );

    // U22 first tuple: compound 4 m^2 to cm2 is 40000 after one read, one reverse edge and its
    // two events, whose largest amount is 17 at the divide.
    let square = Quantity::new(ratio(4, 1), compound(&graph, &[(M, 2)]));
    let (conversion, meter) = completed(&square, &declared(&graph, CM2), limits(17, 0, 1, 1, 6, 1));
    assert_eq!(exact_value(&conversion), &ratio(40000, 1));
    assert_eq!(meter.admitted_charges(), schedule(1, 2));
    assert_consumed(
        &meter,
        [
            (LimitKind::IntegerBits, 17),
            (LimitKind::DecimalDigits, 0),
            (LimitKind::UnitEdges, 1),
            (LimitKind::ValueOccurrences, 1),
            (LimitKind::WorkUnits, 6),
            (LimitKind::ResultUnits, 1),
        ],
    );

    // U22 m2: one read, target admission and retention, no edge and no event.
    let (conversion, meter) = completed(&square, &declared(&graph, M2), limits(3, 0, 0, 1, 3, 1));
    assert_eq!(exact_value(&conversion), &ratio(4, 1));
    assert_eq!(meter.admitted_charges(), schedule(0, 0));
    assert_consumed(
        &meter,
        [
            (LimitKind::IntegerBits, 3),
            (LimitKind::DecimalDigits, 0),
            (LimitKind::UnitEdges, 0),
            (LimitKind::ValueOccurrences, 1),
            (LimitKind::WorkUnits, 3),
            (LimitKind::ResultUnits, 1),
        ],
    );

    // U25: 1 N_m to the compound kg*m^2*s^-2 is exact 1 in that compound unit, then that
    // result to J; each step is one read, target admission and retention with no edge.
    let tuple = limits(1, 0, 0, 1, 3, 1);
    let torque = quantity(&graph, N_M, 1, 1);
    let base_form = compound(&graph, &[(KG, 1), (M, 2), (S, -2)]);
    let (first, meter) = completed(&torque, &base_form, tuple);
    assert_eq!(exact_value(&first), &ratio(1, 1));
    assert_eq!(first.unit(), &base_form);
    assert_eq!(meter.admitted_charges(), schedule(0, 0));
    assert_consumed(
        &meter,
        [
            (LimitKind::IntegerBits, 1),
            (LimitKind::DecimalDigits, 0),
            (LimitKind::UnitEdges, 0),
            (LimitKind::ValueOccurrences, 1),
            (LimitKind::WorkUnits, 3),
            (LimitKind::ResultUnits, 1),
        ],
    );
    let second_source = Quantity::new(exact_value(&first).clone(), first.unit().clone());
    let (second, meter) = completed(&second_source, &declared(&graph, J), tuple);
    assert_eq!(exact_value(&second), &ratio(1, 1));
    assert_eq!(second.unit(), &declared(&graph, J));
    assert_eq!(meter.admitted_charges(), schedule(0, 0));
    assert_eq!(meter.consumed(LimitKind::WorkUnits), 3);

    // U26 first tuple: 1 u2 to u1 is -9 after one read, three edges (u2 to u1, u1 to K, K to
    // u1) and six events, with no common-ancestor shortcut.
    let (conversion, meter) = completed(
        &quantity(&graph, U2, 1, 1),
        &declared(&graph, U1),
        limits(6, 0, 3, 1, 12, 1),
    );
    assert_eq!(exact_value(&conversion), &ratio(-9, 1));
    assert_eq!(meter.admitted_charges(), schedule(3, 6));
    assert_eq!(meter.consumed(LimitKind::UnitEdges), 3);
    assert_eq!(meter.consumed(LimitKind::WorkUnits), 12);
    assert_eq!(meter.consumed(LimitKind::ResultUnits), 1);
}

#[trace("TC-201", "FR-007-AC-23")]
#[test]
fn tc_201_exact_target_admission_carries_no_size_amount() {
    // `unit.target-domain` of an unbounded exact rational is a work unit with no size amount:
    // a zero decimal-digit budget and a zero scale-expansion budget never deny it, and it
    // raises no size counter above the rational events' high-water marks.
    let graph = graph();
    let (_, meter) = completed(
        &quantity(&graph, CM, 100, 1),
        &declared(&graph, M),
        limits(8, 0, 1, 1, 6, 1),
    );
    assert_eq!(meter.consumed(LimitKind::DecimalDigits), 0);
    assert_eq!(meter.consumed(LimitKind::ScaleExpansion), 0);
    assert_eq!(meter.consumed(LimitKind::IntegerBits), 8);
    assert_eq!(meter.consumed(LimitKind::ValueOccurrences), 1);
}

#[trace("TC-201", "FR-007-AC-24")]
#[test]
fn tc_201_affine_arithmetic_is_refused_with_zero_charges() {
    let graph = graph();
    let degc = |value| quantity(&graph, DEGC, value, 1);
    let (one, two) = (degc(1), degc(2));
    let exponent = int(2);
    // U05: Add, Subtract, Multiply and Divide of 1 degC with 2 degC, and 1 degC squared.
    let operations = [
        QuantityOperation::Add(&one, &two),
        QuantityOperation::Subtract(&one, &two),
        QuantityOperation::Multiply(&one, &two),
        QuantityOperation::Divide(&one, &two),
        QuantityOperation::Power(&one, &exponent),
    ];
    // U20: `u1` and `u3` are affine by composed offset.
    let (u1_one, u1_two) = (quantity(&graph, U1, 1, 1), quantity(&graph, U1, 2, 1));
    let (u3, metre) = (quantity(&graph, U3, 1, 1), quantity(&graph, M, 1, 1));
    let u20 = [
        QuantityOperation::Add(&u1_one, &u1_two),
        QuantityOperation::Multiply(&u3, &metre),
    ];
    // The affine operand on the right: `1 m * 1 degC` is refused, and `1 K + 1 degC` is refused
    // as affine arithmetic, not as distinct units. (Extra evidence beyond the AC's left forms.)
    let (kelvin, one_metre, one_degc) = (
        quantity(&graph, K, 1, 1),
        quantity(&graph, M, 1, 1),
        quantity(&graph, DEGC, 1, 1),
    );
    let right = [
        QuantityOperation::Multiply(&one_metre, &one_degc),
        QuantityOperation::Divide(&one_metre, &one_degc),
        QuantityOperation::Add(&kelvin, &one_degc),
        QuantityOperation::Subtract(&kelvin, &one_degc),
    ];
    for operation in operations.into_iter().chain(u20).chain(right) {
        for scalar_limits in [UNLIMITED, limits(0, 0, 0, 0, 0, 0)] {
            let mut meter = Meter::new(scalar_limits);
            let error = evaluate_quantity(operation, &mut meter).unwrap_err();
            assert_eq!(error.cause, IllTypedCause::AffineUnitArithmetic);
            assert_no_charge(&meter);
        }
    }

    // `u2`'s offsets compose to zero, so it is not affine: 1 u2 + 2 u2 is exact 3 u2.
    let (u2_one, u2_two) = (quantity(&graph, U2, 1, 1), quantity(&graph, U2, 2, 1));
    let mut meter = Meter::new(limits(4, 0, 0, 2, 5, 1));
    let sum = evaluate_quantity(QuantityOperation::Add(&u2_one, &u2_two), &mut meter)
        .unwrap()
        .completed()
        .expect("a non-affine sum completes");
    assert_eq!(sum, quantity(&graph, U2, 3, 1));
    assert!(!meter.admitted_charges().contains(&ChargePoint::UnitEdge));
}

#[trace("TC-201", "FR-007-AC-25")]
#[test]
fn tc_201_denied_charge_is_incomplete_with_the_denied_fields_and_no_partial_quantity() {
    let graph = graph();

    // The first U10 tuple with `result_units` lowered to 0 denies `unit.result-retain`.
    let (denied, meter) = incomplete(
        &quantity(&graph, CM, 100, 1),
        &declared(&graph, M),
        limits(8, 0, 1, 1, 6, 0),
    );
    assert_eq!(
        denied,
        record(
            LimitKind::ResultUnits,
            0,
            0,
            1,
            ChargePoint::UnitResultRetain
        )
    );
    assert_eq!(
        meter.admitted_charges().last(),
        Some(&ChargePoint::UnitTargetDomain)
    );

    // The second U15 tuple: both edges are charged before the first rational event.
    let (denied, meter) = incomplete(
        &quantity(&graph, IN, 1, 1),
        &declared(&graph, CM),
        limits(7, 0, 1, 1, 9, 1),
    );
    assert_eq!(
        denied,
        record(LimitKind::UnitEdges, 1, 1, 2, ChargePoint::UnitEdge)
    );
    assert_eq!(
        meter.admitted_charges(),
        [ChargePoint::UnitIdentityRead, ChargePoint::UnitEdge]
    );

    // The second U22 tuple: denied at the divide event after the subtract event's 5.
    let square = Quantity::new(ratio(4, 1), compound(&graph, &[(M, 2)]));
    let (denied, meter) = incomplete(&square, &declared(&graph, CM2), limits(16, 0, 1, 1, 6, 1));
    assert_eq!(
        denied,
        record(
            LimitKind::IntegerBits,
            16,
            5,
            17,
            ChargePoint::UnitRationalArithmetic
        )
    );
    assert_eq!(
        meter.admitted_charges(),
        [
            ChargePoint::UnitIdentityRead,
            ChargePoint::UnitEdge,
            ChargePoint::UnitRationalArithmetic
        ]
    );

    // The second U26 tuple: no common-ancestor shortcut, so three edges are needed.
    let (denied, meter) = incomplete(
        &quantity(&graph, U2, 1, 1),
        &declared(&graph, U1),
        limits(6, 0, 2, 1, 12, 1),
    );
    assert_eq!(
        denied,
        record(LimitKind::UnitEdges, 2, 2, 3, ChargePoint::UnitEdge)
    );
    assert_eq!(
        meter.admitted_charges(),
        [
            ChargePoint::UnitIdentityRead,
            ChargePoint::UnitEdge,
            ChargePoint::UnitEdge
        ]
    );
}
