const CARGO_MANIFEST: &str = include_str!("../Cargo.toml");
const CRATE_ROOT: &str = include_str!("../src/lib.rs");

/// Trace: TC-005, FR-003-AC-2, NFR-001-AC-1, StR-001-VC-2
#[test]
fn tc_005_optional_surface_is_explicitly_feature_gated() {
    assert!(CARGO_MANIFEST.contains("default = []"));
    assert!(CARGO_MANIFEST.contains("proptest = [\"std\", \"dep:proptest\"]"));
}

/// Trace: TC-007, NFR-001-AC-2, NFR-001-AC-3, NFR-002-AC-2, StR-001-VC-2
#[test]
fn tc_007_release_controls_are_mandatory() {
    assert!(CARGO_MANIFEST.contains("license = \"MIT OR Apache-2.0\""));
    assert!(CARGO_MANIFEST.contains("publish = false"));
    assert!(CRATE_ROOT.contains("#![no_std]"));
    assert!(CRATE_ROOT.contains("#![forbid(unsafe_code)]"));
}

/// Trace: TC-008, FR-001-AC-3, FR-004-AC-3, NFR-002-AC-3
#[test]
fn tc_008_evidence_model_is_non_exhaustive_and_opaque() {
    let verdicts = include_str!("../src/verdict.rs");
    let observations = include_str!("../src/observation.rs");
    let accounting = include_str!("../src/accounting.rs");

    let verdict_offsets = [
        assert_non_exhaustive(verdicts, "pub enum VerdictKind {", "VerdictKind"),
        assert_non_exhaustive(verdicts, "pub enum Verdict<'a> {", "Verdict"),
    ];
    assert_ne!(verdict_offsets[0], verdict_offsets[1]);
    for (declaration, enum_name) in [
        ("pub enum ClauseKind {", "ClauseKind"),
        ("pub enum ClauseOutcome {", "ClauseOutcome"),
        ("pub enum FailureKind {", "FailureKind"),
    ] {
        assert_non_exhaustive(observations, declaration, enum_name);
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

    assert_eq!(
        public_mutating_methods(accounting, "impl CampaignCounts"),
        Vec::<String>::new()
    );
    assert_eq!(
        public_mutating_methods(accounting, "impl<'a> CampaignReport<'a>"),
        ["record_verdict", "record_discard"]
    );
    let mutation_probe = "impl CampaignCounts {\n\
        pub const fn overwrite(&mut self) { self.accepted = 0; }\n\
        pub fn set_counts(&mut self) { self.rejected = 0; }\n\
    }";
    assert_eq!(
        public_mutating_methods(mutation_probe, "impl CampaignCounts"),
        ["overwrite", "set_counts"]
    );
}

fn assert_non_exhaustive(source: &str, declaration: &str, enum_name: &str) -> usize {
    let declaration_at = source.find(declaration).expect("enum declaration exists");
    let attributes = &source[..declaration_at];
    let nearest_non_exhaustive = attributes
        .rfind("#[non_exhaustive]")
        .expect("non-exhaustive attribute exists");
    let intervening = &attributes[nearest_non_exhaustive..];
    assert!(
        !intervening.contains("pub enum"),
        "{enum_name} is not the enum governed by the nearest non-exhaustive attribute"
    );
    declaration_at
}

fn public_mutating_methods(source: &str, implementation: &str) -> Vec<String> {
    let start = source.find(implementation).expect("implementation exists");
    let body_start = source[start..].find('{').expect("implementation body") + start;
    let mut depth = 0_u32;
    let mut body_end = None;
    for (offset, character) in source[body_start..].char_indices() {
        match character {
            '{' => depth += 1,
            '}' => {
                depth -= 1;
                if depth == 0 {
                    body_end = Some(body_start + offset);
                    break;
                }
            }
            _ => {}
        }
    }
    let body = &source[body_start..body_end.expect("implementation closes")];
    body.match_indices("pub ")
        .filter_map(|(offset, _)| {
            let signature = &body[offset..body[offset..].find('{')? + offset];
            if !signature.contains("&mut self") {
                return None;
            }
            let name_start = signature.find("fn ")? + "fn ".len();
            let name_end = signature[name_start..].find(['(', '<'])? + name_start;
            Some(signature[name_start..name_end].to_owned())
        })
        .collect()
}
