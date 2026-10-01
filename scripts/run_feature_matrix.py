#!/usr/bin/env python3
"""Run every declared feature set.

One row per feature set, and each row is decided by two structured facts rather
than by reading a transcript:

  * the build phase runs `cargo test --no-run --message-format=json`, so a
    compilation error is a `compiler-message` object with `level: "error"` — a
    field, not a sentence;
  * the test phase runs the same invocation without `--no-run` and takes libtest's
    own verdict, which on stable Rust is the process exit status.

Those are the two channels stable Rust actually publishes. Per-test granularity
would need libtest's unstable JSON formatter, which requires a nightly compiler
and would therefore report on a different compiler than the one the crate ships
on.

Separating the phases matters: a crate that no longer compiles and a crate whose
tests fail are different facts, and a single exit status conflates them.

Outcomes: pass, fail (with `phase` naming which one), unavailable when cargo
itself cannot be run.

Exit status: 0 when every feature set passed, 1 otherwise.
"""

from __future__ import annotations

import json
import os
import pwd
import subprocess
import sys
from pathlib import Path
from typing import Any

ROOT = Path(__file__).resolve().parent.parent

# The crate's own test targets.
DOMAIN_TARGETS = [
    "--lib",
    "--test",
    "integration",
    "--test",
    "operators",
    "--test",
    "proptest_adapter",
    "--test",
    "snapshot",
]

# Every feature set, named.
#
# Doc tests get their own row per feature set because `cargo test --doc` cannot be
# combined with an explicit target selection, and because the crate's
# `compile_fail` doctests are the default-surface and non-exhaustive-enum
# evidence. Dropping them to keep the table tidy
# would silently delete a verification.
FEATURE_SETS = (
    ("test-core", ["--no-default-features", *DOMAIN_TARGETS], ["NFR-001"], True),
    ("test-core-doc", ["--no-default-features", "--doc"], ["NFR-002"], False),
    ("test-alloc", ["--features", "alloc", *DOMAIN_TARGETS], ["NFR-001"], True),
    ("test-alloc-doc", ["--features", "alloc", "--doc"], ["NFR-002"], False),
    ("test-std", ["--features", "std", *DOMAIN_TARGETS], ["NFR-001"], True),
    ("test-std-doc", ["--features", "std", "--doc"], ["NFR-002"], False),
    ("test-snapshot-json", ["--locked", "--no-default-features", "--features", "snapshot-json", *DOMAIN_TARGETS], ["TC-015", "FR-004"], True),
    ("test-snapshot-json-doc", ["--locked", "--no-default-features", "--features", "snapshot-json", "--doc"], ["NFR-002", "TC-015"], False),
    ("test-all", ["--all-features", *DOMAIN_TARGETS], ["NFR-001"], True),
    ("test-all-doc", ["--all-features", "--doc"], ["NFR-002"], False),
    ("test-footprint", ["-p", "quire-contract-runtime-footprint"], ["NFR-001"], True),
    # The exact oracle tests are `#![cfg(feature = "exact")]`, so no other row runs
    # them. The QSL shared-corpus agreement oracle is removed from this repository; recreating
    # it in agent-ix/quire-integration is planned under Linear IR-430.
    ("test-exact", ["--locked", "--no-default-features", "--features", "exact", "--test", "exact_outcomes", "--test", "exact_arithmetic", "--test", "exact_allocation"], ["TC-016", "TC-017", "TC-023", "FR-006", "FR-007"], True),
)

# Actual no_std-target library builds, not host tests that can obtain std through
# dev dependencies. No linked-footprint claim is made for either feature.
NO_STD_BUILDS = (
    ("build-snapshot-json-no-std-msrv", "snapshot-json", ["TC-015", "NFR-001"]),
    ("build-exact-no-std-msrv", "exact", ["TC-016", "FR-006", "NFR-001"]),
)


def environment() -> dict[str, str]:
    home = Path(pwd.getpwuid(os.getuid()).pw_dir)
    return {
        **os.environ,
        "HOME": str(home),
        "CARGO_HOME": str(home / ".cargo"),
        "RUSTUP_HOME": str(home / ".rustup"),
        "CARGO_TARGET_DIR": str(ROOT / "target"),
    }


def no_std_row(symbol: str, feature: str, traces: list[str], available: bool) -> dict[str, Any]:
    native_flags = [
        "+1.82.0", "build", "--locked", "--lib", "--no-default-features",
        "--features", feature, "--target", "thumbv7em-none-eabi",
        "--message-format=json",
    ]
    if available:
        built = run(native_flags)
        errors = build_errors(built.stdout)
        return {
            "symbol": symbol,
            "outcome": "pass" if built.returncode == 0 and not errors else "fail",
            "traceIds": traces,
            "phase": "build-only",
            "exitStatus": built.returncode,
            "argv": native_flags,
            "detail": errors[:3] or (None if built.returncode == 0 else ["cargo reported a non-zero build status"]),
        }
    return {
        "symbol": symbol,
        "outcome": "unavailable",
        "traceIds": traces,
        "phase": None,
        "detail": "the trusted cargo executable is absent",
    }


def cargo_path() -> Path:
    return Path(pwd.getpwuid(os.getuid()).pw_dir) / ".cargo" / "bin" / "cargo"


def run(arguments: list[str]) -> subprocess.CompletedProcess[str]:
    return subprocess.run(
        [str(cargo_path()), *arguments],
        cwd=ROOT,
        env=environment(),
        capture_output=True,
        text=True,
        check=False,
    )


def build_errors(stdout: str) -> list[str]:
    """Read cargo's own JSON message stream for compiler errors.

    Lines that are not JSON objects with a `reason` are ignored rather than
    guessed at: cargo owns this stream's shape and anything else on the channel
    is not cargo speaking.
    """
    errors = []
    for line in stdout.splitlines():
        line = line.strip()
        if not line.startswith("{"):
            continue
        try:
            message = json.loads(line)
        except json.JSONDecodeError:
            continue
        if message.get("reason") != "compiler-message":
            continue
        if message.get("message", {}).get("level") == "error":
            errors.append(message["message"].get("rendered", "").strip().splitlines()[0:1])
    return [item[0] for item in errors if item]


def collect() -> list[dict[str, Any]]:
    cargo = cargo_path()
    available = cargo.is_file()
    entries = []
    for name, flags, traces, two_phase in FEATURE_SETS:
        if not available:
            entries.append(
                {
                    "symbol": name,
                    "outcome": "unavailable",
                    "traceIds": traces,
                    "phase": None,
                    "detail": "the trusted cargo executable is absent",
                }
            )
            continue
        # `cargo test --doc --no-run` is not a thing cargo accepts, so a doc row
        # has one phase. It is marked as such rather than silently reported as if
        # its build had been checked separately.
        if two_phase:
            built = run(["test", *flags, "--no-run", "--message-format=json"])
            errors = build_errors(built.stdout)
            if built.returncode != 0 or errors:
                entries.append(
                    {
                        "symbol": name,
                        "outcome": "fail",
                        "traceIds": traces,
                        "phase": "build",
                        "exitStatus": built.returncode,
                        "detail": errors[:3] or ["cargo reported a non-zero build status"],
                    }
                )
                continue
        tested = run(["test", *flags])
        entries.append(
            {
                "symbol": name,
                "outcome": "pass" if tested.returncode == 0 else "fail",
                "traceIds": traces,
                "phase": "test" if two_phase else "test-only",
                "exitStatus": tested.returncode,
                "detail": None
                if tested.returncode == 0
                else tested.stdout.strip().splitlines()[-6:],
            }
        )
    entries.extend(no_std_row(symbol, feature, traces, available) for symbol, feature, traces in NO_STD_BUILDS)
    return entries


def main() -> int:
    try:
        entries = collect()
    except OSError as error:
        print(f"FEATURE_MATRIX_UNAVAILABLE: {error}", file=sys.stderr)
        return 2
    failures = [row for row in entries if row["outcome"] != "pass"]
    for row in failures:
        print(
            f"FEATURE_MATRIX_{row['outcome'].upper()}: {row['symbol']} "
            f"phase={row['phase']} {row['detail']}",
            file=sys.stderr,
        )
    if failures:
        return 1
    print(f"verified {len(entries)} feature sets")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
