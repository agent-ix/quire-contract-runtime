#![cfg(not(feature = "proptest"))]

use std::{fs, path::Path, process::Command};

use serde_json::Value;

/// Trace: TC-004, FR-003-AC-2
#[test]
fn tc_004_adapter_import_fails_without_proptest_feature() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    // The probe has its own stable worktree target so nested Cargo cannot wait on the parent
    // `cargo test` target lock. It is reused across feature-matrix runs.
    let probe = root.join("target/feature-off-probe");
    let source = probe.join("source");
    let binaries = source.join("src/bin");
    fs::create_dir_all(&binaries).expect("create the local compile probe");
    fs::write(
        source.join("Cargo.toml"),
        format!(
            "[package]\nname = \"rt-feature-off-probe\"\nversion = \"0.0.0\"\nedition = \"2021\"\n\
             [dependencies]\nquire-contract-runtime = {{ path = {:?}, default-features = false }}\n\
             [workspace]\n",
            root
        ),
    )
    .expect("write probe manifest");
    fs::write(
        binaries.join("control.rs"),
        "use quire_contract_runtime::RequirementId;\nfn main() { let _ = RequirementId::new(\"x\"); }\n",
    )
    .expect("write public-import control");
    fs::write(
        binaries.join("missing.rs"),
        "use quire_contract_runtime::proptest_adapter;\nfn main() {}\n",
    )
    .expect("write absent-adapter probe");

    let check = |binary: &str| {
        Command::new("cargo")
            .args([
                "check",
                "--offline",
                "--manifest-path",
                source.join("Cargo.toml").to_str().expect("UTF-8 path"),
                "--target-dir",
                probe.join("build").to_str().expect("UTF-8 path"),
                "--message-format=json",
                "--bin",
                binary,
            ])
            .output()
            .expect("run Cargo compile probe")
    };

    let control = check("control");
    assert!(
        control.status.success(),
        "ordinary public import must compile before interpreting a failure: {}",
        String::from_utf8_lossy(&control.stderr)
    );

    let missing = check("missing");
    assert!(
        !missing.status.success(),
        "feature-off adapter import compiled"
    );
    let exact_import_refusal = String::from_utf8_lossy(&missing.stdout)
        .lines()
        .filter_map(|line| serde_json::from_str::<Value>(line).ok())
        .filter(|event| event["reason"] == "compiler-message")
        .any(|event| {
            event["message"]["code"]["code"] == "E0432"
                && event["message"]["spans"].as_array().is_some_and(|spans| {
                    spans.iter().any(|span| {
                        span["is_primary"] == true
                            && span["line_start"] == 1
                            && span["file_name"]
                                .as_str()
                                .is_some_and(|path| path.ends_with("missing.rs"))
                    })
                })
        });
    assert!(
        exact_import_refusal,
        "expected E0432 at the absent adapter import: {}",
        String::from_utf8_lossy(&missing.stderr)
    );
}
