//! FR-012 carried compiler vocabulary: node keys, refusal vocabularies, semantic-graph causes and
//! normalized unit values, through the public `exact` surface.
#![cfg(feature = "exact")]

use ix_trace_rs::trace;
use proc_macro2::{TokenStream, TokenTree};
use quire_contract_runtime::exact::{
    CompoundUnit, Dimension, EnumDeclaration, EnumValue, PackageRefusalCode, SelectionRefusalCode,
    UnitDeclaration, UnitGraph, COMPOUND_UNIT_DOMAIN,
};
use quire_exact::{Integer, NodeKey, Rational, NODE_KEY_DOMAIN};
use quire_semantic_value::semantic_node::{InvalidSemanticGraph, SemanticGraphCause};
use std::path::{Path, PathBuf};
use syn::visit::{self, Visit};

fn rust_sources(directory: &Path, sources: &mut Vec<PathBuf>) {
    for entry in std::fs::read_dir(directory).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            rust_sources(&path, sources);
        } else if path.extension().is_some_and(|extension| extension == "rs") {
            sources.push(path);
        }
    }
}

fn is_test_only(attrs: &[syn::Attribute]) -> bool {
    attrs.iter().any(|attr| {
        attr.path().is_ident("cfg")
            && attr
                .parse_args::<syn::Path>()
                .is_ok_and(|path| path.is_ident("test"))
    })
}

fn item_attrs(item: &syn::Item) -> &[syn::Attribute] {
    match item {
        syn::Item::Const(x) => &x.attrs,
        syn::Item::Enum(x) => &x.attrs,
        syn::Item::ExternCrate(x) => &x.attrs,
        syn::Item::Fn(x) => &x.attrs,
        syn::Item::ForeignMod(x) => &x.attrs,
        syn::Item::Impl(x) => &x.attrs,
        syn::Item::Macro(x) => &x.attrs,
        syn::Item::Mod(x) => &x.attrs,
        syn::Item::Static(x) => &x.attrs,
        syn::Item::Struct(x) => &x.attrs,
        syn::Item::Trait(x) => &x.attrs,
        syn::Item::TraitAlias(x) => &x.attrs,
        syn::Item::Type(x) => &x.attrs,
        syn::Item::Union(x) => &x.attrs,
        syn::Item::Use(x) => &x.attrs,
        _ => &[],
    }
}

fn use_exposes_key_owner(tree: &syn::UseTree, from_kernel: bool) -> bool {
    match tree {
        syn::UseTree::Path(path) => {
            use_exposes_key_owner(&path.tree, from_kernel || path.ident == "quire_exact")
        }
        syn::UseTree::Name(name) => name.ident == "NodeKey" || name.ident == "NODE_KEY_DOMAIN",
        syn::UseTree::Rename(rename) => {
            rename.rename == "NodeKey"
                || rename.ident == "NodeKey"
                || rename.ident == "NODE_KEY_DOMAIN"
        }
        syn::UseTree::Glob(_) => from_kernel,
        syn::UseTree::Group(group) => group
            .items
            .iter()
            .any(|item| use_exposes_key_owner(item, from_kernel)),
    }
}

fn use_defines_node_key_alias(tree: &syn::UseTree) -> bool {
    match tree {
        syn::UseTree::Path(path) => use_defines_node_key_alias(&path.tree),
        syn::UseTree::Rename(rename) => rename.rename == "NodeKey" && rename.ident != "NodeKey",
        syn::UseTree::Group(group) => group.items.iter().any(use_defines_node_key_alias),
        _ => false,
    }
}

#[derive(Default)]
struct ProductionKeyInspection {
    findings: Vec<String>,
}

impl<'ast> Visit<'ast> for ProductionKeyInspection {
    fn visit_macro(&mut self, item: &'ast syn::Macro) {
        inspect_macro_tokens(item.tokens.clone(), &mut self.findings);
    }

    fn visit_item(&mut self, item: &'ast syn::Item) {
        if is_test_only(item_attrs(item)) {
            return;
        }
        match item {
            syn::Item::Struct(x) if x.ident == "NodeKey" => {
                self.findings.push("struct NodeKey".into())
            }
            syn::Item::Enum(x) if x.ident == "NodeKey" => self.findings.push("enum NodeKey".into()),
            syn::Item::Union(x) if x.ident == "NodeKey" => {
                self.findings.push("union NodeKey".into())
            }
            syn::Item::Type(x) if x.ident == "NodeKey" => self.findings.push("type NodeKey".into()),
            syn::Item::Const(x) if x.ident == "NODE_KEY_DOMAIN" => {
                self.findings.push("local NODE_KEY_DOMAIN".into())
            }
            syn::Item::Use(x) => {
                if use_defines_node_key_alias(&x.tree) {
                    self.findings.push("local NodeKey use alias".into());
                }
                if matches!(x.vis, syn::Visibility::Public(_))
                    && use_exposes_key_owner(&x.tree, false)
                {
                    self.findings.push("public key owner re-export".into());
                }
            }
            _ => {}
        }
        visit::visit_item(self, item);
    }

    fn visit_impl_item(&mut self, item: &'ast syn::ImplItem) {
        if let syn::ImplItem::Fn(method) = item {
            if is_test_only(&method.attrs) {
                return;
            }
            let name = method.sig.ident.to_string();
            if is_key_constructor_or_hex_decoder(&name)
                || has_text_or_bytes_to_digest_signature(&method.sig)
            {
                self.findings
                    .push(format!("key constructor or hex decoder: {name}"));
            }
        }
        visit::visit_impl_item(self, item);
    }

    fn visit_trait_item(&mut self, item: &'ast syn::TraitItem) {
        if let syn::TraitItem::Fn(method) = item {
            if is_test_only(&method.attrs) {
                return;
            }
        }
        visit::visit_trait_item(self, item);
    }

    fn visit_foreign_item(&mut self, item: &'ast syn::ForeignItem) {
        if let syn::ForeignItem::Fn(function) = item {
            if is_test_only(&function.attrs) {
                return;
            }
        }
        visit::visit_foreign_item(self, item);
    }

    fn visit_path(&mut self, path: &'ast syn::Path) {
        for segment in &path.segments {
            let name = segment.ident.to_string();
            if matches!(
                name.as_str(),
                "from_digest"
                    | "decode_admitted"
                    | "from_bytes"
                    | "from_hex"
                    | "from_str_radix"
                    | "to_digit"
            ) {
                self.findings
                    .push(format!("key construction or hex decoding: {name}"));
            }
        }
        visit::visit_path(self, path);
    }

    fn visit_item_fn(&mut self, item: &'ast syn::ItemFn) {
        let name = item.sig.ident.to_string();
        if is_key_constructor_or_hex_decoder(&name)
            || has_text_or_bytes_to_digest_signature(&item.sig)
        {
            self.findings
                .push(format!("key constructor or hex decoder: {name}"));
        }
        visit::visit_item_fn(self, item);
    }
}

fn inspect_macro_tokens(tokens: TokenStream, findings: &mut Vec<String>) {
    fn collect_idents(tokens: TokenStream, idents: &mut Vec<String>) {
        for token in tokens {
            match token {
                TokenTree::Ident(ident) => idents.push(ident.to_string()),
                TokenTree::Group(group) => collect_idents(group.stream(), idents),
                _ => {}
            }
        }
    }

    let mut idents = Vec::new();
    collect_idents(tokens, &mut idents);
    for name in &idents {
        if matches!(
            name.as_str(),
            "from_digest"
                | "decode_admitted"
                | "from_bytes"
                | "from_hex"
                | "from_str_radix"
                | "to_digit"
        ) {
            findings.push(format!("key construction or hex decoding in macro: {name}"));
        }
    }
    for pair in idents.windows(2) {
        if matches!(pair[0].as_str(), "struct" | "enum" | "type" | "union") && pair[1] == "NodeKey"
        {
            findings.push(format!("local {} NodeKey in macro", pair[0]));
        }
    }
}

fn is_key_constructor_or_hex_decoder(name: &str) -> bool {
    matches!(
        name,
        "from_digest" | "decode_admitted" | "from_bytes" | "from_hex"
    ) || (name.contains("hex") && (name.contains("key") || name.contains("decode")))
}

fn has_text_or_bytes_to_digest_signature(signature: &syn::Signature) -> bool {
    let takes_text_or_bytes = signature.inputs.iter().any(|input| match input {
        syn::FnArg::Typed(argument) => match argument.ty.as_ref() {
            syn::Type::Reference(reference) => match reference.elem.as_ref() {
                syn::Type::Path(path) => path.path.is_ident("str"),
                syn::Type::Slice(slice) => {
                    matches!(slice.elem.as_ref(), syn::Type::Path(path) if path.path.is_ident("u8"))
                }
                _ => false,
            },
            _ => false,
        },
        syn::FnArg::Receiver(_) => false,
    });
    takes_text_or_bytes
        && matches!(&signature.output, syn::ReturnType::Type(_, output) if contains_digest(output))
}

fn contains_digest(ty: &syn::Type) -> bool {
    match ty {
        syn::Type::Array(array) => {
            matches!(array.elem.as_ref(), syn::Type::Path(path) if path.path.is_ident("u8"))
                && matches!(&array.len, syn::Expr::Lit(expr) if matches!(&expr.lit, syn::Lit::Int(size) if size.base10_parse::<usize>().is_ok_and(|value| value == 32)))
        }
        syn::Type::Path(path) => path.path.segments.iter().any(|segment| {
            if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                args.args.iter().any(|arg| {
                    matches!(arg, syn::GenericArgument::Type(inner) if contains_digest(inner))
                })
            } else {
                false
            }
        }),
        _ => false,
    }
}

#[derive(Default)]
struct RetiredConstructorInspection {
    findings: Vec<String>,
}

impl<'ast> Visit<'ast> for RetiredConstructorInspection {
    fn visit_item_fn(&mut self, item: &'ast syn::ItemFn) {
        let name = item.sig.ident.to_string();
        if name.starts_with("tc_033_")
            && (name.contains("parser")
                || name.contains("roundtrip")
                || name.contains("round_trip"))
        {
            self.findings.push(format!("retired TC-033 test: {name}"));
        }
        visit::visit_item_fn(self, item);
    }

    fn visit_path(&mut self, path: &'ast syn::Path) {
        if path.segments.iter().any(|segment| {
            matches!(
                segment.ident.to_string().as_str(),
                "from_bytes" | "from_hex"
            )
        }) {
            self.findings.push("retired constructor in TC-033".into());
        }
        visit::visit_path(self, path);
    }
}

#[trace("TC-033", "FR-012-AC-6")]
#[test]
fn tc_033_runtime_has_no_local_node_key_constructor() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut sources = Vec::new();
    for directory in ["src", "verification", "examples", "measurement", "scripts"] {
        rust_sources(&root.join(directory), &mut sources);
    }
    assert!(!sources.is_empty());
    let mut failures = Vec::new();
    for path in sources {
        let source = std::fs::read_to_string(&path).unwrap();
        let parsed =
            syn::parse_file(&source).unwrap_or_else(|error| panic!("{}: {error}", path.display()));
        let mut inspection = ProductionKeyInspection::default();
        inspection.visit_file(&parsed);
        failures.extend(
            inspection
                .findings
                .into_iter()
                .map(|finding| format!("{}: {finding}", path.display())),
        );
    }
    let tc_033 = root.join("tests/exact_vocabulary.rs");
    let source = std::fs::read_to_string(&tc_033).unwrap();
    let parsed = syn::parse_file(&source).unwrap();
    let mut retired = RetiredConstructorInspection::default();
    retired.visit_file(&parsed);
    failures.extend(retired.findings);
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

fn key(byte: u8) -> NodeKey {
    NodeKey::from_digest([byte; 32])
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

#[trace("TC-033", "FR-012-AC-4")]
#[test]
fn tc_033_admit_raises_zero_exponent() {
    assert_eq!(
        zero_exponent_admit().unwrap_err().cause,
        SemanticGraphCause::ZeroExponent
    );
}

#[trace("TC-033", "FR-012-AC-4")]
#[test]
fn tc_033_admit_raises_duplicate_term() {
    assert_eq!(
        duplicate_term_admit().unwrap_err().cause,
        SemanticGraphCause::DuplicateTerm
    );
}

#[trace("TC-033", "FR-012-AC-4")]
#[test]
fn tc_033_admit_raises_unsorted_terms() {
    assert_eq!(
        unsorted_terms_admit().unwrap_err().cause,
        SemanticGraphCause::UnsortedTerms
    );
}

#[trace("TC-033", "FR-012-AC-4")]
#[test]
fn tc_033_admit_raises_zero_scale() {
    assert_eq!(
        zero_scale_admit().unwrap_err().cause,
        SemanticGraphCause::ZeroScale
    );
}

#[trace("TC-033", "FR-012-AC-4")]
#[test]
fn tc_033_admit_raises_non_identity_root() {
    assert_eq!(
        non_identity_root_admit().unwrap_err().cause,
        SemanticGraphCause::NonIdentityRoot
    );
}

#[trace("TC-033", "FR-012-AC-4")]
#[test]
fn tc_033_admit_raises_duplicate_node() {
    assert_eq!(
        duplicate_node_admit().unwrap_err().cause,
        SemanticGraphCause::DuplicateNode
    );
}

#[trace("TC-033", "FR-012-AC-4")]
#[test]
fn tc_033_admit_raises_unknown_dimension() {
    assert_eq!(
        unknown_dimension_admit().unwrap_err().cause,
        SemanticGraphCause::UnknownDimension
    );
}

#[trace("TC-033", "FR-012-AC-4")]
#[test]
fn tc_033_admit_raises_non_base_dimension_term() {
    assert_eq!(
        non_base_dimension_term_admit().unwrap_err().cause,
        SemanticGraphCause::NonBaseDimensionTerm
    );
}

#[trace("TC-033", "FR-012-AC-4")]
#[test]
fn tc_033_admit_raises_unknown_target() {
    assert_eq!(
        unknown_target_admit().unwrap_err().cause,
        SemanticGraphCause::UnknownTarget
    );
}

#[trace("TC-033", "FR-012-AC-4")]
#[test]
fn tc_033_admit_raises_cross_dimension_target() {
    assert_eq!(
        cross_dimension_target_admit().unwrap_err().cause,
        SemanticGraphCause::CrossDimensionTarget
    );
}

#[trace("TC-033", "FR-012-AC-4")]
#[test]
fn tc_033_admit_raises_missing_root() {
    assert_eq!(
        missing_root_admit().unwrap_err().cause,
        SemanticGraphCause::MissingRoot
    );
}

#[trace("TC-033", "FR-012-AC-4")]
#[test]
fn tc_033_admit_raises_duplicate_root() {
    assert_eq!(
        duplicate_root_admit().unwrap_err().cause,
        SemanticGraphCause::DuplicateRoot
    );
}

#[trace("TC-033", "FR-012-AC-4")]
#[test]
fn tc_033_admit_raises_target_cycle() {
    assert_eq!(
        target_cycle_admit().unwrap_err().cause,
        SemanticGraphCause::TargetCycle
    );
}

#[trace("TC-033", "FR-012-AC-4")]
#[test]
fn tc_033_enum_declaration_member_raises_undeclared_case() {
    assert_eq!(
        undeclared_case_member().unwrap_err().cause,
        SemanticGraphCause::UndeclaredCase
    );
}

#[trace("TC-033", "FR-012-AC-4")]
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

#[trace("TC-033", "FR-012-AC-2")]
#[test]
fn tc_033_digest_domains_equal_their_normative_strings() {
    assert_eq!(NODE_KEY_DOMAIN, "quire.checked-semantic-node/v1");
    assert_eq!(COMPOUND_UNIT_DOMAIN, "quire.value.compound-unit/v1");
}

#[trace("TC-033", "FR-012-AC-3")]
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

#[trace("TC-033", "FR-012-AC-3")]
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

#[trace("TC-033", "FR-012-AC-3")]
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

#[trace("TC-033", "FR-012-AC-3")]
#[test]
fn tc_033_package_refusal_code_as_str_is_invalid_package() {
    assert_eq!(
        PackageRefusalCode::InvalidPackage.as_str(),
        "invalid_package"
    );
}

#[trace("TC-033", "FR-012-AC-5")]
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
    assert_ne!(forward, low.power(&int(2)));
    assert_ne!(forward, low.power(&int(3)).multiply(&high.power(&int(2))));
    assert_ne!(forward, low.power(&int(3)).multiply(&high.power(&int(3))));
    assert_ne!(forward, low.power(&int(2)).multiply(&high.power(&int(2))));

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

#[trace("TC-033", "FR-012-AC-5")]
#[test]
fn tc_033_compound_unit_is_independent_of_admission_order_and_ascending() {
    let build =
        |dimension_order: [u8; 2], unit_order: [u8; 2], exponents: [i128; 2]| -> CompoundUnit {
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
                .compound_unit(&[(key(11), int(exponents[0])), (key(12), int(exponents[1]))])
                .unwrap()
        };

    let forward = build([1, 2], [1, 2], [1, 1]);
    let backward = build([2, 1], [2, 1], [1, 1]);
    assert_eq!(forward, backward);
    assert_ne!(forward, CompoundUnit::dimensionless());
    assert_ne!(build([1, 2], [1, 2], [2, 3]), build([2, 1], [2, 1], [3, 2]));
    assert_ne!(build([1, 2], [1, 2], [2, 3]), build([2, 1], [2, 1], [3, 3]));
    assert_ne!(build([1, 2], [1, 2], [2, 3]), build([2, 1], [2, 1], [2, 2]));

    let terms: Vec<(NodeKey, Integer)> = forward
        .terms()
        .map(|(node, exponent)| (node, exponent.clone()))
        .collect();
    assert_eq!(terms, [(key(11), int(1)), (key(12), int(1))]);
    assert!(terms.iter().all(|(_, exponent)| !exponent.is_zero()));

    assert!(CompoundUnit::dimensionless().terms().next().is_none());
}

#[trace("TC-033", "FR-012-AC-4")]
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
