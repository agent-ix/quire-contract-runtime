#!/usr/bin/env python3
"""Run the Kani proof suite and publish its result as a declared structured document.

This is the one place in the repository where a transcript is parsed, and the
reason is specific rather than habitual: `cargo kani` publishes no machine-readable
result document. It prints a human transcript and returns a process status. The
migration contract forbids recovering a verdict from stdout *when structured
output exists*; for Kani it does not exist, so the parsing has to live somewhere.

It lives here, in the domain tool that owns Kani, and it stops here. What leaves
this file is `runtime.kani-proof/v1`: one row per declared harness, each carrying
its own outcome and the number of proof obligations it actually discharged. Every
downstream consumer reads a field. Quoin never sees the transcript and never runs
Kani; `scripts/assurance_chain.py` reads this document and attests what it says.

Five outcomes, kept apart on purpose:

  pass          the harness was checked, verification succeeded, and it discharged
                at least its declared obligation floor
  vacuous       verification succeeded but the harness discharged fewer obligations
                than its floor, which is a proof that simplified away rather than a
                proof that held
  fail          the harness was checked and verification did not succeed
  not-computed  a declared harness the transcript never mentions
  unavailable   cargo-kani is not installed, so nothing was checked at all

`unavailable` is not `pass` and it is not an empty document. A machine without
Kani produces a document that says so, and the gate exits non-zero. Six review
rounds on the deleted collector were spent learning that "could not check" and
"checked and passed" must never reach the same exit code.

Exit status
  gate mode : 0 when every declared harness passed, 1 otherwise (including
              unavailable and vacuous), 2 when the repository itself is unreadable
  --json    : 0 whenever a document was produced, whatever it says; 2 when no
              document could be produced at all
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

from check_kani_harnesses import (
    EXPECTED_KANI_CHECK_FLOORS,
    EXPECTED_KANI_HARNESSES,
    EXPECTED_KANI_VERSION,
    declared_harness_traces,
    proof_check_counts,
    validate_kani_success,
)

ROOT = Path(__file__).resolve().parent.parent
PROTOCOL = "runtime.kani-proof/v1"


class ProducerError(RuntimeError):
    """No result document could be produced at all."""


def trusted_paths() -> tuple[Path, Path, Path]:
    home = Path(pwd.getpwuid(os.getuid()).pw_dir)
    return home, home / ".cargo" / "bin" / "cargo", home / ".cargo" / "bin" / "cargo-kani"


def kani_version(cargo_kani: Path, home: Path) -> str | None:
    """Observe the Kani version, or report that it could not be observed.

    `None` is the answer when the probe failed. It is never replaced with a
    plausible-looking default: a fabricated version in a sealed attestation is
    worse than an absent one, because a reader cannot tell it from a measurement.
    """
    try:
        result = subprocess.run(
            [str(cargo_kani), "--version"],
            capture_output=True,
            text=True,
            check=False,
            env={**os.environ, "HOME": str(home), "CARGO_HOME": str(home / ".cargo")},
        )
    except OSError:
        return None
    if result.returncode != 0:
        return None
    return result.stdout.strip() or None


def run_kani(stream: bool) -> tuple[int, str]:
    """Execute `cargo kani` under the trusted toolchain and capture its transcript."""
    home, cargo, cargo_kani = trusted_paths()
    process = subprocess.Popen(
        [str(cargo), "kani"],
        stdout=subprocess.PIPE,
        stderr=subprocess.STDOUT,
        text=True,
        bufsize=1,
        env={
            **os.environ,
            "HOME": str(home),
            "CARGO_HOME": str(home / ".cargo"),
            "RUSTUP_HOME": str(home / ".rustup"),
            "CARGO_TARGET_DIR": str(ROOT / "target"),
        },
    )
    captured: list[str] = []
    assert process.stdout is not None
    for line in process.stdout:
        if stream:
            sys.stdout.write(line)
            sys.stdout.flush()
        captured.append(line)
    return process.wait(), "".join(captured)


def unavailable_document(reason: str, traces: dict[str, list[str]]) -> dict[str, Any]:
    """One row per declared harness, every one of them explicitly not checked."""
    return {
        "protocol": PROTOCOL,
        "tool": {"identity": "cargo-kani", "version": None},
        "expected_version": EXPECTED_KANI_VERSION,
        "reason": reason,
        "entries": [
            {
                "symbol": f"kani_proofs::{name}",
                "outcome": "unavailable",
                "traceIds": traces.get(name, []),
                "dischargedObligations": None,
                "floor": EXPECTED_KANI_CHECK_FLOORS[name],
            }
            for name in EXPECTED_KANI_HARNESSES
        ],
    }


def collect(stream: bool = False) -> dict[str, Any]:
    """Produce the result document. Never raises for a failing proof."""
    try:
        traces = declared_harness_traces()
    except OSError as error:
        raise ProducerError(f"cannot read the declared harness census: {error}") from error

    home, cargo, cargo_kani = trusted_paths()
    if not cargo.is_file() or not cargo_kani.is_file():
        return unavailable_document(
            "the trusted cargo and cargo-kani executables are required and one is absent",
            traces,
        )

    status, transcript = run_kani(stream)
    counts = proof_check_counts(transcript)
    version = kani_version(cargo_kani, home)
    summary_ok = validate_kani_success(transcript)

    entries = []
    for name in EXPECTED_KANI_HARNESSES:
        floor = EXPECTED_KANI_CHECK_FLOORS[name]
        discharged = counts.get(name)
        if discharged is None:
            # The transcript never reported this harness as checked and
            # successful. That is not a failing proof and it is certainly not a
            # passing one; nothing was computed for it.
            outcome = "fail" if status != 0 else "not-computed"
        elif discharged < floor:
            # It verified, but on fewer obligations than the floor this
            # repository declared for it. A proof that simplified away is
            # vacuous, and vacuous is not passed.
            outcome = "vacuous"
        else:
            outcome = "pass"
        entries.append(
            {
                "symbol": f"kani_proofs::{name}",
                "outcome": outcome,
                "traceIds": traces.get(name, []),
                "dischargedObligations": discharged,
                "floor": floor,
            }
        )

    # The suite-level census is its own row. A run in which every named harness
    # passed but the transcript's own completion summary disagrees — a harness
    # that ran under a different name, an extra failure line, a version that is
    # not the pinned one — must not read as clean.
    entries.append(
        {
            "symbol": "kani_proofs::suite-census",
            "outcome": "pass" if (status == 0 and summary_ok) else "fail",
            "traceIds": ["NFR-002"],
            "dischargedObligations": len(counts),
            "floor": len(EXPECTED_KANI_HARNESSES),
        }
    )
    return {
        "protocol": PROTOCOL,
        "tool": {"identity": "cargo-kani", "version": version},
        "expected_version": EXPECTED_KANI_VERSION,
        "process_status": status,
        "entries": entries,
    }


def main(argv: list[str]) -> int:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument(
        "--json",
        action="store_true",
        help="emit runtime.kani-proof/v1 on stdout; the producer role",
    )
    arguments = parser.parse_args(argv[1:])

    try:
        document = collect(stream=not arguments.json)
    except ProducerError as error:
        print(str(error), file=sys.stderr)
        return 2

    if arguments.json:
        print(json.dumps(document, indent=2, sort_keys=True))
        return 0

    failures = [row for row in document["entries"] if row["outcome"] != "pass"]
    for row in failures:
        print(
            f"KANI_GATE {row['outcome'].upper()}: {row['symbol']} "
            f"discharged={row['dischargedObligations']} floor={row['floor']}",
            file=sys.stderr,
        )
    if failures:
        print(
            f"KANI_GATE_FAILED: {len(failures)} of {len(document['entries'])} rows "
            "are not pass. An absent toolchain is one of these rows and is not a skip.",
            file=sys.stderr,
        )
        return 1
    print(
        f"verified {len(EXPECTED_KANI_HARNESSES)} Kani harnesses above their "
        f"declared obligation floors with {document['tool']['version']}"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv))
