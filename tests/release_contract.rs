use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use syn::visit::{self, Visit};
use syn::{Expr, ExprCall, ExprMethodCall, ImplItem, Item, Type, UseTree, Visibility};

const CARGO_MANIFEST: &str = include_str!("../Cargo.toml");
const CRATE_ROOT: &str = include_str!("../src/lib.rs");
const FOOTPRINT_MANIFEST: &str = include_str!("../measurement/footprint/Cargo.toml");
const FOOTPRINT_HARNESS: &str = include_str!("../measurement/footprint/src/lib.rs");
const FOOTPRINT_AUDIT: &str = include_str!("../scripts/check_linked_footprint.sh");
const MAKEFILE: &str = include_str!("../Makefile");
const TEST_ONLY_ACCOUNTING_SOURCE: &str = "src/accounting_tests.rs";

/// Trace: TC-005, FR-003-AC-2, NFR-001-AC-1, StR-001-VC-2
#[test]
fn tc_005_optional_surface_is_explicitly_feature_gated() {
    assert!(CARGO_MANIFEST.contains("default = []"));
    assert!(CARGO_MANIFEST.contains("proptest = [\"std\", \"dep:proptest\"]"));
}

/// Trace: TC-007, NFR-001-AC-2, NFR-001-AC-3, NFR-002-AC-2, NFR-002-AC-4, StR-001-VC-2
#[test]
fn tc_007_release_controls_are_mandatory() {
    assert!(CARGO_MANIFEST.contains("license = \"MIT OR Apache-2.0\""));
    assert!(CARGO_MANIFEST.contains("publish = false"));
    assert!(CRATE_ROOT.contains("#![no_std]"));
    assert!(CRATE_ROOT.contains("#![forbid(unsafe_code)]"));
    assert!(CARGO_MANIFEST.contains("panic = \"abort\""));
    assert!(!FOOTPRINT_MANIFEST.contains("[profile.release]"));
    assert!(MAKEFILE.contains("python3 -m unittest discover -s tests -p 'test_*.py'"));
    assert!(FOOTPRINT_AUDIT.contains("readonly minimum_bytes=500"));
    assert!(FOOTPRINT_AUDIT.contains("section_bytes < minimum_bytes"));
    assert!(FOOTPRINT_AUDIT.contains("rust_begin_unwind"));
    let footprint_call_set = footprint_calls(FOOTPRINT_HARNESS);
    assert!(!footprint_calls(
        "// operators::checked_add(1, 2);\n\
             pub extern \"C\" fn quire_runtime_footprint(_: u32) -> u64 { 0 }"
    )
    .contains("operators::checked_add"));
    for call in [
        "RequirementId::new",
        "RevisionId::new",
        "ContractIdentity::new",
        "ExecutionPoint::new",
        "ClauseId::new",
        "FailureDetail::new",
        "Observation::new",
        "VerdictContext::new",
        "Verdict::passed",
        "Verdict::failed_postcondition",
        "Verdict::rejected_precondition",
        "CampaignCounts::new",
        "CampaignReport::new",
        ".record_verdict",
        ".record_discard",
        "operators::and_short_circuit",
        "operators::or_short_circuit",
        "operators::implies_short_circuit",
        "operators::and_total",
        "operators::or_total",
        "operators::implies_total",
        "operators::option_ref",
        "operators::option_copied",
        "operators::index",
        "operators::checked_add",
        "operators::checked_sub",
        "operators::checked_mul",
        "operators::checked_div",
        "operators::checked_rem",
    ] {
        assert!(
            footprint_call_set.contains(call),
            "footprint population no longer calls {call}"
        );
    }
}

/// Trace: TC-008, FR-001-AC-3, FR-004-AC-3, NFR-002-AC-3
#[test]
fn tc_008_evidence_model_is_non_exhaustive_and_opaque() {
    let verdicts = include_str!("../src/verdict.rs");
    let observations = include_str!("../src/observation.rs");
    let accounting = include_str!("../src/accounting.rs");

    assert!(accounting.contains("#[cfg(test)]\n#[path = \"accounting_tests.rs\"]\nmod tests;"));

    let verdict_offsets = [
        assert_non_exhaustive(verdicts, "pub enum VerdictKind {", "VerdictKind"),
        assert_non_exhaustive(verdicts, "pub enum Verdict<'a> {", "Verdict"),
    ];
    assert_ne!(verdict_offsets[0], verdict_offsets[1]);
    for (declaration, enum_name) in [
        ("pub enum ClauseKind {", "ClauseKind"),
        ("pub enum ClauseOutcome {", "ClauseOutcome"),
        ("pub enum FailureKind {", "FailureKind"),
    ] {
        assert_non_exhaustive(observations, declaration, enum_name);
    }
    for forbidden in [
        "pub accepted:",
        "pub rejected:",
        "pub failed:",
        "pub discarded:",
        "pub identity:",
        "pub counts:",
    ] {
        assert!(
            !accounting.contains(forbidden),
            "public mutation seam: {forbidden}"
        );
    }

    let runtime_sources = runtime_source_files();
    assert!(runtime_sources
        .iter()
        .any(|(path, _)| path == "verification/kani.rs"));
    let surface = crate_accounting_surface(&runtime_sources);
    assert_eq!(
        surface.public_items,
        [
            "src/accounting.rs::struct CampaignCounts",
            "src/accounting.rs::struct CampaignReport",
            "src/accounting.rs::struct IdentityMismatch",
            "src/identity.rs::struct ContractIdentity",
            "src/lib.rs::const RUNTIME_CONTRACT_VERSION",
            "src/lib.rs::mod accounting",
            "src/lib.rs::mod identity",
            "src/lib.rs::mod observation",
            "src/lib.rs::mod operators",
            "src/lib.rs::mod proptest_adapter",
            "src/lib.rs::mod verdict",
            "src/lib.rs::use accounting::CampaignCounts",
            "src/lib.rs::use accounting::CampaignReport",
            "src/lib.rs::use accounting::IdentityMismatch",
            "src/lib.rs::use identity::ClauseId",
            "src/lib.rs::use identity::ContractIdentity",
            "src/lib.rs::use identity::ExecutionPoint",
            "src/lib.rs::use identity::RequirementId",
            "src/lib.rs::use identity::RevisionId",
            "src/lib.rs::use observation::ClauseKind",
            "src/lib.rs::use observation::ClauseOutcome",
            "src/lib.rs::use observation::FailureDetail",
            "src/lib.rs::use observation::FailureKind",
            "src/lib.rs::use observation::Observation",
            "src/lib.rs::use verdict::Verdict",
            "src/lib.rs::use verdict::VerdictContext",
            "src/lib.rs::use verdict::VerdictKind",
            "src/observation.rs::enum ClauseKind",
            "src/observation.rs::enum ClauseOutcome",
            "src/observation.rs::enum FailureKind",
            "src/observation.rs::struct FailureDetail",
            "src/observation.rs::struct Observation",
            "src/operators.rs::fn and_short_circuit",
            "src/operators.rs::fn and_total",
            "src/operators.rs::fn checked_add",
            "src/operators.rs::fn checked_div",
            "src/operators.rs::fn checked_mul",
            "src/operators.rs::fn checked_rem",
            "src/operators.rs::fn checked_sub",
            "src/operators.rs::fn implies_short_circuit",
            "src/operators.rs::fn implies_total",
            "src/operators.rs::fn index",
            "src/operators.rs::fn option_copied",
            "src/operators.rs::fn option_ref",
            "src/operators.rs::fn or_short_circuit",
            "src/operators.rs::fn or_total",
            "src/operators.rs::trait CheckedInteger",
            "src/proptest_adapter.rs::fn adapt",
            "src/proptest_adapter.rs::fn adapt_recording",
            "src/verdict.rs::enum Verdict",
            "src/verdict.rs::enum VerdictKind",
            "src/verdict.rs::struct VerdictContext",
        ]
    );
    assert_eq!(surface.inherent_blocks.get("CampaignCounts"), Some(&1));
    assert_eq!(surface.inherent_blocks.get("CampaignReport"), Some(&1));
    assert_eq!(
        surface.trait_impls.get("CampaignCounts"),
        Some(&vec!["Display".to_owned()])
    );
    assert_eq!(
        surface.trait_impls.get("CampaignReport"),
        Some(&vec!["Display".to_owned()])
    );
    assert_eq!(
        surface.public_accounting_functions,
        ["src/proptest_adapter.rs::adapt_recording"]
    );
    assert_eq!(
        surface.public_methods.get("CampaignCounts"),
        Some(
            &[
                "new",
                "accepted",
                "rejected",
                "failed",
                "discarded",
                "total"
            ]
            .map(String::from)
            .to_vec()
        )
    );
    assert_eq!(
        surface.public_methods.get("CampaignReport"),
        Some(
            &[
                "new",
                "identity",
                "counts",
                "record_verdict",
                "record_discard"
            ]
            .map(String::from)
            .to_vec()
        )
    );

    assert_eq!(
        surface.macros,
        [
            "src/identity.rs::define:borrowed_id",
            "src/identity.rs::invoke:borrowed_id",
            "src/identity.rs::invoke:borrowed_id",
            "src/identity.rs::invoke:borrowed_id",
            "src/identity.rs::invoke:borrowed_id",
            "src/operators.rs::define:checked_integer",
            "src/operators.rs::inline-mod:sealed",
            "src/operators.rs::invoke:checked_integer",
        ]
    );

    let mutation_probe = crate_accounting_surface(&[(
        "src/accounting.rs".to_owned(),
        "pub struct CampaignReport;\n\
         pub fn restate_census(_: &mut CampaignReport) {}\n\
         impl CampaignReport { pub fn first(&mut self) {} }\n\
         impl CampaignReport { pub const fn second(&mut self) {} }"
            .to_owned(),
    )]);
    assert!(mutation_probe
        .public_items
        .contains(&"src/accounting.rs::fn restate_census".to_owned()));
    assert_eq!(
        mutation_probe.inherent_blocks.get("CampaignReport"),
        Some(&2)
    );
    assert_eq!(
        mutation_probe.public_methods.get("CampaignReport"),
        Some(&vec!["first".to_owned(), "second".to_owned()])
    );

    let bypass_probe = crate_accounting_surface(&[
        (
            "src/accounting.rs".to_owned(),
            "pub struct CampaignReport<'a>(&'a ());\n\
             type Census<'a> = CampaignReport<'a>;\n\
             impl<'a> CampaignReport<'a> { pub fn first(&mut self) {} }\n\
             impl<'a> Census<'a> { pub fn alias_set(&mut self) {} }\n\
             impl crate::verdict::Restate for CampaignReport<'_> { fn restate(&mut self) {} }"
                .to_owned(),
        ),
        (
            "src/accounting_extra.rs".to_owned(),
            "impl CampaignReport<'_> { pub fn extra_set(&mut self) {} }".to_owned(),
        ),
        (
            "src/lib.rs".to_owned(),
            "pub fn restate_census(_: &mut CampaignReport<'_>) {}".to_owned(),
        ),
        (
            "src/verdict.rs".to_owned(),
            "pub trait Restate { fn restate(&mut self); }".to_owned(),
        ),
    ]);
    assert_eq!(bypass_probe.inherent_blocks.get("CampaignReport"), Some(&3));
    assert_eq!(
        bypass_probe.trait_impls.get("CampaignReport"),
        Some(&vec!["Restate".to_owned()])
    );
    assert_eq!(
        bypass_probe.public_accounting_functions,
        ["src/lib.rs::restate_census"]
    );

    let alias_and_reference_probe = crate_accounting_surface(&[
        (
            "src/accounting.rs".to_owned(),
            "pub struct CampaignReport<'a>(&'a ());\n\
             pub type Census<'a> = CampaignReport<'a>;"
                .to_owned(),
        ),
        (
            "src/observation.rs".to_owned(),
            "pub fn restated<'a>(value: &Census<'a>) -> Census<'a> { *value }\n\
             impl Restate for &mut CampaignReport<'_> { fn restate(&mut self) {} }"
                .to_owned(),
        ),
    ]);
    assert_eq!(
        alias_and_reference_probe.public_accounting_functions,
        ["src/observation.rs::restated"]
    );
    assert_eq!(
        alias_and_reference_probe.trait_impls.get("CampaignReport"),
        Some(&vec!["Restate".to_owned()])
    );

    let macro_probe = crate_accounting_surface(&[(
        "src/accounting.rs".to_owned(),
        "macro_rules! census_setter { () => { pub fn set_counts(&mut self) {} } }\n\
         pub struct CampaignReport;\n\
         impl CampaignReport { census_setter!(); }"
            .to_owned(),
    )]);
    assert!(!macro_probe.macros.is_empty());

    let use_probe = crate_accounting_surface(&[(
        "src/lib.rs".to_owned(),
        "pub use accounting::{CampaignCounts as Tally, CampaignReport};".to_owned(),
    )]);
    assert_eq!(
        use_probe.public_items,
        [
            "src/lib.rs::use accounting::CampaignCounts as Tally",
            "src/lib.rs::use accounting::CampaignReport",
        ]
    );
}

fn assert_non_exhaustive(source: &str, declaration: &str, enum_name: &str) -> usize {
    let declaration_at = source.find(declaration).expect("enum declaration exists");
    let attributes = &source[..declaration_at];
    let nearest_non_exhaustive = attributes
        .rfind("#[non_exhaustive]")
        .expect("non-exhaustive attribute exists");
    let intervening = &attributes[nearest_non_exhaustive..];
    assert!(
        !intervening.contains("pub enum"),
        "{enum_name} is not the enum governed by the nearest non-exhaustive attribute"
    );
    declaration_at
}

#[derive(Debug, Eq, PartialEq)]
struct AccountingSurface {
    public_items: Vec<String>,
    inherent_blocks: BTreeMap<String, usize>,
    public_methods: BTreeMap<String, Vec<String>>,
    trait_impls: BTreeMap<String, Vec<String>>,
    public_accounting_functions: Vec<String>,
    macros: Vec<String>,
}

fn runtime_source_files() -> Vec<(String, String)> {
    fn visit(directory: &Path, files: &mut Vec<(String, String)>) {
        let mut entries = fs::read_dir(directory)
            .expect("runtime source directory is readable")
            .map(|entry| entry.expect("runtime source entry is readable"))
            .collect::<Vec<_>>();
        entries.sort_by_key(|entry| entry.path());
        for entry in entries {
            let path = entry.path();
            if path.is_dir() {
                visit(&path, files);
            } else if path.extension().and_then(|value| value.to_str()) == Some("rs") {
                let relative = path
                    .strip_prefix(env!("CARGO_MANIFEST_DIR"))
                    .expect("runtime source is beneath the manifest")
                    .to_string_lossy()
                    .replace('\\', "/");
                if relative != TEST_ONLY_ACCOUNTING_SOURCE {
                    files.push((
                        relative,
                        fs::read_to_string(path).expect("runtime source is readable"),
                    ));
                }
            }
        }
    }

    let mut files = Vec::new();
    visit(
        &Path::new(env!("CARGO_MANIFEST_DIR")).join("src"),
        &mut files,
    );
    let manifest = Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut known = files
        .iter()
        .map(|(path, _)| path.clone())
        .collect::<BTreeSet<_>>();
    let mut index = 0;
    while index < files.len() {
        let (relative, source) = &files[index];
        let parsed = syn::parse_file(source).expect("runtime source parses");
        let parent = manifest
            .join(relative)
            .parent()
            .expect("runtime source has a parent")
            .to_path_buf();
        let mut attached = Vec::<PathBuf>::new();
        for item in parsed.items {
            let Item::Mod(module) = item else {
                continue;
            };
            for attribute in module.attrs {
                if !attribute.path().is_ident("path") {
                    continue;
                }
                let syn::Meta::NameValue(value) = attribute.meta else {
                    continue;
                };
                let Expr::Lit(value) = value.value else {
                    continue;
                };
                let syn::Lit::Str(value) = value.lit else {
                    continue;
                };
                attached.push(parent.join(value.value()));
            }
        }
        for path in attached {
            let path = path
                .canonicalize()
                .expect("path-attached runtime source resolves");
            let relative = path
                .strip_prefix(manifest)
                .expect("path-attached runtime source stays beneath the manifest")
                .to_string_lossy()
                .replace('\\', "/");
            if known.insert(relative.clone()) && relative != TEST_ONLY_ACCOUNTING_SOURCE {
                files.push((
                    relative,
                    fs::read_to_string(path).expect("path-attached runtime source is readable"),
                ));
            }
        }
        index += 1;
    }
    files.sort_by(|left, right| left.0.cmp(&right.0));
    files
}

fn crate_accounting_surface(sources: &[(String, String)]) -> AccountingSurface {
    let mut public_items = Vec::new();
    let mut inherent_blocks = BTreeMap::new();
    let mut public_methods: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut trait_impls: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut public_accounting_functions = Vec::new();
    let mut macros = Vec::new();
    let mut aliases = BTreeMap::new();

    let parsed = sources
        .iter()
        .map(|(path, source)| {
            (
                path,
                syn::parse_file(source)
                    .unwrap_or_else(|error| panic!("runtime source {path} parses: {error}")),
            )
        })
        .collect::<Vec<_>>();

    for (_, file) in &parsed {
        for item in &file.items {
            if let Item::Type(alias) = item {
                if let Some(target) = type_name(&alias.ty) {
                    aliases.insert(alias.ident.to_string(), target);
                }
            }
        }
    }

    for (path, file) in parsed {
        for item in file.items {
            match item {
                Item::Struct(item) if matches!(item.vis, Visibility::Public(_)) => {
                    public_items.push(format!("{path}::struct {}", item.ident));
                }
                Item::Fn(item) if matches!(item.vis, Visibility::Public(_)) => {
                    public_items.push(format!("{path}::fn {}", item.sig.ident));
                    let mut names = TypeNameCollector::default();
                    names.visit_signature(&item.sig);
                    if names.names.iter().any(|name| {
                        let resolved = resolve_alias(name.clone(), &aliases);
                        resolved == "CampaignCounts" || resolved == "CampaignReport"
                    }) {
                        public_accounting_functions.push(format!("{path}::{}", item.sig.ident));
                    }
                }
                Item::Static(item) if matches!(item.vis, Visibility::Public(_)) => {
                    public_items.push(format!("{path}::static {}", item.ident));
                }
                Item::Const(item) if matches!(item.vis, Visibility::Public(_)) => {
                    public_items.push(format!("{path}::const {}", item.ident));
                }
                Item::Mod(item) => {
                    if matches!(item.vis, Visibility::Public(_)) {
                        public_items.push(format!("{path}::mod {}", item.ident));
                    }
                    if item.content.is_some() {
                        macros.push(format!("{path}::inline-mod:{}", item.ident));
                    }
                }
                Item::Enum(item) if matches!(item.vis, Visibility::Public(_)) => {
                    public_items.push(format!("{path}::enum {}", item.ident));
                }
                Item::Type(item) if matches!(item.vis, Visibility::Public(_)) => {
                    public_items.push(format!("{path}::type {}", item.ident));
                }
                Item::Trait(item) if matches!(item.vis, Visibility::Public(_)) => {
                    public_items.push(format!("{path}::trait {}", item.ident));
                }
                Item::Union(item) if matches!(item.vis, Visibility::Public(_)) => {
                    public_items.push(format!("{path}::union {}", item.ident));
                }
                Item::Use(item) if matches!(item.vis, Visibility::Public(_)) => {
                    public_items.extend(public_use_labels(path, &item.tree));
                }
                Item::Macro(item) => macros.push(macro_label(path, &item)),
                Item::Impl(item) => {
                    if let Some((_, trait_path, _)) = &item.trait_ {
                        let mut names = TypeNameCollector::default();
                        names.visit_type(&item.self_ty);
                        let Some(name) = names.names.iter().find_map(|name| {
                            let resolved = resolve_alias(name.clone(), &aliases);
                            (resolved == "CampaignCounts" || resolved == "CampaignReport")
                                .then_some(resolved)
                        }) else {
                            continue;
                        };
                        if let Some(trait_name) = trait_path.segments.last() {
                            trait_impls
                                .entry(name)
                                .or_default()
                                .push(trait_name.ident.to_string());
                        }
                        continue;
                    }
                    let Some(name) = type_name(&item.self_ty) else {
                        continue;
                    };
                    let name = resolve_alias(name, &aliases);
                    if name != "CampaignCounts" && name != "CampaignReport" {
                        continue;
                    }
                    *inherent_blocks.entry(name.clone()).or_insert(0) += 1;
                    for implementation_item in item.items {
                        match implementation_item {
                            ImplItem::Fn(method) => {
                                if matches!(method.vis, Visibility::Public(_)) {
                                    public_methods
                                        .entry(name.clone())
                                        .or_default()
                                        .push(method.sig.ident.to_string());
                                }
                            }
                            ImplItem::Macro(item) => {
                                macros.push(format!(
                                    "{path}::impl:{}",
                                    item.mac.path.segments.last().expect("macro path").ident
                                ));
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }
    }

    public_items.sort();
    public_accounting_functions.sort();
    macros.sort();
    for values in trait_impls.values_mut() {
        values.sort();
    }

    AccountingSurface {
        public_items,
        inherent_blocks,
        public_methods,
        trait_impls,
        public_accounting_functions,
        macros,
    }
}

fn type_name(value: &Type) -> Option<String> {
    match value {
        Type::Path(path) => path
            .path
            .segments
            .last()
            .map(|segment| segment.ident.to_string()),
        Type::Reference(reference) => type_name(&reference.elem),
        Type::Group(group) => type_name(&group.elem),
        Type::Paren(paren) => type_name(&paren.elem),
        _ => None,
    }
}

fn resolve_alias(mut name: String, aliases: &BTreeMap<String, String>) -> String {
    let mut seen = BTreeSet::new();
    while let Some(target) = aliases.get(&name) {
        if !seen.insert(name.clone()) {
            break;
        }
        name.clone_from(target);
    }
    name
}

fn macro_label(path: &str, item: &syn::ItemMacro) -> String {
    match &item.ident {
        Some(name) => format!("{path}::define:{name}"),
        None => format!(
            "{path}::invoke:{}",
            item.mac.path.segments.last().expect("macro path").ident
        ),
    }
}

fn public_use_labels(path: &str, tree: &UseTree) -> Vec<String> {
    fn visit(tree: &UseTree, prefix: &str, labels: &mut Vec<String>) {
        match tree {
            UseTree::Path(branch) => {
                let prefix = if prefix.is_empty() {
                    branch.ident.to_string()
                } else {
                    format!("{prefix}::{}", branch.ident)
                };
                visit(&branch.tree, &prefix, labels);
            }
            UseTree::Name(leaf) => labels.push(if prefix.is_empty() {
                leaf.ident.to_string()
            } else {
                format!("{prefix}::{}", leaf.ident)
            }),
            UseTree::Rename(leaf) => {
                let source = if prefix.is_empty() {
                    leaf.ident.to_string()
                } else {
                    format!("{prefix}::{}", leaf.ident)
                };
                labels.push(format!("{source} as {}", leaf.rename));
            }
            UseTree::Glob(_) => labels.push(if prefix.is_empty() {
                "*".to_owned()
            } else {
                format!("{prefix}::*")
            }),
            UseTree::Group(group) => {
                for item in &group.items {
                    visit(item, prefix, labels);
                }
            }
        }
    }

    let mut labels = Vec::new();
    visit(tree, "", &mut labels);
    labels
        .into_iter()
        .map(|label| format!("{path}::use {label}"))
        .collect()
}

#[derive(Default)]
struct TypeNameCollector {
    names: BTreeSet<String>,
}

impl<'ast> Visit<'ast> for TypeNameCollector {
    fn visit_type_path(&mut self, path: &'ast syn::TypePath) {
        for segment in &path.path.segments {
            self.names.insert(segment.ident.to_string());
        }
        visit::visit_type_path(self, path);
    }
}

#[derive(Default)]
struct CallCollector {
    calls: BTreeSet<String>,
}

impl<'ast> Visit<'ast> for CallCollector {
    fn visit_expr_call(&mut self, expression: &'ast ExprCall) {
        if let Expr::Path(path) = &*expression.func {
            let call = path
                .path
                .segments
                .iter()
                .map(|segment| segment.ident.to_string())
                .collect::<Vec<_>>()
                .join("::");
            self.calls.insert(call);
        }
        visit::visit_expr_call(self, expression);
    }

    fn visit_expr_method_call(&mut self, expression: &'ast ExprMethodCall) {
        self.calls.insert(format!(".{}", expression.method));
        visit::visit_expr_method_call(self, expression);
    }
}

fn footprint_calls(source: &str) -> BTreeSet<String> {
    let file = syn::parse_file(source).expect("footprint harness parses");
    let entry = file
        .items
        .iter()
        .find_map(|item| match item {
            Item::Fn(function) if function.sig.ident == "quire_runtime_footprint" => Some(function),
            _ => None,
        })
        .expect("footprint entry function exists");
    let mut collector = CallCollector::default();
    collector.visit_block(&entry.block);
    collector.calls
}
