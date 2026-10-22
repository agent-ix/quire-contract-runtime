//! Local verification gates driven by controlled command and filesystem seams.

use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::Command;
use users::os::unix::UserExt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandOutput {
    pub status: i32,
    pub stdout: String,
    pub stderr: String,
}

#[derive(Debug, Clone)]
pub struct CommandSpec {
    pub program: PathBuf,
    pub args: Vec<String>,
    pub cwd: PathBuf,
    pub env: Vec<(String, String)>,
}

pub trait CommandRunner {
    fn run(&mut self, command: &CommandSpec) -> io::Result<CommandOutput>;
}

pub struct SystemRunner;

impl CommandRunner for SystemRunner {
    fn run(&mut self, command: &CommandSpec) -> io::Result<CommandOutput> {
        let output = Command::new(&command.program)
            .args(&command.args)
            .current_dir(&command.cwd)
            .envs(command.env.iter().map(|(key, value)| (key, value)))
            .output()?;
        Ok(CommandOutput {
            status: output.status.code().unwrap_or(-1),
            stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
            stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Outcome {
    Pass,
    Fail,
    Inconclusive,
    Malformed,
    Unavailable,
}

impl Outcome {
    pub fn label(self) -> &'static str {
        match self {
            Self::Pass => "PASS",
            Self::Fail => "FAIL",
            Self::Inconclusive => "INCONCLUSIVE",
            Self::Malformed => "MALFORMED",
            Self::Unavailable => "UNAVAILABLE",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Phase {
    Build,
    Test,
    TestOnly,
    BuildOnly,
}

impl Phase {
    pub fn label(self) -> &'static str {
        match self {
            Self::Build => "build",
            Self::Test => "test",
            Self::TestOnly => "test-only",
            Self::BuildOnly => "build-only",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Row {
    pub symbol: String,
    pub outcome: Outcome,
    pub phase: Option<Phase>,
    pub detail: Option<String>,
    pub trace_ids: &'static [&'static str],
}

impl Row {
    fn with_trace_ids(mut self, trace_ids: &'static [&'static str]) -> Self {
        self.trace_ids = trace_ids;
        self
    }
}

fn row(symbol: &str, outcome: Outcome, phase: Option<Phase>, detail: Option<String>) -> Row {
    Row {
        symbol: symbol.into(),
        outcome,
        phase,
        detail,
        trace_ids: &[],
    }
}

fn unavailable(symbol: &str, phase: Option<Phase>, error: &io::Error) -> Row {
    row(symbol, Outcome::Unavailable, phase, Some(error.to_string()))
}

/// Classify the result of one owning Kani harness run.
pub fn classify_kani<R: CommandRunner>(
    runner: &mut R,
    command: &CommandSpec,
    harness: &str,
) -> Row {
    let output = match runner.run(command) {
        Ok(output) => output,
        Err(error) => return unavailable(harness, None, &error),
    };
    if output.status == 0 {
        return row(
            harness,
            Outcome::Fail,
            None,
            Some(format!("Kani accepted the injected defect for {harness}")),
        );
    }
    let combined = format!("{}\n{}", output.stdout, output.stderr);
    let marker = format!("Checking harness kani_proofs::{harness}...");
    if combined.contains(&marker) && combined.contains("VERIFICATION:- FAILED") {
        return row(harness, Outcome::Pass, None, None);
    }
    let tail: String = combined
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .chars()
        .rev()
        .take(200)
        .collect::<String>()
        .chars()
        .rev()
        .collect();
    row(harness, Outcome::Inconclusive, None, Some(format!("cargo kani exited {} for {harness} without reaching a verification result; output tail: {tail:?}", output.status)))
}

/// Extract structured compiler errors from Cargo's JSON message stream.
pub fn build_errors(stdout: &str) -> Vec<String> {
    stdout
        .lines()
        .filter_map(|line| {
            let value: serde_json::Value = serde_json::from_str(line).ok()?;
            if value.get("reason")?.as_str()? != "compiler-message"
                || value.pointer("/message/level")?.as_str()? != "error"
            {
                return None;
            }
            Some(
                value
                    .pointer("/message/rendered")
                    .and_then(serde_json::Value::as_str)
                    .and_then(|rendered| rendered.trim().lines().next())
                    .unwrap_or("compiler reported an error without rendered text")
                    .to_owned(),
            )
        })
        .collect()
}

/// Run one feature row, stopping before tests when the build phase fails.
pub fn run_feature_row<R: CommandRunner>(
    runner: &mut R,
    name: &str,
    build: Option<&CommandSpec>,
    test: &CommandSpec,
) -> Row {
    if let Some(build) = build {
        let output = match runner.run(build) {
            Ok(output) => output,
            Err(error) => return unavailable(name, Some(Phase::Build), &error),
        };
        let errors = build_errors(&output.stdout);
        if output.status != 0 || !errors.is_empty() {
            let detail = if errors.is_empty() {
                "cargo reported a non-zero build status".into()
            } else {
                errors.into_iter().take(3).collect::<Vec<_>>().join("; ")
            };
            return row(name, Outcome::Fail, Some(Phase::Build), Some(detail));
        }
    }
    let phase = if build.is_some() {
        Phase::Test
    } else {
        Phase::TestOnly
    };
    match runner.run(test) {
        Ok(output) if output.status == 0 => row(name, Outcome::Pass, Some(phase), None),
        Ok(output) => row(
            name,
            Outcome::Fail,
            Some(phase),
            Some(
                output
                    .stdout
                    .lines()
                    .rev()
                    .take(6)
                    .collect::<Vec<_>>()
                    .into_iter()
                    .rev()
                    .collect::<Vec<_>>()
                    .join("\n"),
            ),
        ),
        Err(error) => unavailable(name, Some(phase), &error),
    }
}

pub struct FeatureSet {
    pub name: &'static str,
    pub selection: &'static [&'static str],
    pub domain_targets: bool,
    pub trace_ids: &'static [&'static str],
    pub two_phase: bool,
}

pub const FEATURE_SETS: &[FeatureSet] = &[
    FeatureSet {
        name: "test-core",
        selection: &["--no-default-features"],
        domain_targets: true,
        trace_ids: &["NFR-001"],
        two_phase: true,
    },
    FeatureSet {
        name: "test-core-doc",
        selection: &["--no-default-features", "--doc"],
        domain_targets: false,
        trace_ids: &["NFR-002"],
        two_phase: false,
    },
    FeatureSet {
        name: "test-alloc",
        selection: &["--features", "alloc"],
        domain_targets: true,
        trace_ids: &["NFR-001"],
        two_phase: true,
    },
    FeatureSet {
        name: "test-alloc-doc",
        selection: &["--features", "alloc", "--doc"],
        domain_targets: false,
        trace_ids: &["NFR-002"],
        two_phase: false,
    },
    FeatureSet {
        name: "test-std",
        selection: &["--features", "std"],
        domain_targets: true,
        trace_ids: &["NFR-001"],
        two_phase: true,
    },
    FeatureSet {
        name: "test-std-doc",
        selection: &["--features", "std", "--doc"],
        domain_targets: false,
        trace_ids: &["NFR-002"],
        two_phase: false,
    },
    FeatureSet {
        name: "test-snapshot-json",
        selection: &[
            "--locked",
            "--no-default-features",
            "--features",
            "snapshot-json",
        ],
        domain_targets: true,
        trace_ids: &["TC-015", "FR-004"],
        two_phase: true,
    },
    FeatureSet {
        name: "test-snapshot-json-doc",
        selection: &[
            "--locked",
            "--no-default-features",
            "--features",
            "snapshot-json",
            "--doc",
        ],
        domain_targets: false,
        trace_ids: &["NFR-002", "TC-015"],
        two_phase: false,
    },
    FeatureSet {
        name: "test-all",
        selection: &["--all-features"],
        domain_targets: true,
        trace_ids: &["NFR-001"],
        two_phase: true,
    },
    FeatureSet {
        name: "test-all-doc",
        selection: &["--all-features", "--doc"],
        domain_targets: false,
        trace_ids: &["NFR-002"],
        two_phase: false,
    },
    FeatureSet {
        name: "test-footprint",
        selection: &["-p", "quire-contract-runtime-footprint"],
        domain_targets: false,
        trace_ids: &["NFR-001"],
        two_phase: true,
    },
    FeatureSet {
        name: "test-exact",
        selection: &[
            "--locked",
            "--no-default-features",
            "--features",
            "exact",
            "--test",
            "exact_outcomes",
            "--test",
            "exact_meter_state",
        ],
        domain_targets: false,
        trace_ids: &["TC-031", "TC-032", "FR-006", "FR-007"],
        two_phase: true,
    },
];

const DOMAIN_TARGETS: &[&str] = &[
    "--lib",
    "--test",
    "integration",
    "--test",
    "operators",
    "--test",
    "proptest_adapter",
    "--test",
    "proptest_adapter_absent",
    "--test",
    "snapshot",
];

pub struct NoStdBuild {
    pub name: &'static str,
    pub feature: &'static str,
    pub trace_ids: &'static [&'static str],
}

pub const NO_STD_BUILDS: &[NoStdBuild] = &[
    NoStdBuild {
        name: "build-snapshot-json-no-std-msrv",
        feature: "snapshot-json",
        trace_ids: &["TC-015", "NFR-001"],
    },
    NoStdBuild {
        name: "build-exact-no-std-msrv",
        feature: "exact",
        trace_ids: &["TC-016", "TC-199", "FR-006", "FR-275", "NFR-001"],
    },
];

pub fn trusted_home() -> io::Result<PathBuf> {
    users::get_user_by_uid(users::get_current_uid())
        .map(|user| user.home_dir().to_path_buf())
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "current user has no passwd home"))
}

fn isolated_env(home: &Path, target: &Path) -> Vec<(String, String)> {
    vec![
        ("HOME".into(), home.display().to_string()),
        (
            "CARGO_HOME".into(),
            home.join(".cargo").display().to_string(),
        ),
        (
            "RUSTUP_HOME".into(),
            home.join(".rustup").display().to_string(),
        ),
        ("CARGO_TARGET_DIR".into(), target.display().to_string()),
    ]
}

fn cargo(home: &Path) -> PathBuf {
    home.join(".cargo/bin/cargo")
}

fn spec(home: &Path, root: &Path, target: &Path, args: &[&str]) -> CommandSpec {
    CommandSpec {
        program: cargo(home),
        args: args.iter().map(|arg| (*arg).into()).collect(),
        cwd: root.into(),
        env: isolated_env(home, target),
    }
}

pub fn collect_features<R: CommandRunner>(runner: &mut R, root: &Path, home: &Path) -> Vec<Row> {
    let available = cargo(home).is_file();
    let target = root.join("target");
    let mut rows = Vec::new();
    for feature in FEATURE_SETS {
        if !available {
            rows.push(
                row(
                    feature.name,
                    Outcome::Unavailable,
                    None,
                    Some("the trusted cargo executable is absent".into()),
                )
                .with_trace_ids(feature.trace_ids),
            );
            continue;
        }
        let mut flags = vec!["test"];
        flags.extend_from_slice(feature.selection);
        if feature.domain_targets {
            flags.extend_from_slice(DOMAIN_TARGETS);
        }
        let mut build_flags = flags.clone();
        build_flags.extend(["--no-run", "--message-format=json"]);
        let build = spec(home, root, &target, &build_flags);
        let test = spec(home, root, &target, &flags);
        rows.push(
            run_feature_row(
                runner,
                feature.name,
                feature.two_phase.then_some(&build),
                &test,
            )
            .with_trace_ids(feature.trace_ids),
        );
    }
    for build in NO_STD_BUILDS {
        if !available {
            rows.push(
                row(
                    build.name,
                    Outcome::Unavailable,
                    None,
                    Some("the trusted cargo executable is absent".into()),
                )
                .with_trace_ids(build.trace_ids),
            );
            continue;
        }
        let command = spec(
            home,
            root,
            &target,
            &[
                "+1.98.1",
                "build",
                "--locked",
                "--lib",
                "--no-default-features",
                "--features",
                build.feature,
                "--target",
                "thumbv7em-none-eabi",
                "--message-format=json",
            ],
        );
        let result = match runner.run(&command) {
            Ok(output) => {
                let errors = build_errors(&output.stdout);
                if output.status == 0 && errors.is_empty() {
                    row(build.name, Outcome::Pass, Some(Phase::BuildOnly), None)
                } else {
                    row(
                        build.name,
                        Outcome::Fail,
                        Some(Phase::BuildOnly),
                        Some(if errors.is_empty() {
                            "cargo reported a non-zero build status".into()
                        } else {
                            errors.into_iter().take(3).collect::<Vec<_>>().join("; ")
                        }),
                    )
                }
            }
            Err(error) => unavailable(build.name, Some(Phase::BuildOnly), &error),
        };
        rows.push(result.with_trace_ids(build.trace_ids));
    }
    rows
}

pub struct Mutation {
    pub relative: &'static str,
    pub anchor: &'static str,
    pub replacement: &'static str,
    pub harness: &'static str,
}

pub const MUTATIONS: &[Mutation] = &[
    Mutation {
        relative: "src/operators.rs",
        anchor: "    left.checked_add(right)\n",
        replacement: "    None\n",
        harness: "tc_003_checked_i8_arithmetic_matches_primitives",
    },
    Mutation {
        relative: "src/operators.rs",
        anchor: "        R::from(false)\n",
        replacement: "        R::from(true)\n",
        harness: "tc_002_boolean_truth_tables",
    },
    Mutation {
        relative: "src/accounting.rs",
        anchor: "VerdictKind::Passed => self.accepted = self.accepted.saturating_add(1),",
        replacement: "VerdictKind::Passed => {},",
        harness: "tc_003_campaign_accounting_saturates",
    },
];

fn copy_candidate(source: &Path, destination: &Path) -> io::Result<()> {
    fs::create_dir(destination)?;
    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let name = entry.file_name();
        if matches!(name.to_str(), Some(".git" | "target" | "__pycache__")) {
            continue;
        }
        let kind = entry.file_type()?;
        let destination = destination.join(name);
        if kind.is_dir() {
            copy_candidate(&entry.path(), &destination)?;
        } else if kind.is_file() {
            fs::copy(entry.path(), destination)?;
        }
    }
    Ok(())
}

pub fn run_mutation<R: CommandRunner>(
    runner: &mut R,
    root: &Path,
    home: &Path,
    mutation: &Mutation,
) -> Row {
    let symbol = format!("mutation::{}::{}", mutation.harness, mutation.relative);
    let scratch = match tempfile::tempdir() {
        Ok(dir) => dir,
        Err(error) => return unavailable(&symbol, None, &error),
    };
    let candidate = scratch.path().join("candidate");
    if let Err(error) = copy_candidate(root, &candidate) {
        return unavailable(&symbol, None, &error);
    }
    let source = candidate.join(mutation.relative);
    let text = match fs::read_to_string(&source) {
        Ok(text) => text,
        Err(error) => return unavailable(&symbol, None, &error),
    };
    let count = text.matches(mutation.anchor).count();
    if count != 1 {
        return row(&symbol, Outcome::Malformed, None, Some(format!("mutation anchor for {} occurs {count} times, not once; the campaign no longer describes this source", mutation.relative)));
    }
    if let Err(error) = fs::write(
        &source,
        text.replacen(mutation.anchor, mutation.replacement, 1),
    ) {
        return unavailable(&symbol, None, &error);
    }
    let command = spec(
        home,
        &candidate,
        &candidate.join("target"),
        &["kani", "--harness", mutation.harness, "--features", "exact"],
    );
    let mut result = classify_kani(runner, &command, mutation.harness);
    result.symbol = symbol;
    result
}

pub fn collect_mutations<R: CommandRunner>(runner: &mut R, root: &Path, home: &Path) -> Vec<Row> {
    if !cargo(home).is_file() || !home.join(".cargo/bin/cargo-kani").is_file() {
        return MUTATIONS
            .iter()
            .map(|mutation| {
                row(
                    &format!("mutation::{}::{}", mutation.harness, mutation.relative),
                    Outcome::Unavailable,
                    None,
                    Some("the trusted cargo-kani toolchain is absent".into()),
                )
            })
            .collect();
    }
    MUTATIONS
        .iter()
        .map(|mutation| run_mutation(runner, root, home, mutation))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use ix_trace_rs::trace;
    use std::collections::VecDeque;

    struct FakeRunner(VecDeque<io::Result<CommandOutput>>);
    impl CommandRunner for FakeRunner {
        fn run(&mut self, _: &CommandSpec) -> io::Result<CommandOutput> {
            self.0.pop_front().expect("unexpected command")
        }
    }
    fn command() -> CommandSpec {
        CommandSpec {
            program: "cargo".into(),
            args: vec![],
            cwd: ".".into(),
            env: vec![],
        }
    }
    fn output(status: i32, stdout: &str) -> io::Result<CommandOutput> {
        Ok(CommandOutput {
            status,
            stdout: stdout.into(),
            stderr: String::new(),
        })
    }

    #[test]
    #[trace("TC-002", "TC-003")]
    fn kani_requires_owning_harness_and_verification_failure() {
        let mut runner = FakeRunner(VecDeque::from([output(
            1,
            "Checking harness kani_proofs::owner...\nVERIFICATION:- FAILED",
        )]));
        assert_eq!(
            classify_kani(&mut runner, &command(), "owner").outcome,
            Outcome::Pass
        );
        let mut runner = FakeRunner(VecDeque::from([output(
            0,
            "Checking harness kani_proofs::owner...\nVERIFICATION:- SUCCESSFUL",
        )]));
        assert_eq!(
            classify_kani(&mut runner, &command(), "owner").outcome,
            Outcome::Fail
        );
        let mut runner = FakeRunner(VecDeque::from([output(1, "compiler error")]));
        assert_eq!(
            classify_kani(&mut runner, &command(), "owner").outcome,
            Outcome::Inconclusive
        );
        let mut runner = FakeRunner(VecDeque::from([output(
            1,
            "Checking harness kani_proofs::other...\nVERIFICATION:- FAILED",
        )]));
        assert_eq!(
            classify_kani(&mut runner, &command(), "owner").outcome,
            Outcome::Inconclusive
        );
    }

    #[test]
    #[trace("TC-199", "FR-006-AC-8")]
    fn feature_build_error_skips_test_phase() {
        let mut runner = FakeRunner(VecDeque::from([output(
            0,
            r#"{"reason":"compiler-message","message":{"level":"error","rendered":"error: failed\n"}}"#,
        )]));
        let row = run_feature_row(&mut runner, "test-exact", Some(&command()), &command());
        assert_eq!(row.outcome, Outcome::Fail);
        assert_eq!(row.phase, Some(Phase::Build));
        assert!(runner.0.is_empty());
        let mut runner = FakeRunner(VecDeque::from([output(
            0,
            r#"{"reason":"compiler-message","message":{"level":"error"}}"#,
        )]));
        assert_eq!(
            run_feature_row(&mut runner, "test-exact", Some(&command()), &command()).outcome,
            Outcome::Fail
        );
        assert!(runner.0.is_empty());
        let mut runner = FakeRunner(VecDeque::from([output(101, "non-json cargo failure")]));
        let row = run_feature_row(&mut runner, "test-exact", Some(&command()), &command());
        assert_eq!(row.phase, Some(Phase::Build));
        assert_eq!(row.outcome, Outcome::Fail);
        assert!(runner.0.is_empty());
    }

    #[test]
    #[trace("TC-199", "FR-006-AC-8")]
    fn feature_test_exit_is_distinct_from_build_failure_and_unavailable() {
        let mut runner = FakeRunner(VecDeque::from([output(0, ""), output(7, "test failed")]));
        let row = run_feature_row(&mut runner, "test-exact", Some(&command()), &command());
        assert_eq!(row.outcome, Outcome::Fail);
        assert_eq!(row.phase, Some(Phase::Test));
        let mut runner = FakeRunner(VecDeque::from([Err(io::Error::new(
            io::ErrorKind::NotFound,
            "cargo absent",
        ))]));
        assert_eq!(
            run_feature_row(&mut runner, "test-exact", Some(&command()), &command()).outcome,
            Outcome::Unavailable
        );
    }

    #[test]
    #[trace("TC-002", "TC-003", "TC-006")]
    fn mutation_anchor_must_occur_once_before_invoking_kani() {
        struct ObservingRunner {
            calls: usize,
        }
        impl CommandRunner for ObservingRunner {
            fn run(&mut self, command: &CommandSpec) -> io::Result<CommandOutput> {
                self.calls += 1;
                let source = fs::read_to_string(command.cwd.join("src/operators.rs"))?;
                assert_eq!(source, "replacement\n");
                assert_eq!(
                    command.args,
                    ["kani", "--harness", "owner", "--features", "exact"]
                );
                assert!(command
                    .env
                    .iter()
                    .any(|(key, value)| key == "CARGO_TARGET_DIR"
                        && value == &command.cwd.join("target").display().to_string()));
                for (key, suffix) in [
                    ("HOME", ""),
                    ("CARGO_HOME", "/.cargo"),
                    ("RUSTUP_HOME", "/.rustup"),
                ] {
                    assert!(command.env.iter().any(|(name, value)| name == key
                        && value
                            == &format!(
                                "{}{suffix}",
                                command
                                    .program
                                    .parent()
                                    .expect("bin")
                                    .parent()
                                    .expect("cargo")
                                    .parent()
                                    .expect("home")
                                    .display()
                            )));
                }
                output(
                    1,
                    "Checking harness kani_proofs::owner...\nVERIFICATION:- FAILED",
                )
            }
        }
        let root = tempfile::tempdir().expect("root");
        fs::create_dir(root.path().join("src")).expect("src");
        fs::write(root.path().join("src/operators.rs"), "anchor\n").expect("source");
        let mutation = Mutation {
            relative: "src/operators.rs",
            anchor: "anchor\n",
            replacement: "replacement\n",
            harness: "owner",
        };
        let mut runner = ObservingRunner { calls: 0 };
        assert_eq!(
            run_mutation(&mut runner, root.path(), root.path(), &mutation).outcome,
            Outcome::Pass
        );
        assert_eq!(runner.calls, 1);
        fs::write(root.path().join("src/operators.rs"), "anchor\nanchor\n").expect("source");
        assert_eq!(
            run_mutation(&mut runner, root.path(), root.path(), &mutation).outcome,
            Outcome::Malformed
        );
        assert_eq!(runner.calls, 1);
        assert_eq!(
            fs::read_to_string(root.path().join("src/operators.rs")).expect("source"),
            "anchor\nanchor\n"
        );
    }

    #[test]
    #[trace("TC-002", "TC-003", "TC-006")]
    fn absent_kani_marks_each_declared_mutation_unavailable() {
        let root = tempfile::tempdir().expect("root");
        let home = tempfile::tempdir().expect("home");
        let mut runner = FakeRunner(VecDeque::new());
        let rows = collect_mutations(&mut runner, root.path(), home.path());
        assert_eq!(rows.len(), 3);
        assert!(rows.iter().all(|row| row.outcome == Outcome::Unavailable));
    }

    #[test]
    #[trace("TC-199", "FR-006-AC-8", "interface-001-AC-13")]
    fn matrix_runs_all_declared_rows_and_two_no_std_builds() {
        struct RecordingRunner(Vec<CommandSpec>);
        impl CommandRunner for RecordingRunner {
            fn run(&mut self, command: &CommandSpec) -> io::Result<CommandOutput> {
                self.0.push(command.clone());
                output(0, "")
            }
        }
        let root = tempfile::tempdir().expect("root");
        let home = tempfile::tempdir().expect("home");
        fs::create_dir_all(home.path().join(".cargo/bin")).expect("cargo bin");
        fs::write(home.path().join(".cargo/bin/cargo"), "").expect("cargo executable");
        let mut runner = RecordingRunner(Vec::new());
        let rows = collect_features(&mut runner, root.path(), home.path());
        assert_eq!(rows.len(), 14);
        assert_eq!(runner.0.len(), 21);
        assert_eq!(
            rows.iter()
                .map(|row| row.symbol.as_str())
                .collect::<Vec<_>>(),
            [
                "test-core",
                "test-core-doc",
                "test-alloc",
                "test-alloc-doc",
                "test-std",
                "test-std-doc",
                "test-snapshot-json",
                "test-snapshot-json-doc",
                "test-all",
                "test-all-doc",
                "test-footprint",
                "test-exact",
                "build-snapshot-json-no-std-msrv",
                "build-exact-no-std-msrv"
            ]
        );
        assert!(rows.iter().all(|row| row.outcome == Outcome::Pass));
        let no_std = runner
            .0
            .iter()
            .filter(|command| command.args.first().is_some_and(|arg| arg == "+1.98.1"))
            .collect::<Vec<_>>();
        assert_eq!(no_std.len(), 2);
        for command in no_std {
            assert!(command
                .args
                .windows(2)
                .any(|pair| pair == ["--target", "thumbv7em-none-eabi"]));
            assert!(command
                .args
                .iter()
                .any(|arg| arg == "--no-default-features"));
        }
    }
}
