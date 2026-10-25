// SPDX-License-Identifier: AGPL-3.0-or-later
//! Inspect the retained exact-profile archive and exercise its negative controls.

use std::collections::BTreeSet;
use std::fs;
use std::path::Path;
use std::process::{Command, Output};

const EXPECTED_CALLS: [&str; 5] = [
    "negotiate_integer_division",
    "negotiate_ieee",
    "PackageDeclarations>::check",
    "CheckedPackage>::check_expression",
    "CheckedPackage>::evaluate",
];
const GENERIC_CALL: &str =
    "evaluate_boolean_short_circuit::<quire_contract_runtime_exact_panic::quire_runtime_exact_panic";

#[derive(Debug)]
struct Scan {
    runtime: BTreeSet<String>,
    probe: BTreeSet<String>,
    expected: BTreeSet<&'static str>,
    panic: Vec<String>,
    relocations: usize,
}

fn output(command: &mut Command, label: &str) -> Result<Output, String> {
    let result = command
        .output()
        .map_err(|error| format!("could not run {label}: {error}"))?;
    if !result.status.success() {
        return Err(format!(
            "{label} failed: {}",
            String::from_utf8_lossy(&result.stderr)
        ));
    }
    Ok(result)
}

fn text(command: &mut Command, label: &str) -> Result<String, String> {
    let result = output(command, label)?;
    let value = String::from_utf8(result.stdout).map_err(|error| format!("{label}: {error}"))?;
    if value.trim().is_empty() {
        return Err(format!("empty output from {label}"));
    }
    Ok(value)
}

fn members(archive: &Path) -> Result<(BTreeSet<String>, BTreeSet<String>), String> {
    let listing = text(Command::new("ar").arg("t").arg(archive), "ar t")?;
    let runtime: BTreeSet<String> = listing
        .lines()
        .filter(|member| member.starts_with("quire_contract_runtime-"))
        .map(str::to_owned)
        .collect();
    let probe: BTreeSet<String> = listing
        .lines()
        .filter(|member| member.starts_with("quire_contract_runtime_exact_panic-"))
        .map(str::to_owned)
        .collect();
    if runtime.is_empty() || probe.is_empty() {
        return Err("empty RT-owned or probe object population".into());
    }
    Ok((runtime, probe))
}

fn panic_target(target: &str) -> bool {
    target.contains("rust_begin_unwind")
        || target.contains("panic_bounds_check")
        || (target.contains("core") && target.contains("panicking"))
        || (target.contains("slice") && target.contains("index") && target.contains("fail"))
}

fn scan_report(
    report: &str,
    runtime: &BTreeSet<String>,
    probe: &BTreeSet<String>,
) -> Result<Scan, String> {
    let mut scan = Scan {
        runtime: BTreeSet::new(),
        probe: BTreeSet::new(),
        expected: BTreeSet::new(),
        panic: Vec::new(),
        relocations: 0,
    };
    let mut current = "";
    let mut section = "";
    let mut awaiting_columns = false;
    for line in report.lines() {
        if let Some((member, format)) = line.split_once(":") {
            if format.trim_start().starts_with("file format ") {
                if awaiting_columns && runtime.contains(current) {
                    return Err(format!(
                        "missing relocation columns in RT-owned object {current}"
                    ));
                }
                current = member;
                section = "";
                awaiting_columns = false;
                if runtime.contains(current) {
                    scan.runtime.insert(current.to_owned());
                }
                if probe.contains(current) {
                    scan.probe.insert(current.to_owned());
                }
                continue;
            }
        }
        if let Some(name) = line
            .strip_prefix("RELOCATION RECORDS FOR [")
            .and_then(|line| line.strip_suffix("]:"))
        {
            if awaiting_columns && runtime.contains(current) {
                return Err(format!(
                    "missing relocation columns in RT-owned object {current}"
                ));
            }
            section = name;
            awaiting_columns = true;
            continue;
        }
        if current.is_empty() || section.is_empty() || line.trim().is_empty() {
            continue;
        }
        if awaiting_columns {
            if runtime.contains(current)
                && line.split_whitespace().collect::<Vec<_>>() != ["OFFSET", "TYPE", "VALUE"]
            {
                return Err(format!(
                    "malformed RT-owned relocation columns in {current}: {line}"
                ));
            }
            awaiting_columns = false;
            continue;
        }
        let fields: Vec<_> = line.split_whitespace().collect();
        if fields.len() < 3 || !fields[0].bytes().all(|byte| byte.is_ascii_hexdigit()) {
            if runtime.contains(current) {
                return Err(format!(
                    "malformed RT-owned relocation in {current}: {line}"
                ));
            }
            continue;
        }
        if runtime.contains(current) || probe.contains(current) {
            scan.relocations += 1;
        }
        let target = fields[2..].join(" ");
        if runtime.contains(current) && panic_target(&target) {
            scan.panic.push(format!("{current}: {}", line.trim()));
        }
        if probe.contains(current) && section.starts_with(".text") {
            scan.expected.extend(
                EXPECTED_CALLS
                    .into_iter()
                    .filter(|call| target.contains(call)),
            );
        }
    }
    if awaiting_columns && runtime.contains(current) {
        return Err(format!(
            "missing relocation columns in RT-owned object {current}"
        ));
    }
    if scan.runtime != *runtime || scan.probe != *probe || scan.relocations == 0 {
        return Err("unparseable or incomplete object relocation report".into());
    }
    Ok(scan)
}

fn inspect(archive: &Path) -> Result<(String, String, BTreeSet<String>, BTreeSet<String>), String> {
    if !archive.is_file() {
        return Err(format!("missing archive {}", archive.display()));
    }
    let (runtime, probe) = members(archive)?;
    let report = text(
        Command::new("objdump").arg("-rC").arg(archive),
        "objdump -rC",
    )?;
    let scan = scan_report(&report, &runtime, &probe)?;
    let symbols = text(
        Command::new("nm")
            .args(["-A", "-C", "--defined-only"])
            .arg(archive),
        "nm -A -C --defined-only",
    )?;
    let mut missing: BTreeSet<_> = EXPECTED_CALLS
        .into_iter()
        .filter(|call| !scan.expected.contains(call))
        .collect();
    if !symbols.lines().any(|line| {
        probe
            .iter()
            .any(|member| line.contains(&format!(":{member}:")) && line.contains(GENERIC_CALL))
    }) {
        missing.insert("evaluate_boolean_short_circuit");
    }
    if !missing.is_empty() {
        return Err(format!(
            "probe omits required RT calls: {}",
            missing.into_iter().collect::<Vec<_>>().join(", ")
        ));
    }
    if !scan.panic.is_empty() {
        return Err(format!(
            "RT-owned panic relocations:\n{}",
            scan.panic.join("\n")
        ));
    }
    let required = EXPECTED_CALLS
        .into_iter()
        .chain(["evaluate_boolean_short_circuit"])
        .collect::<Vec<_>>()
        .join(",");
    let summary = format!(
        "artifact={} command=objdump -rC runtime_objects={} probe_objects={} required_calls={} panic_references=0",
        archive.display(),
        runtime.iter().cloned().collect::<Vec<_>>().join(","),
        probe.iter().cloned().collect::<Vec<_>>().join(","),
        required,
    );
    Ok((summary, report, runtime, probe))
}

fn malformed_control(
    report: &str,
    runtime: &BTreeSet<String>,
    probe: &BTreeSet<String>,
) -> Result<(), String> {
    let mut lines = Vec::new();
    let mut in_runtime = false;
    let mut inserted = false;
    for line in report.lines() {
        if let Some((member, format)) = line.split_once(':') {
            if format.trim_start().starts_with("file format ") {
                in_runtime = runtime.contains(member);
            }
        }
        lines.push(line);
        if in_runtime && line.starts_with("OFFSET") && !inserted {
            lines.push("00000000 core::panicking::panic");
            inserted = true;
        }
    }
    if !inserted {
        return Err("no RT-owned relocation table to mutate".into());
    }
    let mutated = lines.join("\n");
    match scan_report(&mutated, runtime, probe) {
        Err(error) if error.contains("malformed RT-owned relocation") => Ok(()),
        other => Err(format!(
            "malformed RT relocation report was not rejected: {other:?}"
        )),
    }
}

fn panic_control(
    root: &Path,
    target: &Path,
    msrv: &str,
    triple: &str,
    cargo: &str,
) -> Result<(), String> {
    let scratch = tempfile::tempdir_in(target).map_err(|error| error.to_string())?;
    let archive = scratch.path().join("candidate.tar");
    output(
        Command::new("git")
            .current_dir(root)
            .args(["archive", "HEAD", "-o"])
            .arg(&archive),
        "git archive HEAD",
    )?;
    let candidate = scratch.path().join("candidate");
    fs::create_dir(&candidate).map_err(|error| error.to_string())?;
    output(
        Command::new("tar")
            .arg("-xf")
            .arg(&archive)
            .arg("-C")
            .arg(&candidate),
        "tar extract candidate",
    )?;
    let scalar = candidate.join("src/scalar/division.rs");
    let source = fs::read_to_string(&scalar).map_err(|error| error.to_string())?;
    let anchor = "    items\n        .iter()";
    if source.matches(anchor).count() != 1 {
        return Err("scalar insertion point absent or ambiguous".into());
    }
    fs::write(
        &scalar,
        source.replacen(
            anchor,
            "    if core::hint::black_box(!items.is_empty()) { panic!(\"TC-206 injected panic\"); }\n    items\n        .iter()",
            1,
        ),
    )
    .map_err(|error| error.to_string())?;
    output(
        Command::new(cargo)
            .current_dir(&candidate)
            .arg(format!("+{msrv}"))
            .args([
                "build",
                "--locked",
                "--profile",
                "exact-panic",
                "--manifest-path",
            ])
            .arg("measurement/exact-panic/Cargo.toml")
            .args(["--target", triple, "--target-dir"])
            .arg(target),
        "scratch exact-panic build",
    )?;
    let mutant = target
        .join(triple)
        .join("exact-panic/libquire_contract_runtime_exact_panic.a");
    match inspect(&mutant) {
        Err(error) if error.contains("RT-owned panic relocations:") => Ok(()),
        other => Err(format!(
            "injected panic did not trigger RT relocation verdict: {other:?}"
        )),
    }
}

pub fn run(root: &Path, args: &[String]) -> Result<String, String> {
    let [archive, target, msrv, triple, cargo] = args else {
        return Err(
            "usage: runtime-gates exact-panic-probe ARCHIVE TARGET_DIR MSRV TARGET CARGO".into(),
        );
    };
    let archive = root.join(archive);
    let target = root.join(target);
    fs::create_dir_all(&target).map_err(|error| error.to_string())?;
    let (summary, report, runtime, probe) = inspect(&archive)?;
    malformed_control(&report, &runtime, &probe)?;
    panic_control(root, &target, msrv, triple, cargo)?;
    Ok(format!(
        "{summary}\nexact panic control: malformed RT relocation report rejected\nexact panic control: reachable RT-owned panic relocation detected"
    ))
}
