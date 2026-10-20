//! IR-882 ownership check for the semantic-node refusal vocabulary.
#![cfg(feature = "exact")]

use ix_trace_rs::trace;
use proc_macro2::{TokenStream, TokenTree};
use std::collections::BTreeSet;
use std::path::Path;
use syn::visit::{self, Visit};

fn owned_name(name: &str) -> bool {
    let name = name.strip_prefix("r#").unwrap_or(name);
    matches!(name, "SemanticGraphCause" | "InvalidSemanticGraph")
}

fn term_checker(name: &str) -> bool {
    let name = name.strip_prefix("r#").unwrap_or(name);
    matches!(
        name,
        "check_terms" | "validate_terms" | "verify_terms" | "ensure_terms"
    )
}

const SEMANTIC_GRAPH_CAUSES: [&str; 20] = [
    "NonCanonicalPreimage",
    "UnsortedUnorderedMembers",
    "OwnerNotSelected",
    "StaleKey",
    "ForeignDeclaration",
    "UndeclaredCase",
    "ZeroExponent",
    "DuplicateTerm",
    "UnsortedTerms",
    "UnreducedRational",
    "ZeroScale",
    "NonIdentityRoot",
    "DuplicateNode",
    "UnknownDimension",
    "NonBaseDimensionTerm",
    "UnknownTarget",
    "CrossDimensionTarget",
    "MissingRoot",
    "DuplicateRoot",
    "TargetCycle",
];

fn copied_cause_variants(item: &syn::ItemEnum) -> bool {
    let actual: BTreeSet<_> = item
        .variants
        .iter()
        .map(|variant| variant.ident.to_string())
        .collect();
    let owned: BTreeSet<_> = SEMANTIC_GRAPH_CAUSES
        .iter()
        .map(|name| (*name).to_owned())
        .collect();
    actual == owned
}

fn owned_target(path: &syn::Path) -> bool {
    path.segments.last().is_some_and(|segment| {
        let name = segment.ident.to_string();
        owned_name(&name) || term_checker(&name)
    })
}

#[derive(Default)]
struct TermRefusals(BTreeSet<String>);

impl<'ast> Visit<'ast> for TermRefusals {
    fn visit_expr_call(&mut self, call: &'ast syn::ExprCall) {
        if let syn::Expr::Path(path) = call.func.as_ref() {
            if path.path.is_ident("Err") {
                for argument in &call.args {
                    self.visit_expr(argument);
                }
            }
        }
    }

    fn visit_expr_path(&mut self, path: &'ast syn::ExprPath) {
        if path
            .path
            .segments
            .iter()
            .any(|segment| segment.ident == "SemanticGraphCause")
        {
            if let Some(cause) = path.path.segments.last() {
                self.0.insert(cause.ident.to_string());
            }
        }
    }
}

fn duplicated_term_checker(block: &syn::Block) -> bool {
    let mut refusals = TermRefusals::default();
    refusals.visit_block(block);
    ["ZeroExponent", "DuplicateTerm", "UnsortedTerms"]
        .iter()
        .all(|cause| refusals.0.contains(*cause))
}

#[derive(Default)]
struct LocalVocabulary {
    definitions: Vec<String>,
}

impl<'ast> Visit<'ast> for LocalVocabulary {
    fn visit_item(&mut self, item: &'ast syn::Item) {
        let definition = match item {
            syn::Item::Struct(x) => Some(("struct", x.ident.to_string())),
            syn::Item::Enum(x) => Some(("enum", x.ident.to_string())),
            syn::Item::Union(x) => Some(("union", x.ident.to_string())),
            syn::Item::Type(x) => Some(("type", x.ident.to_string())),
            syn::Item::Trait(x) => Some(("trait", x.ident.to_string())),
            syn::Item::Fn(x) => Some(("fn", x.sig.ident.to_string())),
            _ => None,
        };
        if let Some((kind, name)) = definition {
            if owned_name(&name) || (kind == "fn" && term_checker(&name)) {
                self.definitions.push(format!("local {kind} {name}"));
            }
        }
        if let syn::Item::Enum(item) = item {
            if copied_cause_variants(item) {
                self.definitions
                    .push(format!("local enum copy {}", item.ident));
            }
        }
        if let syn::Item::Type(item) = item {
            if let syn::Type::Path(target) = item.ty.as_ref() {
                if owned_target(&target.path) {
                    self.definitions
                        .push(format!("local type alias {}", item.ident));
                }
            }
        }
        if let syn::Item::Fn(item) = item {
            if duplicated_term_checker(&item.block) {
                self.definitions
                    .push(format!("local term checker {}", item.sig.ident));
            }
        }
        if let syn::Item::Use(x) = item {
            inspect_use_alias(&x.tree, &mut self.definitions);
        }
        visit::visit_item(self, item);
    }

    fn visit_macro(&mut self, mac: &'ast syn::Macro) {
        inspect_macro(mac.tokens.clone(), &mut self.definitions);
    }

    fn visit_impl_item(&mut self, item: &'ast syn::ImplItem) {
        if let syn::ImplItem::Fn(method) = item {
            let name = method.sig.ident.to_string();
            if term_checker(&name) || duplicated_term_checker(&method.block) {
                self.definitions.push(format!("local method {name}"));
            }
        }
        visit::visit_impl_item(self, item);
    }

    fn visit_trait_item(&mut self, item: &'ast syn::TraitItem) {
        if let syn::TraitItem::Fn(method) = item {
            let name = method.sig.ident.to_string();
            if term_checker(&name) {
                self.definitions.push(format!("local trait method {name}"));
            }
        }
        visit::visit_trait_item(self, item);
    }
}

fn inspect_use_alias(tree: &syn::UseTree, definitions: &mut Vec<String>) {
    match tree {
        syn::UseTree::Path(path) => inspect_use_alias(&path.tree, definitions),
        syn::UseTree::Group(group) => {
            for tree in &group.items {
                inspect_use_alias(tree, definitions);
            }
        }
        syn::UseTree::Rename(rename)
            if owned_name(&rename.ident.to_string())
                || term_checker(&rename.ident.to_string())
                || owned_name(&rename.rename.to_string())
                || term_checker(&rename.rename.to_string()) =>
        {
            definitions.push(format!("local use alias {}", rename.rename));
        }
        _ => {}
    }
}

fn inspect_macro(tokens: TokenStream, definitions: &mut Vec<String>) {
    fn identifiers(tokens: TokenStream, names: &mut Vec<String>) {
        for token in tokens {
            match token {
                TokenTree::Ident(ident) => names.push(ident.to_string()),
                TokenTree::Group(group) => identifiers(group.stream(), names),
                _ => {}
            }
        }
    }
    let mut names = Vec::new();
    identifiers(tokens, &mut names);
    for pair in names.windows(2) {
        if (matches!(
            pair[0].as_str(),
            "struct" | "enum" | "union" | "type" | "trait"
        ) && owned_name(&pair[1]))
            || (pair[0] == "fn" && (owned_name(&pair[1]) || term_checker(&pair[1])))
        {
            definitions.push(format!("local {} {} in macro", pair[0], pair[1]));
        }
    }
}

fn inspect_sources(directory: &Path, definitions: &mut Vec<String>) {
    for entry in std::fs::read_dir(directory).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            inspect_sources(&path, definitions);
        } else if path.extension().is_some_and(|extension| extension == "rs") {
            let source = std::fs::read_to_string(&path).unwrap();
            let parsed = syn::parse_file(&source)
                .unwrap_or_else(|error| panic!("{}: {error}", path.display()));
            let mut inspection = LocalVocabulary::default();
            inspection.visit_file(&parsed);
            definitions.extend(
                inspection
                    .definitions
                    .into_iter()
                    .map(|definition| format!("{}: {definition}", path.display())),
            );
        }
    }
}

#[trace("TC-205", "FR-275-AC-22")]
#[test]
fn qsv_owns_semantic_node_vocabulary() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut definitions = Vec::new();
    inspect_sources(&root.join("src"), &mut definitions);
    assert!(definitions.is_empty(), "{}", definitions.join("\n"));
}

#[trace("TC-205", "FR-275-AC-22")]
#[test]
fn inspection_rejects_renamed_semantic_node_copies() {
    let cases = [
        "pub type GraphCause = quire_semantic_value::semantic_node::SemanticGraphCause;",
        "pub use quire_semantic_value::semantic_node::InvalidSemanticGraph as Refusal;",
        "pub enum GraphCause { NonCanonicalPreimage, UnsortedUnorderedMembers, OwnerNotSelected, StaleKey, ForeignDeclaration, UndeclaredCase, ZeroExponent, DuplicateTerm, UnsortedTerms, UnreducedRational, ZeroScale, NonIdentityRoot, DuplicateNode, UnknownDimension, NonBaseDimensionTerm, UnknownTarget, CrossDimensionTarget, MissingRoot, DuplicateRoot, TargetCycle }",
        "fn dimension_terms_ok(terms: &[(NodeKey, Integer)]) -> Result<(), SemanticGraphCause> { if terms.iter().any(|(_, exponent)| exponent.is_zero()) { return Err(SemanticGraphCause::ZeroExponent); } let distinct: BTreeSet<_> = terms.iter().map(|(key, _)| key).collect(); if distinct.len() != terms.len() { return Err(SemanticGraphCause::DuplicateTerm); } if !terms.is_sorted_by(|(left, _), (right, _)| left < right) { return Err(SemanticGraphCause::UnsortedTerms); } Ok(()) }",
    ];
    for source in cases {
        let parsed = syn::parse_file(source).unwrap();
        let mut inspection = LocalVocabulary::default();
        inspection.visit_file(&parsed);
        assert!(!inspection.definitions.is_empty(), "missed: {source}");
    }
}
