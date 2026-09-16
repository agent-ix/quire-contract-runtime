// SPDX-License-Identifier: AGPL-3.0-or-later
//! FR-142 dimensions, declared units, their admitted unit graph and normalized
//! compound units.
//!
//! Base dimensions and units are I04 nominal nodes whose keys the compiler
//! computes and admits. A dimension is a sorted map from base-dimension key to
//! a nonzero mathematical exponent. The units of one dimension node form an
//! acyclic graph with exactly one targetless canonical root; each edge maps
//! `target_value = scale × source_value + offset` exactly. The runtime checks
//! that topology over the admitted keys; owner joins and stale keys stay
//! compiler checks.

use alloc::collections::{BTreeMap, BTreeSet};
use alloc::vec::Vec;
use core::fmt;

use super::integer::Integer;
use super::node::{check_terms, refuse, InvalidSemanticGraph, NodeKey, SemanticGraphCause};
use super::rational::Rational;

/// Evaluator domain of a compound-unit value.
pub const COMPOUND_UNIT_DOMAIN: &str = "quire.value.compound-unit/v1";

/// A normalized dimension: base-dimension node keys to nonzero exponents.
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct Dimension(BTreeMap<NodeKey, Integer>);

impl Dimension {
    /// The empty (dimensionless) map.
    pub fn dimensionless() -> Self {
        Self::default()
    }

    /// Whether every exponent is absent.
    pub fn is_dimensionless(&self) -> bool {
        self.0.is_empty()
    }

    /// Ascending `(base dimension, exponent)` terms; no exponent is zero.
    pub fn exponents(&self) -> impl Iterator<Item = (NodeKey, &Integer)> {
        self.0.iter().map(|(key, exponent)| (*key, exponent))
    }

    /// Add exponents.
    pub fn multiply(&self, other: &Self) -> Self {
        Self(combine(&self.0, &other.0, Integer::add))
    }

    /// Subtract exponents.
    pub fn divide(&self, other: &Self) -> Self {
        Self(combine(&self.0, &other.0, Integer::sub))
    }

    /// Multiply every exponent by `exponent`.
    pub fn power(&self, exponent: &Integer) -> Self {
        Self(scale_exponents(&self.0, exponent))
    }
}

/// Combine two exponent maps key by key and drop zero exponents.
fn combine(
    left: &BTreeMap<NodeKey, Integer>,
    right: &BTreeMap<NodeKey, Integer>,
    operation: fn(&Integer, &Integer) -> Integer,
) -> BTreeMap<NodeKey, Integer> {
    let keys: BTreeSet<_> = left.keys().chain(right.keys()).copied().collect();
    let zero = Integer::zero();
    keys.into_iter()
        .map(|key| {
            let exponent = operation(
                left.get(&key).unwrap_or(&zero),
                right.get(&key).unwrap_or(&zero),
            );
            (key, exponent)
        })
        .filter(|(_, exponent)| !exponent.is_zero())
        .collect()
}

fn scale_exponents(
    terms: &BTreeMap<NodeKey, Integer>,
    exponent: &Integer,
) -> BTreeMap<NodeKey, Integer> {
    terms
        .iter()
        .map(|(key, value)| (*key, value.mul(exponent)))
        .filter(|(_, value)| !value.is_zero())
        .collect()
}

/// One admitted `quire.unit-node/v1` declaration, referenced by node keys.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UnitDeclaration {
    /// The unit's dimension node key.
    pub dimension: NodeKey,
    /// The target unit key, or `None` for a canonical root.
    pub target: Option<NodeKey>,
    /// Exact nonzero scale of the edge to `target`.
    pub scale: Rational,
    /// Exact offset of the edge to `target`.
    pub offset: Rational,
}

impl UnitDeclaration {
    /// Refuse a zero scale, then a non-identity root.
    fn check_semantics(&self) -> Result<UnitEdge, SemanticGraphCause> {
        if self.scale.is_zero() {
            return Err(SemanticGraphCause::ZeroScale);
        }
        let identity =
            self.scale == Rational::from_integer(Integer::one()) && self.offset.is_zero();
        if self.target.is_none() && !identity {
            return Err(SemanticGraphCause::NonIdentityRoot);
        }
        Ok(UnitEdge {
            scale: self.scale.clone(),
            offset: self.offset.clone(),
        })
    }
}

/// One exact affine edge `target = scale × source + offset`.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct UnitEdge {
    scale: Rational,
    offset: Rational,
}

impl UnitEdge {
    /// Nonzero exact scale.
    pub fn scale(&self) -> &Rational {
        &self.scale
    }

    /// Exact offset.
    pub fn offset(&self) -> &Rational {
        &self.offset
    }
}

/// An admitted declared unit and its exact path to the canonical root.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Unit {
    key: NodeKey,
    dimension_node: NodeKey,
    dimension: Dimension,
    root: NodeKey,
    path: Vec<UnitEdge>,
    canonical: UnitEdge,
}

impl Unit {
    /// The unit node key.
    pub fn key(&self) -> NodeKey {
        self.key
    }

    /// The unit's dimension node key.
    pub fn dimension_node(&self) -> NodeKey {
        self.dimension_node
    }

    /// The normalized dimension map.
    pub fn dimension(&self) -> &Dimension {
        &self.dimension
    }

    /// The canonical root unit of the unit's dimension node.
    pub fn root(&self) -> NodeKey {
        self.root
    }

    /// Edges from this unit to the root, in source-to-root order.
    pub fn path(&self) -> &[UnitEdge] {
        &self.path
    }

    /// The composed exact mapping to the canonical root.
    pub fn canonical(&self) -> &UnitEdge {
        &self.canonical
    }

    /// Whether this unit is affine: a nonzero-offset point unit that admits
    /// only conversion and comparison. The composed canonical offset
    /// decides, so a zero-offset unit that targets an affine unit is affine.
    pub fn is_affine(&self) -> bool {
        !self.canonical.offset.is_zero()
    }
}

/// An admitted closed set of dimension and unit nodes.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct UnitGraph {
    dimensions: BTreeMap<NodeKey, Dimension>,
    units: BTreeMap<NodeKey, Unit>,
}

/// A unit node after its per-node semantic checks.
struct AdmittedUnit {
    dimension: NodeKey,
    target: Option<NodeKey>,
    edge: UnitEdge,
}

impl UnitGraph {
    /// Check the topology of compiler-admitted dimension and unit nodes.
    ///
    /// `dimensions` pairs each dimension key with its retained
    /// `(base dimension, exponent)` terms (empty for a base dimension). Each
    /// node is first checked for semantic well-formedness and the node set for
    /// repeated keys. The graph is then checked for unknown or derived
    /// dimension terms, unknown dimensions, unknown and cross-dimension
    /// targets, per-dimension root count and target cycles. Every refusal is
    /// `invalid_semantic_graph`; the typed cause names the first failed check.
    pub fn admit(
        dimensions: impl IntoIterator<Item = (NodeKey, Vec<(NodeKey, Integer)>)>,
        units: impl IntoIterator<Item = (NodeKey, UnitDeclaration)>,
    ) -> Result<Self, InvalidSemanticGraph> {
        let mut keys = BTreeSet::new();
        let mut admitted_dimensions = BTreeMap::new();
        for (key, terms) in dimensions {
            check_terms(&terms).map_err(refuse)?;
            if !keys.insert(key) {
                return Err(refuse(SemanticGraphCause::DuplicateNode));
            }
            admitted_dimensions.insert(key, terms);
        }
        let mut admitted_units = BTreeMap::new();
        for (key, declaration) in units {
            let edge = declaration.check_semantics().map_err(refuse)?;
            if !keys.insert(key) {
                return Err(refuse(SemanticGraphCause::DuplicateNode));
            }
            admitted_units.insert(
                key,
                AdmittedUnit {
                    dimension: declaration.dimension,
                    target: declaration.target,
                    edge,
                },
            );
        }
        let dimensions = dimension_maps(&admitted_dimensions)?;
        let units = unit_paths(&admitted_units, &dimensions)?;
        Ok(Self { dimensions, units })
    }

    /// The normalized map of an admitted dimension node.
    pub fn dimension(&self, key: NodeKey) -> Option<&Dimension> {
        self.dimensions.get(&key)
    }

    /// An admitted unit.
    pub fn unit(&self, key: NodeKey) -> Option<&Unit> {
        self.units.get(&key)
    }

    /// Construct a compound unit: every term must name an admitted canonical
    /// root unit with a nonzero exponent, strictly ascending by key.
    pub fn compound_unit(
        &self,
        terms: &[(NodeKey, Integer)],
    ) -> Result<CompoundUnit, InvalidCompoundUnit> {
        check_terms(terms).map_err(|cause| InvalidCompoundUnit {
            cause: match cause {
                SemanticGraphCause::ZeroExponent => CompoundUnitCause::ZeroExponent,
                SemanticGraphCause::DuplicateTerm => CompoundUnitCause::DuplicateTerm,
                // `check_terms` raises only the three term causes.
                _ => CompoundUnitCause::UnsortedTerms,
            },
        })?;
        let mut dimension = Dimension::dimensionless();
        for (key, exponent) in terms {
            let root = self.units.get(key).filter(|unit| unit.root == *key).ok_or(
                InvalidCompoundUnit {
                    cause: CompoundUnitCause::NotRootUnit,
                },
            )?;
            dimension = dimension.multiply(&root.dimension.power(exponent));
        }
        Ok(CompoundUnit {
            terms: terms.iter().cloned().collect(),
            dimension,
        })
    }
}

/// A base dimension maps to itself; a derived dimension to its base terms.
fn dimension_maps(
    dimensions: &BTreeMap<NodeKey, Vec<(NodeKey, Integer)>>,
) -> Result<BTreeMap<NodeKey, Dimension>, InvalidSemanticGraph> {
    dimensions
        .iter()
        .map(|(key, terms)| {
            if terms.is_empty() {
                return Ok((*key, Dimension([(*key, Integer::one())].into())));
            }
            for (term, _) in terms {
                match dimensions.get(term) {
                    None => return Err(refuse(SemanticGraphCause::UnknownDimension)),
                    Some(base) if !base.is_empty() => {
                        return Err(refuse(SemanticGraphCause::NonBaseDimensionTerm));
                    }
                    Some(_) => {}
                }
            }
            Ok((*key, Dimension(terms.iter().cloned().collect())))
        })
        .collect()
}

/// Check unit graph topology and compose each unit's exact path to its root.
fn unit_paths(
    units: &BTreeMap<NodeKey, AdmittedUnit>,
    dimensions: &BTreeMap<NodeKey, Dimension>,
) -> Result<BTreeMap<NodeKey, Unit>, InvalidSemanticGraph> {
    if units
        .values()
        .any(|unit| !dimensions.contains_key(&unit.dimension))
    {
        return Err(refuse(SemanticGraphCause::UnknownDimension));
    }
    let targets = || units.values().filter_map(|unit| Some((unit, unit.target?)));
    if targets().any(|(_, target)| !units.contains_key(&target)) {
        return Err(refuse(SemanticGraphCause::UnknownTarget));
    }
    if targets().any(|(unit, target)| {
        units
            .get(&target)
            .is_some_and(|target| target.dimension != unit.dimension)
    }) {
        return Err(refuse(SemanticGraphCause::CrossDimensionTarget));
    }
    let mut roots: BTreeMap<NodeKey, Vec<NodeKey>> = BTreeMap::new();
    for (key, unit) in units {
        let slot = roots.entry(unit.dimension).or_default();
        if unit.target.is_none() {
            slot.push(*key);
        }
    }
    if roots.values().any(Vec::is_empty) {
        return Err(refuse(SemanticGraphCause::MissingRoot));
    }
    if roots.values().any(|found| found.len() > 1) {
        return Err(refuse(SemanticGraphCause::DuplicateRoot));
    }
    units
        .iter()
        .map(|(key, unit)| {
            let mut path = Vec::new();
            let mut canonical = UnitEdge {
                scale: Rational::from_integer(Integer::one()),
                offset: Rational::from_integer(Integer::zero()),
            };
            let mut current = (*key, unit);
            while let Some(target) = current.1.target {
                if path.len() >= units.len() {
                    return Err(refuse(SemanticGraphCause::TargetCycle));
                }
                let edge = current.1.edge.clone();
                canonical = UnitEdge {
                    scale: edge.scale.mul(&canonical.scale),
                    offset: edge.scale.mul(&canonical.offset).add(&edge.offset),
                };
                path.push(edge);
                let next = units
                    .get(&target)
                    .ok_or(refuse(SemanticGraphCause::UnknownTarget))?;
                current = (target, next);
            }
            let dimension = dimensions
                .get(&unit.dimension)
                .cloned()
                .ok_or(refuse(SemanticGraphCause::UnknownDimension))?;
            Ok((
                *key,
                Unit {
                    key: *key,
                    dimension_node: unit.dimension,
                    dimension,
                    root: current.0,
                    path,
                    canonical,
                },
            ))
        })
        .collect()
}

/// Why a compound unit was refused at construction.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct InvalidCompoundUnit {
    /// The typed reason.
    pub cause: CompoundUnitCause,
}

impl fmt::Display for InvalidCompoundUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid compound unit: {:?}", self.cause)
    }
}

/// The check that refused a compound unit.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CompoundUnitCause {
    /// The preimage does not satisfy `value-compound-unit.schema.json`; raised
    /// by the compiler reader only.
    NonCanonicalPreimage,
    /// A term exponent is zero.
    ZeroExponent,
    /// A root unit appears twice.
    DuplicateTerm,
    /// Terms are not strictly ascending by node key.
    UnsortedTerms,
    /// A term does not name an admitted canonical root unit.
    NotRootUnit,
}

/// A normalized compound unit: canonical root-unit keys to nonzero exponents.
/// The empty map is the sole dimensionless unit. Structural equality is value
/// identity.
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct CompoundUnit {
    terms: BTreeMap<NodeKey, Integer>,
    dimension: Dimension,
}

impl CompoundUnit {
    /// The dimensionless unit.
    pub fn dimensionless() -> Self {
        Self::default()
    }

    /// The single-term compound unit `root^1` of a declared unit's root.
    pub(crate) fn of_root(unit: &Unit) -> Self {
        Self {
            terms: [(unit.root, Integer::one())].into(),
            dimension: unit.dimension.clone(),
        }
    }

    /// Ascending `(root unit, exponent)` terms.
    pub fn terms(&self) -> impl Iterator<Item = (NodeKey, &Integer)> {
        self.terms.iter().map(|(key, exponent)| (*key, exponent))
    }

    /// The normalized dimension map.
    pub fn dimension(&self) -> &Dimension {
        &self.dimension
    }

    pub(crate) fn multiply(&self, other: &Self) -> Self {
        Self {
            terms: combine(&self.terms, &other.terms, Integer::add),
            dimension: self.dimension.multiply(&other.dimension),
        }
    }

    pub(crate) fn divide(&self, other: &Self) -> Self {
        Self {
            terms: combine(&self.terms, &other.terms, Integer::sub),
            dimension: self.dimension.divide(&other.dimension),
        }
    }

    pub(crate) fn power(&self, exponent: &Integer) -> Self {
        Self {
            terms: scale_exponents(&self.terms, exponent),
            dimension: self.dimension.power(exponent),
        }
    }
}
