#!/usr/bin/env python3
"""Require representative semantic defects to be rejected by the Kani proofs.

A proof that has never been observed to fail is indistinguishable from a proof
that cannot fail. This is the campaign that makes the difference observable: each
declared defect is injected into a scratch copy of the source — never into the
working tree — and the harness that owns it must reject it.

The result is published as `runtime.kani-mutation/v1`. A row is `pass` when the
proof rejected the defect, which is the outcome that means the control held.

Four outcomes, kept apart:

  pass          the owning harness rejected the injected defect
  fail          the harness accepted it, so that harness proves less than it claims
  malformed     the mutation's anchor text is no longer in the source exactly once,
                so the campaign no longer describes this repository
  unavailable   cargo-kani is not installed, so nothing was injected at all

Exit status
  gate mode : 0 when every declared defect was rejected, 1 otherwise
  --json    : 0 whenever a document was produced, 2 when none could be
"""

from __future__ import annotations

import argparse
import json
import os
import pwd
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path
from typing import Any

ROOT = Path(__file__).resolve().parent.parent
PROTOCOL = "runtime.kani-mutation/v1"

# The floor is stated here, beside the campaign it governs, rather than imported
# from a shared policy module. The module it used to come from existed to give
# the deleted evidence framework a single place to state quantitative floors;
# there is one campaign and it owns its own floor.
MINIMUM_KANI_MUTATIONS = 3

MUTATIONS = (
    (
        "src/operators.rs",
        "    left.checked_add(right)\n",
        "    None\n",
        "tc_003_checked_i8_arithmetic_matches_primitives",
        "TC-003",
    ),
    (
        "src/operators.rs",
        "    left && right()\n",
        "    false\n",
        "tc_002_boolean_truth_tables",
        "TC-002",
    ),
    (
        "src/accounting.rs",
        "VerdictKind::Passed => self.accepted = self.accepted.saturating_add(1),",
        "VerdictKind::Passed => {},",
        "tc_003_campaign_accounting_saturates",
        "TC-003",
    ),
)


class ProducerError(RuntimeError):
    """No result document could be produced at all."""


def copy_candidate(destination: Path) -> None:
    shutil.copytree(
        ROOT,
        destination,
        ignore=shutil.ignore_patterns(
            ".git", "evidence", "target", "__pycache__", ".venv-assurance"
        ),
    )


def prove(argv: list[str], cwd: Path, environment: dict[str, str]) -> subprocess.CompletedProcess:
    """Run the model checker.

    This is a named seam, and the reason it exists is a finding. A campaign that
    injects defects can itself be hollowed out to report success without running
    anything, and nothing downstream would tell the difference: an all-`pass`
    document is what a working campaign also produces. The seam lets a test
    supply a prover that accepts the defect and require this module to report
    `fail`, which is the only way the failure direction is ever exercised.
    """
    return subprocess.run(
        argv, cwd=cwd, env=environment, check=False, capture_output=True, text=True
    )


def run_mutation(relative: str, old: str, new: str, harness: str) -> tuple[str, str | None]:
    """Inject one defect into a scratch copy and report what the proof did."""
    home = Path(pwd.getpwuid(os.getuid()).pw_dir)
    cargo = home / ".cargo" / "bin" / "cargo"
    with tempfile.TemporaryDirectory() as directory:
        candidate = Path(directory) / "candidate"
        copy_candidate(candidate)
        source = candidate / relative
        text = source.read_text(encoding="utf-8")
        if text.count(old) != 1:
            return "malformed", (
                f"mutation anchor for {relative} occurs {text.count(old)} times, not once; "
                "the campaign no longer describes this source"
            )
        source.write_text(text.replace(old, new), encoding="utf-8")
        environment = dict(os.environ)
        environment.update(
            HOME=str(home),
            CARGO_HOME=str(home / ".cargo"),
            RUSTUP_HOME=str(home / ".rustup"),
            CARGO_TARGET_DIR=str(candidate / "target"),
        )
        completed = prove(
            [str(cargo), "kani", "--harness", harness], candidate, environment
        )
    combined = completed.stdout + "\n" + completed.stderr
    if completed.returncode == 0:
        return "fail", f"Kani accepted the injected defect for {harness}"
    if (
        f"Checking harness kani_proofs::{harness}..." not in combined
        or "VERIFICATION:- FAILED" not in combined
    ):
        # A non-zero exit that never reached a verification failure is a broken
        # run, not a control that held. Counting it as a rejection is how a
        # campaign starts passing because the compiler fell over.
        return "fail", f"the mutation for {harness} did not reach a proof failure"
    return "pass", None


def observed_kani_version(home: Path) -> str | None:
    """The prover's own version, or None. Never a placeholder.

    A field named `version` carrying the word "observed" is worse than an absent
    one: a reader cannot tell it from a measurement.
    """
    try:
        result = subprocess.run(
            [str(home / ".cargo" / "bin" / "cargo-kani"), "--version"],
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


def collect() -> dict[str, Any]:
    if len(MUTATIONS) < MINIMUM_KANI_MUTATIONS:
        raise ProducerError(
            f"configured {len(MUTATIONS)} mutations, minimum {MINIMUM_KANI_MUTATIONS}"
        )
    home = Path(pwd.getpwuid(os.getuid()).pw_dir)
    have_kani = (home / ".cargo" / "bin" / "cargo").is_file() and (
        home / ".cargo" / "bin" / "cargo-kani"
    ).is_file()

    entries = []
    for relative, old, new, harness, trace in MUTATIONS:
        if not have_kani:
            outcome, detail = "unavailable", "the trusted cargo-kani toolchain is absent"
        else:
            outcome, detail = run_mutation(relative, old, new, harness)
        entries.append(
            {
                "symbol": f"mutation::{harness}::{relative}",
                "outcome": outcome,
                "traceIds": [trace],
                "detail": detail,
                "mutation": {"file": relative, "from": old, "to": new, "harness": harness},
            }
        )
    return {
        "protocol": PROTOCOL,
        "tool": {
            "identity": "cargo-kani",
            "version": observed_kani_version(home) if have_kani else None,
        },
        "minimum": MINIMUM_KANI_MUTATIONS,
        "entries": entries,
    }


def main(argv: list[str]) -> int:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument(
        "--json",
        action="store_true",
        help="emit runtime.kani-mutation/v1 on stdout; the producer role",
    )
    arguments = parser.parse_args(argv[1:])
    try:
        document = collect()
    except (ProducerError, OSError) as error:
        print(f"KANI_MUTATION_UNAVAILABLE: {error}", file=sys.stderr)
        return 2
    if arguments.json:
        print(json.dumps(document, indent=2, sort_keys=True))
        return 0
    failures = [row for row in document["entries"] if row["outcome"] != "pass"]
    for row in failures:
        print(f"KANI_MUTATION_{row['outcome'].upper()}: {row['detail']}", file=sys.stderr)
    if failures:
        return 1
    print(f"verified {len(document['entries'])} Kani semantic mutation controls")
    return 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv))
