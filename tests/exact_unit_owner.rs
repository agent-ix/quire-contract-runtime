//! RT acceptance of QSV's checked unit graph and normalized compound units.
#![cfg(feature = "exact")]

use std::collections::BTreeMap;

use ix_trace_rs::trace;
use quire_exact::{Integer, NodeKey, Rational};
use quire_semantic_value::semantic_node::SemanticGraphCause;
use quire_semantic_value::unit::{
    CompoundUnitCause, DimensionNode, NominalDeclaration, UnitGraph, UnitNode,
};

fn key(byte: u8) -> NodeKey {
    NodeKey::from_digest([byte; 32])
}

fn int(value: i64) -> Integer {
    Integer::from(value)
}

fn rational(value: i64) -> Rational {
    Rational::from_integer(int(value))
}

fn dimension(terms: &[(u8, i64)]) -> DimensionNode {
    DimensionNode::checked(
        terms
            .iter()
            .map(|(key, exponent)| ([*key; 32], int(*exponent)))
            .collect(),
        NominalDeclaration::default(),
    )
    .unwrap()
}

fn unit(dimension: u8, target: Option<u8>) -> UnitNode {
    UnitNode::checked(
        [dimension; 32],
        target.map(|byte| [byte; 32]),
        rational(1),
        rational(0),
        NominalDeclaration::default(),
    )
    .unwrap()
}

/// Trace: FR-012-AC-4, FR-011-AC-7
#[trace("TC-033", "FR-012-AC-4", "FR-011-AC-7")]
#[test]
fn tc_033_qsv_checked_nodes_and_topology_refuse_in_declared_order() {
    let term = |id: u8, exponent: i64| ([id; 32], int(exponent));
    for (terms, cause) in [
        (
            vec![term(2, 1), term(1, 0), term(1, 1)],
            SemanticGraphCause::ZeroExponent,
        ),
        (
            vec![term(1, 1), term(1, 1)],
            SemanticGraphCause::DuplicateTerm,
        ),
        (
            vec![term(2, 1), term(1, 1)],
            SemanticGraphCause::UnsortedTerms,
        ),
    ] {
        assert_eq!(
            DimensionNode::checked(terms, NominalDeclaration::default()),
            Err(cause)
        );
    }
    assert_eq!(
        UnitNode::checked(
            [1; 32],
            None,
            rational(0),
            rational(5),
            NominalDeclaration::default(),
        ),
        Err(SemanticGraphCause::ZeroScale)
    );
    assert_eq!(
        UnitNode::checked(
            [1; 32],
            None,
            rational(2),
            rational(0),
            NominalDeclaration::default(),
        ),
        Err(SemanticGraphCause::NonIdentityRoot)
    );

    let topology = [
        (
            BTreeMap::from([(key(1), dimension(&[])), (key(2), dimension(&[(9, 1)]))]),
            BTreeMap::new(),
            SemanticGraphCause::UnknownDimension,
        ),
        (
            BTreeMap::from([
                (key(1), dimension(&[])),
                (key(2), dimension(&[(1, 1)])),
                (key(3), dimension(&[(2, 1)])),
            ]),
            BTreeMap::new(),
            SemanticGraphCause::NonBaseDimensionTerm,
        ),
        (
            BTreeMap::from([(key(1), dimension(&[]))]),
            BTreeMap::from([(key(11), unit(1, None)), (key(12), unit(1, Some(99)))]),
            SemanticGraphCause::UnknownTarget,
        ),
        (
            BTreeMap::from([(key(1), dimension(&[])), (key(2), dimension(&[]))]),
            BTreeMap::from([(key(11), unit(1, None)), (key(12), unit(2, Some(11)))]),
            SemanticGraphCause::CrossDimensionTarget,
        ),
        (
            BTreeMap::from([(key(1), dimension(&[]))]),
            BTreeMap::from([(key(11), unit(1, Some(12))), (key(12), unit(1, Some(11)))]),
            SemanticGraphCause::MissingRoot,
        ),
        (
            BTreeMap::from([(key(1), dimension(&[]))]),
            BTreeMap::from([(key(11), unit(1, None)), (key(12), unit(1, None))]),
            SemanticGraphCause::DuplicateRoot,
        ),
        (
            BTreeMap::from([(key(1), dimension(&[]))]),
            BTreeMap::from([
                (key(11), unit(1, None)),
                (key(12), unit(1, Some(13))),
                (key(13), unit(1, Some(12))),
            ]),
            SemanticGraphCause::TargetCycle,
        ),
    ];
    for (dimensions, units, cause) in topology {
        assert_eq!(
            UnitGraph::from_checked_nodes(&dimensions, &units)
                .unwrap_err()
                .cause,
            cause
        );
    }

    // BTreeMap keys are unique at the API boundary, so DuplicateNode has no graph input.
}

/// Trace: FR-012-AC-5, FR-011-AC-7
#[trace("TC-033", "FR-012-AC-5", "FR-011-AC-7")]
#[test]
fn tc_033_qsv_dimension_and_compound_terms_are_canonical_across_graph_order() {
    let dimensions_forward = BTreeMap::from([
        (key(1), dimension(&[])),
        (key(2), dimension(&[])),
        (key(3), dimension(&[(1, 1), (2, -1)])),
    ]);
    let mut dimensions_reverse = BTreeMap::new();
    for (key, node) in dimensions_forward.iter().rev() {
        dimensions_reverse.insert(*key, node.clone());
    }
    let units_forward = BTreeMap::from([(key(11), unit(1, None)), (key(12), unit(2, None))]);
    let mut units_reverse = BTreeMap::new();
    for (key, node) in units_forward.iter().rev() {
        units_reverse.insert(*key, node.clone());
    }
    let forward = UnitGraph::from_checked_nodes(&dimensions_forward, &units_forward).unwrap();
    let reverse = UnitGraph::from_checked_nodes(&dimensions_reverse, &units_reverse).unwrap();
    assert_eq!(forward.dimension(key(3)), reverse.dimension(key(3)));
    let derived: Vec<_> = forward.dimension(key(3)).unwrap().exponents().collect();
    assert_eq!(derived, vec![(key(1), &int(1)), (key(2), &int(-1))]);
    let length = forward.dimension(key(1)).unwrap();
    assert!(length.divide(length).is_dimensionless());

    let terms = [([11; 32], int(2)), ([12; 32], int(-2))];
    let first = forward.compound_unit(&terms).unwrap();
    let second = reverse.compound_unit(&terms).unwrap();
    assert_eq!(first, second);
    assert_eq!(first.id().unwrap(), second.id().unwrap());
    let different = forward
        .compound_unit(&[([11; 32], int(2)), ([12; 32], int(-1))])
        .unwrap();
    assert_ne!(first, different);
    assert_eq!(
        forward
            .compound_unit(&[([12; 32], int(1)), ([11; 32], int(1))])
            .unwrap_err()
            .cause,
        CompoundUnitCause::UnsortedTerms,
    );
    let visited: Vec<_> = first.terms().collect();
    assert_eq!(visited, vec![(key(11), &int(2)), (key(12), &int(-2))]);
    assert!(visited.iter().all(|(_, exponent)| !exponent.is_zero()));
}

/// Trace: FR-007-AC-13
#[trace("TC-035", "FR-007-AC-13")]
#[test]
fn tc_035_qsv_compound_unit_debug_names_every_boxed_field() {
    let graph = UnitGraph::from_checked_nodes(&BTreeMap::new(), &BTreeMap::new()).unwrap();
    let dimensionless = graph.compound_unit(&[]).unwrap();
    assert_eq!(
        format!("{dimensionless:?}"),
        "CompoundUnit { terms: {}, dimension: Dimension({}) }"
    );
}
