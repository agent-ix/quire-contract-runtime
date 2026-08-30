const CARGO_MANIFEST: &str = include_str!("../Cargo.toml");
const CRATE_ROOT: &str = include_str!("../src/lib.rs");
const MAKEFILE: &str = include_str!("../Makefile");

/// Trace: TC-005, FR-003-AC-2, NFR-001-AC-1, StR-001-VC-2
#[test]
fn tc_005_optional_surface_is_explicitly_feature_gated() {
    assert!(CARGO_MANIFEST.contains("default = []"));
    assert!(CARGO_MANIFEST.contains("proptest = [\"std\", \"dep:proptest\"]"));
    assert!(CRATE_ROOT.contains(
        "#[cfg(feature = \"proptest\")]\n/// Implements: FR-003\npub mod proptest_adapter;"
    ));
}

/// Trace: TC-007, NFR-001-AC-2, NFR-001-AC-3, NFR-002-AC-2, StR-001-VC-2
#[test]
fn tc_007_release_controls_are_mandatory() {
    assert!(CARGO_MANIFEST.contains("license = \"MIT OR Apache-2.0\""));
    assert!(CARGO_MANIFEST.contains("publish = false"));
    assert!(CRATE_ROOT.contains("#![no_std]"));
    assert!(CRATE_ROOT.contains("#![forbid(unsafe_code)]"));
    assert!(MAKEFILE.contains(
        "ci: fmt-check spec lint test-features msrv size deny audit-unsafe audit-panic evidence-tool"
    ));
}

/// Trace: TC-008, FR-001-AC-3, FR-004-AC-3, NFR-002-AC-3
#[test]
fn tc_008_evidence_model_is_non_exhaustive_and_opaque() {
    let verdicts = include_str!("../src/verdict.rs");
    let observations = include_str!("../src/observation.rs");
    let accounting = include_str!("../src/accounting.rs");

    for enum_name in ["VerdictKind", "Verdict"] {
        assert_non_exhaustive(verdicts, enum_name);
    }
    for enum_name in ["ClauseKind", "ClauseOutcome", "FailureKind"] {
        assert_non_exhaustive(observations, enum_name);
    }
    for forbidden in [
        "pub accepted:",
        "pub rejected:",
        "pub failed:",
        "pub discarded:",
        "pub identity:",
        "pub counts:",
    ] {
        assert!(
            !accounting.contains(forbidden),
            "public mutation seam: {forbidden}"
        );
    }
}

fn assert_non_exhaustive(source: &str, enum_name: &str) {
    let declaration = format!("pub enum {enum_name}");
    let declaration_at = source.find(&declaration).expect("enum declaration exists");
    let attributes = &source[..declaration_at];
    let nearest_non_exhaustive = attributes
        .rfind("#[non_exhaustive]")
        .expect("non-exhaustive attribute exists");
    let intervening = &attributes[nearest_non_exhaustive..];
    assert!(
        !intervening.contains("pub enum"),
        "{enum_name} is not the enum governed by the nearest non-exhaustive attribute"
    );
}
