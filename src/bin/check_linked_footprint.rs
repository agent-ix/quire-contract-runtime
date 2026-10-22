// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

//! Check linked runtime footprint sections and panic relocations.

use std::{
    env, fmt,
    path::{Path, PathBuf},
    process::{Command, ExitCode},
};

const TARGET: &str = "thumbv7em-none-eabi";
const MINIMUM_BYTES: u64 = 500;
const LIMIT_BYTES: u64 = 4096;

#[derive(Debug, PartialEq, Eq)]
enum GateError {
    Unavailable(&'static str),
    PanicPath(Vec<String>),
    BelowFloor { bytes: u64 },
    AboveCeiling { bytes: u64 },
}

impl GateError {
    fn exit_code(&self) -> u8 {
        match self {
            Self::Unavailable(_) => 2,
            Self::PanicPath(_) | Self::BelowFloor { .. } | Self::AboveCeiling { .. } => 1,
        }
    }
}

impl fmt::Display for GateError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unavailable(message) => formatter.write_str(message),
            Self::PanicPath(references) => {
                for reference in references {
                    writeln!(formatter, "{reference}")?;
                }
                formatter.write_str("linked footprint retains a panic path")
            }
            Self::BelowFloor { .. } => formatter
                .write_str("linked release footprint fell below the fixed population floor"),
            Self::AboveCeiling { .. } => {
                formatter.write_str("linked release footprint exceeds the fixed byte ceiling")
            }
        }
    }
}

fn included_object(name: &str) -> bool {
    name.contains("quire_contract_runtime") && !name.contains(".core-")
}

fn section_name(name: &str) -> bool {
    name == ".text"
        || name.starts_with(".text.")
        || name == ".rodata"
        || name.starts_with(".rodata.")
}

fn linked_section_bytes(report: &str) -> Result<u64, GateError> {
    let mut include = false;
    let mut found = false;
    let mut bytes = 0_u64;
    for line in report.lines() {
        if line.contains("(ex ")
            && line
                .trim_end()
                .ends_with("libquire_contract_runtime_footprint.a):")
        {
            include = line.split_whitespace().next().is_some_and(included_object);
            continue;
        }
        if !include {
            continue;
        }
        let mut fields = line.split_whitespace();
        let Some(section) = fields.next() else {
            continue;
        };
        if !section_name(section) {
            continue;
        }
        let size = fields
            .next()
            .and_then(|field| field.parse::<u64>().ok())
            .ok_or(GateError::Unavailable(
                "could not measure linked .text/.rodata sections",
            ))?;
        bytes = bytes
            .checked_add(size)
            .ok_or(GateError::Unavailable("linked section size overflow"))?;
        found = true;
    }
    if !found {
        return Err(GateError::Unavailable(
            "could not measure linked .text/.rodata sections",
        ));
    }
    Ok(bytes)
}

fn panic_references(report: &str) -> Vec<String> {
    let mut include = false;
    let mut references = Vec::new();
    for line in report.lines() {
        if line.contains(":     file format") {
            include = line
                .split_whitespace()
                .next()
                .and_then(|name| name.strip_suffix(':'))
                .is_some_and(included_object);
            continue;
        }
        let panic_symbol = line.contains("rust_begin_unwind")
            || line.contains("panic_bounds_check")
            || in_order(line, &["core", "panicking"])
            || in_order(line, &["slice", "index", "fail"]);
        if include && panic_symbol {
            references.push(line.to_owned());
        }
    }
    references
}

fn in_order(mut text: &str, parts: &[&str]) -> bool {
    for part in parts {
        let Some(index) = text.find(part) else {
            return false;
        };
        text = &text[index + part.len()..];
    }
    true
}

fn evaluate_reports(size_report: &str, relocation_report: &str) -> Result<u64, GateError> {
    let bytes = linked_section_bytes(size_report)?;
    let references = panic_references(relocation_report);
    if !references.is_empty() {
        return Err(GateError::PanicPath(references));
    }
    if bytes < MINIMUM_BYTES {
        return Err(GateError::BelowFloor { bytes });
    }
    if bytes > LIMIT_BYTES {
        return Err(GateError::AboveCeiling { bytes });
    }
    Ok(bytes)
}

fn tool_output(
    tool: &str,
    flag: &str,
    artifact: &Path,
    error: &'static str,
) -> Result<String, GateError> {
    let output = Command::new(tool)
        .arg(flag)
        .arg(artifact)
        .output()
        .map_err(|_| GateError::Unavailable(error))?;
    if !output.status.success() {
        return Err(GateError::Unavailable(error));
    }
    String::from_utf8(output.stdout).map_err(|_| GateError::Unavailable(error))
}

fn check(artifact: &Path) -> Result<u64, GateError> {
    if !artifact.is_file() {
        return Err(GateError::Unavailable("footprint staticlib not found"));
    }
    let size = tool_output(
        "size",
        "-A",
        artifact,
        "could not measure linked .text/.rodata sections",
    )?;
    let relocations = tool_output(
        "objdump",
        "-r",
        artifact,
        "could not inspect linked footprint relocations",
    )?;
    evaluate_reports(&size, &relocations)
}

fn main() -> ExitCode {
    let artifact = env::args_os().nth(1).map_or_else(
        || {
            PathBuf::from(format!(
                "target/footprint-msrv/{TARGET}/release/libquire_contract_runtime_footprint.a"
            ))
        },
        PathBuf::from,
    );
    match check(&artifact) {
        Ok(bytes) => {
            println!(
                "artifact={} target={TARGET} sections=.text+.rodata bytes={bytes} minimum={MINIMUM_BYTES} limit={LIMIT_BYTES} panic_references=0",
                artifact.display()
            );
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("{error}");
            ExitCode::from(error.exit_code())
        }
    }
}

#[cfg(test)]
mod tests {
    use ix_trace_rs::trace;

    use super::{evaluate_reports, GateError};

    const SIZE_IN_BAND: &str = "quire_contract_runtime_footprint-abc.o (ex libquire_contract_runtime_footprint.a):\nsection size addr\n.text 600 0\n.rodata 100 0\n";
    const RELOC_HEADER: &str =
        "quire_contract_runtime_footprint-abc.o:     file format elf32-littlearm\n";

    /// A retained panic relocation invalidates a linked footprint inside the size band.
    #[trace("TC-007", "NFR-001-AC-3")]
    #[test]
    fn tc_007_linked_footprint_rejects_a_runtime_panic_relocation() {
        let relocations = format!("{RELOC_HEADER}00000000 UNKNOWN rust_begin_unwind\n");
        assert!(matches!(
            evaluate_reports(SIZE_IN_BAND, &relocations),
            Err(GateError::PanicPath(_))
        ));
    }

    /// The fixed population must remain within the declared linked section band.
    #[trace("TC-007", "NFR-001-AC-3")]
    #[test]
    fn tc_007_linked_footprint_applies_both_size_bounds() {
        let too_small = SIZE_IN_BAND.replace(".text 600", ".text 399");
        let too_large = SIZE_IN_BAND.replace(".text 600", ".text 4000");
        assert!(matches!(
            evaluate_reports(&too_small, RELOC_HEADER),
            Err(GateError::BelowFloor { bytes: 499 })
        ));
        assert!(matches!(
            evaluate_reports(&too_large, RELOC_HEADER),
            Err(GateError::AboveCeiling { bytes: 4100 })
        ));
        assert_eq!(evaluate_reports(SIZE_IN_BAND, RELOC_HEADER).unwrap(), 700);
    }
}
