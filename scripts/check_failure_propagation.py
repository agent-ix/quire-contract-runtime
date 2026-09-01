#!/usr/bin/env python3
"""Prove every mandatory local-check recipe propagates command failures."""

from __future__ import annotations

import argparse
import os
import pwd
import re
import shlex
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path


ROOT = Path(__file__).resolve().parent.parent
GUARD_TARGET = "ci-guard"
CI_ORDER = (
    GUARD_TARGET,
    "fmt-check",
    "spec",
    "lint",
    "test-features",
    "doc",
    "msrv",
    "size",
    "deny",
    "audit-unsafe",
    "audit-panic",
    "coverage",
    "kani",
    "evidence-tool",
    "verify-evidence",
    "assurance-anchor",
)
CI_PROBES = set(CI_ORDER) - {GUARD_TARGET}
TRANSITIVE_PROBES = {"kani-census", "kani-mutations"}
TARGET = re.compile(r"^([A-Za-z0-9_.-]+):(?:\s+(.*?))?\s*$")
SHELL_CONTROL = re.compile(r"&&|\|\||[;|&]")
MAKEFLAGS_ASSIGNMENT = re.compile(
    r"^\s*(?:(?:export|override)\s+)*MAKEFLAGS\s*(?::|\+|\?)?=\s*(.*)$"
)
INCLUDE_DIRECTIVE = re.compile(r"^\s*(?:-?include|sinclude)(?:\s|$)")


def trusted_home() -> Path:
    return Path(pwd.getpwuid(os.getuid()).pw_dir)


def trusted_environment() -> dict[str, str]:
    home = trusted_home()
    environment = dict(os.environ)
    environment.update(
        HOME=str(home),
        CARGO_HOME=str(home / ".cargo"),
        RUSTUP_HOME=str(home / ".rustup"),
    )
    return environment


def parse_makefile(text: str) -> tuple[dict[str, list[str]], dict[str, list[str]]]:
    dependencies: dict[str, list[str]] = {}
    recipes: dict[str, list[str]] = {}
    current: str | None = None
    for line in text.splitlines():
        if line.startswith("\t"):
            if current is not None:
                recipes.setdefault(current, []).append(line[1:])
            continue
        current = None
        if not line or line[0].isspace() or line.startswith("#"):
            continue
        match = TARGET.fullmatch(line)
        if match is None or match.group(1).startswith("."):
            continue
        current = match.group(1)
        dependencies[current] = (match.group(2) or "").split()
    return dependencies, recipes


def makeflags_errors(value: str) -> list[str]:
    """Allow only GNU Make parallelism/load flags; reject execution modifiers."""
    try:
        tokens = shlex.split(value)
    except ValueError:
        return ["MAKEFLAGS cannot be parsed safely"]
    errors: list[str] = []
    optional_value = False
    for token in tokens:
        if optional_value and re.fullmatch(r"[0-9]+(?:\.[0-9]+)?", token):
            optional_value = False
            continue
        optional_value = False
        if token in {"-j", "--jobs", "-l", "--load-average"}:
            optional_value = True
        elif re.fullmatch(r"-[jl][0-9]+(?:\.[0-9]+)?", token):
            continue
        elif re.fullmatch(r"--(?:jobs|load-average)=[0-9]+(?:\.[0-9]+)?", token):
            continue
        elif re.fullmatch(r"--jobserver-(?:auth|fds)=.+", token):
            continue
        elif token:
            errors.append(f"unsafe MAKEFLAGS token: {token}")
    return errors


def command_parts(command: str) -> tuple[str, str]:
    stripped = command.lstrip()
    modifiers = ""
    while stripped[:1] in {"@", "+", "-"}:
        modifiers += stripped[0]
        stripped = stripped[1:].lstrip()
    return modifiers, stripped


def inspect(makefile: Path) -> list[str]:
    text = makefile.read_text(encoding="utf-8")
    dependencies, recipes = parse_makefile(text)
    errors: list[str] = []
    required_ci = set(CI_ORDER)
    observed_order = dependencies.get("ci", [])
    observed = set(observed_order)
    if tuple(observed_order) != CI_ORDER:
        errors.append(
            "ci prerequisite order/census drift: "
            f"missing={sorted(required_ci - observed)}, extra={sorted(observed - required_ci)}, "
            f"observed={observed_order}"
        )
    if dependencies.get("kani") != ["kani-census", "kani-mutations"]:
        errors.append("kani must depend exactly on kani-census and kani-mutations")
    for number, line in enumerate(text.splitlines(), start=1):
        if INCLUDE_DIRECTIVE.match(line):
            errors.append(f"Makefile:{number} includes unreviewed Make control text")
        if re.match(r"^\s*\.(?:IGNORE|SILENT)\s*(?::|$)", line):
            errors.append(f"Makefile:{number} declares a global recipe-control directive")
        assignment = MAKEFLAGS_ASSIGNMENT.match(line)
        if assignment is not None:
            errors.extend(
                f"Makefile:{number} {error}"
                for error in makeflags_errors(assignment.group(1))
            )
    for target in sorted(required_ci | TRANSITIVE_PROBES):
        commands = recipes.get(target, [])
        if not commands:
            errors.append(f"mandatory target {target} has no recipe")
            continue
        for command in commands:
            modifiers, stripped = command_parts(command)
            if "-" in modifiers:
                errors.append(f"mandatory target {target} ignores a recipe failure: {command}")
            if SHELL_CONTROL.search(stripped):
                errors.append(
                    f"mandatory target {target} uses forbidden shell control operators: {command}"
                )
    return errors


def probe_command_positions(makefile: Path) -> list[str]:
    """Substitute false at every mandatory recipe position and require Make to fail."""
    _, recipes = parse_makefile(makefile.read_text(encoding="utf-8"))
    errors: list[str] = []
    make = shutil.which("make")
    if make != "/usr/bin/make":
        return [f"make must resolve to /usr/bin/make, got {make}"]
    clean_env = dict(os.environ)
    clean_env.pop("MAKEFLAGS", None)
    with tempfile.TemporaryDirectory() as directory:
        probe = Path(directory) / "Makefile"
        for target in sorted(CI_PROBES | TRANSITIVE_PROBES):
            commands = recipes.get(target, [])
            for selected in range(len(commands)):
                lines = [f".PHONY: {target}", f"{target}:"]
                for index, command in enumerate(commands):
                    modifiers, _ = command_parts(command)
                    lines.append(f"\t{modifiers}{'false' if index == selected else 'true'}")
                probe.write_text("\n".join(lines) + "\n", encoding="utf-8")
                result = subprocess.run(
                    [make, "--no-print-directory", "-f", str(probe), target],
                    check=False,
                    stdout=subprocess.DEVNULL,
                    stderr=subprocess.DEVNULL,
                    env=clean_env,
                )
                if result.returncode == 0:
                    errors.append(
                        f"mandatory target {target} swallowed failure at recipe position "
                        f"{selected + 1}"
                    )
    return errors


def inspect_toolchain() -> list[str]:
    home = trusted_home()
    expected = {
        "cargo": str(home / ".cargo" / "bin" / "cargo"),
        "make": "/usr/bin/make",
        "python3": "/usr/bin/python3",
        "quire": str(home / ".npm-global" / "bin" / "quire"),
    }
    errors = [
        f"{name} must resolve to {path}, got {shutil.which(name)}"
        for name, path in expected.items()
        if shutil.which(name) != path
    ]
    version_commands = {
        "cargo": ([expected["cargo"], "--version"], r"^cargo \d+\.\d+\.\d+"),
        "python3": ([expected["python3"], "--version"], r"^Python 3\.\d+\.\d+"),
        "quire": ([expected["quire"], "--version"], r"^quire \d+\.\d+\.\d+"),
    }
    for name, (command, pattern) in version_commands.items():
        try:
            completed = subprocess.run(
                command,
                check=False,
                capture_output=True,
                text=True,
                env=trusted_environment(),
            )
        except OSError as error:
            errors.append(f"cannot execute {name}: {error}")
            continue
        output = (completed.stdout + completed.stderr).strip()
        if completed.returncode != 0 or re.search(pattern, output) is None:
            errors.append(f"unexpected {name} identity: {output!r}")
    return errors


def inspect_shell_gate_failures() -> list[str]:
    """Require shell gates to reject known-absent artifact inputs."""
    probes = {
        "linked-footprint": [
            "/usr/bin/bash",
            "scripts/check_linked_footprint.sh",
            "/definitely/absent/runtime-footprint.a",
        ],
        "rlib-size-observation": [
            "/usr/bin/bash",
            "scripts/measure_rlib_size.sh",
            "/definitely/absent/runtime-rlibs",
        ],
    }
    errors = []
    for name, command in probes.items():
        completed = subprocess.run(
            command,
            cwd=ROOT,
            check=False,
            stdout=subprocess.DEVNULL,
            stderr=subprocess.DEVNULL,
        )
        if completed.returncode == 0:
            errors.append(f"{name} shell gate accepted its missing-artifact probe")
    return errors


# Implements: NFR-002
def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--makefile", type=Path, default=ROOT / "Makefile")
    parser.add_argument("--inspect-only", action="store_true")
    parser.add_argument("--static-only", action="store_true")
    args = parser.parse_args()
    errors = inspect(args.makefile)
    errors.extend(makeflags_errors(os.environ.get("MAKEFLAGS", "")))
    if os.environ.get("MAKE"):
        errors.append("ambient MAKE override is not permitted")
    if os.environ.get("PYTHONOPTIMIZE") or sys.flags.optimize:
        errors.append("optimized Python disables policy assertions")
    if not args.static_only:
        errors.extend(inspect_toolchain())
    if not args.inspect_only and not args.static_only and not errors:
        errors.extend(probe_command_positions(args.makefile))
        errors.extend(inspect_shell_gate_failures())
    for error in errors:
        print(error, file=sys.stderr)
    if errors:
        return 1
    print(f"all {len(CI_PROBES)} mandatory local-check targets propagate failures")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
