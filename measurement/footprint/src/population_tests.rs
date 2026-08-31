use super::quire_runtime_footprint;

/// Trace: TC-007, NFR-001-AC-3
#[test]
fn tc_007_footprint_population_executes_expected_semantics() {
    assert_eq!(quire_runtime_footprint(0), 6);
    assert_eq!(quire_runtime_footprint(1), 14);
}
