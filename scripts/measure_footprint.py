#!/usr/bin/env python3
"""Take MP-001's governed footprint measurement and publish it as a document.

`runtime.footprint/v1`. Two rows: the crate's own linked `.text`+`.rodata` bytes
against the governed floor and ceiling, and the count of relocations that would
drag a panic path into a no_std image.

This producer does the whole job the record's declared command names: it links
the footprint staticlib on the declared MSRV compiler for `thumbv7em-none-eabi`,
then measures it. Splitting the build out into the Makefile would leave the
declared command describing half of what happened, and a declared command that is
not the executed command is a lie in a sealed attestation.

The measurement itself lives in `scripts/check_linked_footprint.sh`, which is the
tool that owns `size` and `objdump` and which publishes the same structured
document. Nothing here re-derives its numbers; a second implementation of a
measurement is a second answer.

Outcomes: pass, fail (with the phase that failed), unavailable when the toolchain
or the binutils the measurement needs are absent.

Exit status
  gate mode : 0 inside the governed band with no panic relocation, 1 otherwise
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
PROTOCOL = "runtime.footprint/v1"
MSRV = "1.75.0"
TARGET = "thumbv7em-none-eabi"
TARGET_DIR = ROOT / "target" / "footprint-msrv"
ARTIFACT = TARGET_DIR / TARGET / "release" / "libquire_contract_runtime_footprint.a"
MEASURER = ROOT / "scripts" / "check_linked_footprint.sh"


class ProducerError(RuntimeError):
    """No result document could be produced at all."""


def environment(target_dir: Path) -> dict[str, str]:
    home = Path(pwd.getpwuid(os.getuid()).pw_dir)
    return {
        **os.environ,
        "HOME": str(home),
        "CARGO_HOME": str(home / ".cargo"),
        "RUSTUP_HOME": str(home / ".rustup"),
        "CARGO_TARGET_DIR": str(target_dir),
    }


def degraded(outcome: str, detail: str, phase: str | None) -> dict[str, Any]:
    """Every row carries the same non-success answer, with the phase that caused it."""
    return {
        "protocol": PROTOCOL,
        "artifact": str(ARTIFACT.relative_to(ROOT)),
        "target": TARGET,
        "msrv": MSRV,
        "entries": [
            {
                "symbol": "linked-text-rodata-bytes",
                "outcome": outcome,
                "traceIds": ["NFR-001", "MP-001"],
                "phase": phase,
                "detail": detail,
            },
            {
                "symbol": "panic-relocations",
                "outcome": outcome,
                "traceIds": ["NFR-002", "MP-001"],
                "phase": phase,
                "detail": detail,
            },
        ],
    }


def collect() -> dict[str, Any]:
    home = Path(pwd.getpwuid(os.getuid()).pw_dir)
    cargo = home / ".cargo" / "bin" / "cargo"
    if not cargo.is_file():
        return degraded("unavailable", "the trusted cargo executable is absent", None)

    build = subprocess.run(
        [
            str(cargo),
            f"+{MSRV}",
            "build",
            "--locked",
            "--release",
            "--manifest-path",
            "measurement/footprint/Cargo.toml",
            "--target",
            TARGET,
        ],
        cwd=ROOT,
        env=environment(TARGET_DIR),
        capture_output=True,
        text=True,
        check=False,
    )
    if build.returncode != 0:
        # A toolchain that is not installed and a crate that does not compile are
        # different facts. rustup says which one this is on its own channel.
        unavailable = "is not installed" in build.stderr or "no such command" in build.stderr
        return degraded(
            "unavailable" if unavailable else "fail",
            build.stderr.strip().splitlines()[-1] if build.stderr.strip() else "build failed",
            "build",
        )

    measured = subprocess.run(
        ["bash", str(MEASURER), "--json", str(ARTIFACT)],
        cwd=ROOT,
        capture_output=True,
        text=True,
        check=False,
    )
    if measured.returncode != 0:
        raise ProducerError(
            f"the footprint measurer exited {measured.returncode}: {measured.stderr.strip()}"
        )
    try:
        document = json.loads(measured.stdout)
    except json.JSONDecodeError as error:
        raise ProducerError(f"the footprint measurer did not emit its document: {error}") from error
    if document.get("protocol") != PROTOCOL:
        raise ProducerError(
            f"the footprint measurer declared protocol {document.get('protocol')!r}, "
            f"not {PROTOCOL}"
        )
    document["msrv"] = MSRV
    return document


def main(argv: list[str]) -> int:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument(
        "--json",
        action="store_true",
        help="emit runtime.footprint/v1 on stdout; the producer role",
    )
    arguments = parser.parse_args(argv[1:])
    try:
        document = collect()
    except (ProducerError, OSError) as error:
        print(f"FOOTPRINT_UNAVAILABLE: {error}", file=sys.stderr)
        return 2
    if arguments.json:
        print(json.dumps(document, indent=2, sort_keys=True))
        return 0
    for row in document["entries"]:
        print(
            f"{row['symbol']}: {row['outcome']} measured={row.get('measured')} "
            f"minimum={row.get('minimum')} limit={row.get('limit')}"
        )
    failures = [row for row in document["entries"] if row["outcome"] != "pass"]
    if failures:
        print(
            f"FOOTPRINT_GATE_FAILED: {len(failures)} of {len(document['entries'])} rows "
            "are not pass",
            file=sys.stderr,
        )
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv))
