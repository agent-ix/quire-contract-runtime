//! FR-012 carried compiler vocabulary: node keys, refusal vocabularies, semantic-graph causes and
//! owner types. Enum declarations use the QSV owner directly.
#![cfg(feature = "exact")]

use ix_trace_rs::trace;
use proc_macro2::{TokenStream, TokenTree};
use quire_semantic_value::definition::{
    PackageCause, PackageRefusal, PackageRefusalCode, SelectionRefusalCode,
};
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

fn is_definition_type_ident(ident: &syn::Ident) -> bool {
    matches!(
        ident.to_string().as_str(),
        "PackageCause" | "PackageRefusal" | "PackageRefusalCode" | "SelectionRefusalCode"
    )
}

fn is_definition_cause_variant(ident: &syn::Ident) -> bool {
    matches!(
        ident.to_string().as_str(),
        "MissingMember"
            | "ConflictingDefinition"
            | "IncompatibleDefinition"
            | "RevisionMismatch"
            | "DigestDomainMismatch"
            | "ByteDigestMismatch"
    )
}

#[derive(Default)]
struct DefinitionTypeUse(bool);

impl<'ast> Visit<'ast> for DefinitionTypeUse {
    fn visit_ident(&mut self, ident: &'ast syn::Ident) {
        self.0 |= is_definition_type_ident(ident);
    }
}

fn type_uses_definition(ty: &syn::Type) -> bool {
    let mut use_check = DefinitionTypeUse::default();
    use_check.visit_type(ty);
    use_check.0
}

fn use_mentions_definition(tree: &syn::UseTree, renamed_only: bool) -> bool {
    match tree {
        syn::UseTree::Path(path) => use_mentions_definition(&path.tree, renamed_only),
        syn::UseTree::Name(name) => !renamed_only && is_definition_type_ident(&name.ident),
        syn::UseTree::Rename(rename) => is_definition_type_ident(&rename.ident),
        syn::UseTree::Glob(_) => false,
        syn::UseTree::Group(group) => group
            .items
            .iter()
            .any(|item| use_mentions_definition(item, renamed_only)),
    }
}

fn inspect_definition_macro_tokens(
    tokens: TokenStream,
    in_declaration: bool,
    findings: &mut Vec<String>,
) {
    let mut declaration = in_declaration;
    for token in tokens {
        match token {
            TokenTree::Ident(ident) => {
                declaration |= matches!(
                    ident.to_string().as_str(),
                    "enum" | "struct" | "type" | "union"
                );
                if declaration
                    && (is_definition_type_ident(&ident) || is_definition_cause_variant(&ident))
                {
                    findings.push(format!("local definition in macro: {ident}"));
                }
            }
            TokenTree::Group(group) => {
                inspect_definition_macro_tokens(group.stream(), declaration, findings)
            }
            _ => {}
        }
    }
}

fn publicly_exposes_qsv_definition(tree: &syn::UseTree, path_state: u8) -> bool {
    match tree {
        syn::UseTree::Path(path) => {
            let state = match (path_state, path.ident.to_string().as_str()) {
                (0, "quire_semantic_value") => 1,
                (1, "definition") => 2,
                (2, _) => 2,
                _ => 0,
            };
            publicly_exposes_qsv_definition(&path.tree, state)
        }
        syn::UseTree::Name(name) => {
            path_state == 2 || (path_state == 1 && name.ident == "definition")
        }
        syn::UseTree::Rename(rename) => {
            path_state == 2 || (path_state == 1 && rename.ident == "definition")
        }
        syn::UseTree::Glob(_) => path_state == 2,
        syn::UseTree::Group(group) => group
            .items
            .iter()
            .any(|item| publicly_exposes_qsv_definition(item, path_state)),
    }
}

#[derive(Default)]
struct ProductionDefinitionInspection {
    findings: Vec<String>,
}

impl<'ast> Visit<'ast> for ProductionDefinitionInspection {
    fn visit_item(&mut self, item: &'ast syn::Item) {
        if is_test_only(item_attrs(item)) {
            return;
        }
        match item {
            syn::Item::Use(use_item) => {
                let public = matches!(use_item.vis, syn::Visibility::Public(_));
                if (public && publicly_exposes_qsv_definition(&use_item.tree, 0))
                    || use_mentions_definition(&use_item.tree, !public)
                {
                    self.findings.push("definition alias or re-export".into());
                }
            }
            syn::Item::Enum(item) => {
                if is_definition_type_ident(&item.ident)
                    || item.variants.iter().any(|variant| {
                        is_definition_cause_variant(&variant.ident)
                            || variant
                                .fields
                                .iter()
                                .any(|field| type_uses_definition(&field.ty))
                    })
                {
                    self.findings
                        .push("local definition enum or adapter".into());
                }
            }
            syn::Item::Struct(item) => {
                let newtype_adapter = match &item.fields {
                    syn::Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
                        type_uses_definition(&fields.unnamed[0].ty)
                    }
                    _ => false,
                };
                if is_definition_type_ident(&item.ident) || newtype_adapter {
                    self.findings
                        .push("local definition struct or adapter".into());
                }
            }
            syn::Item::Type(item) => {
                if is_definition_type_ident(&item.ident) || type_uses_definition(&item.ty) {
                    self.findings.push("local definition type alias".into());
                }
            }
            syn::Item::Union(item) if is_definition_type_ident(&item.ident) => {
                self.findings.push("local definition union".into());
            }
            syn::Item::Mod(item) if item.ident == "definition" => {
                self.findings.push("local definition module".into());
            }
            _ => {}
        }
        visit::visit_item(self, item);
    }

    fn visit_macro(&mut self, item: &'ast syn::Macro) {
        inspect_definition_macro_tokens(item.tokens.clone(), false, &mut self.findings);
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
    for directory in ["src", "verification", "examples", "measurement", "tools"] {
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

#[derive(Default)]
struct FixtureIdentityInspection {
    findings: Vec<String>,
}

impl<'ast> Visit<'ast> for FixtureIdentityInspection {
    fn visit_expr_call(&mut self, call: &'ast syn::ExprCall) {
        if let syn::Expr::Path(path) = call.func.as_ref() {
            let parts: Vec<_> = path
                .path
                .segments
                .iter()
                .map(|part| part.ident.to_string())
                .collect();
            let method = parts.last().map(String::as_str);
            let owner = parts.iter().rev().nth(1).map(String::as_str);
            let identity_owner = matches!(
                owner,
                Some(
                    "NodeKey"
                        | "EffectiveId"
                        | "UniverseId"
                        | "VariantId"
                        | "OriginalDeclarationKey"
                        | "UnitId"
                )
            );
            if matches!(
                method,
                Some("from_digest" | "decode_admitted" | "from_bytes" | "from_hex")
            ) || (identity_owner && matches!(method, Some("new" | "declared" | "compound")))
            {
                self.findings.push(parts.join("::"));
            }
        }
        visit::visit_expr_call(self, call);
    }

    fn visit_item_fn(&mut self, function: &'ast syn::ItemFn) {
        if let syn::ReturnType::Type(_, ty) = &function.sig.output {
            if let syn::Type::Path(path) = ty.as_ref() {
                if path.path.segments.iter().any(|part| {
                    matches!(
                        part.ident.to_string().as_str(),
                        "NodeKey"
                            | "EffectiveId"
                            | "UniverseId"
                            | "VariantId"
                            | "OriginalDeclarationKey"
                            | "UnitId"
                    )
                }) {
                    self.findings
                        .push(format!("identity fixture function: {}", function.sig.ident));
                }
            }
        }
        visit::visit_item_fn(self, function);
    }
}

#[trace("TC-033", "FR-012-AC-8")]
#[test]
fn tc_033_rt_does_not_construct_semantic_identities() {
    let mut sources = Vec::new();
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    for directory in [
        "src",
        "tests",
        "verification",
        "examples",
        "measurement",
        "tools",
    ] {
        rust_sources(&root.join(directory), &mut sources);
    }
    assert!(!sources.is_empty());
    let mut failures = Vec::new();
    for path in sources {
        let source = std::fs::read_to_string(&path).unwrap();
        let parsed = syn::parse_file(&source).unwrap();
        let mut inspection = FixtureIdentityInspection::default();
        inspection.visit_file(&parsed);
        failures.extend(
            inspection
                .findings
                .into_iter()
                .map(|finding| format!("{}: {finding}", path.display())),
        );
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));

    for forbidden in [
        "let key = NodeKey::from_digest([1; 32]);",
        "let key = NodeKey::new(\"package\", vec![], 0);",
        "let universe = UniverseId::new(key);",
        "let unit = UnitId::declared(key);",
        "fn key() -> NodeKey { unreachable!() }",
    ] {
        let parsed = syn::parse_file(&format!("fn fixture() {{ {forbidden} }}")).unwrap();
        let mut inspection = FixtureIdentityInspection::default();
        inspection.visit_file(&parsed);
        assert!(
            !inspection.findings.is_empty(),
            "undetected fixture: {forbidden}"
        );
    }
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

#[trace("TC-033", "FR-012-AC-7")]
#[test]
fn tc_033_definition_refusals_have_only_the_qsv_owner() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    assert!(
        !root.join("src/exact/definition.rs").exists(),
        "RT still defines a local definition refusal vocabulary"
    );
    let exact_module = std::fs::read_to_string(root.join("src/exact/mod.rs")).unwrap();
    assert!(!exact_module.contains("mod definition;"));
    assert!(!exact_module.contains("pub use definition::"));

    let mut sources = Vec::new();
    rust_sources(&root.join("src"), &mut sources);
    assert!(!sources.is_empty());
    let mut findings = Vec::new();
    for path in sources {
        let source = std::fs::read_to_string(&path).unwrap();
        let parsed =
            syn::parse_file(&source).unwrap_or_else(|error| panic!("{}: {error}", path.display()));
        let mut inspection = ProductionDefinitionInspection::default();
        inspection.visit_file(&parsed);
        findings.extend(
            inspection
                .findings
                .into_iter()
                .map(|finding| format!("{}: {finding}", path.display())),
        );
    }
    assert!(findings.is_empty(), "{}", findings.join("\n"));

    for source in [
        "pub enum ShadowPackageCause { MissingMember, ConflictingDefinition }",
        "use quire_semantic_value::definition::PackageCause as LocalCause;",
        "pub type RefusalAlias = quire_semantic_value::definition::PackageRefusal;",
        "pub use quire_semantic_value::definition::*;",
        "pub struct LocalAdapter(quire_semantic_value::definition::PackageCause);",
        "macro_rules! local_copy { () => { enum Copy { ByteDigestMismatch } } }",
    ] {
        let parsed = syn::parse_file(source).unwrap();
        let mut inspection = ProductionDefinitionInspection::default();
        inspection.visit_file(&parsed);
        assert!(
            !inspection.findings.is_empty(),
            "source-inspection mutation escaped: {source}"
        );
    }
    let direct_owner_use = syn::parse_file(
        "use quire_semantic_value::definition::PackageCause; \
         pub fn carry(cause: PackageCause) -> PackageCause { cause } \
         pub struct Holder { cause: PackageCause }",
    )
    .unwrap();
    let mut inspection = ProductionDefinitionInspection::default();
    inspection.visit_file(&direct_owner_use);
    assert!(
        inspection.findings.is_empty(),
        "private direct QSV use must remain allowed: {:?}",
        inspection.findings
    );

    let causes = [
        (PackageCause::MissingMember, "missing-member"),
        (
            PackageCause::ConflictingDefinition,
            "conflicting-definition",
        ),
        (
            PackageCause::IncompatibleDefinition,
            "incompatible-definition",
        ),
    ];
    for (cause, spelling) in causes {
        assert_eq!(cause.as_str(), spelling);
        assert_eq!(
            PackageRefusal::invalid_package(cause).to_string(),
            format!("invalid_package: {spelling}")
        );
    }
}
