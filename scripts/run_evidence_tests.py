#!/usr/bin/env python3
"""Run the evidence-tool suite while enforcing its behavioral test census."""

from __future__ import annotations

import sys
import unittest
from pathlib import Path


ROOT = Path(__file__).resolve().parent.parent
MINIMUM_EVIDENCE_TESTS = 44


# Implements: NFR-002
def main() -> int:
    try:
        suite = unittest.defaultTestLoader.discover(
            str(ROOT / "tests"), pattern="*.py"
        )
    except (ImportError, OSError) as error:
        print(f"EVIDENCE_TEST_CENSUS_FAILED: cannot discover suite: {error}", file=sys.stderr)
        return 1
    count = suite.countTestCases()
    if count < MINIMUM_EVIDENCE_TESTS:
        print(
            f"EVIDENCE_TEST_CENSUS_FAILED: discovered {count}, "
            f"minimum {MINIMUM_EVIDENCE_TESTS}",
            file=sys.stderr,
        )
        return 1
    result = unittest.TextTestRunner(verbosity=1).run(suite)
    if result.testsRun != count:
        print(
            f"EVIDENCE_TEST_CENSUS_FAILED: ran {result.testsRun}, discovered {count}",
            file=sys.stderr,
        )
        return 1
    if not result.wasSuccessful():
        return 1
    print(f"verified {result.testsRun} evidence-tool behavioral tests")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
