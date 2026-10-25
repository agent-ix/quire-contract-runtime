#!/usr/bin/env python3
"""Inspect the retained RT object relocations in the exact no-std archive."""

import re
import subprocess
import sys
from pathlib import Path


EXPECTED_CALLS = (
    "negotiate_integer_division",
    "negotiate_ieee",
    "evaluate_boolean_short_circuit",
    "PackageDeclarations>::check",
    "CheckedPackage>::evaluate",
)
PANIC_TARGET = re.compile(
    r"rust_begin_unwind|panic_bounds_check|core::panicking|core.*panicking|"
    r"slice::index::fail|slice.*index.*fail"
)
OBJECT_HEADER = re.compile(r"^(\S+):\s+file format\s+\S+")
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
    found_rt = set()
    found_probe = set()
    expected = set()
    panic = []
    relocations = 0
    for line in report.splitlines():
        header = OBJECT_HEADER.match(line)
        if header:
            current = header.group(1)
            if current in runtime_members:
                found_rt.add(current)
            if current in probe_members:
                found_probe.add(current)
            continue
        relocation = RELOCATION.match(line)
        if relocation is None or current is None:
            continue
        if current in runtime_members or current in probe_members:
            relocations += 1
        if current in runtime_members and PANIC_TARGET.search(relocation.group(1)):
            panic.append(f"{current}: {line.strip()}")
        if current in probe_members:
            expected.update(call for call in EXPECTED_CALLS if call in relocation.group(1))

    if found_rt != runtime_members or found_probe != probe_members or relocations == 0:
        fail("unparseable or incomplete object relocation report")
    missing = set(EXPECTED_CALLS) - expected
    if missing:
        fail(f"probe omits required RT calls: {', '.join(sorted(missing))}")
    if panic:
        fail("RT-owned panic relocations:\n" + "\n".join(panic))
    print(
        f"artifact={archive} command=objdump -rC "
        f"runtime_objects={','.join(sorted(runtime_members))} "
        f"probe_objects={','.join(sorted(probe_members))} "
        f"required_calls={','.join(EXPECTED_CALLS)} panic_references=0"
    )


if __name__ == "__main__":
    main()
