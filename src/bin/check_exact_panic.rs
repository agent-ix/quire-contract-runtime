// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

//! Inspect the retained runtime objects in the no-std exact probe archive.

use std::{
    collections::BTreeSet,
    env,
    path::Path,
    process::{Command, ExitCode},
};

const CALLS: [&str; 6] = [
    "negotiate_integer_division",
    "negotiate_ieee",
    "evaluate_boolean_short_circuit",
    "PackageDeclarations5check",
    "CheckedPackage16check_expression",
    "CheckedPackage8evaluate",
];

fn in_order(mut text: &str, parts: &[&str]) -> bool {
    for part in parts {
        let Some(index) = text.find(part) else {
            return false;
        };
        text = &text[index + part.len()..];
    }
    true
}

fn panic_symbol(symbol: &str) -> bool {
    symbol.contains("rust_begin_unwind")
        || symbol.contains("panic_bounds_check")
        || in_order(symbol, &["core", "panicking"])
        || in_order(symbol, &["slice", "index", "fail"])
}

fn inspect(report: &str) -> Result<(BTreeSet<String>, usize), String> {
    let mut object = String::new();
    let mut section = String::new();
    let mut runtime_objects = BTreeSet::new();
    let mut seen_calls = BTreeSet::new();
    let mut relocations = 0;
    let mut headers = 0;
    for line in report.lines() {
        if let Some(name) = line.strip_suffix(":     file format elf32-little") {
            object = name.to_owned();
            section.clear();
            headers += 1;
            if object.contains(".quire_contract_runtime-")
                && object.contains(".quire_contract_runtime.")
            {
                runtime_objects.insert(object.clone());
            }
            continue;
        }
        if line.starts_with("RELOCATION RECORDS FOR [") {
            section = line.to_owned();
            continue;
        }
        if section.is_empty() || !section.starts_with("RELOCATION RECORDS FOR [.text") {
            continue;
        }
        let mut fields = line.split_whitespace();
        let (Some(offset), Some(kind), Some(symbol)) =
            (fields.next(), fields.next(), fields.next())
        else {
            continue;
        };
        if !offset.bytes().all(|byte| byte.is_ascii_hexdigit()) || kind == "TYPE" {
            continue;
        }
        if fields.next().is_some() {
            return Err(format!("unparseable relocation: {line}"));
        }
        relocations += 1;
        if object.contains(".quire_contract_runtime_exact_panic.") {
            for call in CALLS {
                if symbol.contains(call) {
                    seen_calls.insert(call);
                }
            }
        }
        if runtime_objects.contains(&object) && panic_symbol(symbol) {
            return Err(format!("runtime panic relocation in {object}: {line}"));
        }
    }
    if headers == 0 || relocations == 0 || runtime_objects.is_empty() {
        return Err("missing or unparseable runtime object relocation population".into());
    }
    for call in CALLS {
        if !seen_calls.contains(call) {
            return Err(format!(
                "probe does not retain required runtime call: {call}"
            ));
        }
    }
    Ok((runtime_objects, relocations))
}

fn main() -> ExitCode {
    let Some(artifact) = env::args_os().nth(1) else {
        eprintln!("expected exact probe archive path");
        return ExitCode::FAILURE;
    };
    let artifact = Path::new(&artifact);
    if !artifact.is_file() {
        eprintln!("missing exact probe archive: {}", artifact.display());
        return ExitCode::FAILURE;
    }
    let output = match Command::new("objdump").arg("-r").arg(artifact).output() {
        Ok(output) if output.status.success() => output,
        Ok(output) => {
            eprintln!("objdump -r failed: {}", output.status);
            return ExitCode::FAILURE;
        }
        Err(error) => {
            eprintln!("objdump -r unavailable: {error}");
            return ExitCode::FAILURE;
        }
    };
    let report = match std::str::from_utf8(&output.stdout) {
        Ok(report) => report,
        Err(error) => {
            eprintln!("unparseable objdump report: {error}");
            return ExitCode::FAILURE;
        }
    };
    match inspect(report) {
        Ok((objects, relocations)) => {
            println!("artifact={} command=objdump -r {} runtime_objects={} text_relocations={} panic_relocations=0", artifact.display(), artifact.display(), objects.len(), relocations);
            for object in objects {
                println!("runtime_object={object}");
            }
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!(
                "artifact={} command=objdump -r {}: {error}",
                artifact.display(),
                artifact.display()
            );
            ExitCode::FAILURE
        }
    }
}

#[cfg(test)]
mod tests {
    use super::inspect;

    const REPORT: &str = "probe.quire_contract_runtime-1.quire_contract_runtime.1.o:     file format elf32-little\nRELOCATION RECORDS FOR [.text.foo]:\nOFFSET TYPE VALUE\n0000 UNKNOWN malloc\nprobe.quire_contract_runtime_exact_panic.1.o:     file format elf32-little\nRELOCATION RECORDS FOR [.text.entry]:\nOFFSET TYPE VALUE\n0000 UNKNOWN negotiate_integer_division\n0001 UNKNOWN negotiate_ieee\n0002 UNKNOWN evaluate_boolean_short_circuit\n0003 UNKNOWN PackageDeclarations5check\n0004 UNKNOWN CheckedPackage16check_expression\n0005 UNKNOWN CheckedPackage8evaluate\n";

    /// Trace: TC-206, NFR-002-AC-8, NFR-002-AC-9
    #[test]
    fn exact_probe_requires_calls_and_rejects_runtime_panic() {
        assert!(inspect(REPORT).is_ok());
        assert!(inspect(&REPORT.replace("negotiate_ieee", "other")).is_err());
        assert!(inspect(&REPORT.replace("malloc", "_RNvNtC_4core9panicking9panic_fmt")).is_err());
        assert!(inspect(&REPORT.replace(
            "probe.quire_contract_runtime-1.quire_contract_runtime.1.o",
            "other.o"
        ))
        .is_err());
        assert!(inspect("garbage").is_err());
    }
}
