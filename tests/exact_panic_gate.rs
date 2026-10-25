//! Tests the linked exact-profile gate against the actual RT archive.

use std::process::{Command, Output};
use std::sync::OnceLock;

use ix_trace_rs::trace;

static PROBE: OnceLock<Output> = OnceLock::new();

fn probe() -> &'static Output {
    PROBE.get_or_init(|| {
        Command::new("make")
            .arg("exact-panic-probe")
            .current_dir(env!("CARGO_MANIFEST_DIR"))
            .output()
            .expect("make exact-panic-probe must start")
    })
}

fn gate_output() -> String {
    let result = probe();
    let stdout = String::from_utf8_lossy(&result.stdout);
    let stderr = String::from_utf8_lossy(&result.stderr);
    assert!(result.status.success(), "probe failed:\n{stdout}\n{stderr}");
    stdout.into_owned()
}

#[trace("NFR-002-AC-8", "TC-206")]
#[test]
fn exact_probe_retains_required_rt_calls_without_panic_relocations() {
    let output = gate_output();
    assert!(
        output.contains("runtime_objects="),
        "missing RT population: {output}"
    );
    assert!(
        output.contains("required_calls="),
        "missing call report: {output}"
    );
    assert!(
        output.contains("panic_references=0"),
        "panic verdict absent: {output}"
    );
}

#[trace("NFR-002-AC-9", "TC-206")]
#[test]
fn injected_rt_panic_is_rejected_by_the_same_gate() {
    let output = gate_output();
    assert!(
        output.contains("exact panic control: reachable RT-owned panic relocation detected"),
        "negative control did not run: {output}"
    );
}
