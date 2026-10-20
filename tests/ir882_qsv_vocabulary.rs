//! IR-882 ownership check for the semantic-node refusal vocabulary.
#![cfg(feature = "exact")]

use ix_trace_rs::trace;
use proc_macro2::{TokenStream, TokenTree};
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
            if term_checker(&name) {
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
            if owned_name(&rename.rename.to_string())
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

#[trace("TC-197", "FR-275-AC-22")]
#[test]
fn qsv_owns_semantic_node_vocabulary() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut definitions = Vec::new();
    inspect_sources(&root.join("src"), &mut definitions);
    assert!(definitions.is_empty(), "{}", definitions.join("\n"));
}
