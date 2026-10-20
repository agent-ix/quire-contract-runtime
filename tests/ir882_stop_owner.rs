//! IR-882 ownership check for the early-exit carrier used by exact residue.
#![cfg(feature = "exact")]

use ix_trace_rs::trace;
use std::path::Path;

#[trace("TC-197")]
#[test]
fn runtime_has_no_local_stop_carrier() {
    let exact = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/exact");
    assert!(!exact.join("stop.rs").exists());
    let module = std::fs::read_to_string(exact.join("mod.rs")).unwrap();
    assert!(!module.contains("mod stop;"));
}
