#!/usr/bin/env python3
"""Run every declared feature set and publish the result as a structured document.

`runtime.feature-matrix/v1`. One row per feature set, and each row is decided by
two structured facts rather than by reading a transcript:

  * the build phase runs `cargo test --no-run --message-format=json`, so a
    compilation error is a `compiler-message` object with `level: "error"` — a
    field, not a sentence;
  * the test phase runs the same invocation without `--no-run` and takes libtest's
    own verdict, which on stable Rust is the process exit status.

Those are the two channels stable Rust actually publishes. Per-test granularity
would need libtest's unstable JSON formatter, which requires a nightly compiler
and would therefore report on a different compiler than the one the crate ships
on. That limitation is stated in the record's unknowns rather than papered over
by parsing `test result: ok.` out of the human output.

Separating the phases matters: a crate that no longer compiles and a crate whose
tests fail are different facts, and a single exit status conflates them.

Outcomes: pass, fail (with `phase` naming which one), unavailable when cargo
itself cannot be run.

Exit status
  gate mode : 0 when every feature set passed, 1 otherwise
  --json    : 0 whenever a document was produced, 2 when none could be
"""

from __future__ import annotations

import argparse
import json
import os
import pwd
import subprocess
import sys
from pathlib import Path
from typing import Any

ROOT = Path(__file__).resolve().parent.parent
PROTOCOL = "runtime.feature-matrix/v1"

# The crate's own test targets. Named explicitly, and `tests/shared_assurance.rs`
# is deliberately not among them.
#
# That test binary drives the assurance chain, which consumes this producer's
# output. Running it from inside this producer would make the producer depend on
# its own result: on a clean tree the chain would have no inputs and fail, and on
# a dirty tree it would read whatever the previous run happened to leave behind.
# A green run that depends on a leftover file is not a green run.
#
# The shared-assurance tests are gates on the intake path, not members of the
# crate's feature matrix, and `make test` runs them after the producers have run.
DOMAIN_TARGETS = [
    "--lib",
    "--test",
    "integration",
    "--test",
    "operators",
    "--test",
    "proptest_adapter",
    "--test",
    "release_contract",
    "--test",
    "snapshot",
]

# Every feature set, named. The names are the symbols the rows are keyed on. The
# first four kept the names the retired collector used so that its records read
# like this document; those records were deleted under
# agent-ix/quire-contract-runtime#11 and the names are kept only because renaming
# a row's symbol renames what a trace binds to.
#
# Doc tests get their own row per feature set because `cargo test --doc` cannot be
# combined with an explicit target selection, and because the crate's
# `compile_fail` doctests are the whole of TC-005's default-surface evidence and
# TC-008's non-exhaustive-enum evidence. Dropping them to keep the table tidy
# would silently delete a verification.
FEATURE_SETS = (
    ("test-core", ["--no-default-features", *DOMAIN_TARGETS], ["TC-005", "NFR-001"], True),
    ("test-core-doc", ["--no-default-features", "--doc"], ["TC-005", "TC-008"], False),
    ("test-alloc", ["--features", "alloc", *DOMAIN_TARGETS], ["TC-005", "NFR-001"], True),
    ("test-alloc-doc", ["--features", "alloc", "--doc"], ["TC-005", "TC-008"], False),
    ("test-std", ["--features", "std", *DOMAIN_TARGETS], ["TC-005", "NFR-001"], True),
    ("test-std-doc", ["--features", "std", "--doc"], ["TC-005", "TC-008"], False),
    ("test-snapshot-json", ["--locked", "--no-default-features", "--features", "snapshot-json", *DOMAIN_TARGETS], ["TC-005", "TC-015", "FR-004"], True),
    ("test-snapshot-json-doc", ["--locked", "--no-default-features", "--features", "snapshot-json", "--doc"], ["TC-005", "TC-008", "TC-015"], False),
    ("test-all", ["--all-features", *DOMAIN_TARGETS], ["TC-005"], True),
    ("test-all-doc", ["--all-features", "--doc"], ["TC-005", "TC-008"], False),
    ("test-footprint", ["-p", "quire-contract-runtime-footprint"], ["TC-005", "NFR-001"], True),
    # The exact oracle tests are `#![cfg(feature = "exact")]`, so no other row runs
    # them. The shared-corpus agreement package is `make conformance`, not a row
    # here: it needs Rust 1.98 and a git dependency this crate's graph never has.
    ("test-exact", ["--locked", "--no-default-features", "--features", "exact", "--test", "exact_outcomes", "--test", "exact_arithmetic", "--test", "exact_allocation", "--test", "release_contract"], ["TC-005", "TC-016", "TC-017", "TC-023", "FR-006", "FR-007"], True),
)

# Actual no_std-target library builds, not host tests that can obtain std through
# dev dependencies. No linked-footprint claim is made for either feature.
NO_STD_BUILDS = (
    ("build-snapshot-json-no-std-msrv", "snapshot-json", ["TC-005", "TC-015", "NFR-001"]),
    ("build-exact-no-std-msrv", "exact", ["TC-005", "TC-016", "FR-006", "NFR-001"]),
)


class ProducerError(RuntimeError):
    """No result document could be produced at all."""


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
        "+1.75.0", "build", "--locked", "--lib", "--no-default-features",
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


def collect() -> dict[str, Any]:
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
    return {
        "protocol": PROTOCOL,
        "tool": {
            "identity": "quire-contract-runtime-feature-matrix",
            "cargo": run(["--version"]).stdout.strip() if available else None,
        },
        "verdict_channels": {
            "build": "cargo --message-format=json compiler-message level",
            "test": "libtest process exit status",
        },
        "entries": entries,
    }


def main(argv: list[str]) -> int:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument(
        "--json",
        action="store_true",
        help="emit runtime.feature-matrix/v1 on stdout; the producer role",
    )
    arguments = parser.parse_args(argv[1:])
    try:
        document = collect()
    except (ProducerError, OSError) as error:
        print(f"FEATURE_MATRIX_UNAVAILABLE: {error}", file=sys.stderr)
        return 2
    if arguments.json:
        print(json.dumps(document, indent=2, sort_keys=True))
        return 0
    failures = [row for row in document["entries"] if row["outcome"] != "pass"]
    for row in failures:
        print(
            f"FEATURE_MATRIX_{row['outcome'].upper()}: {row['symbol']} "
            f"phase={row['phase']} {row['detail']}",
            file=sys.stderr,
        )
    if failures:
        return 1
    print(f"verified {len(document['entries'])} feature sets")
    return 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv))
