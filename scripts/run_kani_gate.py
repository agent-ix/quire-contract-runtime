#!/usr/bin/env python3
"""Run the mandatory Kani gate with an explicit unavailable status."""

from __future__ import annotations

import shutil
import subprocess
import sys


# Implements: NFR-002
def main() -> int:
    if shutil.which("cargo-kani") is None:
        print(
            "KANI_STATUS=unavailable; cargo-kani is required for this gate",
            file=sys.stderr,
        )
        return 2
    return subprocess.run(["cargo", "kani"], check=False).returncode


if __name__ == "__main__":
    raise SystemExit(main())
