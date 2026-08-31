#!/usr/bin/env python3
"""Fail closed unless the declared Kani harness census is present and traced."""

from __future__ import annotations

import re
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parent.parent
KANI_SOURCE = ROOT / "verification" / "kani.rs"
EXPECTED_KANI_HARNESSES = (
    "tc_002_boolean_truth_tables",
    "tc_003_campaign_accounting_saturates",
    "tc_003_checked_i8_arithmetic_matches_primitives",
    "tc_003_i32_division_boundaries_are_undefined",
    "tc_003_option_helpers_preserve_definedness",
    "tc_003_slice_index_is_defined_exactly_in_bounds",
)
PROOF_FUNCTION = re.compile(
    r"(?m)^// Implements: (TC-\d{3})\n#\[kani::proof\]\nfn ([a-z0-9_]+)\(\)"
)


# Implements: NFR-002
def main() -> int:
    source = KANI_SOURCE.read_text(encoding="utf-8")
    found = PROOF_FUNCTION.findall(source)
    names = tuple(sorted(name for _, name in found))
    if names != EXPECTED_KANI_HARNESSES:
        missing = sorted(set(EXPECTED_KANI_HARNESSES) - set(names))
        unexpected = sorted(set(names) - set(EXPECTED_KANI_HARNESSES))
        print(
            f"KANI_CENSUS_FAILED: missing={missing}, unexpected={unexpected}",
            file=sys.stderr,
        )
        return 1
    for trace_id, name in found:
        expected_trace = "TC-002" if name.startswith("tc_002_") else "TC-003"
        if trace_id != expected_trace:
            print(
                f"KANI_CENSUS_FAILED: {name} traces {trace_id}, expected {expected_trace}",
                file=sys.stderr,
            )
            return 1
    print(f"verified {len(names)} declared and trace-bound Kani harnesses")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
