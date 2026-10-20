//! IR-882 enum owner cutover: RT carries no second enumeration implementation.
#![cfg(feature = "exact")]

use ix_trace_rs::trace;
use std::path::Path;

#[trace("TC-197")]
#[test]
fn enum_declarations_and_comparison_have_one_owner() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let exact = root.join("src/exact");
    assert!(!exact.join("enumeration.rs").exists());
    let module = std::fs::read_to_string(exact.join("mod.rs")).unwrap();
    assert!(!module.contains("mod enumeration;"));
    assert!(!module.contains("pub use enumeration::"));
}
