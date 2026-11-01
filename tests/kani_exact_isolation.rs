//! interface-001-AC-15: the `cfg(kani)` verification module names `quire_exact` only under `exact`.

use ix_trace_rs::trace;
use proc_macro2::{TokenStream, TokenTree};
use std::path::Path;
use syn::{spanned::Spanned, Attribute, Expr, ExprLit, Item, Lit, Meta};

/// Identifiers that tie an item to the optional crate: the crate itself, what the module
/// imports from it, and the local const built from it.
const IMPORTED: &[&str] = &[
    "quire_exact",
    "compare_ieee",
    "IeeeComparison",
    "IeeeValue",
    "Meter",
    "Outcome",
    "ScalarLimits",
    "EXACT_UNLIMITED",
];

fn is_feature_exact(meta: &Meta) -> bool {
    match meta {
        Meta::NameValue(nv) => {
            nv.path.is_ident("feature")
                && matches!(&nv.value, Expr::Lit(ExprLit { lit: Lit::Str(s), .. }) if s.value() == "exact")
        }
        // `all(..)` holds only when every operand does, so one `feature = "exact"` operand gates.
        Meta::List(l) if l.path.is_ident("all") => l
            .parse_args_with(syn::punctuated::Punctuated::<Meta, syn::Token![,]>::parse_terminated)
            .map(|ops| ops.iter().any(is_feature_exact))
            .unwrap_or(false),
        _ => false,
    }
}

fn gated_on_exact(attrs: &[Attribute]) -> bool {
    attrs
        .iter()
        .filter(|a| a.path().is_ident("cfg"))
        .any(|a| a.parse_args::<Meta>().is_ok_and(|m| is_feature_exact(&m)))
}

fn attrs_of(item: &Item) -> Option<&[Attribute]> {
    Some(match item {
        Item::Use(i) => &i.attrs,
        Item::Const(i) => &i.attrs,
        Item::Static(i) => &i.attrs,
        Item::Fn(i) => &i.attrs,
        Item::Struct(i) => &i.attrs,
        Item::Enum(i) => &i.attrs,
        Item::Type(i) => &i.attrs,
        Item::Impl(i) => &i.attrs,
        Item::Mod(i) => &i.attrs,
        Item::Trait(i) => &i.attrs,
        Item::Macro(i) => &i.attrs,
        _ => return None,
    })
}

/// Token-level identifier search: comments and string literals are not identifiers.
fn mentions(tokens: TokenStream) -> bool {
    tokens.into_iter().any(|t| match t {
        TokenTree::Ident(i) => IMPORTED.contains(&i.to_string().as_str()),
        TokenTree::Group(g) => mentions(g.stream()),
        _ => false,
    })
}

#[trace("TC-208", "interface-001-AC-15")]
#[test]
fn kani_module_gates_every_quire_exact_reference_on_the_exact_feature() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("verification/kani.rs");
    let text = std::fs::read_to_string(path).unwrap();
    let file = syn::parse_file(&text).unwrap();
    let mut gated = 0;
    for item in &file.items {
        let source = item.span().source_text().unwrap();
        let tokens: TokenStream = source.parse().unwrap();
        if !mentions(tokens) {
            continue;
        }
        let attrs =
            attrs_of(item).expect("an item kind this guard cannot inspect names quire_exact");
        assert!(
            gated_on_exact(attrs),
            "ungated quire_exact reference under cfg(kani):\n{source}"
        );
        gated += 1;
    }
    assert!(
        gated >= 3,
        "the IEEE harness, its limits and its import must stay present"
    );
}
