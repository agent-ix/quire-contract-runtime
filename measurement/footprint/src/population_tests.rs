use super::quire_runtime_footprint;

/// Trace: TC-007, NFR-001-AC-3
#[test]
fn tc_007_footprint_population_executes_expected_semantics() {
    // input 0: report total 2 + Boolean score 2 + operator score 2.
    // input 1: report total 2 + Boolean score 4 + operator score 8.
    assert_eq!(quire_runtime_footprint(0), 6);
    assert_eq!(quire_runtime_footprint(1), 14);
}
