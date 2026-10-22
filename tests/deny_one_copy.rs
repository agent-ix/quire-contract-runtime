// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Peter Krenesky

use std::collections::BTreeMap;
use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use ix_trace_rs::trace;

/// A disposable consumer tree running the normal recipe with the real checker.
struct ScratchRepo {
    root: PathBuf,
}

impl ScratchRepo {
    fn new() -> Self {
        let root = (0..100)
            .map(|nonce| {
                std::env::temp_dir()
                    .join(format!("quire-rt-lock-gate-{}-{nonce}", std::process::id()))
            })
            .find(|path| fs::create_dir(path).is_ok())
            .expect("a unique temporary directory is available");
        let scratch = Self { root };
        for file in ["Makefile", "Cargo.lock"] {
            fs::copy(
                Path::new(env!("CARGO_MANIFEST_DIR")).join(file),
                scratch.root.join(file),
            )
            .unwrap();
        }
        scratch.git(&["init", "--quiet"]);
        scratch.git(&["add", "Cargo.lock"]);
        scratch
    }

    fn git(&self, args: &[&str]) {
        let output = Command::new("git")
            .current_dir(&self.root)
            .args(args)
            .output()
            .expect("git can prepare the scratch repository");
        assert!(output.status.success(), "git setup failed: {output:?}");
    }

    fn write_lock(&self, lock: &str) {
        fs::write(self.root.join("Cargo.lock"), lock).unwrap();
    }

    fn make_deny(&self) -> Output {
        let checker = std::env::var_os("QUIRE").unwrap_or_else(|| OsString::from("quire"));
        self.make_deny_with(checker)
    }

    fn make_deny_with(&self, checker: OsString) -> Output {
        self.make_deny_with_path(checker, std::env::var_os("PATH").unwrap())
    }

    fn make_deny_with_path(&self, checker: OsString, path: OsString) -> Output {
        // Isolate cargo-deny's unrelated policy; the checker itself is never replaced.
        let mut assignment = OsString::from("QUIRE=");
        assignment.push(checker);
        Command::new("make")
            .current_dir(&self.root)
            .args(["deny", "CARGO=true"])
            .arg(assignment)
            .env("PATH", path)
            .output()
            .expect("make deny can run in the scratch repository")
    }
}

impl Drop for ScratchRepo {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

fn package(name: &str, revision: char) -> String {
    format!(
        "\n[[package]]\nname = \"{name}\"\nversion = \"0.0.0\"\n\
         source = \"git+https://github.com/agent-ix/lock-gate-control?branch=main#{}\"\n",
        revision.to_string().repeat(40)
    )
}

fn require_policy_refusal(output: &Output, names: &[&str]) {
    assert!(!output.status.success(), "incoherent lock was accepted");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("agent-ix Git coherence refused"),
        "wrong refusal: {output:?}"
    );
    for name in names {
        assert!(
            stderr.contains(name),
            "missing affected package {name}: {output:?}"
        );
    }
}

#[trace("FR-275-AC-5", "FR-275-AC-24", "FR-275-AC-28")]
#[test]
fn deny_checks_the_unchanged_committed_lock_with_the_owned_command() {
    let scratch = ScratchRepo::new();
    let before = fs::read(scratch.root.join("Cargo.lock")).unwrap();
    let output = scratch.make_deny();
    assert!(output.status.success(), "committed lock failed: {output:?}");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("check-lock "),
        "missing invocation: {output:?}"
    );
    assert!(
        stdout.contains("Git lock coherence checked: Cargo.lock"),
        "missing result: {output:?}"
    );
    assert!(
        output.stderr.is_empty(),
        "unexpected diagnostics: {output:?}"
    );
    assert_eq!(fs::read(scratch.root.join("Cargo.lock")).unwrap(), before);
}

#[trace("TC-202", "FR-275-AC-6", "FR-275-AC-25")]
#[test]
fn deny_rejects_duplicate_first_party_package_names_with_valid_sources() {
    let scratch = ScratchRepo::new();
    let original = fs::read_to_string(scratch.root.join("Cargo.lock")).unwrap();
    let first_party: BTreeMap<&str, &str> = original
        .split("[[package]]")
        .filter_map(|record| {
            let name = record
                .lines()
                .find_map(|line| line.strip_prefix("name = \"")?.strip_suffix('"'))?;
            let source = record
                .lines()
                .find_map(|line| line.strip_prefix("source = \"")?.strip_suffix('"'))?;
            source
                .starts_with("git+https://github.com/agent-ix/")
                .then_some((name, source))
        })
        .collect();
    assert!(!first_party.is_empty(), "no first-party packages found");
    for (name, source) in first_party {
        scratch.write_lock(&format!("{original}\n[[package]]\nname = \"{name}\"\nversion = \"0.0.0\"\nsource = \"{source}\"\n"));
        require_policy_refusal(&scratch.make_deny(), &[name]);
    }
    scratch.write_lock(
        &(package("synthetic-first-party", 'a') + &package("synthetic-first-party", 'a')),
    );
    require_policy_refusal(&scratch.make_deny(), &["synthetic-first-party"]);
}

#[trace("FR-275-AC-25", "FR-275-AC-26")]
#[test]
fn deny_accepts_aligned_crates_and_refuses_distinct_name_repository_splits() {
    let scratch = ScratchRepo::new();
    scratch.write_lock(&(package("control-first", 'a') + &package("control-second", 'a')));
    let aligned = scratch.make_deny();
    assert!(
        aligned.status.success(),
        "aligned multi-crate lock failed: {aligned:?}"
    );
    scratch.write_lock(&(package("control-first", 'a') + &package("control-second", 'b')));
    let split = scratch.make_deny();
    require_policy_refusal(&split, &["control-first", "control-second"]);
    let stderr = String::from_utf8_lossy(&split.stderr);
    assert!(stderr.contains("github.com/agent-ix/lock-gate-control"));
    assert!(stderr.contains(&"a".repeat(40)) && stderr.contains(&"b".repeat(40)));
}

#[trace("FR-275-AC-24", "FR-275-AC-25", "FR-275-AC-26")]
#[test]
fn deny_checks_every_tracked_lock_in_one_owned_invocation() {
    let scratch = ScratchRepo::new();
    fs::create_dir(scratch.root.join("nested")).unwrap();
    fs::write(
        scratch.root.join("nested/Cargo.lock"),
        package("nested-first", 'a') + &package("nested-second", 'b'),
    )
    .unwrap();
    scratch.git(&["add", "nested/Cargo.lock"]);
    let output = scratch.make_deny();
    require_policy_refusal(&output, &["nested-first", "nested-second"]);
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(
        stdout.matches("check-lock ").count(),
        1,
        "wrong invocation: {output:?}"
    );
    assert!(String::from_utf8_lossy(&output.stderr).contains("nested/Cargo.lock"));
}

#[trace("FR-275-AC-26")]
#[test]
fn deny_propagates_malformed_input_and_missing_checker_failures() {
    let scratch = ScratchRepo::new();
    scratch.write_lock("[[package]");
    let malformed = scratch.make_deny();
    assert!(!malformed.status.success(), "malformed input was accepted");
    let stderr = String::from_utf8_lossy(&malformed.stderr);
    assert!(
        stderr.contains("Cargo.lock") && !stderr.contains("agent-ix Git coherence refused"),
        "wrong input refusal: {malformed:?}"
    );
    let missing = scratch.make_deny_with(scratch.root.join("absent-quire").into_os_string());
    assert!(!missing.status.success(), "missing checker was accepted");
    assert!(!String::from_utf8_lossy(&missing.stdout).contains("Git lock coherence checked"));
}

#[trace("FR-275-AC-27")]
#[test]
fn owned_gate_has_no_local_checker_copy_or_fallback() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    assert!(!root.join("scripts/check_one_copy.awk").exists());
    let makefile = include_str!("../Makefile");
    assert!(!makefile.contains("check_one_copy.awk"));
    assert!(makefile.contains("$(QUIRE) check-lock"));
}

#[trace("FR-275-AC-24", "FR-275-AC-25", "FR-275-AC-26")]
#[test]
fn deny_preserves_tracked_filename_boundaries_for_the_real_checker() {
    let scratch = ScratchRepo::new();
    let original = fs::read_to_string(scratch.root.join("Cargo.lock")).unwrap();
    let directories = ["space dir", "café", "line\nbreak", "literal[ab]*?"];
    for directory in directories {
        let path = scratch.root.join(directory).join("Cargo.lock");
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, &original).unwrap();
        scratch.git(&["add", &format!("{directory}/Cargo.lock")]);
    }
    let aligned = scratch.make_deny();
    assert!(
        aligned.status.success(),
        "filename boundaries corrupted: {aligned:?}"
    );
    assert!(
        aligned.stderr.is_empty(),
        "aligned filename diagnostic: {aligned:?}"
    );
    let stdout = String::from_utf8_lossy(&aligned.stdout);
    assert_eq!(stdout.matches("check-lock ").count(), 1);
    for directory in directories {
        assert!(
            stdout.contains(&format!("{directory}/Cargo.lock")),
            "missing path: {aligned:?}"
        );
    }
    for directory in directories {
        let path = scratch.root.join(directory).join("Cargo.lock");
        fs::write(
            &path,
            package("boundary-first", 'a') + &package("boundary-second", 'b'),
        )
        .unwrap();
        let split = scratch.make_deny();
        require_policy_refusal(&split, &["boundary-first", "boundary-second"]);
        let stderr = String::from_utf8_lossy(&split.stderr);
        assert!(
            stderr.contains(&format!("{directory}/Cargo.lock")),
            "wrong path: {split:?}"
        );
        assert!(stderr.contains(&"a".repeat(40)) && stderr.contains(&"b".repeat(40)));
        fs::write(path, &original).unwrap();
    }
}

#[trace("TC-202", "FR-275-AC-24", "FR-275-AC-26")]
#[test]
fn deny_rejects_a_git_producer_failure_after_a_nonempty_prefix() {
    use std::os::unix::fs::PermissionsExt;

    let scratch = ScratchRepo::new();
    fs::create_dir(scratch.root.join("nested")).unwrap();
    fs::write(
        scratch.root.join("nested/Cargo.lock"),
        package("omitted-first", 'a') + &package("omitted-second", 'b'),
    )
    .unwrap();
    scratch.git(&["add", "nested/Cargo.lock"]);
    require_policy_refusal(&scratch.make_deny(), &["omitted-first", "omitted-second"]);

    // Fault only the producer: emit a valid prefix, then fail before the second lock.
    // The consumer remains the genuine owner CLI used by the other controls.
    let bin = scratch.root.join("bin");
    fs::create_dir(&bin).unwrap();
    let git = bin.join("git");
    fs::write(
        &git,
        "#!/bin/sh\nprintf 'Cargo.lock\\0'\nprintf 'lock enumeration failed after prefix\\n' >&2\nexit 42\n",
    )
    .unwrap();
    fs::set_permissions(&git, fs::Permissions::from_mode(0o755)).unwrap();
    let path = std::env::join_paths(
        std::iter::once(bin).chain(std::env::split_paths(&std::env::var_os("PATH").unwrap())),
    )
    .unwrap();
    let checker = std::env::var_os("QUIRE").unwrap_or_else(|| OsString::from("quire"));
    let output = scratch.make_deny_with_path(checker, path);
    assert!(
        !output.status.success(),
        "a failed producer's incomplete lock prefix was accepted: {output:?}"
    );
    assert!(
        String::from_utf8_lossy(&output.stderr).contains("lock enumeration failed after prefix")
    );
    assert!(
        !String::from_utf8_lossy(&output.stdout).contains("Git lock coherence checked"),
        "the checker ran before enumeration succeeded: {output:?}"
    );
}
