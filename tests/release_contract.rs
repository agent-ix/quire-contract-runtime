use std::collections::{BTreeMap, BTreeSet};

use syn::visit::{self, Visit};
use syn::{Expr, ExprCall, ExprMethodCall, ImplItem, Item, Type, Visibility};

const CARGO_MANIFEST: &str = include_str!("../Cargo.toml");
const CRATE_ROOT: &str = include_str!("../src/lib.rs");
const FOOTPRINT_MANIFEST: &str = include_str!("../measurement/footprint/Cargo.toml");
const FOOTPRINT_HARNESS: &str = include_str!("../measurement/footprint/src/lib.rs");

/// Trace: TC-005, FR-003-AC-2, NFR-001-AC-1, StR-001-VC-2
#[test]
fn tc_005_optional_surface_is_explicitly_feature_gated() {
    assert!(CARGO_MANIFEST.contains("default = []"));
    assert!(CARGO_MANIFEST.contains("proptest = [\"std\", \"dep:proptest\"]"));
}

/// Trace: TC-007, NFR-001-AC-2, NFR-001-AC-3, NFR-002-AC-2, StR-001-VC-2
#[test]
fn tc_007_release_controls_are_mandatory() {
    assert!(CARGO_MANIFEST.contains("license = \"MIT OR Apache-2.0\""));
    assert!(CARGO_MANIFEST.contains("publish = false"));
    assert!(CRATE_ROOT.contains("#![no_std]"));
    assert!(CRATE_ROOT.contains("#![forbid(unsafe_code)]"));
    assert!(CARGO_MANIFEST.contains("panic = \"abort\""));
    assert!(!FOOTPRINT_MANIFEST.contains("[profile.release]"));
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

    let surface = accounting_surface(accounting);
    assert_eq!(
        surface.public_items,
        [
            "struct CampaignCounts",
            "struct IdentityMismatch",
            "struct CampaignReport"
        ]
    );
    assert_eq!(surface.inherent_blocks.get("CampaignCounts"), Some(&1));
    assert_eq!(surface.inherent_blocks.get("CampaignReport"), Some(&1));
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

    let mutation_probe = accounting_surface(
        "pub struct CampaignReport;\n\
         pub fn restate_census(_: &mut CampaignReport) {}\n\
         impl CampaignReport { pub fn first(&mut self) {} }\n\
         impl CampaignReport { pub const fn second(&mut self) {} }",
    );
    assert!(mutation_probe
        .public_items
        .contains(&"fn restate_census".to_owned()));
    assert_eq!(
        mutation_probe.inherent_blocks.get("CampaignReport"),
        Some(&2)
    );
    assert_eq!(
        mutation_probe.public_methods.get("CampaignReport"),
        Some(&vec!["first".to_owned(), "second".to_owned()])
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
}

fn accounting_surface(source: &str) -> AccountingSurface {
    let file = syn::parse_file(source).expect("accounting source parses");
    let mut public_items = Vec::new();
    let mut inherent_blocks = BTreeMap::new();
    let mut public_methods: BTreeMap<String, Vec<String>> = BTreeMap::new();

    for item in file.items {
        match item {
            Item::Struct(item) if matches!(item.vis, Visibility::Public(_)) => {
                public_items.push(format!("struct {}", item.ident));
            }
            Item::Fn(item) if matches!(item.vis, Visibility::Public(_)) => {
                public_items.push(format!("fn {}", item.sig.ident));
            }
            Item::Static(item) if matches!(item.vis, Visibility::Public(_)) => {
                public_items.push(format!("static {}", item.ident));
            }
            Item::Const(item) if matches!(item.vis, Visibility::Public(_)) => {
                public_items.push(format!("const {}", item.ident));
            }
            Item::Mod(item) if matches!(item.vis, Visibility::Public(_)) => {
                public_items.push(format!("mod {}", item.ident));
            }
            Item::Enum(item) if matches!(item.vis, Visibility::Public(_)) => {
                public_items.push(format!("enum {}", item.ident));
            }
            Item::Type(item) if matches!(item.vis, Visibility::Public(_)) => {
                public_items.push(format!("type {}", item.ident));
            }
            Item::Trait(item) if matches!(item.vis, Visibility::Public(_)) => {
                public_items.push(format!("trait {}", item.ident));
            }
            Item::Union(item) if matches!(item.vis, Visibility::Public(_)) => {
                public_items.push(format!("union {}", item.ident));
            }
            Item::Use(item) if matches!(item.vis, Visibility::Public(_)) => {
                public_items.push("use".to_owned());
            }
            Item::Impl(item) if item.trait_.is_none() => {
                let Type::Path(self_type) = &*item.self_ty else {
                    continue;
                };
                let Some(segment) = self_type.path.segments.last() else {
                    continue;
                };
                let name = segment.ident.to_string();
                *inherent_blocks.entry(name.clone()).or_insert(0) += 1;
                for implementation_item in item.items {
                    if let ImplItem::Fn(method) = implementation_item {
                        if matches!(method.vis, Visibility::Public(_)) {
                            public_methods
                                .entry(name.clone())
                                .or_default()
                                .push(method.sig.ident.to_string());
                        }
                    }
                }
            }
            _ => {}
        }
    }

    AccountingSurface {
        public_items,
        inherent_blocks,
        public_methods,
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
