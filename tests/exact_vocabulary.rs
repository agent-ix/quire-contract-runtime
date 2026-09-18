//! FR-012 carried compiler vocabulary: node keys, refusal vocabularies, semantic-graph causes and
//! normalized unit values, through the public `exact` surface.
#![cfg(feature = "exact")]

use std::fs;
use std::path::Path;

use quire_contract_runtime::exact::{
    CompoundUnit, Dimension, EnumDeclaration, EnumValue, Integer, InvalidSemanticGraph, NodeKey,
    PackageRefusalCode, Rational, SelectionRefusalCode, SemanticGraphCause, UnitDeclaration,
    UnitGraph, COMPOUND_UNIT_DOMAIN, NODE_KEY_DOMAIN,
};

fn key(byte: u8) -> NodeKey {
    NodeKey::from_bytes([byte; 32])
}

fn int(value: i128) -> Integer {
    Integer::from(value)
}

fn rational(value: i128) -> Rational {
    Rational::from_integer(int(value))
}

fn root(dimension: NodeKey) -> UnitDeclaration {
    UnitDeclaration {
        dimension,
        target: None,
        scale: rational(1),
        offset: rational(0),
    }
}

// --- AC-4 admissible inputs, one per structural `SemanticGraphCause` reachable through the
// public `UnitGraph::admit`. `check_terms` is `pub(crate)`, so the three term causes are reached
// through `admit`'s `dimensions` parameter, as the requirement's own `Behavior` clause describes
// them: dimension terms it checks for well-formedness before graph topology.

fn zero_exponent_admit() -> Result<UnitGraph, InvalidSemanticGraph> {
    let dimensions = vec![(key(1), vec![]), (key(2), vec![(key(1), int(0))])];
    UnitGraph::admit(dimensions, vec![])
}

fn duplicate_term_admit() -> Result<UnitGraph, InvalidSemanticGraph> {
    let dimensions = vec![
        (key(1), vec![]),
        (key(2), vec![(key(1), int(1)), (key(1), int(2))]),
    ];
    UnitGraph::admit(dimensions, vec![])
}

fn unsorted_terms_admit() -> Result<UnitGraph, InvalidSemanticGraph> {
    let dimensions = vec![
        (key(1), vec![]),
        (key(2), vec![]),
        (key(3), vec![(key(2), int(1)), (key(1), int(1))]),
    ];
    UnitGraph::admit(dimensions, vec![])
}

fn zero_scale_admit() -> Result<UnitGraph, InvalidSemanticGraph> {
    let dimensions = vec![(key(1), vec![])];
    let units = vec![(
        key(2),
        UnitDeclaration {
            dimension: key(1),
            target: None,
            scale: rational(0),
            offset: rational(0),
        },
    )];
    UnitGraph::admit(dimensions, units)
}

fn non_identity_root_admit() -> Result<UnitGraph, InvalidSemanticGraph> {
    let dimensions = vec![(key(1), vec![])];
    let units = vec![(
        key(2),
        UnitDeclaration {
            dimension: key(1),
            target: None,
            scale: rational(2),
            offset: rational(0),
        },
    )];
    UnitGraph::admit(dimensions, units)
}

fn duplicate_node_admit() -> Result<UnitGraph, InvalidSemanticGraph> {
    let dimensions = vec![(key(1), vec![])];
    let units = vec![(key(1), root(key(1)))];
    UnitGraph::admit(dimensions, units)
}

fn unknown_dimension_admit() -> Result<UnitGraph, InvalidSemanticGraph> {
    let dimensions = vec![(key(1), vec![])];
    let units = vec![(key(2), root(key(3)))];
    UnitGraph::admit(dimensions, units)
}

fn non_base_dimension_term_admit() -> Result<UnitGraph, InvalidSemanticGraph> {
    let dimensions = vec![
        (key(1), vec![]),
        (key(2), vec![(key(1), int(1))]),
        (key(3), vec![(key(2), int(1))]),
    ];
    UnitGraph::admit(dimensions, vec![])
}

fn unknown_target_admit() -> Result<UnitGraph, InvalidSemanticGraph> {
    let dimensions = vec![(key(1), vec![])];
    let units = vec![(
        key(2),
        UnitDeclaration {
            dimension: key(1),
            target: Some(key(3)),
            scale: rational(1),
            offset: rational(0),
        },
    )];
    UnitGraph::admit(dimensions, units)
}

fn cross_dimension_target_admit() -> Result<UnitGraph, InvalidSemanticGraph> {
    let dimensions = vec![(key(1), vec![]), (key(2), vec![])];
    let units = vec![
        (key(3), root(key(2))),
        (
            key(4),
            UnitDeclaration {
                dimension: key(1),
                target: Some(key(3)),
                scale: rational(2),
                offset: rational(0),
            },
        ),
    ];
    UnitGraph::admit(dimensions, units)
}

fn missing_root_admit() -> Result<UnitGraph, InvalidSemanticGraph> {
    let dimensions = vec![(key(1), vec![])];
    let units = vec![
        (
            key(2),
            UnitDeclaration {
                dimension: key(1),
                target: Some(key(3)),
                scale: rational(1),
                offset: rational(0),
            },
        ),
        (
            key(3),
            UnitDeclaration {
                dimension: key(1),
                target: Some(key(2)),
                scale: rational(1),
                offset: rational(0),
            },
        ),
    ];
    UnitGraph::admit(dimensions, units)
}

fn duplicate_root_admit() -> Result<UnitGraph, InvalidSemanticGraph> {
    let dimensions = vec![(key(1), vec![])];
    let units = vec![(key(2), root(key(1))), (key(3), root(key(1)))];
    UnitGraph::admit(dimensions, units)
}

fn target_cycle_admit() -> Result<UnitGraph, InvalidSemanticGraph> {
    let dimensions = vec![(key(1), vec![])];
    let units = vec![
        (key(2), root(key(1))),
        (
            key(3),
            UnitDeclaration {
                dimension: key(1),
                target: Some(key(4)),
                scale: rational(2),
                offset: rational(0),
            },
        ),
        (
            key(4),
            UnitDeclaration {
                dimension: key(1),
                target: Some(key(3)),
                scale: rational(3),
                offset: rational(0),
            },
        ),
    ];
    UnitGraph::admit(dimensions, units)
}

/// `UndeclaredCase` is structural but is never raised by `UnitGraph::admit` or `check_terms`: it
/// is raised only by `EnumDeclaration::member` (`src/exact/enumeration.rs`), a public item that
/// FR-012-AC-4's text does not name as a witness path. Reached here for completeness; see the
/// report on this gap.
fn undeclared_case_member() -> Result<EnumValue, InvalidSemanticGraph> {
    let declaration = EnumDeclaration::new(key(1), true, &["red", "green"]).unwrap();
    declaration.member("blue", key(2))
}

/// Trace: TC-033, FR-012-AC-4
#[test]
fn tc_033_admit_raises_zero_exponent() {
    assert_eq!(
        zero_exponent_admit().unwrap_err().cause,
        SemanticGraphCause::ZeroExponent
    );
}

/// Trace: TC-033, FR-012-AC-4
#[test]
fn tc_033_admit_raises_duplicate_term() {
    assert_eq!(
        duplicate_term_admit().unwrap_err().cause,
        SemanticGraphCause::DuplicateTerm
    );
}

/// Trace: TC-033, FR-012-AC-4
#[test]
fn tc_033_admit_raises_unsorted_terms() {
    assert_eq!(
        unsorted_terms_admit().unwrap_err().cause,
        SemanticGraphCause::UnsortedTerms
    );
}

/// Trace: TC-033, FR-012-AC-4
#[test]
fn tc_033_admit_raises_zero_scale() {
    assert_eq!(
        zero_scale_admit().unwrap_err().cause,
        SemanticGraphCause::ZeroScale
    );
}

/// Trace: TC-033, FR-012-AC-4
#[test]
fn tc_033_admit_raises_non_identity_root() {
    assert_eq!(
        non_identity_root_admit().unwrap_err().cause,
        SemanticGraphCause::NonIdentityRoot
    );
}

/// Trace: TC-033, FR-012-AC-4
#[test]
fn tc_033_admit_raises_duplicate_node() {
    assert_eq!(
        duplicate_node_admit().unwrap_err().cause,
        SemanticGraphCause::DuplicateNode
    );
}

/// Trace: TC-033, FR-012-AC-4
#[test]
fn tc_033_admit_raises_unknown_dimension() {
    assert_eq!(
        unknown_dimension_admit().unwrap_err().cause,
        SemanticGraphCause::UnknownDimension
    );
}

/// Trace: TC-033, FR-012-AC-4
#[test]
fn tc_033_admit_raises_non_base_dimension_term() {
    assert_eq!(
        non_base_dimension_term_admit().unwrap_err().cause,
        SemanticGraphCause::NonBaseDimensionTerm
    );
}

/// Trace: TC-033, FR-012-AC-4
#[test]
fn tc_033_admit_raises_unknown_target() {
    assert_eq!(
        unknown_target_admit().unwrap_err().cause,
        SemanticGraphCause::UnknownTarget
    );
}

/// Trace: TC-033, FR-012-AC-4
#[test]
fn tc_033_admit_raises_cross_dimension_target() {
    assert_eq!(
        cross_dimension_target_admit().unwrap_err().cause,
        SemanticGraphCause::CrossDimensionTarget
    );
}

/// Trace: TC-033, FR-012-AC-4
#[test]
fn tc_033_admit_raises_missing_root() {
    assert_eq!(
        missing_root_admit().unwrap_err().cause,
        SemanticGraphCause::MissingRoot
    );
}

/// Trace: TC-033, FR-012-AC-4
#[test]
fn tc_033_admit_raises_duplicate_root() {
    assert_eq!(
        duplicate_root_admit().unwrap_err().cause,
        SemanticGraphCause::DuplicateRoot
    );
}

/// Trace: TC-033, FR-012-AC-4
#[test]
fn tc_033_admit_raises_target_cycle() {
    assert_eq!(
        target_cycle_admit().unwrap_err().cause,
        SemanticGraphCause::TargetCycle
    );
}

/// Trace: TC-033, FR-012-AC-4
#[test]
fn tc_033_enum_declaration_member_raises_undeclared_case() {
    assert_eq!(
        undeclared_case_member().unwrap_err().cause,
        SemanticGraphCause::UndeclaredCase
    );
}

/// Trace: TC-033, FR-012-AC-4
#[test]
fn tc_033_structural_inputs_never_raise_a_compiler_admission_cause() {
    let compiler_admission = |cause: SemanticGraphCause| {
        matches!(
            cause,
            SemanticGraphCause::NonCanonicalPreimage
                | SemanticGraphCause::UnsortedUnorderedMembers
                | SemanticGraphCause::OwnerNotSelected
                | SemanticGraphCause::StaleKey
        )
    };
    let causes = [
        zero_exponent_admit().unwrap_err().cause,
        duplicate_term_admit().unwrap_err().cause,
        unsorted_terms_admit().unwrap_err().cause,
        zero_scale_admit().unwrap_err().cause,
        non_identity_root_admit().unwrap_err().cause,
        duplicate_node_admit().unwrap_err().cause,
        unknown_dimension_admit().unwrap_err().cause,
        non_base_dimension_term_admit().unwrap_err().cause,
        unknown_target_admit().unwrap_err().cause,
        cross_dimension_target_admit().unwrap_err().cause,
        missing_root_admit().unwrap_err().cause,
        duplicate_root_admit().unwrap_err().cause,
        target_cycle_admit().unwrap_err().cause,
        undeclared_case_member().unwrap_err().cause,
    ];
    for cause in causes {
        assert!(
            !compiler_admission(cause),
            "{cause:?} is a compiler-admission cause"
        );
    }
}

/// Trace: TC-033, FR-012-AC-1
#[test]
fn tc_033_from_hex_accepts_exactly_64_lowercase_hex_digits() {
    let valid = "0123456789abcdef".repeat(4);
    assert_eq!(valid.len(), 64);
    assert!(NodeKey::from_hex(&valid).is_some());

    assert_eq!(NodeKey::from_hex(&"0".repeat(63)), None, "63 digits");
    assert_eq!(NodeKey::from_hex(&"0".repeat(65)), None, "65 digits");
    assert_eq!(NodeKey::from_hex(""), None, "empty");

    let mut uppercase = "0".repeat(64);
    uppercase.replace_range(63..64, "A");
    assert_eq!(NodeKey::from_hex(&uppercase), None, "uppercase digit");

    let mut non_hex = "0".repeat(64);
    non_hex.replace_range(63..64, "g");
    assert_eq!(NodeKey::from_hex(&non_hex), None, "non-hex ASCII byte");

    // 62 ASCII '0' bytes plus the two UTF-8 bytes of 'é': 64 bytes, but the trailing byte pair is
    // not a hex digit pair.
    let non_ascii = format!("{}{}", "0".repeat(62), 'é');
    assert_eq!(non_ascii.len(), 64);
    assert_eq!(NodeKey::from_hex(&non_ascii), None, "non-ASCII byte");
}

/// Trace: TC-033, FR-012-AC-1
#[test]
fn tc_033_digest_round_trips_through_bytes_hex_and_display() {
    let digests: [[u8; 32]; 3] = [
        [0_u8; 32],
        [0xff_u8; 32],
        std::array::from_fn(|index| index as u8),
    ];
    for digest in digests {
        let parsed_key = NodeKey::from_bytes(digest);
        assert_eq!(parsed_key.as_bytes(), &digest);

        let text = parsed_key.to_string();
        assert_eq!(text.len(), 64);
        assert!(text
            .chars()
            .all(|c| c.is_ascii_hexdigit() && !c.is_ascii_uppercase()));

        let round_tripped = NodeKey::from_hex(&text).unwrap();
        assert_eq!(round_tripped, parsed_key);
        assert_eq!(round_tripped.as_bytes(), &digest);
    }
}

/// Trace: TC-033, FR-012-AC-2
#[test]
fn tc_033_digest_domains_equal_their_normative_strings() {
    assert_eq!(NODE_KEY_DOMAIN, "quire.checked-semantic-node/v1");
    assert_eq!(COMPOUND_UNIT_DOMAIN, "quire.value.compound-unit/v1");
}

/// Trace: TC-033, FR-012-AC-3
#[test]
fn tc_033_selection_refusal_code_all_is_eight_codes_in_check_order() {
    assert_eq!(
        SelectionRefusalCode::ALL,
        [
            SelectionRefusalCode::SelectionUnknownTrigger,
            SelectionRefusalCode::SelectionDuplicateTrigger,
            SelectionRefusalCode::SelectionUnknownRole,
            SelectionRefusalCode::SelectionDuplicateRole,
            SelectionRefusalCode::SelectionRequiredMissing,
            SelectionRefusalCode::SelectionAlternativeConflict,
            SelectionRefusalCode::SelectionTriggerUnsatisfied,
            SelectionRefusalCode::SelectionUntriggeredProfile,
        ]
    );
}

/// Trace: TC-033, FR-012-AC-3
#[test]
fn tc_033_selection_refusal_code_as_str_matches_lock_spelling() {
    let spellings: Vec<&str> = SelectionRefusalCode::ALL
        .iter()
        .map(|code| code.as_str())
        .collect();
    assert_eq!(
        spellings,
        [
            "selection_unknown_trigger",
            "selection_duplicate_trigger",
            "selection_unknown_role",
            "selection_duplicate_role",
            "selection_required_missing",
            "selection_alternative_conflict",
            "selection_trigger_unsatisfied",
            "selection_untriggered_profile",
        ]
    );
}

/// Trace: TC-033, FR-012-AC-3
#[test]
fn tc_033_selection_refusal_code_from_code_is_exact_inverse() {
    for code in SelectionRefusalCode::ALL {
        assert_eq!(SelectionRefusalCode::from_code(code.as_str()), Some(code));
    }
    assert_eq!(SelectionRefusalCode::from_code(""), None, "empty string");
    assert_eq!(
        SelectionRefusalCode::from_code("selection_unknown_trigger_"),
        None,
        "trailing-underscore near miss"
    );
    assert_eq!(
        SelectionRefusalCode::from_code("invalid_package"),
        None,
        "unrelated word"
    );
}

/// Trace: TC-033, FR-012-AC-3
#[test]
fn tc_033_package_refusal_code_as_str_is_invalid_package() {
    assert_eq!(
        PackageRefusalCode::InvalidPackage.as_str(),
        "invalid_package"
    );
}

/// Trace: TC-033, FR-012-AC-5
#[test]
fn tc_033_dimension_multiply_is_order_independent_ascending_and_zero_free() {
    let base = |byte: u8| -> Dimension {
        let graph = UnitGraph::admit(vec![(key(byte), vec![])], vec![]).unwrap();
        graph.dimension(key(byte)).unwrap().clone()
    };
    let low = base(1);
    let high = base(2);

    let forward = low.power(&int(2)).multiply(&high.power(&int(3)));
    let backward = high.power(&int(3)).multiply(&low.power(&int(2)));
    assert_eq!(forward, backward);

    let terms: Vec<(NodeKey, Integer)> = forward
        .exponents()
        .map(|(node, exponent)| (node, exponent.clone()))
        .collect();
    assert_eq!(terms, [(key(1), int(2)), (key(2), int(3))]);
    assert!(terms.iter().all(|(_, exponent)| !exponent.is_zero()));

    // Exponents that cancel are removed entirely, never retained as zero.
    let cancelled = low.multiply(&low.power(&int(-1)));
    assert!(cancelled.is_dimensionless());
    assert_eq!(cancelled.exponents().count(), 0);
    assert_eq!(cancelled, Dimension::dimensionless());
}

/// Trace: TC-033, FR-012-AC-5
#[test]
fn tc_033_compound_unit_is_independent_of_admission_order_and_ascending() {
    let build = |dimension_order: [u8; 2], unit_order: [u8; 2]| -> CompoundUnit {
        let dimensions: Vec<(NodeKey, Vec<(NodeKey, Integer)>)> = dimension_order
            .into_iter()
            .map(|byte| (key(byte), vec![]))
            .collect();
        let units: Vec<(NodeKey, UnitDeclaration)> = unit_order
            .into_iter()
            .map(|byte| (key(byte + 10), root(key(byte))))
            .collect();
        let graph = UnitGraph::admit(dimensions, units).unwrap();
        graph
            .compound_unit(&[(key(11), int(1)), (key(12), int(1))])
            .unwrap()
    };

    let forward = build([1, 2], [1, 2]);
    let backward = build([2, 1], [2, 1]);
    assert_eq!(forward, backward);

    let terms: Vec<(NodeKey, Integer)> = forward
        .terms()
        .map(|(node, exponent)| (node, exponent.clone()))
        .collect();
    assert_eq!(terms, [(key(11), int(1)), (key(12), int(1))]);
    assert!(terms.iter().all(|(_, exponent)| !exponent.is_zero()));

    assert!(CompoundUnit::dimensionless().terms().next().is_none());
    assert_eq!(CompoundUnit::dimensionless(), CompoundUnit::default());
}

/// Trace: TC-033, FR-012-AC-6
#[test]
fn tc_033_node_definition_unit_reexports_never_touch_a_meter_and_are_all_named() {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/exact");
    let mod_rs = fs::read_to_string(directory.join("mod.rs")).unwrap();

    // Collect the names `mod.rs` re-exports from node.rs, definition.rs and unit.rs.
    let mut reexported: Vec<String> = Vec::new();
    for item in mod_rs.split("pub use ").skip(1) {
        let item = item.split(';').next().unwrap();
        let Some((module, names)) = item.split_once("::") else {
            continue;
        };
        if !matches!(module, "node" | "definition" | "unit") {
            continue;
        }
        reexported.extend(
            names
                .trim_matches(|c| c == '{' || c == '}')
                .split(',')
                .map(str::trim)
                .filter(|name| !name.is_empty())
                .map(|name| format!("{module}::{name}")),
        );
    }
    assert_eq!(reexported.len(), 4 + 4 + 9, "{reexported:?}");

    // `Meter` does not appear anywhere in these three files at all, which is stricter than "no
    // re-exported signature mentions it" and fails closed if a metered function is ever added.
    for module in ["node", "definition", "unit"] {
        let source = fs::read_to_string(directory.join(format!("{module}.rs"))).unwrap();
        let code: String = source
            .lines()
            .filter(|line| !line.trim_start().starts_with("//"))
            .collect::<Vec<_>>()
            .join("\n");
        let mentions_meter = code.match_indices("Meter").any(|(at, _)| {
            let before = code[..at].chars().next_back();
            let after = code[at + "Meter".len()..].chars().next();
            let is_word = |c: Option<char>| c.is_some_and(|c| c.is_alphanumeric() || c == '_');
            !is_word(before) && !is_word(after)
        });
        assert!(!mentions_meter, "{module}.rs mentions Meter");
    }

    // Every re-export must be named by FR-012 itself, as a backtick-quoted identifier somewhere
    // in the requirement's own text, so an unnamed re-export fails this test rather than passing
    // silently.
    let requirement = fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("spec/functional/FR-012-carried-compiler-vocabulary.md"),
    )
    .unwrap();
    let unnamed: Vec<&String> = reexported
        .iter()
        .filter(|qualified| {
            let name = qualified.rsplit("::").next().unwrap();
            !requirement.contains(&format!("`{name}`"))
        })
        .collect();
    assert!(unnamed.is_empty(), "FR-012 does not name: {unnamed:?}");
}

/// Trace: TC-033, FR-012-AC-4
#[test]
fn tc_033_enum_declaration_new_is_the_only_runtime_admission_point() {
    let empty = EnumDeclaration::new(key(1), true, &[]);
    let repeated = EnumDeclaration::new(key(1), true, &["a", "a"]);
    let non_identifier = EnumDeclaration::new(key(1), true, &["1a"]);
    for refused in [empty, repeated, non_identifier] {
        assert_eq!(
            refused.unwrap_err().cause,
            SemanticGraphCause::NonCanonicalPreimage
        );
    }

    let unsorted = EnumDeclaration::new(key(1), false, &["b", "a"]);
    assert_eq!(
        unsorted.unwrap_err().cause,
        SemanticGraphCause::UnsortedUnorderedMembers
    );

    // An ordered declaration may list its cases in any order.
    assert!(EnumDeclaration::new(key(1), true, &["b", "a"]).is_ok());
    assert!(EnumDeclaration::new(key(1), false, &["a", "b"]).is_ok());
}

/// Trace: TC-033, FR-012-AC-4
#[test]
fn tc_033_carried_only_causes_have_no_raise_site_in_the_crate() {
    // `OwnerNotSelected` and `StaleKey` need a compiler that computes keys and resolves a lock
    // selection; `ForeignDeclaration` and `UnreducedRational` cannot be presented through this
    // API. All four are carried for a generated oracle to report, so a raise site appearing here
    // would mean the runtime started re-deciding a compiler decision.
    let directory = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut sources = String::new();
    for entry in fs::read_dir(directory.join("exact")).unwrap() {
        let path = entry.unwrap().path();
        if path.extension().is_some_and(|extension| extension == "rs") {
            sources.push_str(&fs::read_to_string(&path).unwrap());
        }
    }
    let code: String = sources
        .lines()
        .filter(|line| !line.trim_start().starts_with("//"))
        .collect::<Vec<_>>()
        .join("\n");
    for cause in [
        "OwnerNotSelected",
        "StaleKey",
        "ForeignDeclaration",
        "UnreducedRational",
    ] {
        let raised = format!("SemanticGraphCause::{cause}");
        assert!(!code.contains(&raised), "{cause} has a raise site");
        assert!(
            code.contains(cause),
            "{cause} is not declared in the vocabulary"
        );
    }
}
