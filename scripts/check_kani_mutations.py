#!/usr/bin/env python3
"""Require representative semantic defects to be rejected by the Kani proofs.

A proof that has never been observed to fail is indistinguishable from a proof
that cannot fail. This is the campaign that makes the difference observable: each
declared defect is injected into a scratch copy of the source — never into the
working tree — and the harness that owns it must reject it.

A row is `pass` when the proof rejected the defect, which is the outcome that
means the control held.

Five outcomes, kept apart:

  pass          the owning harness rejected the injected defect
  fail          the harness accepted it, so that harness proves less than it claims
  inconclusive  the run never reached a verification result, so nothing was proved
                either way
  malformed     the mutation's anchor text is no longer in the source exactly once,
                so the campaign no longer describes this repository
  unavailable   cargo-kani is not installed, so nothing was injected at all

Exit status: 0 when every declared defect was rejected, 1 otherwise.
"""

from __future__ import annotations

import os
import pwd
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path
from typing import Any

ROOT = Path(__file__).resolve().parent.parent

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
        "        R::from(false)\n",
        "        R::from(true)\n",
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
    # `tc_003_exact_ieee_numeric_equal_matches_nan_unordered` proves a property of `compare_ieee`,
    # which is `quire-exact`'s code since FR-275, so there is no source of it in this repository to
    # inject a defect into: its two former injections (NaN comparison and NaN decoding in
    # `src/exact/ieee.rs`) left with that file. The proof itself still runs under `make kani`.
)


def copy_candidate(destination: Path) -> None:
    shutil.copytree(
        ROOT,
        destination,
        ignore=shutil.ignore_patterns(".git", "target", "__pycache__"),
    )


def prove(argv: list[str], cwd: Path, environment: dict[str, str]) -> subprocess.CompletedProcess:
    """Run the model checker."""
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
            [str(cargo), "kani", "--harness", harness, "--features", "exact"],
            candidate,
            environment,
        )
    combined = completed.stdout + "\n" + completed.stderr
    if completed.returncode == 0:
        return "fail", f"Kani accepted the injected defect for {harness}"
    if (
        f"Checking harness kani_proofs::{harness}..." not in combined
        or "VERIFICATION:- FAILED" not in combined
    ):
        # A non-zero exit that never reached a verification failure is an
        # inconclusive run, not a control that held and not a harness that
        # accepted the defect. Counting it as either is how a campaign starts
        # passing (or blames the wrong harness) because the compiler fell over.
        #
        # The detail names what actually happened rather than describing it from
        # the proof's point of view: the exit status, which distinguishes a
        # compiler crash from a process `kill`, and a bounded tail of the
        # captured output, which is usually where the compiler said why.
        tail = " ".join(combined.split())[-200:]
        return "inconclusive", (
            f"cargo kani exited {completed.returncode} for {harness} without reaching a "
            f"verification result; output tail: {tail!r}"
        )
    return "pass", None


def collect() -> list[dict[str, Any]]:
    home = Path(pwd.getpwuid(os.getuid()).pw_dir)
    have_kani = (home / ".cargo" / "bin" / "cargo").is_file() and (
        home / ".cargo" / "bin" / "cargo-kani"
    ).is_file()

    entries = []
    for relative, old, new, harness, _trace in MUTATIONS:
        if not have_kani:
            outcome, detail = "unavailable", "the trusted cargo-kani toolchain is absent"
        else:
            outcome, detail = run_mutation(relative, old, new, harness)
        entries.append(
            {
                "symbol": f"mutation::{harness}::{relative}",
                "outcome": outcome,
                "detail": detail,
            }
        )
    return entries


def main() -> int:
    entries = collect()
    failures = [row for row in entries if row["outcome"] != "pass"]
    for row in failures:
        print(f"KANI_MUTATION_{row['outcome'].upper()}: {row['detail']}", file=sys.stderr)
    if failures:
        return 1
    print(f"verified {len(entries)} Kani semantic mutation controls")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
