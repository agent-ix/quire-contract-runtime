//! interface-001-AC-15: the `cfg(kani)` verification module names `quire_exact` only under `exact`.

use ix_trace_rs::trace;
use std::path::Path;
use syn::{spanned::Spanned, Attribute, Item};

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

fn gated_on_exact(attrs: &[Attribute]) -> bool {
    attrs.iter().any(|a| {
        let t = a.meta.span().source_text().unwrap_or_default();
        a.path().is_ident("cfg") && t.contains("feature") && t.contains("\"exact\"")
    })
}

fn mentions(tokens: &str) -> bool {
    tokens
        .split(|c: char| !(c.is_alphanumeric() || c == '_'))
        .any(|w| IMPORTED.contains(&w))
}

#[trace("TC-208", "interface-001-AC-15")]
#[test]
fn kani_module_gates_every_quire_exact_reference_on_the_exact_feature() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("verification/kani.rs");
    let text = std::fs::read_to_string(path).unwrap();
    let file = syn::parse_file(&text).unwrap();
    let mut gated = 0;
    for item in &file.items {
        let (attrs, body): (&[Attribute], String) = match item {
            Item::Use(i) => (&i.attrs, item_text(&text, i)),
            Item::Const(i) => (&i.attrs, item_text(&text, i)),
            Item::Fn(i) => (&i.attrs, item_text(&text, i)),
            _ => continue,
        };
        // Strip comments so prose naming quire-exact does not count.
        let code: String = body
            .lines()
            .filter(|l| !l.trim_start().starts_with("//"))
            .map(|l| l.split("//").next().unwrap_or(""))
            .collect::<Vec<_>>()
            .join("\n");
        if mentions(&code) {
            assert!(
                gated_on_exact(attrs),
                "ungated quire_exact reference under cfg(kani):\n{code}"
            );
            gated += 1;
        }
    }
    assert!(
        gated >= 3,
        "the IEEE harness, its limits and its import must stay present"
    );
}

fn item_text<T: Spanned>(_text: &str, i: &T) -> String {
    i.span().source_text().unwrap_or_default()
}
