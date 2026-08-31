#!/usr/bin/env python3
"""Run the mandatory Kani gate with an explicit unavailable status."""

from __future__ import annotations

import os
import pwd
import subprocess
import sys
from pathlib import Path

from check_kani_harnesses import validate_kani_success


# Implements: NFR-002
def main() -> int:
    trusted_home = Path(pwd.getpwuid(os.getuid()).pw_dir)
    cargo = trusted_home / ".cargo" / "bin" / "cargo"
    cargo_kani = trusted_home / ".cargo" / "bin" / "cargo-kani"
    if not cargo.is_file() or not cargo_kani.is_file():
        print(
            "KANI_STATUS=unavailable; trusted cargo and cargo-kani are required for this gate",
            file=sys.stderr,
        )
        return 2
    completed = subprocess.run(
        [str(cargo), "kani"],
        check=False,
        capture_output=True,
        text=True,
        env={
            **os.environ,
            "HOME": str(trusted_home),
            "CARGO_HOME": str(trusted_home / ".cargo"),
            "RUSTUP_HOME": str(trusted_home / ".rustup"),
            "CARGO_TARGET_DIR": str(Path(__file__).resolve().parent.parent / "target"),
        },
    )
    sys.stdout.write(completed.stdout)
    sys.stderr.write(completed.stderr)
    if completed.returncode != 0:
        return completed.returncode
    if not validate_kani_success(completed.stdout + "\n" + completed.stderr):
        print(
            "KANI_FAILED: successful process did not meet the harness and proof-obligation floors",
            file=sys.stderr,
        )
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
