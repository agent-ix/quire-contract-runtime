#!/usr/bin/env python3
"""Inspect the retained RT object relocations in the exact no-std archive."""

import re
import subprocess
import sys
from pathlib import Path


EXPECTED_CALLS = (
    "negotiate_integer_division",
    "negotiate_ieee",
    "PackageDeclarations>::check",
    "CheckedPackage>::check_expression",
    "CheckedPackage>::evaluate",
)
GENERIC_CALL = "evaluate_boolean_short_circuit::<quire_contract_runtime_exact_panic::quire_runtime_exact_panic"
PANIC_TARGET = re.compile(
    r"rust_begin_unwind|panic_bounds_check|core::panicking|core.*panicking|"
    r"slice::index::fail|slice.*index.*fail"
)
OBJECT_HEADER = re.compile(r"^(\S+):\s+file format\s+\S+")
SECTION_HEADER = re.compile(r"^RELOCATION RECORDS FOR \[(.+)\]:$")
COLUMN_HEADER = re.compile(r"^OFFSET\s+TYPE\s+VALUE$")
RELOCATION = re.compile(r"^\s*[0-9a-fA-F]+\s+\S+\s+(.+?)\s*$")


def fail(reason: str) -> None:
    print(f"exact panic gate: {reason}", file=sys.stderr)
    raise SystemExit(1)


def tool(*argv: str) -> str:
    try:
        result = subprocess.run(argv, text=True, capture_output=True, check=True)
    except (OSError, subprocess.CalledProcessError) as error:
        fail(f"could not inspect archive with {' '.join(argv)}: {error}")
    if not result.stdout.strip():
        fail(f"empty output from {' '.join(argv)}")
    return result.stdout


def main() -> None:
    if len(sys.argv) != 2:
        fail("usage: check_exact_panic.py ARCHIVE")
    archive = Path(sys.argv[1])
    if not archive.is_file():
        fail(f"missing archive {archive}")

    members = tool("ar", "t", str(archive)).splitlines()
    runtime_members = {member for member in members if member.startswith("quire_contract_runtime-")}
    probe_members = {
        member for member in members if member.startswith("quire_contract_runtime_exact_panic-")
    }
    if not runtime_members or not probe_members:
        fail("empty RT-owned or probe object population")

    report = tool("objdump", "-rC", str(archive))
    current = None
    section = None
    awaiting_columns = False
    found_rt = set()
    found_probe = set()
    expected = set()
    panic = []
    relocations = 0
    for line in report.splitlines():
        header = OBJECT_HEADER.match(line)
        if header:
            if awaiting_columns and current in runtime_members:
                fail(f"missing relocation columns in RT-owned object {current}")
            current = header.group(1)
            section = None
            awaiting_columns = False
            if current in runtime_members:
                found_rt.add(current)
            if current in probe_members:
                found_probe.add(current)
            continue
        section_header = SECTION_HEADER.match(line)
        if section_header:
            if awaiting_columns and current in runtime_members:
                fail(f"missing relocation columns in RT-owned object {current}")
            section = section_header.group(1)
            awaiting_columns = True
            continue
        if current is None or section is None or not line.strip():
            continue
        if awaiting_columns:
            if current in runtime_members and not COLUMN_HEADER.fullmatch(line):
                fail(f"malformed RT-owned relocation columns in {current}: {line}")
            awaiting_columns = False
            continue
        relocation = RELOCATION.match(line)
        if relocation is None:
            if current in runtime_members:
                fail(f"malformed RT-owned relocation in {current}: {line}")
            continue
        if current in runtime_members or current in probe_members:
            relocations += 1
        if current in runtime_members and PANIC_TARGET.search(relocation.group(1)):
            panic.append(f"{current}: {line.strip()}")
        if current in probe_members and section.startswith(".text"):
            expected.update(call for call in EXPECTED_CALLS if call in relocation.group(1))

    if awaiting_columns and current in runtime_members:
        fail(f"missing relocation columns in RT-owned object {current}")
    if found_rt != runtime_members or found_probe != probe_members or relocations == 0:
        fail("unparseable or incomplete object relocation report")
    missing = set(EXPECTED_CALLS) - expected
    symbols = tool("nm", "-A", "-C", "--defined-only", str(archive))
    if not any(
        f":{member}:" in line and GENERIC_CALL in line
        for member in probe_members
        for line in symbols.splitlines()
    ):
        missing.add("evaluate_boolean_short_circuit")
    if missing:
        fail(f"probe omits required RT calls: {', '.join(sorted(missing))}")
    if panic:
        fail("RT-owned panic relocations:\n" + "\n".join(panic))
    print(
        f"artifact={archive} command=objdump -rC "
        f"runtime_objects={','.join(sorted(runtime_members))} "
        f"probe_objects={','.join(sorted(probe_members))} "
        f"required_calls={','.join((*EXPECTED_CALLS, 'evaluate_boolean_short_circuit'))} "
        "panic_references=0"
    )


if __name__ == "__main__":
    main()
