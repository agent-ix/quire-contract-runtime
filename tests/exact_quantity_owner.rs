//! The runtime consumes QSV's identified quantity handoff for the two-step U25 conversion.
#![cfg(feature = "exact")]

use std::collections::BTreeMap;

use ix_trace_rs::trace;
use quire_exact::{
    Integer, LimitKind, Meter, NodeKey, Outcome, Quantity, Rational, ScalarLimits, UnitId,
};
use quire_semantic_value::quantity::{
    convert_quantity, QuantityTarget, QuantityUnit, UnitScope, UnitTable,
};
use quire_semantic_value::unit::{DimensionNode, NominalDeclaration, UnitGraph, UnitNode};

fn key(byte: u8) -> NodeKey {
    NodeKey::from_digest([byte; 32])
}

fn rational(value: i64) -> Rational {
    Rational::from_integer(Integer::from(value))
}

fn meter() -> Meter {
    Meter::new(ScalarLimits {
        integer_bits: 1024,
        decimal_digits: 1024,
        scale_expansion: 1024,
        text_input_bytes: 1024,
        text_scalars: 1024,
        normalized_scalars: 1024,
        unit_edges: 1024,
        value_occurrences: 1024,
        work_units: 1024,
        result_units: 1024,
    })
}

fn energy_graph() -> UnitGraph {
    let dimensions = BTreeMap::from([
        (
            key(1),
            DimensionNode::checked(vec![], NominalDeclaration::default()).unwrap(),
        ),
        (
            key(2),
            DimensionNode::checked(vec![], NominalDeclaration::default()).unwrap(),
        ),
        (
            key(3),
            DimensionNode::checked(vec![], NominalDeclaration::default()).unwrap(),
        ),
        (
            key(4),
            DimensionNode::checked(
                vec![
                    ([1; 32], Integer::from(1_i64)),
                    ([2; 32], Integer::from(2_i64)),
                    ([3; 32], Integer::from(-2_i64)),
                ],
                NominalDeclaration::default(),
            )
            .unwrap(),
        ),
    ]);
    let unit = |dimension: u8| {
        UnitNode::checked(
            [dimension; 32],
            None,
            rational(1),
            rational(0),
            NominalDeclaration::default(),
        )
        .unwrap()
    };
    let units = BTreeMap::from([
        (key(11), unit(1)),
        (key(12), unit(2)),
        (key(13), unit(3)),
        (key(14), unit(4)),
    ]);
    UnitGraph::from_checked_nodes(&dimensions, &units).unwrap()
}

/// Trace: FR-275-AC-23
#[trace("TC-201", "FR-275-AC-23")]
#[test]
fn tc_201_qsv_handoff_resolves_compound_unit_for_second_conversion() {
    let graph = energy_graph();
    let table = UnitTable::declared(&graph);
    let joule = Quantity::new(rational(1), UnitId::declared(key(14)));
    let compound = QuantityUnit::Compound(
        graph
            .compound_unit(&[
                ([11; 32], Integer::from(1_i64)),
                ([12; 32], Integer::from(2_i64)),
                ([13; 32], Integer::from(-2_i64)),
            ])
            .unwrap(),
    );
    let mut first = meter();
    let Outcome::Completed(converted) = convert_quantity(
        table.resolve(&joule).unwrap(),
        &compound,
        &QuantityTarget::Exact,
        &mut first,
    )
    .unwrap() else {
        panic!("first conversion completes");
    };
    let before_handoff = first.admission_count();
    let (quantity, identified) = converted.identified_quantity().unwrap();
    assert_eq!(first.admission_count(), before_handoff);
    assert_eq!(quantity.magnitude(), &rational(1));
    assert_eq!(quantity.unit(), identified.id());
    assert_ne!(quantity.unit(), UnitId::declared(key(14)));

    let mut scope = UnitScope::new(&table);
    assert_eq!(scope.form(identified), quantity.unit());
    let mut second = meter();
    let Outcome::Completed(back) = convert_quantity(
        scope.resolve(&quantity).unwrap(),
        table.get(UnitId::declared(key(14))).unwrap(),
        &QuantityTarget::Exact,
        &mut second,
    )
    .unwrap() else {
        panic!("second conversion completes");
    };
    let (joules, _) = back.identified_quantity().unwrap();
    assert_eq!(joules.magnitude(), &rational(1));
    assert_eq!(joules.unit(), UnitId::declared(key(14)));
    for meter in [&first, &second] {
        assert_eq!(meter.admission_count(), 3);
        assert_eq!(meter.consumed(LimitKind::UnitEdges), 0);
    }

    let exact_module = include_str!("../src/exact/mod.rs");
    assert!(!exact_module.contains("mod quantity;"));
    assert!(!exact_module.contains("mod unit;"));
}
