//! Consumer controls for the shared exact value and checked equality owner.
#![cfg(feature = "exact")]

use ix_trace_rs::trace;
use std::fs;

#[trace("TC-203", "FR-277-AC-8")]
#[test]
fn runtime_defines_no_local_equality_or_value_owner() {
    let module = include_str!("../src/exact/mod.rs");
    assert!(!module.contains("mod equality;"));
    assert!(!module.contains("mod composite;"));
    assert!(!module.contains("pub use equality::"));
    assert!(!module.contains("pub use composite::"));
    let source = format!("{}/src/exact", env!("CARGO_MANIFEST_DIR"));
    for entry in fs::read_dir(source).unwrap() {
        let path = entry.unwrap().path();
        if path.extension().is_none_or(|extension| extension != "rs") {
            continue;
        }
        let source = fs::read_to_string(&path).unwrap();
        for duplicate in [
            "pub enum Value {",
            "pub struct CheckedEquality {",
            "fn plan_pairs(",
            "fn planned_equality(",
            "pub type Value =",
            "pub use quire_exact::Value",
            "pub use quire_semantic_value::declaration::CheckedEquality",
        ] {
            assert!(
                !source.contains(duplicate),
                "{} defines {duplicate}",
                path.display()
            );
        }
    }
}
