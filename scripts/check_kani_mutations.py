#!/usr/bin/env python3
"""Require representative semantic defects to be rejected by the Kani proofs."""

from __future__ import annotations

import os
import pwd
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path

from evidence_policy import MINIMUM_KANI_MUTATIONS

ROOT = Path(__file__).resolve().parent.parent
MUTATIONS = (
    (
        "src/operators.rs",
        "    left.checked_add(right)\n",
        "    None\n",
        "tc_003_checked_i8_arithmetic_matches_primitives",
    ),
    (
        "src/operators.rs",
        "    left && right()\n",
        "    false\n",
        "tc_002_boolean_truth_tables",
    ),
    (
        "src/accounting.rs",
        "VerdictKind::Passed => self.accepted = self.accepted.saturating_add(1),",
        "VerdictKind::Passed => {},",
        "tc_003_campaign_accounting_saturates",
    ),
)


def copy_candidate(destination: Path) -> None:
    shutil.copytree(
        ROOT,
        destination,
        ignore=shutil.ignore_patterns(".git", "evidence", "target", "__pycache__"),
    )


def run_mutation(relative: str, old: str, new: str, harness: str) -> str | None:
    home = Path(pwd.getpwuid(os.getuid()).pw_dir)
    cargo = home / ".cargo" / "bin" / "cargo"
    if not cargo.is_file() or not (home / ".cargo" / "bin" / "cargo-kani").is_file():
        raise FileNotFoundError("trusted cargo-kani toolchain is unavailable")
    with tempfile.TemporaryDirectory() as directory:
        candidate = Path(directory) / "candidate"
        copy_candidate(candidate)
        source = candidate / relative
        text = source.read_text(encoding="utf-8")
        if text.count(old) != 1:
            return f"mutation anchor drift for {relative}: {old!r}"
        source.write_text(text.replace(old, new), encoding="utf-8")
        environment = dict(os.environ)
        environment.update(
            HOME=str(home),
            CARGO_HOME=str(home / ".cargo"),
            RUSTUP_HOME=str(home / ".rustup"),
            CARGO_TARGET_DIR=str(candidate / "target"),
        )
        completed = subprocess.run(
            [str(cargo), "kani", "--harness", harness],
            cwd=candidate,
            env=environment,
            check=False,
            capture_output=True,
            text=True,
        )
    combined = completed.stdout + "\n" + completed.stderr
    if completed.returncode == 0:
        return f"Kani accepted the injected defect for {harness}"
    if f"Checking harness kani_proofs::{harness}..." not in combined or "VERIFICATION:- FAILED" not in combined:
        return f"mutation for {harness} did not reach a proof failure"
    return None


# Implements: NFR-002
def main() -> int:
    if len(MUTATIONS) < MINIMUM_KANI_MUTATIONS:
        print(
            f"KANI_MUTATION_FAILED: configured {len(MUTATIONS)} mutations, "
            f"minimum {MINIMUM_KANI_MUTATIONS}",
            file=sys.stderr,
        )
        return 1
    try:
        errors = [
            error
            for mutation in MUTATIONS
            if (error := run_mutation(*mutation)) is not None
        ]
    except (FileNotFoundError, OSError) as error:
        print(f"KANI_MUTATION_STATUS=unavailable; {error}", file=sys.stderr)
        return 2
    for error in errors:
        print(f"KANI_MUTATION_FAILED: {error}", file=sys.stderr)
    if errors:
        return 1
    print(f"verified {len(MUTATIONS)} Kani semantic mutation controls")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
