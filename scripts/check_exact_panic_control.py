#!/usr/bin/env python3
"""Require a reachable RT-owned injected panic to fail the exact relocation gate."""

import io
import os
import shlex
import subprocess
import sys
import tarfile
import tempfile
from pathlib import Path


def run(*argv: str, cwd: Path, env: dict[str, str] | None = None) -> subprocess.CompletedProcess[str]:
    return subprocess.run(argv, cwd=cwd, env=env, text=True, capture_output=True, check=False)


def check_malformed_report(archive: Path, scratch: Path) -> None:
    report = run("objdump", "-rC", str(archive), cwd=scratch)
    if report.returncode:
        raise SystemExit(f"exact panic control: objdump failed:\n{report.stderr[-2000:]}")
    lines = report.stdout.splitlines(keepends=True)
    in_runtime = False
    for index, line in enumerate(lines):
        if "file format" in line:
            in_runtime = line.startswith("quire_contract_runtime-")
        if in_runtime and line.startswith("OFFSET"):
            lines.insert(index + 1, "00000000 core::panicking::panic\n")
            break
    else:
        raise SystemExit("exact panic control: no RT-owned relocation table to mutate")

    tools = scratch / "tools"
    tools.mkdir()
    malformed = scratch / "malformed-relocations.txt"
    malformed.write_text("".join(lines))
    wrapper = tools / "objdump"
    wrapper.write_text(f"#!/bin/sh\ncat {shlex.quote(str(malformed))}\n")
    wrapper.chmod(0o755)
    env = os.environ.copy()
    env["PATH"] = f"{tools}:{env['PATH']}"
    check = run(sys.executable, "scripts/check_exact_panic.py", str(archive), cwd=scratch, env=env)
    if check.returncode == 0 or "malformed RT-owned relocation" not in check.stderr:
        raise SystemExit(
            "exact panic control: malformed RT relocation report was not rejected:\n"
            + f"check status: {check.returncode}\n{check.stderr[-2000:]}"
        )
    print("exact panic control: malformed RT relocation report rejected")


def main() -> None:
    if len(sys.argv) != 5:
        raise SystemExit("usage: check_exact_panic_control.py TARGET_DIR MSRV TARGET CARGO")
    root = Path(__file__).resolve().parent.parent
    target = (root / sys.argv[1]).resolve()
    msrv, target_triple, cargo = sys.argv[2:]
    target.mkdir(parents=True, exist_ok=True)
    archive = subprocess.run(["git", "archive", "HEAD"], cwd=root, capture_output=True, check=True)

    with tempfile.TemporaryDirectory(prefix="exact-panic-control-", dir=target) as temp:
        scratch = Path(temp)
        with tarfile.open(fileobj=io.BytesIO(archive.stdout), mode="r:") as source:
            source.extractall(scratch, filter="data")
        artifact = target / target_triple / "exact-panic/libquire_contract_runtime_exact_panic.a"
        check_malformed_report(artifact, scratch)
        scalar = scratch / "src/scalar/division.rs"
        content = scalar.read_text()
        anchor = "    items\n        .iter()"
        if content.count(anchor) != 1:
            raise SystemExit("exact panic control: scalar insertion point absent or ambiguous")
        scalar.write_text(
            content.replace(
                anchor,
                '    if core::hint::black_box(!items.is_empty()) { panic!("TC-206 injected panic"); }\n' + anchor,
            )
        )
        env = os.environ.copy()
        env["CARGO_TARGET_DIR"] = str(target)
        build = run(
            cargo, f"+{msrv}", "build", "--locked", "--profile", "exact-panic",
            "--manifest-path", "measurement/exact-panic/Cargo.toml", "--target",
            target_triple, cwd=scratch, env=env,
        )
        if build.returncode:
            raise SystemExit(f"exact panic control: scratch build failed:\n{build.stderr[-4000:]}")
        check = run(sys.executable, "scripts/check_exact_panic.py", str(artifact), cwd=scratch)
        if check.returncode == 0 or "RT-owned panic relocations:" not in check.stderr:
            raise SystemExit(
                "exact panic control: injected panic did not trigger the RT relocation verdict:\n"
                + f"build: {build.stderr[-2000:]}\ncheck status: {check.returncode}\n"
                + check.stderr[-2000:]
            )
        print("exact panic control: reachable RT-owned panic relocation detected")


if __name__ == "__main__":
    main()
