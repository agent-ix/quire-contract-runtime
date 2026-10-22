// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

//! Verify that cargo-deny refuses injected QSL Git source dependencies.

use std::{
    env, fmt, fs,
    fs::{DirBuilder, OpenOptions},
    io::{self, ErrorKind, Write},
    os::unix::{ffi::OsStringExt, fs::symlink, fs::DirBuilderExt},
    path::{Component, Path, PathBuf},
    process::{Command, ExitCode},
};

const QSL_GIT: &str = "https://github.com/agent-ix/quire-spec-language";
const QSL_SOURCE: &str = "git+https://github.com/agent-ix/quire-spec-language?branch=main";
const DEFAULT_CRATES: [&str; 4] = [
    "qsl-eval",
    "qsl-replay",
    "qsl-semantics",
    "quire-spec-language",
];

#[derive(Debug)]
enum GateError {
    Io(io::Error),
    InvalidCrateName(String),
    TrackedListFailed,
    BaselineDenied,
    MutationsFailed(usize),
}

impl From<io::Error> for GateError {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}

impl fmt::Display for GateError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(error) => write!(formatter, "{error}"),
            Self::InvalidCrateName(name) => write!(formatter, "invalid crate name: {name}"),
            Self::TrackedListFailed => formatter.write_str("git ls-files failed"),
            Self::BaselineDenied => formatter.write_str("unmodified tree: cargo deny failed"),
            Self::MutationsFailed(count) => {
                write!(formatter, "{count} QSL Git source mutation controls failed")
            }
        }
    }
}

struct Scratch(PathBuf);

impl Scratch {
    fn new() -> io::Result<Self> {
        for attempt in 0..128 {
            let path = env::temp_dir().join(format!(
                "quire-contract-runtime-deny-mutations-{}-{attempt}",
                std::process::id()
            ));
            match DirBuilder::new().mode(0o700).create(&path) {
                Ok(()) => return Ok(Self(path)),
                Err(error) if error.kind() == ErrorKind::AlreadyExists => {}
                Err(error) => return Err(error),
            }
        }
        Err(io::Error::new(
            ErrorKind::AlreadyExists,
            "no free deny-mutation scratch directory",
        ))
    }
}

impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn tracked_paths(root: &Path) -> Result<Vec<PathBuf>, GateError> {
    let output = Command::new("git")
        .args(["ls-files", "-z", "--cached"])
        .current_dir(root)
        .output()?;
    if !output.status.success() {
        return Err(GateError::TrackedListFailed);
    }
    output
        .stdout
        .split(|byte| *byte == 0)
        .filter(|bytes| !bytes.is_empty())
        .map(|bytes| {
            let path = PathBuf::from(std::ffi::OsString::from_vec(bytes.to_vec()));
            if path
                .components()
                .all(|component| matches!(component, Component::Normal(_)))
            {
                Ok(path)
            } else {
                Err(GateError::Io(io::Error::new(
                    ErrorKind::InvalidData,
                    "tracked path escapes repository",
                )))
            }
        })
        .collect()
}

fn copy_workspace(root: &Path, destination: &Path, paths: &[PathBuf]) -> io::Result<()> {
    fs::create_dir(destination)?;
    for relative in paths {
        let source = root.join(relative);
        let target = destination.join(relative);
        if let Some(parent) = target.parent() {
            fs::create_dir_all(parent)?;
        }
        let kind = fs::symlink_metadata(&source)?.file_type();
        if kind.is_symlink() {
            symlink(fs::read_link(source)?, target)?;
        } else if kind.is_file() {
            fs::copy(source, target)?;
        } else {
            return Err(io::Error::new(
                ErrorKind::InvalidData,
                "tracked entry is neither a file nor a symlink",
            ));
        }
    }
    Ok(())
}

fn cargo_deny(directory: &Path) -> io::Result<std::process::Output> {
    Command::new("cargo")
        .args([
            "deny",
            "--workspace",
            "check",
            "licenses",
            "bans",
            "sources",
        ])
        .current_dir(directory)
        .output()
}

fn rejects_qsl_source(exit_success: bool, output: &str) -> bool {
    if exit_success {
        return false;
    }
    let lines = output.lines().collect::<Vec<_>>();
    lines.iter().enumerate().any(|(index, line)| {
        line.contains("error[source-not-allowed]")
            && lines[index..]
                .iter()
                .take(7)
                .any(|next| next.contains(QSL_SOURCE))
    })
}

fn crate_names() -> Result<Vec<String>, GateError> {
    let supplied = env::args_os()
        .skip(1)
        .map(|name| {
            name.into_string()
                .map_err(|_| GateError::InvalidCrateName("non-UTF-8 argument".to_owned()))
        })
        .collect::<Result<Vec<_>, _>>()?;
    let crates = if supplied.is_empty() {
        DEFAULT_CRATES
            .iter()
            .map(|name| (*name).to_owned())
            .collect()
    } else {
        supplied
    };
    for name in &crates {
        if name.is_empty()
            || !name
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_'))
        {
            return Err(GateError::InvalidCrateName(name.clone()));
        }
    }
    Ok(crates)
}

fn run() -> Result<(), GateError> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let paths = tracked_paths(root)?;
    let crates = crate_names()?;
    let scratch = Scratch::new()?;
    let baseline = scratch.0.join("base");
    copy_workspace(root, &baseline, &paths)?;
    if !cargo_deny(&baseline)?.status.success() {
        return Err(GateError::BaselineDenied);
    }
    println!("unmodified tree: cargo deny passes");

    let mut failures = 0;
    for crate_name in crates {
        for table in ["dependencies", "dev-dependencies", "build-dependencies"] {
            let candidate = scratch.0.join(format!("{crate_name}-{table}"));
            copy_workspace(root, &candidate, &paths)?;
            writeln!(
                OpenOptions::new()
                    .append(true)
                    .open(candidate.join("Cargo.toml"))?,
                "\n[{table}.{crate_name}]\ngit = \"{QSL_GIT}\"\nbranch = \"main\""
            )?;
            let outcome = cargo_deny(&candidate)?;
            let combined = format!(
                "{}\n{}",
                String::from_utf8_lossy(&outcome.stdout),
                String::from_utf8_lossy(&outcome.stderr)
            );
            if rejects_qsl_source(outcome.status.success(), &combined) {
                println!("ok: {crate_name} as {table} rejects the QSL Git source");
            } else {
                eprintln!(
                    "FAIL: {crate_name} as {table} (exit {}) did not reject the QSL Git source",
                    outcome.status
                );
                failures += 1;
            }
        }
    }
    if failures == 0 {
        Ok(())
    } else {
        Err(GateError::MutationsFailed(failures))
    }
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error}");
            ExitCode::FAILURE
        }
    }
}

#[cfg(test)]
mod tests {
    use ix_trace_rs::trace;

    use super::rejects_qsl_source;

    /// A policy check passes only for cargo-deny's refusal of the QSL Git source.
    #[trace("TC-198", "FR-275-AC-8")]
    #[test]
    fn tc_198_source_mutation_rejects_other_failures_and_accepts_only_qsl_refusal() {
        let qsl = "error[source-not-allowed]: Git source not allowed\n  source: git+https://github.com/agent-ix/quire-spec-language?branch=main";
        let other = "error[source-not-allowed]: Git source not allowed\n  source: git+https://github.com/agent-ix/other?branch=main";
        let licence = "error[unlicensed]: this crate is not allowed";
        assert!(rejects_qsl_source(false, qsl));
        assert!(!rejects_qsl_source(true, qsl));
        assert!(!rejects_qsl_source(false, other));
        assert!(!rejects_qsl_source(false, licence));
    }
}
