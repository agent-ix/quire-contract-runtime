//! Local verification gates driven by controlled command and filesystem seams.

#![forbid(unsafe_code)]

use std::io;
use std::path::PathBuf;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Outcome {
    Pass,
    Fail,
    Inconclusive,
    Malformed,
    Unavailable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Phase {
    Build,
    Test,
    TestOnly,
    BuildOnly,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Row {
    pub symbol: String,
    pub outcome: Outcome,
    pub phase: Option<Phase>,
    pub detail: Option<String>,
}

pub fn classify_kani<R: CommandRunner>(runner: &mut R, command: &CommandSpec, harness: &str) -> Row {
    let _ = (runner, command);
    Row { symbol: harness.into(), outcome: Outcome::Pass, phase: None, detail: None }
}

pub fn run_feature_row<R: CommandRunner>(runner: &mut R, name: &str, build: Option<&CommandSpec>, test: &CommandSpec) -> Row {
    let _ = (runner, build, test);
    Row { symbol: name.into(), outcome: Outcome::Pass, phase: Some(Phase::Test), detail: None }
}

pub fn build_errors(stdout: &str) -> Vec<String> {
    let _ = stdout;
    Vec::new()
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
        CommandSpec { program: "cargo".into(), args: vec![], cwd: ".".into(), env: vec![] }
    }
    fn output(status: i32, stdout: &str) -> io::Result<CommandOutput> {
        Ok(CommandOutput { status, stdout: stdout.into(), stderr: String::new() })
    }

    #[test]
    #[trace("TC-002", "TC-003")]
    fn kani_requires_owning_harness_and_verification_failure() {
        let mut runner = FakeRunner(VecDeque::from([output(1, "Checking harness kani_proofs::owner...\nVERIFICATION:- FAILED")]));
        assert_eq!(classify_kani(&mut runner, &command(), "owner").outcome, Outcome::Pass);
        let mut runner = FakeRunner(VecDeque::from([output(0, "Checking harness kani_proofs::owner...\nVERIFICATION:- SUCCESSFUL")]));
        assert_eq!(classify_kani(&mut runner, &command(), "owner").outcome, Outcome::Fail);
        let mut runner = FakeRunner(VecDeque::from([output(1, "compiler error")]));
        assert_eq!(classify_kani(&mut runner, &command(), "owner").outcome, Outcome::Inconclusive);
        let mut runner = FakeRunner(VecDeque::from([output(1, "Checking harness kani_proofs::other...\nVERIFICATION:- FAILED")]));
        assert_eq!(classify_kani(&mut runner, &command(), "owner").outcome, Outcome::Inconclusive);
    }

    #[test]
    #[trace("TC-199", "FR-006-AC-8")]
    fn feature_build_error_skips_test_phase() {
        let mut runner = FakeRunner(VecDeque::from([output(0, r#"{"reason":"compiler-message","message":{"level":"error","rendered":"error: failed\n"}}"#)]));
        let row = run_feature_row(&mut runner, "test-exact", Some(&command()), &command());
        assert_eq!(row.outcome, Outcome::Fail);
        assert_eq!(row.phase, Some(Phase::Build));
        assert!(runner.0.is_empty());
    }

    #[test]
    #[trace("TC-199", "FR-006-AC-8")]
    fn feature_test_exit_is_distinct_from_build_failure_and_unavailable() {
        let mut runner = FakeRunner(VecDeque::from([output(0, ""), output(7, "test failed")]));
        let row = run_feature_row(&mut runner, "test-exact", Some(&command()), &command());
        assert_eq!(row.outcome, Outcome::Fail);
        assert_eq!(row.phase, Some(Phase::Test));
        let mut runner = FakeRunner(VecDeque::from([Err(io::Error::new(io::ErrorKind::NotFound, "cargo absent"))]));
        assert_eq!(run_feature_row(&mut runner, "test-exact", Some(&command()), &command()).outcome, Outcome::Unavailable);
    }
}
