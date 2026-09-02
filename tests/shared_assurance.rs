//! Tests for the shared assurance intake path (FR-005).
//!
//! These follow this repository's own binding idiom: a `/// Trace:` comment above
//! each `#[test]`, which is what Quire's census reads. They invoke the gates
//! rather than reimplementing them, because a test that recomputes what a gate
//! computes is a second implementation that can agree with itself while both are
//! wrong.
//!
//! A missing prerequisite is a failure here, never a skip. A gate that stands
//! down when its dependency is absent reports the same green as one that ran.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::OnceLock;

use serde_json::Value;

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

/// The interpreter `make assurance-env` builds. Its absence is an error.
fn assurance_python() -> PathBuf {
    let path = std::env::var_os("ASSURANCE_PYTHON")
        .map(PathBuf::from)
        .unwrap_or_else(|| root().join(".venv-assurance/bin/python"));
    assert!(
        path.is_file(),
        "the pinned assurance interpreter is missing at {}. Run `make assurance-env`. \
         This is a failure and not a skip: a gate that stands down when its dependency \
         is absent reports the same green as one that ran.",
        path.display()
    );
    path
}

fn run(program: &Path, arguments: &[&str]) -> (i32, String, String) {
    let output = Command::new(program)
        .args(arguments)
        .current_dir(root())
        .output()
        .unwrap_or_else(|error| panic!("failed to run {}: {error}", program.display()));
    (
        output.status.code().unwrap_or(-1),
        String::from_utf8_lossy(&output.stdout).into_owned(),
        String::from_utf8_lossy(&output.stderr).into_owned(),
    )
}

fn json_gate(program: &Path, arguments: &[&str]) -> Value {
    let (code, stdout, stderr) = run(program, arguments);
    assert_eq!(code, 0, "{arguments:?} exited {code}\n{stdout}\n{stderr}");
    serde_json::from_str(&stdout)
        .unwrap_or_else(|error| panic!("{arguments:?} did not emit JSON: {error}\n{stdout}"))
}

fn head_revision() -> String {
    let output = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .current_dir(root())
        .output()
        .expect("git rev-parse failed");
    String::from_utf8_lossy(&output.stdout).trim().to_owned()
}

fn sha256_of(path: &Path) -> String {
    let output = Command::new("sha256sum")
        .arg(path)
        .output()
        .expect("sha256sum failed");
    String::from_utf8_lossy(&output.stdout)
        .split_whitespace()
        .next()
        .expect("sha256sum output")
        .to_owned()
}

/// The chain is expensive and several tests read it. It runs once per test
/// binary, and every reader sees the same run rather than a different one.
static CHAIN: OnceLock<Value> = OnceLock::new();

fn chain_report() -> &'static Value {
    CHAIN.get_or_init(|| {
        // The chain runs under the system interpreter: it only shells out to
        // quoin and never imports engineering-assurance.
        let revision = head_revision();
        let (code, stdout, stderr) = run(
            Path::new("python3"),
            &[
                "scripts/assurance_chain.py",
                "--candidate-revision",
                &revision,
                "--json",
            ],
        );
        assert_eq!(code, 0, "the assurance chain exited {code}\n{stderr}");
        serde_json::from_str(&stdout).expect("the assurance chain did not emit JSON")
    })
}

/// Trace: TC-009, FR-005-AC-1
#[test]
fn tc_009_every_shared_pin_is_classified_by_the_packaged_matrix() {
    let python = assurance_python();
    let report = json_gate(&python, &["scripts/check_shared_pins.py", "--json"]);

    let components = report["components"].as_array().expect("components array");
    assert_eq!(
        components.len(),
        4,
        "the matrix pins four components; this run classified {}",
        components.len()
    );
    for component in components {
        assert_eq!(
            component["verdict"], "compatible",
            "{} is {} ({})",
            component["component"], component["verdict"], component["reason"]
        );
    }
    assert_eq!(report["accepted"], true);
    assert!(report["artifact_mismatches"].as_array().unwrap().is_empty());
    assert!(report["mirror_references"].as_array().unwrap().is_empty());

    // Acceptance is reported and never gated on: the pinned release records
    // `pending_human_acceptance` and ships no predicate for it
    // (agent-ix/engineering-assurance#20). Reading an absent field as approval,
    // in either direction, is the mistake this asserts against.
    assert_eq!(report["acceptance_recorded_here"], false);
    assert!(report["acceptance_state"].is_string());

    // The digest check must be seen to refuse, and must be seen to have anything
    // to check. Until agent-ix/quire-contract-runtime#11 this repository pinned
    // four artifacts, all of them read only by the retained-evidence
    // compatibility view; deleting that view would have left this assertion
    // iterating an empty list and passing because there was nothing to compare.
    // The pin is now `compatibility.py`, the module check_shared_pins.py imports
    // for every version verdict, and the probe below alters the recorded digest
    // and requires the mismatch to be reported.
    let (code, stdout, stderr) = run(
        &python,
        &[
            "-c",
            "import json,sys;sys.path.insert(0,'scripts');\
             import check_shared_pins as m;\
             pins=json.load(open('assurance/pins.json'));\
             pinned=[a for a in pins['consumed_artifacts'] if a.get('sha256')];\
             [a.update(sha256='0'*64) for a in pinned];\
             print(json.dumps({'pinned':len(pinned),\
             'mismatches':m.artifact_digest_mismatches(pins)}))",
        ],
    );
    assert_eq!(code, 0, "the digest probe failed: {stderr}");
    let probe: Value = serde_json::from_str(stdout.trim()).unwrap();
    assert!(
        probe["pinned"].as_u64().unwrap() > 0,
        "no consumed artifact carries a digest; the digest check has an empty \
         population and cannot fail"
    );
    assert_eq!(
        probe["mismatches"].as_array().unwrap().len(),
        probe["pinned"].as_u64().unwrap() as usize,
        "a falsified digest was not reported; the digest check matches nothing"
    );

    // The mirror check must be seen to refuse. Without this it is indistinguishable
    // from a check that matches nothing.
    let (code, stdout, stderr) = run(
        &python,
        &[
            "-c",
            "import json,sys;sys.path.insert(0,'scripts');\
             import check_shared_pins as m;\
             pins=json.load(open('assurance/pins.json'));\
             pins['engineering_assurance']['requirement']+=' --registry=https://npm.ix/';\
             print(json.dumps(m.mirror_references(pins)))",
        ],
    );
    assert_eq!(code, 0, "the mirror probe failed: {stderr}");
    let offenders: Vec<String> = serde_json::from_str(stdout.trim()).unwrap();
    assert!(
        !offenders.is_empty(),
        "a mirror registry reference was not detected; the check matches nothing"
    );
}

/// Trace: TC-010, FR-005-AC-2
#[test]
fn tc_010_the_chain_reaches_quoin_without_quoin_or_quire_executing_a_producer() {
    let report = chain_report();
    assert_eq!(report["matched"], true, "{report:#}");

    for group in ["scenarios", "controls", "adapter_probes"] {
        let items = report[group]
            .as_array()
            .unwrap_or_else(|| panic!("{group}"));
        assert!(!items.is_empty(), "{group} is empty");
        for item in items {
            assert_eq!(
                item["matched"], true,
                "{group} entry did not match: {item:#}"
            );
        }
    }

    // The adapter transcribes one named protocol and refuses another, rather than
    // guessing. A verdict recovered from an unrecognised stream is a verdict
    // recovered from nothing.
    let probes = report["adapter_probes"].as_array().unwrap();
    for required in [
        "refuses-a-foreign-protocol",
        "refuses-an-unnamed-outcome",
        "accepts-the-real-run",
        // The two demonstrations that used to come from the retained-evidence
        // compatibility census. Named here so that deleting either one is a
        // failure rather than a silently smaller census.
        "audit-reports-an-unsupported-method",
        "adapter-carries-a-malformed-row-as-non-success",
    ] {
        assert!(
            probes.iter().any(|probe| probe["probe"] == required),
            "adapter probe {required} is missing"
        );
    }

    // Every producer this repository owns must have been attested from its own
    // bytes and have reported success at this revision. `unavailable` here would
    // mean the Kani toolchain was absent, which is a state the chain reports and
    // is not permitted to pass.
    let attested = &report["attested_results"];
    for proof in [
        "PROOF-feature-matrix",
        "PROOF-kani-proofs",
        "PROOF-kani-mutations",
        "PROOF-footprint",
        "PROOF-quire-static-export",
        "PROOF-msrv",
    ] {
        assert_eq!(
            attested[proof], "passed",
            "{proof} was attested as {}",
            attested[proof]
        );
    }
}

/// Trace: TC-010, FR-005-AC-2
#[test]
fn tc_010_every_declared_proof_command_is_the_command_make_actually_runs() {
    // A declared command that is not the executed command is a lie inside a
    // sealed attestation, and it is the kind of lie nothing downstream can catch:
    // Quoin records what the caller says the command was.
    //
    // So it is asked of Make. `make -n assurance-inputs` prints the plan without
    // running it; line continuations are rejoined; every proof obligation's
    // declared argv must appear in that plan verbatim.
    let root = root();
    let declaration: Value = serde_json::from_str(
        &fs::read_to_string(root.join("assurance/change-assurance.json")).unwrap(),
    )
    .expect("the change-assurance declaration is JSON");

    // `cargo test` exports CARGO as an absolute path to the toolchain binary, and
    // the Makefile's `CARGO ?= cargo` picks it up. That substitution is an
    // artifact of the test runner, not of the repository, so it is removed here
    // and the plan is read as a plain shell would see it. The declaration names
    // tools, not paths.
    let plan = Command::new("make")
        .args(["-n", "assurance-inputs"])
        .current_dir(&root)
        .env_remove("CARGO")
        .output()
        .expect("make -n assurance-inputs failed to run");
    assert!(
        plan.status.success(),
        "make -n assurance-inputs did not resolve: {}",
        String::from_utf8_lossy(&plan.stderr)
    );
    let joined = String::from_utf8_lossy(&plan.stdout).replace("\\\n", " ");
    let normalised: String = joined.split_whitespace().collect::<Vec<_>>().join(" ");

    let obligations = declaration["record"]["definition"]["proof_obligations"]
        .as_array()
        .expect("proof_obligations");
    assert_eq!(
        obligations.len(),
        6,
        "the declaration names {} proof obligations; the chain and this test expect 6",
        obligations.len()
    );
    for proof in obligations {
        let argv: Vec<String> = proof["command"]["argv"]
            .as_array()
            .expect("argv")
            .iter()
            .map(|value| value.as_str().expect("argv element").to_owned())
            .collect();
        let command = argv.join(" ");
        assert!(
            normalised.contains(&command),
            "{} declares `{command}`, which `make assurance-inputs` does not run.\nPlan: {normalised}",
            proof["proof_id"]
        );
    }
}

/// Run a Python snippet against a producer module and return its stdout.
///
/// The snippet imports the module and replaces exactly one named function, then
/// asks the module what it reports. That seam is the only way the failure
/// direction of a prover-driven producer can be exercised without a prover that
/// genuinely fails: a campaign that always answers `pass` and a campaign that is
/// working produce the same document, and the difference is only visible when
/// the prover is made to disagree.
fn producer_probe(snippet: &str) -> String {
    let (code, stdout, stderr) = run(Path::new("python3"), &["-c", snippet]);
    assert_eq!(code, 0, "the producer probe failed\n{stdout}\n{stderr}");
    stdout.trim().to_owned()
}

/// Trace: TC-010, FR-005-AC-2, FR-005-AC-5
#[test]
fn tc_010_the_producers_report_failure_when_the_prover_does() {
    // Every other test in this file asks a producer whether it says `pass` when
    // the thing it measures is healthy. None of them asks whether it can say
    // anything else. A producer hollowed out to `return "pass"` satisfies all of
    // them, runs in milliseconds, and turns the whole chain green — which is the
    // finding this test exists to close, and which this repository had already
    // closed once before the machinery that closed it was deleted.

    // A prover that reports a verification failure must not read as a pass.
    let outcomes = producer_probe(
        "import json,sys; sys.path.insert(0,'scripts')\n\
         import run_kani_gate as g\n\
         g.run_kani = lambda stream: (1, 'VERIFICATION:- FAILED\\n')\n\
         print(json.dumps(sorted({e['outcome'] for e in g.collect()['entries']})))",
    );
    assert_eq!(
        outcomes, "[\"fail\"]",
        "a failing prover did not produce failing rows; got {outcomes}"
    );

    // A prover that exits zero having checked nothing is not a pass either. The
    // harnesses were not computed and the suite census must say the run did not
    // meet its declared shape.
    let outcomes = producer_probe(
        "import json,sys; sys.path.insert(0,'scripts')\n\
         import run_kani_gate as g\n\
         g.run_kani = lambda stream: (0, '')\n\
         print(json.dumps(sorted({e['outcome'] for e in g.collect()['entries']})))",
    );
    assert_eq!(
        outcomes, "[\"fail\", \"not-computed\"]",
        "an empty transcript with a zero exit was not reported as uncomputed; got {outcomes}"
    );

    // A harness that verifies below its declared obligation floor is vacuous.
    // The floor is what makes it so, and this is the assertion that makes the
    // floor load-bearing in the direction that matters.
    let outcomes = producer_probe(
        "import json,sys; sys.path.insert(0,'scripts')\n\
         import run_kani_gate as g\n\
         g.run_kani = lambda stream: (0, '')\n\
         g.proof_check_counts = lambda text: {n: 1 for n in g.EXPECTED_KANI_HARNESSES}\n\
         print(json.dumps(sorted({e['outcome'] for e in g.collect()['entries']})))",
    );
    assert_eq!(
        outcomes, "[\"fail\", \"vacuous\"]",
        "a proof below its floor was not reported as vacuous; got {outcomes}"
    );

    // And the mutation campaign: a prover that accepts an injected defect is a
    // control that did not hold, whatever its exit status says.
    let verdict = producer_probe(
        "import json,sys; sys.path.insert(0,'scripts')\n\
         import check_kani_mutations as m\n\
         class R:\n\
         \x20   returncode = 0\n\
         \x20   stdout = ''\n\
         \x20   stderr = ''\n\
         m.prove = lambda argv, cwd, env: R()\n\
         print(json.dumps(m.run_mutation(*m.MUTATIONS[0][:4])[0]))",
    );
    assert_eq!(
        verdict, "\"fail\"",
        "the campaign reported a control as held while the prover accepted the defect"
    );

    // A non-zero exit that never reached a verification failure is a broken run,
    // not a rejection. Counting it as one is how a campaign starts passing
    // because the compiler fell over.
    let verdict = producer_probe(
        "import json,sys; sys.path.insert(0,'scripts')\n\
         import check_kani_mutations as m\n\
         class R:\n\
         \x20   returncode = 1\n\
         \x20   stdout = 'error: could not compile'\n\
         \x20   stderr = ''\n\
         m.prove = lambda argv, cwd, env: R()\n\
         print(json.dumps(m.run_mutation(*m.MUTATIONS[0][:4])[0]))",
    );
    assert_eq!(
        verdict, "\"fail\"",
        "a run that never reached a proof failure was counted as a rejection"
    );
}

/// Write an executable shim for each name that records every invocation.
///
/// The log is the point. A shim that is never consulted and a producer that is
/// never run look identical from the outside, so the shims write down every call
/// and the test reads the file rather than assuming.
///
/// `--version` is answered rather than refused, and deliberately so. Asking a
/// tool its version is an observation — it is what the compatibility matrix's
/// own `observe` column does — and it is not the thing this test forbids. What
/// is forbidden is asking a tool to build, compile, test, link, or prove
/// anything. Every such invocation is logged and the log must be empty.
fn producer_shims(directory: &Path, names: &[&str]) -> PathBuf {
    // The directory is emptied, not just topped up. A shim left behind by an
    // earlier run silently changes what the next run measures, and it does so in
    // the direction that hides failures: an extra stubbed tool makes the chain
    // fail for a reason the test then misattributes. This was not hypothetical.
    let _ = fs::remove_dir_all(directory);
    fs::create_dir_all(directory).unwrap();
    let log = directory.join("invocations.log");
    for name in names {
        let path = directory.join(name);
        fs::write(
            &path,
            format!(
                "#!/bin/sh\n\
                 case \"$1\" in\n\
                 --version|-V) echo \"{name} 9.9.9 (shim)\"; exit 0 ;;\n\
                 provenance) echo '{{\"cli\":{{\"version\":\"9.9.9\"}},\"engine\":{{\"version\":\"9.9.9\"}}}}'; exit 0 ;;\n\
                 esac\n\
                 echo \"$0 $@\" >> {}\n\
                 exit 97\n",
                log.display()
            ),
        )
        .unwrap();
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(&path, fs::Permissions::from_mode(0o755)).unwrap();
        }
    }
    log
}

fn run_chain_with_path(shims: &Path) -> std::process::Output {
    let inherited = std::env::var("PATH").unwrap_or_default();
    let revision = head_revision();
    Command::new("python3")
        .args([
            "scripts/assurance_chain.py",
            "--candidate-revision",
            &revision,
        ])
        .current_dir(root())
        .env("PATH", format!("{}:{inherited}", shims.display()))
        .output()
        .expect("failed to run the assurance chain")
}

/// Trace: TC-010, FR-005-AC-2
#[test]
fn tc_010_the_chain_never_executes_a_producer_and_the_probe_can_prove_it() {
    // Two runs, because one proves nothing.
    //
    // Run A replaces every producer — cargo, cargo-kani, rustup, rustc — with a
    // stub that logs and fails. The chain must finish, and the log must be empty:
    // not one producer was invoked.
    //
    // Run B is the control. It stubs `quoin`, which the chain is supposed to run,
    // and requires the chain to fail and the log to be non-empty. Without it, an
    // empty log in run A would be equally consistent with PATH never being
    // consulted at all, which is exactly how this test read in the sibling
    // repository before an adversarial review caught it.
    let producers = root().join("target/producer-shims");
    let producer_log = producer_shims(&producers, &["cargo", "cargo-kani", "rustup", "rustc"]);
    let output = run_chain_with_path(&producers);
    let logged = fs::read_to_string(&producer_log).unwrap_or_default();
    assert!(
        output.status.success(),
        "the assurance chain failed with producers stubbed, which means it ran one:\n{}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        logged.trim().is_empty(),
        "the assurance driver asked a producer to do work, not just to name its version:\n{logged}"
    );

    let tools = root().join("target/tool-shims");
    let tool_log = producer_shims(&tools, &["quoin"]);
    let control = run_chain_with_path(&tools);
    let tool_logged = fs::read_to_string(&tool_log).unwrap_or_default();
    assert!(
        !tool_logged.trim().is_empty(),
        "stubbing quoin produced no invocation, so PATH is not being consulted by \
         the subprocess and the run above proves nothing"
    );
    assert!(
        !control.status.success(),
        "the chain succeeded with quoin stubbed out, so it is not actually using it"
    );

    // Run C: what is Quire asked to do?
    //
    // Quire is not in run A's list, and that exclusion is reasoned rather than
    // assumed — it was measured. Shimming `quire` alongside the producers made
    // run A fail, and the log said why: `quoin evidence audit` shells out to
    // `quire coverage --scope <its own scratch repo> --json`. That is Quoin
    // reading static facts, which is exactly what the architecture says Quoin
    // does with Quire's export; it is not Quoin executing a producer.
    //
    // So the claim here is narrower and checkable: every request made of Quire is
    // a static read. This run's chain is expected to fail — the shim cannot serve
    // a real export — and that is fine, because what is being read is the log,
    // not the exit code.
    let quire_shims = root().join("target/quire-shims");
    let quire_log = producer_shims(&quire_shims, &["quire"]);
    let _ = run_chain_with_path(&quire_shims);
    let quire_logged = fs::read_to_string(&quire_log).unwrap_or_default();
    assert!(
        !quire_logged.trim().is_empty(),
        "stubbing quire produced no invocation, so this run observed nothing"
    );
    for line in quire_logged.lines().filter(|line| !line.trim().is_empty()) {
        let subcommand = line.split_whitespace().nth(1).unwrap_or("");
        assert!(
            matches!(subcommand, "provenance" | "coverage"),
            "Quire was asked to do something other than a static read: {line}"
        );
    }
}

/// Trace: TC-011, FR-005-AC-3
#[test]
fn tc_011_the_sealed_records_impact_snapshot_is_the_quire_export() {
    let report = chain_report();
    let export = root().join(report["quire_export"].as_str().expect("quire_export"));
    let bytes =
        fs::read(&export).unwrap_or_else(|error| panic!("{} is absent: {error}", export.display()));

    assert_eq!(
        report["impact_snapshot_digest"],
        sha256_of(&export),
        "the sealed record's impact snapshot does not name the Quire export it claims"
    );
    // An empty object has a digest too. The snapshot is only worth its content,
    // so the export is required to actually carry the coverage facts the record
    // claims it snapshotted, and to name every requirement this repository has.
    let parsed: Value = serde_json::from_slice(&bytes).expect("the Quire export is JSON");
    let text = String::from_utf8_lossy(&bytes);
    for requirement in [
        "FR-001", "FR-002", "FR-003", "FR-004", "FR-005", "NFR-001", "NFR-002", "StR-001",
    ] {
        assert!(
            text.contains(requirement),
            "the Quire export does not mention {requirement}; it is not a coverage \
             export of this repository"
        );
    }
    assert!(
        parsed.is_object() && !parsed.as_object().unwrap().is_empty(),
        "the Quire export is not a populated document"
    );

    // And the chain must have read it as such rather than as a not-computed run.
    assert_eq!(
        report["attested_results"]["PROOF-quire-static-export"], "passed",
        "the Quire export was attested as {}",
        report["attested_results"]["PROOF-quire-static-export"]
    );
}

/// Collect every readable source file under `directory`, recursively.
fn collect_sources(directory: &Path, into: &mut Vec<PathBuf>) {
    // Excluded, and each for its own reason: `.git` is not source, `target` is
    // build output, and `.venv-assurance` is the pinned upstream release rather
    // than anything this repository wrote.
    const EXCLUDED: [&str; 3] = [".git", "target", ".venv-assurance"];
    let Ok(entries) = fs::read_dir(directory) else {
        return;
    };
    for entry in entries {
        let path = entry.expect("directory entry").path();
        if path.is_dir() {
            let name = path
                .file_name()
                .and_then(|value| value.to_str())
                .unwrap_or("");
            if EXCLUDED.contains(&name) {
                continue;
            }
            collect_sources(&path, into);
            continue;
        }
        let extension = path.extension().and_then(|value| value.to_str());
        if matches!(
            extension,
            Some("py" | "sh" | "rs" | "txt" | "toml" | "yml" | "md" | "json")
        ) {
            into.push(path);
        }
    }
}

/// Trace: TC-013, TC-003, FR-005-AC-5, NFR-002-AC-3
#[test]
fn tc_013_all_twelve_verification_outcomes_are_demonstrated_and_paired_with_controls() {
    // The twelve states this migration must keep distinguishable, and the gate
    // that owns each. A state nobody demonstrates is a state nobody would notice
    // the loss of.
    const REQUIRED: [(&str, &str); 12] = [
        ("pass", "chain"),
        ("fail", "chain"),
        ("unavailable", "chain"),
        ("unsupported", "chain"),
        ("inconclusive", "chain"),
        ("not-computed", "chain"),
        ("malformed", "chain"),
        ("partial", "chain"),
        ("stale", "chain"),
        ("suspect", "chain"),
        ("vacuous", "chain"),
        ("tampered", "chain"),
    ];

    // Every one of the twelve now comes from the chain itself. Until
    // agent-ix/quire-contract-runtime#11 this census was a union of the chain's
    // states with the retained-evidence compatibility census's case kinds, and
    // `unsupported` and `malformed` were reachable only through that census —
    // measured on the pre-deletion tree, the chain alone reached ten of twelve.
    // Deleting the retained records without replacing those two demonstrations
    // would have quietly taken FR-005-AC-5 from twelve to ten, so both were
    // re-established on surfaces that never read a retained byte: Quoin naming a
    // declared verification method its catalog does not have, and a producer row
    // carrying the outcome the mutation campaign emits for a mutation anchor
    // that is no longer present exactly once.
    let report = chain_report();

    let demonstrated: BTreeSet<String> = report["states_demonstrated"]
        .as_array()
        .unwrap()
        .iter()
        .map(|value| value.as_str().unwrap().to_owned())
        .collect();

    let missing: Vec<&str> = REQUIRED
        .iter()
        .filter(|(state, _)| !demonstrated.contains(*state))
        .map(|(state, _)| *state)
        .collect();
    assert!(
        missing.is_empty(),
        "these verification outcomes were never demonstrated: {missing:?}; \
         demonstrated: {demonstrated:?}"
    );

    // Every negative names the positive control that proves the step it refuses
    // is a step that works.
    let controls = report["controls"].as_array().unwrap();
    assert!(!controls.is_empty(), "no positive controls were run");
    let negatives: BTreeSet<&str> = controls
        .iter()
        .map(|control| control["pairs_with"].as_str().unwrap())
        .collect();
    for required in [
        "retained-bytes-changed-after-sealing",
        "refuse-an-edited-receipt",
        "stale-candidate-binding",
        "attested-failed",
    ] {
        assert!(
            negatives.contains(required),
            "the negative {required} has no positive control"
        );
    }
}

/// Trace: TC-014, FR-005-AC-6
#[test]
fn tc_014_no_local_evidence_framework_remains() {
    let root = root();

    // The generic machinery is gone, by name.
    for removed in [
        "scripts/build_evidence_envelope.py",
        "scripts/collect_evidence.sh",
        "scripts/verify_evidence.py",
        "scripts/check_assurance_anchor.py",
        "scripts/update_evidence_anchors.py",
        "scripts/check_failure_propagation.py",
        "scripts/check_coverage_status.py",
        "scripts/validate_json_schema.py",
        "scripts/evidence_policy.py",
        "scripts/run_evidence_tests.py",
        "tests/test_evidence_tooling.py",
        "requirements-evidence.txt",
    ] {
        assert!(
            !root.join(removed).exists(),
            "{removed} is still present; the generic evidence machinery was not removed"
        );
    }

    // The retained-evidence tree, its reader, its fixtures and the schema family
    // that was frozen only because those records named it by digest are gone
    // too. The repository owner released the preservation constraint for the
    // pre-stable phase on 2026-09-02 (agent-ix/engineering-assurance#7); nothing
    // was rewritten, backdated or re-sealed to survive the deletion.
    for removed in [
        "evidence",
        "schemas",
        "scripts/legacy_evidence_view.py",
        "tests/fixtures/legacy-compat",
        "spec/test/TC-012-legacy-compatibility-view.md",
    ] {
        assert!(
            !root.join(removed).exists(),
            "{removed} is still present; the retained-evidence machinery was not removed"
        );
    }

    // Nothing may name any of them either. A deleted directory that a Make
    // target, a workflow step or a chain obligation still points at is a gate
    // that fails on a fresh clone, and a deleted reader still named in a
    // declared command is a lie in a sealed attestation.
    //
    // The census walks the repository root and excludes, rather than naming the
    // directories it will look in. An inclusion list is a list of the places a
    // reintroduced reader would have to avoid, and it only has to be incomplete
    // once.
    let mut sources = Vec::new();
    collect_sources(&root, &mut sources);

    // The claim is that nothing *runs* or *validates against* the deleted
    // material, so the assertion runs over the surfaces that can: code,
    // configuration, and workflow files. Markdown is excluded and deliberately
    // so — `planning/pgm-01-reconciliation.md` and this repository's review and
    // plan artifacts name what was deleted because they are records of it, and
    // prose runs nothing. Widening the walk found those references immediately,
    // which is the point of widening it; the fix is to scope the assertion
    // honestly rather than to narrow the walk back.
    //
    // This file is exempt because naming the deleted paths is the whole of this
    // test.
    let mut inspected = 0;
    for path in &sources {
        let extension = path
            .extension()
            .and_then(|value| value.to_str())
            .unwrap_or("");
        if extension == "md" {
            continue;
        }
        if path.file_name().and_then(|value| value.to_str()) == Some("shared_assurance.rs") {
            continue;
        }
        let Ok(source) = fs::read_to_string(path) else {
            continue;
        };
        inspected += 1;
        for gone in [
            "legacy_evidence_view",
            "legacy-compat",
            "PROOF-legacy-compatibility",
            "compat-view",
            "pgm01-derivation-evidence-envelope-v1.schema.json",
            "runtime-evidence-input-v1.schema.json",
            "runtime-evidence-manifest-v1.schema.json",
            "pgm01-validate-governance.py",
        ] {
            assert!(
                !source.contains(gone),
                "{} still references the deleted {gone}",
                path.display()
            );
        }
    }
    assert!(
        inspected > 30,
        "the executable and configuration census is unexpectedly small ({inspected}) \
         to make this claim"
    );

    // The Makefile is orchestration, not a trust root, and carries no gate that
    // polices its own execution. Target definitions are matched, not bare words:
    // `quire coverage --scope . --strict` legitimately contains "coverage".
    let makefile = fs::read_to_string(root.join("Makefile")).unwrap();
    for gone in [
        "\nverify-evidence:",
        "\nassurance-anchor:",
        "\nevidence-tool:",
        "\nci-guard:",
        "\nupdate-evidence-anchors:",
        "\ncoverage:",
    ] {
        assert!(
            !makefile.contains(gone),
            "the Makefile still defines the {gone} self-attestation target"
        );
    }
    assert!(
        !makefile.contains("MAKEFLAGS"),
        "the Makefile still polices its own execution controls"
    );

    // Make can be told to ignore failure, and a single line does it. `.IGNORE:`
    // at the top, a `-` prefix on a recipe line, or `SHELL := /bin/true` each
    // turn a red gate into `make` exit 0 while the gate itself still prints its
    // failure. The 311-line recipe-failure policer that used to catch this went
    // with the collector it was protecting, and its absence was found by probe
    // rather than by reasoning.
    //
    // What replaces it is not another policer target — that would be the
    // Makefile attesting to itself again. It is this assertion, in the test
    // suite, that the file declares none of the directives whose only purpose is
    // to stop a failure propagating.
    // Directives are matched structurally, on the lines that could be one: not
    // comments, not recipe bodies. The header of that file explains this very
    // hazard and names `.IGNORE:` to do so, and a substring scan would report the
    // rule being written down as a violation of itself — the same trap
    // `check_shared_pins.py` avoids when it looks for the mirror registry.
    //
    // `-k` and `--keep-going` are command-line flags and cannot be declared here
    // except through MAKEFLAGS, which is asserted absent above.
    for line in makefile.lines() {
        if line.starts_with('\t') {
            continue;
        }
        let statement = line.split('#').next().unwrap_or("").trim();
        if statement.is_empty() {
            continue;
        }
        for directive in [".IGNORE", ".SILENT", ".ONESHELL", ".SHELLFLAGS"] {
            assert!(
                !statement.starts_with(directive),
                "the Makefile declares {directive}, which stops a failing gate from \
                 failing the build: {line}"
            );
        }
        let assigns_shell = statement
            .split_once([':', '=', '?'])
            .map(|(target, _)| target.trim() == "SHELL")
            .unwrap_or(false);
        assert!(
            !assigns_shell,
            "the Makefile assigns SHELL, which can make every recipe report success: {line}"
        );
    }
    for (number, line) in makefile.lines().enumerate() {
        let Some(recipe) = line.strip_prefix('\t') else {
            continue;
        };
        let command = recipe.trim_start_matches(['@', '+']);
        assert!(
            !command.starts_with('-'),
            "Makefile:{} prefixes a recipe line with `-`, which ignores its exit status: {line}",
            number + 1
        );
    }

    // And the gates that replaced it are actually reachable from `ci:`.
    //
    // This asks Make what it would run, not what the file says. A text assertion
    // that the Makefile mentions a script is satisfied by the script being
    // mentioned in a comment, and survives the entire `ci:` prerequisite list
    // being deleted — which is exactly how the assertion this replaced behaved.
    // `make -n` expands the dependency graph, so removing a prerequisite removes
    // its recipe line from this output.
    let dry_run = Command::new("make")
        .args(["-n", "ci"])
        .current_dir(&root)
        .output()
        .expect("make -n ci failed to run");
    assert!(
        dry_run.status.success(),
        "make -n ci did not resolve: {}",
        String::from_utf8_lossy(&dry_run.stderr)
    );
    let planned = String::from_utf8_lossy(&dry_run.stdout);
    for required in [
        "scripts/run_feature_matrix.py",
        "scripts/run_kani_gate.py",
        "scripts/check_kani_harnesses.py",
        "scripts/check_kani_mutations.py",
        "scripts/measure_footprint.py",
        "scripts/check_shared_pins.py",
        "scripts/assurance_chain.py",
        // The test runner itself. Without this line, deleting `test` from the
        // `ci:` prerequisite list is invisible: `assurance-inputs` still supplies
        // every script named above, so `make ci` stays green while TC-009 through
        // TC-014 — the whole enforcement layer for FR-005 — never runs.
        "cargo test --all-features",
    ] {
        assert!(
            planned.contains(required),
            "`make ci` would not run {required}; it is defined but unreachable"
        );
    }
}
