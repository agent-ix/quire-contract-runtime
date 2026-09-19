// SPDX-License-Identifier: AGPL-3.0-or-later
//! Shared-corpus agreement support.
//!
//! Every vector body is written once and evaluated twice by [`agree!`]: once
//! with the semantic authority `quire_spec_language::value` in scope and once
//! with the runtime port `quire_contract_runtime::exact` in scope. The two
//! results must have identical `Debug` renderings, which include every value,
//! loss record, typed refusal, incomplete record, admitted charge and consumed
//! counter the body returns. The runtime result is then returned for the
//! QSpec expectation assertions.
//!
//! Compiler-owned work (definition-lock admission, node-key hashing, owner
//! selection) is done once, by the authority, here: node keys come from the
//! authority's preimage hashing and both sides consume the same keys.
#![allow(dead_code, unused_imports)]

use std::collections::BTreeMap;

use quire_spec_language::value as authority;
use serde_json::{json, Value};

/// Evaluate a body against the authority and the runtime, require identical
/// `Debug` renderings, and return the runtime result.
macro_rules! agree {
    ($($body:tt)*) => {{
        let authority = {
            #[allow(unused_imports)]
            use crate::support::qsl_side::*;
            format!("{:?}", { $($body)* })
        };
        let runtime = {
            #[allow(unused_imports)]
            use crate::support::rt_side::*;
            { $($body)* }
        };
        assert_eq!(format!("{:?}", runtime), authority, "runtime disagrees with the authority");
        runtime
    }};
}

/// The decimal spelling of `2^exponent`, a side-neutral input.
pub fn pow2(exponent: u32) -> String {
    (num_bigint::BigInt::from(1_u8) << exponent).to_string()
}

/// Helpers whose source is identical on both sides; only the types differ.
macro_rules! shared_helpers {
    () => {
        pub const UNLIMITED: ScalarLimits = ScalarLimits {
            integer_bits: u64::MAX,
            decimal_digits: u64::MAX,
            scale_expansion: u64::MAX,
            text_input_bytes: u64::MAX,
            text_scalars: u64::MAX,
            normalized_scalars: u64::MAX,
            unit_edges: u64::MAX,
            value_occurrences: u64::MAX,
            work_units: u64::MAX,
            result_units: u64::MAX,
        };

        /// `ScalarLimitsV1` in field order.
        pub fn limits(v: [u64; 10]) -> ScalarLimits {
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

        /// TC-187 `L(i,d,e,o,w,r)`.
        pub fn unit_limits(i: u64, d: u64, e: u64, o: u64, w: u64, r: u64) -> ScalarLimits {
            limits([i, d, 0, 0, 0, 0, e, o, w, r])
        }

        /// TC-187 `L(i,d,e,o,w,r)` from one array.
        pub fn unit_tuple(v: [u64; 6]) -> ScalarLimits {
            unit_limits(v[0], v[1], v[2], v[3], v[4], v[5])
        }

        pub fn int(value: i128) -> Integer {
            Integer::from(value)
        }

        /// A distinct node key from one repeated byte, since the authority
        /// exposes only [`NodeKey::from_hex`] and not the runtime's
        /// `from_bytes` convenience constructor.
        pub fn key(byte: u8) -> NodeKey {
            NodeKey::from_hex(&format!("{byte:02x}").repeat(32)).unwrap()
        }

        /// A distinct universe identity from one repeated byte.
        pub fn universe(byte: u8) -> UniverseIdentity {
            UniverseIdentity::new(&[byte; 8]).unwrap()
        }

        /// A distinct object identity from one repeated byte.
        pub fn object_identity(byte: u8) -> ObjectIdentity {
            ObjectIdentity::new(&[byte; 8]).unwrap()
        }

        /// A terminal reference in `universe` of `object_type`, identified by
        /// `identity`.
        pub fn reference(universe: u8, object_type: NodeKey, identity: u8) -> ObjectReference {
            ObjectReference::new(
                self::universe(universe),
                object_type,
                object_identity(identity),
            )
        }

        pub fn big(spelling: &str) -> Integer {
            spelling.parse().unwrap()
        }

        pub fn dec(coefficient: i64, scale: u32) -> Decimal {
            Decimal::new(Integer::from(coefficient), scale)
        }

        pub fn ratio(numerator: i64, denominator: i64) -> Rational {
            Rational::new(Integer::from(numerator), Integer::from(denominator)).unwrap()
        }

        pub fn whole(value: i64) -> Rational {
            Rational::from_integer(Integer::from(value))
        }

        pub fn decimal_type(
            lower: i64,
            upper: i64,
            min: u64,
            max: u64,
            mode: RoundingMode,
        ) -> DecimalType {
            DecimalType::new(Integer::from(lower), Integer::from(upper), min, max, mode).unwrap()
        }

        pub fn text_type(min: u64, max: u64, profile: TextProfile) -> TextType {
            TextType::new(min, max, profile).unwrap()
        }

        pub fn payload(text: &str) -> TextPayload {
            TextPayload::from_utf8(text.as_bytes()).unwrap()
        }

        pub fn text(text: &str, profile: TextProfile) -> Text {
            admit_text(
                &payload(text),
                &text_type(0, 64, profile),
                &mut Meter::new(UNLIMITED),
            )
            .completed()
            .unwrap()
        }

        /// An admission outcome reduced to the retained profile length.
        pub fn admitted_length(outcome: Outcome<Text>) -> Outcome<u64> {
            match outcome {
                Outcome::Completed(text) => Outcome::Completed(text.length()),
                Outcome::Undefined(reason) => Outcome::Undefined(reason),
                Outcome::Refused(reason) => Outcome::Refused(reason),
                Outcome::Incomplete(record) => Outcome::Incomplete(record),
            }
        }

        /// An outcome with every admitted charge and every consumed counter.
        pub fn metered<T>(
            limits: ScalarLimits,
            run: impl FnOnce(&mut Meter) -> T,
        ) -> (T, Vec<ChargePoint>, Vec<u64>) {
            let mut meter = Meter::new(limits);
            let outcome = run(&mut meter);
            let consumed = LimitKind::ALL
                .iter()
                .map(|kind| meter.consumed(*kind))
                .collect();
            (outcome, meter.admitted_charges().to_vec(), consumed)
        }

        /// An outcome with every admitted charge and no consumed counter: the
        /// agreement shape of a vector whose charge amounts QSpec 7d7943a
        /// derives from operands, which the authority d9d5273 does not yet do.
        pub fn scheduled<T>(
            limits: ScalarLimits,
            run: impl FnOnce(&mut Meter) -> T,
        ) -> (T, Vec<ChargePoint>) {
            let (outcome, charges, _) = metered(limits, run);
            (outcome, charges)
        }

        /// Deny each admitted charge occurrence of an unlimited-or-given run in
        /// turn: `(point, occurrence, outcome, result units consumed)`.
        pub fn denials<T>(
            limits: ScalarLimits,
            run: impl Fn(&mut Meter) -> T,
        ) -> Vec<(ChargePoint, u64, T, u64)> {
            let mut meter = Meter::new(limits);
            let _ = run(&mut meter);
            let charges = meter.admitted_charges().to_vec();
            let mut seen: Vec<ChargePoint> = Vec::new();
            charges
                .into_iter()
                .map(|point| {
                    seen.push(point);
                    let occurrence = seen.iter().filter(|p| **p == point).count() as u64;
                    let mut denied = Meter::new(limits).with_injected_denial(InjectedDenial {
                        point,
                        occurrence: to_occurrence(occurrence),
                    });
                    let outcome = run(&mut denied);
                    (
                        point,
                        occurrence,
                        outcome,
                        denied.consumed(LimitKind::ResultUnits),
                    )
                })
                .collect()
        }

        pub fn f32v(bits: u32) -> IeeeValue {
            IeeeValue::binary32(bits)
        }

        pub fn f64v(bits: u64) -> IeeeValue {
            IeeeValue::binary64(bits)
        }

        /// A pattern of `IeeeWidth::ALL[width]`.
        pub fn ieee(width: usize, bits: u64) -> IeeeValue {
            match IeeeWidth::ALL[width] {
                IeeeWidth::Binary32 => IeeeValue::binary32(u32::try_from(bits).unwrap()),
                IeeeWidth::Binary64 => IeeeValue::binary64(bits),
            }
        }

        pub fn flags(list: &[IeeeFlag]) -> IeeeFlags {
            list.iter().copied().collect()
        }

        /// TC-193 `I(i,o,w,r)`.
        pub fn ieee_limits(i: u64, o: u64, w: u64, r: u64) -> ScalarLimits {
            limits([i, 0, 0, 0, 0, 0, 0, o, w, r])
        }

        /// `Rational[lo, hi; dmin, dmax]` from decimal spellings.
        pub fn rational_type(lo: &str, hi: &str, dmin: &str, dmax: &str) -> RationalDomain {
            RationalDomain::new(
                IntegerInterval::new(big(lo), big(hi)).unwrap(),
                IntegerInterval::new(big(dmin), big(dmax)).unwrap(),
            )
            .unwrap()
        }

        pub fn incomplete(
            limit_kind: LimitKind,
            limit: u64,
            consumed: u64,
            next_charge: Integer,
            charge_point: ChargePoint,
        ) -> Incomplete {
            Incomplete {
                limit_kind,
                limit,
                consumed,
                next_charge,
                charge_point,
            }
        }

        /// The injected-denial or work-unit exhaustion record.
        pub fn work_denied(limit: u64, point: ChargePoint) -> Incomplete {
            incomplete(
                LimitKind::WorkUnits,
                limit,
                limit,
                Integer::from(1_i64),
                point,
            )
        }

        impl Fixture {
            pub fn key(&self, name: &str) -> NodeKey {
                self.keys[name]
            }

            pub fn unit(&self, name: &str) -> QuantityUnit {
                QuantityUnit::Declared(Box::new(self.graph.unit(self.key(name)).unwrap().clone()))
            }

            pub fn q(&self, value: Rational, name: &str) -> Quantity {
                Quantity::new(value, self.unit(name))
            }

            pub fn qi(&self, value: i64, name: &str) -> Quantity {
                self.q(Rational::from_integer(Integer::from(value)), name)
            }

            pub fn compound(&self, terms: &[(&str, i64)]) -> QuantityUnit {
                QuantityUnit::Compound(self.compound_unit(terms).unwrap())
            }

            fn sorted(&self, terms: &[(&str, i64)]) -> Vec<(NodeKey, Integer)> {
                let mut sorted: Vec<(NodeKey, Integer)> = terms
                    .iter()
                    .map(|(name, exponent)| (self.key(name), Integer::from(*exponent)))
                    .collect();
                sorted.sort_by_key(|(key, _)| *key);
                sorted
            }
        }

        pub fn fixture() -> Fixture {
            admit_graph(&crate::support::GraphSpec::tc187()).unwrap()
        }
    };
}

// ---- side-neutral graph and enum descriptions -----------------------------

/// A dimension node: `terms` empty for a base dimension.
#[derive(Clone, Copy, Debug)]
pub struct DimSpec {
    pub name: &'static str,
    pub owner: &'static str,
    pub declaration: &'static str,
    pub terms: &'static [(&'static str, i64)],
}

/// A unit node. `key_as` retains another node's key (a stale reference).
#[derive(Clone, Copy, Debug)]
pub struct UnitSpec {
    pub name: &'static str,
    pub dimension: &'static str,
    pub target: Option<&'static str>,
    pub scale: (i64, i64),
    pub offset: (i64, i64),
    pub key_as: Option<&'static str>,
}

/// A closed node set. Keys are computed over every node, including removed
/// ones, so a mutation can keep a retained reference to an absent node.
#[derive(Clone, Debug)]
pub struct GraphSpec {
    pub dimensions: Vec<DimSpec>,
    pub units: Vec<UnitSpec>,
    pub removed: Vec<&'static str>,
}

const fn base(name: &'static str, declaration: &'static str) -> DimSpec {
    DimSpec {
        name,
        owner: "example-model",
        declaration,
        terms: &[],
    }
}

pub const fn unit(
    name: &'static str,
    dimension: &'static str,
    target: Option<&'static str>,
    scale: (i64, i64),
    offset: (i64, i64),
) -> UnitSpec {
    UnitSpec {
        name,
        dimension,
        target,
        scale,
        offset,
        key_as: None,
    }
}

const fn root(name: &'static str, dimension: &'static str) -> UnitSpec {
    unit(name, dimension, None, (1, 1), (0, 1))
}

const WORK: &[(&str, i64)] = &[("L", 2), ("M", 1), ("T", -2)];

impl GraphSpec {
    /// The TC-187 fixture, extended with the U09 look-alike base dimension and
    /// the U09b alias unit.
    pub fn tc187() -> Self {
        Self {
            dimensions: vec![
                base("L", "Length"),
                base("T", "Time"),
                base("Theta", "Temperature"),
                base("M", "Mass"),
                DimSpec {
                    name: "Torque",
                    owner: "example-model",
                    declaration: "Torque",
                    terms: WORK,
                },
                DimSpec {
                    name: "Energy",
                    owner: "example-model",
                    declaration: "Energy",
                    terms: WORK,
                },
                DimSpec {
                    name: "Area",
                    owner: "example-model",
                    declaration: "Area",
                    terms: &[("L", 2)],
                },
                DimSpec {
                    name: "L_other",
                    owner: "other-model",
                    declaration: "Length",
                    terms: &[],
                },
            ],
            units: vec![
                root("m", "L"),
                root("s", "T"),
                root("K", "Theta"),
                unit("cm", "L", Some("m"), (1, 100), (0, 1)),
                unit("in", "L", Some("m"), (127, 5000), (0, 1)),
                unit("degC", "Theta", Some("K"), (1, 1), (5463, 20)),
                unit("degF", "Theta", Some("K"), (5, 9), (45967, 180)),
                root("kg", "M"),
                root("N_m", "Torque"),
                root("J", "Energy"),
                root("m2", "Area"),
                unit("u1", "Theta", Some("K"), (1, 1), (10, 1)),
                unit("u2", "Theta", Some("u1"), (1, 1), (-10, 1)),
                unit("u3", "Theta", Some("degC"), (1, 1), (0, 1)),
                unit("rev", "L", Some("m"), (-1, 1), (0, 1)),
                unit("cm2", "Area", Some("m2"), (1, 10000), (0, 1)),
                root("m_other", "L_other"),
                unit("m_alias", "L", Some("m"), (1, 1), (0, 1)),
            ],
            removed: Vec::new(),
        }
    }

    fn node_id(key: &str) -> Value {
        json!({"domain": authority::NODE_KEY_DOMAIN, "digest": key})
    }

    fn owner(identity: &str) -> Value {
        json!({"kind": "definition", "authority": "agent-ix", "identity": identity})
    }

    pub fn dimension_json(dimension: &DimSpec, keys: &BTreeMap<&'static str, String>) -> Value {
        let mut terms: Vec<(&String, i64)> = dimension
            .terms
            .iter()
            .map(|(name, exponent)| (&keys[name], *exponent))
            .collect();
        terms.sort();
        let terms: Vec<Value> = terms
            .into_iter()
            .map(|(key, exponent)| {
                json!({"dimension_node_id": Self::node_id(key), "exponent": exponent.to_string()})
            })
            .collect();
        json!({
            "version": "quire.dimension-node/v1",
            "owner": Self::owner(dimension.owner),
            "qualified_declaration": ["Example", dimension.declaration],
            "terms": terms,
        })
    }

    pub fn unit_json(unit: &UnitSpec, keys: &BTreeMap<&'static str, String>) -> Value {
        let rational =
            |(n, d): (i64, i64)| json!({"numerator": n.to_string(), "denominator": d.to_string()});
        json!({
            "version": "quire.unit-node/v1",
            "owner": Self::owner("example-model"),
            "qualified_declaration": ["Example", unit.name],
            "dimension_node_id": Self::node_id(&keys[unit.dimension]),
            "target_unit_node_id": unit.target.map_or(Value::Null, |t| Self::node_id(&keys[t])),
            "scale": rational(unit.scale),
            "offset": rational(unit.offset),
        })
    }

    /// Honest keys, computed by the authority in declaration order.
    pub fn keys(&self) -> BTreeMap<&'static str, String> {
        let mut keys = BTreeMap::new();
        for dimension in &self.dimensions {
            let preimage =
                authority::DimensionPreimage::from_json(Self::dimension_json(dimension, &keys))
                    .unwrap();
            keys.insert(dimension.name, preimage.node_key().unwrap().to_string());
        }
        for unit in &self.units {
            let preimage =
                authority::UnitPreimage::from_json(Self::unit_json(unit, &keys)).unwrap();
            if keys
                .insert(unit.name, preimage.node_key().unwrap().to_string())
                .is_some()
            {
                panic!("duplicate fixture name {}", unit.name);
            }
        }
        keys
    }

    /// The admitted key of a node: its retained reference when it has one.
    pub fn admitted_key(&self, unit: &UnitSpec, keys: &BTreeMap<&'static str, String>) -> String {
        keys[unit.key_as.unwrap_or(unit.name)].clone()
    }

    pub fn present_dimensions(&self) -> impl Iterator<Item = &DimSpec> {
        self.dimensions
            .iter()
            .filter(|d| !self.removed.contains(&d.name))
    }

    pub fn present_units(&self) -> impl Iterator<Item = &UnitSpec> {
        self.units
            .iter()
            .filter(|u| !self.removed.contains(&u.name))
    }

    pub fn with(mut self, unit: UnitSpec) -> Self {
        self.units.push(unit);
        self
    }

    pub fn without(mut self, names: &[&'static str]) -> Self {
        self.removed.extend_from_slice(names);
        self
    }
}

/// Authority-computed enum declaration key.
pub fn enum_declaration_json(declaration: &str, ordered: bool, members: &[&str]) -> Value {
    json!({
        "version": "quire.enum-declaration-node/v1",
        "owner": {"kind": "definition", "authority": "agent-ix", "identity": "example-model"},
        "qualified_declaration": ["Example", declaration],
        "ordered": ordered,
        "members": members,
    })
}

pub fn enum_member_json(declaration_key: &str, case: &str) -> Value {
    json!({
        "version": "quire.enum-member-node/v1",
        "declaration_node_id": {"domain": authority::NODE_KEY_DOMAIN, "digest": declaration_key},
        "case": case,
    })
}

pub fn enum_declaration_key(declaration: &str, ordered: bool, members: &[&str]) -> String {
    let json = enum_declaration_json(declaration, ordered, members);
    let preimage = authority::EnumDeclarationPreimage::from_json(json).unwrap();
    preimage.node_key().unwrap().to_string()
}

pub fn enum_member_key(declaration_key: &str, case: &str) -> String {
    let preimage =
        authority::EnumMemberPreimage::from_json(enum_member_json(declaration_key, case)).unwrap();
    preimage.node_key().unwrap().to_string()
}

// ---- the authority side ------------------------------------------------------

pub mod qsl_side {
    use std::collections::BTreeMap;
    use std::sync::OnceLock;

    use quire_spec_language::value as authority;
    pub use quire_spec_language::value::*;

    /// The authority's `InjectedDenial::occurrence` is a plain `u64`; only the runtime side makes
    /// it `NonZeroU64` (`agent-ix/quire-contract-runtime#20`), so `shared_helpers!`'s `denials`,
    /// and any `agree!` vector that injects a denial directly, call this per-side conversion
    /// rather than hard-coding either type.
    pub fn to_occurrence(n: u64) -> u64 {
        n
    }

    shared_helpers!();

    /// A checked package (quire-specification/FR-146) of `functions` over `types`, admitted
    /// under unlimited [`CheckingLimits`]. The authority's `PackageDeclarations::check` takes no
    /// `CheckMode` (that only gates a standalone [`CheckedExpression`], never a named package
    /// function, which is always checked as a linked body); the runtime port instead takes an
    /// explicit `CheckMode` at the package boundary. This helper is the per-side seam that hides
    /// that one shape difference so the shared vector body below stays identical on both sides.
    pub fn linked_package(
        types: TypeEnvironment,
        functions: Vec<FunctionDeclaration>,
    ) -> CheckedPackage {
        PackageDeclarations {
            types,
            functions,
            ..Default::default()
        }
        .check(CheckingLimits::default())
        .unwrap()
    }

    /// A named function of one parameter `x: parameter_type` whose body is exactly its argument
    /// (`x`), declared to return `result_type`. Used only where `parameter_type == result_type`,
    /// so a completed call always agrees on the returned value without depending on any body
    /// computation this crate's Expression tree and the runtime's opaque Rust closure could
    /// diverge on.
    pub fn identity_function(
        name: &str,
        parameter_type: ValueType,
        result_type: ValueType,
    ) -> FunctionDeclaration {
        FunctionDeclaration::new(
            name,
            vec![("x".to_string(), parameter_type)],
            result_type,
            None,
            Expression::Name("x".to_string()),
        )
    }

    fn lock() -> &'static DefinitionLock {
        DefinitionLock::pinned().unwrap()
    }

    /// The pinned lock's admitted division law, then the authority operator.
    pub fn divide(
        profile: DivisionProfile,
        a: &Integer,
        b: &Integer,
        domain: &IntegerDomain,
        meter: &mut Meter,
    ) -> Outcome<QuotientRemainder> {
        let role = match profile {
            DivisionProfile::Truncating => CatalogRole::IntegerDivisionTruncating,
            DivisionProfile::Floor => CatalogRole::IntegerDivisionFloor,
            DivisionProfile::Euclidean => CatalogRole::IntegerDivisionEuclidean,
        };
        let reference = lock().entry(role).unwrap().definition.clone();
        let admitted = lock().admit_integer_division(&[reference], None).unwrap();
        authority::divide(&admitted, a, b, domain, meter)
    }

    fn profile() -> &'static AdmittedIeeeProfile {
        static PROFILE: OnceLock<AdmittedIeeeProfile> = OnceLock::new();
        PROFILE.get_or_init(|| {
            let reference = lock()
                .entry(CatalogRole::IeeeProfile)
                .unwrap()
                .definition
                .clone();
            lock().admit_ieee_profile(&[reference], &[]).unwrap()
        })
    }

    pub fn evaluate_ieee<'a, O: Into<IeeeOperand<'a>>>(
        operation: IeeeOperation<O>,
        rounding: RoundingMode,
        meter: &mut Meter,
    ) -> Result<Outcome<IeeeResult>, IllTyped> {
        authority::evaluate_ieee(profile(), operation, rounding, meter)
    }

    pub fn compare_ieee<'a>(
        comparison: IeeeComparison,
        left: impl Into<IeeeOperand<'a>>,
        right: impl Into<IeeeOperand<'a>>,
        meter: &mut Meter,
    ) -> Result<Outcome<bool>, IllTyped> {
        authority::compare_ieee(profile(), comparison, left, right, meter)
    }

    pub fn convert_ieee_width(
        value: IeeeValue,
        target: IeeeWidth,
        rounding: RoundingMode,
        meter: &mut Meter,
    ) -> Outcome<IeeeResult> {
        authority::convert_ieee_width(profile(), value, target, rounding, meter)
    }

    pub fn ieee_to_exact(
        value: IeeeValue,
        target: IeeeExactTarget<'_>,
        meter: &mut Meter,
    ) -> Result<Outcome<IeeeExact>, IllTyped> {
        authority::ieee_to_exact(profile(), value, target, meter)
    }

    pub fn exact_to_ieee<'a>(
        source: impl Into<ExactScalar<'a>>,
        width: IeeeWidth,
        rounding: RoundingMode,
        meter: &mut Meter,
    ) -> Outcome<IeeeResult> {
        authority::exact_to_ieee(profile(), source, width, rounding, meter)
    }

    pub struct Fixture {
        pub graph: UnitGraph,
        keys: BTreeMap<&'static str, NodeKey>,
    }

    fn owners() -> OwnerSelection {
        let owner = |identity: &str| {
            NodeOwner::Definition(OwnerSubject {
                authority: "agent-ix".into(),
                identity: identity.into(),
            })
        };
        OwnerSelection::new([owner("example-model"), owner("other-model")])
    }

    pub fn admit_graph(spec: &crate::support::GraphSpec) -> Result<Fixture, InvalidSemanticGraph> {
        use crate::support::GraphSpec;
        let hex = spec.keys();
        let key = |digest: &str| NodeKey::from_hex(digest).unwrap();
        let dimensions: Vec<_> = spec
            .present_dimensions()
            .map(|d| {
                let preimage =
                    DimensionPreimage::from_json(GraphSpec::dimension_json(d, &hex)).unwrap();
                (preimage, key(&hex[d.name]))
            })
            .collect();
        let units: Vec<_> = spec
            .present_units()
            .map(|u| {
                let preimage = UnitPreimage::from_json(GraphSpec::unit_json(u, &hex)).unwrap();
                (preimage, key(&spec.admitted_key(u, &hex)))
            })
            .collect();
        let graph = UnitGraph::admit(dimensions, units, &owners())?;
        let keys = hex
            .iter()
            .map(|(name, digest)| (*name, key(digest)))
            .collect();
        Ok(Fixture { graph, keys })
    }

    impl Fixture {
        pub fn compound_unit(
            &self,
            terms: &[(&str, i64)],
        ) -> Result<CompoundUnit, InvalidCompoundUnit> {
            let terms: Vec<serde_json::Value> = self
                .sorted(terms)
                .into_iter()
                .map(|(key, exponent)| {
                    serde_json::json!({
                        "unit_node_id": {"domain": NODE_KEY_DOMAIN, "digest": key.to_string()},
                        "exponent": exponent.to_string(),
                    })
                })
                .collect();
            let preimage = CompoundUnitPreimage::from_json(
                serde_json::json!({"version": COMPOUND_UNIT_DOMAIN, "terms": terms}),
            )?;
            self.graph.compound_unit(&preimage)
        }
    }

    /// An admitted enum declaration under its honest key.
    pub struct Enum(EnumDeclaration);

    pub fn enum_declaration(
        declaration: &str,
        ordered: bool,
        members: &[&str],
    ) -> Result<Enum, InvalidSemanticGraph> {
        let key = crate::support::enum_declaration_key(declaration, ordered, members);
        let json = crate::support::enum_declaration_json(declaration, ordered, members);
        let preimage = EnumDeclarationPreimage::from_json(json)?;
        EnumDeclaration::admit(preimage, NodeKey::from_hex(&key).unwrap(), &owners()).map(Enum)
    }

    impl Enum {
        pub fn value(&self, case: &str) -> Result<EnumValue, InvalidSemanticGraph> {
            let declaration = self.0.key().to_string();
            let key = crate::support::enum_member_key(&declaration, case);
            let preimage = EnumMemberPreimage::from_json(crate::support::enum_member_json(
                &declaration,
                case,
            ))?;
            self.0
                .admit_member(&preimage, NodeKey::from_hex(&key).unwrap())
        }

        pub fn key(&self) -> NodeKey {
            self.0.key()
        }
    }
}

// ---- the runtime side --------------------------------------------------------

pub mod rt_side {
    use std::collections::BTreeMap;

    pub use quire_contract_runtime::exact::*;

    /// See the `qsl_side` twin of this function: the runtime's `InjectedDenial::occurrence` is
    /// `NonZeroU64` (`agent-ix/quire-contract-runtime#20`). Every caller in this crate passes a
    /// literal or a derived count that is always at least one, so this never panics.
    pub fn to_occurrence(n: u64) -> std::num::NonZeroU64 {
        std::num::NonZeroU64::new(n).unwrap()
    }

    shared_helpers!();

    /// See the `qsl_side` twin of this function for why the `CheckMode` this
    /// port's boundary adds at the package level is hidden here rather than
    /// in the shared vector body.
    pub fn linked_package(
        types: TypeEnvironment,
        functions: Vec<FunctionDeclaration>,
    ) -> CheckedPackage {
        PackageDeclarations { types, functions }
            .check(CheckMode::Linked, CheckingLimits::default())
            .unwrap()
    }

    /// See the `qsl_side` twin: an identity function over one parameter,
    /// built from an opaque Rust closure rather than an Expression tree.
    pub fn identity_function(
        name: &str,
        parameter_type: ValueType,
        result_type: ValueType,
    ) -> FunctionDeclaration {
        FunctionDeclaration {
            name: name.to_string(),
            parameters: vec![("x".to_string(), parameter_type)],
            result: result_type,
            ieee_requirements: Vec::new(),
            integer_division_consumers: Vec::new(),
            measure_discharged: true,
            body: Box::new(|_frame, arguments| Outcome::Completed(arguments[0].clone())),
        }
    }

    pub struct Fixture {
        pub graph: UnitGraph,
        keys: BTreeMap<&'static str, NodeKey>,
    }

    fn rational((numerator, denominator): (i64, i64)) -> Rational {
        Rational::new(Integer::from(numerator), Integer::from(denominator)).unwrap()
    }

    pub fn admit_graph(spec: &crate::support::GraphSpec) -> Result<Fixture, InvalidSemanticGraph> {
        let hex = spec.keys();
        let key = |digest: &str| NodeKey::from_hex(digest).unwrap();
        let dimensions: Vec<_> = spec
            .present_dimensions()
            .map(|d| {
                let mut terms: Vec<(NodeKey, Integer)> = d
                    .terms
                    .iter()
                    .map(|(name, exponent)| (key(&hex[name]), Integer::from(*exponent)))
                    .collect();
                terms.sort_by_key(|(term, _)| *term);
                (key(&hex[d.name]), terms)
            })
            .collect();
        let units: Vec<_> = spec
            .present_units()
            .map(|u| {
                let declaration = UnitDeclaration {
                    dimension: key(&hex[u.dimension]),
                    target: u.target.map(|target| key(&hex[target])),
                    scale: rational(u.scale),
                    offset: rational(u.offset),
                };
                (key(&spec.admitted_key(u, &hex)), declaration)
            })
            .collect();
        let graph = UnitGraph::admit(dimensions, units)?;
        let keys = hex
            .iter()
            .map(|(name, digest)| (*name, key(digest)))
            .collect();
        Ok(Fixture { graph, keys })
    }

    impl Fixture {
        pub fn compound_unit(
            &self,
            terms: &[(&str, i64)],
        ) -> Result<CompoundUnit, InvalidCompoundUnit> {
            self.graph.compound_unit(&self.sorted(terms))
        }
    }

    /// A compiler-admitted enum declaration under its authority-computed key.
    pub struct Enum(EnumDeclaration);

    pub fn enum_declaration(
        declaration: &str,
        ordered: bool,
        members: &[&str],
    ) -> Result<Enum, InvalidSemanticGraph> {
        let key = crate::support::enum_declaration_key(declaration, ordered, members);
        EnumDeclaration::new(NodeKey::from_hex(&key).unwrap(), ordered, members).map(Enum)
    }

    impl Enum {
        pub fn value(&self, case: &str) -> Result<EnumValue, InvalidSemanticGraph> {
            let declaration = self.0.key().to_string();
            let key = crate::support::enum_member_key(&declaration, case);
            self.0.member(case, NodeKey::from_hex(&key).unwrap())
        }

        pub fn key(&self) -> NodeKey {
            self.0.key()
        }
    }
}
