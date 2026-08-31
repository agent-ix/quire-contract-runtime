#!/usr/bin/env python3
"""Fail closed unless the declared Kani harness census is present and traced."""

from __future__ import annotations

import os
import re
import sys
from pathlib import Path


ROOT = Path(os.environ.get("QUIRE_RUNTIME_REPO_ROOT", Path(__file__).resolve().parent.parent))
KANI_SOURCE = ROOT / "verification" / "kani.rs"
EXPECTED_KANI_HARNESSES = (
    "tc_001_public_model_preserves_provenance",
    "tc_002_boolean_truth_tables",
    "tc_003_campaign_accounting_saturates",
    "tc_003_checked_i8_arithmetic_matches_primitives",
    "tc_003_i32_division_boundaries_are_undefined",
    "tc_003_option_helpers_preserve_definedness",
    "tc_003_slice_index_is_defined_exactly_in_bounds",
)
EXPECTED_KANI_CHECK_FLOORS = {
    "tc_001_public_model_preserves_provenance": 140,
    "tc_002_boolean_truth_tables": 136,
    "tc_003_campaign_accounting_saturates": 264,
    "tc_003_checked_i8_arithmetic_matches_primitives": 59,
    "tc_003_i32_division_boundaries_are_undefined": 43,
    "tc_003_option_helpers_preserve_definedness": 52,
    "tc_003_slice_index_is_defined_exactly_in_bounds": 24,
}
EXPECTED_KANI_VERSION = "Kani Rust Verifier 0.67.0 (cargo plugin)"
KANI_HARNESS_START = re.compile(
    r"(?m)^Checking harness kani_proofs::([a-z0-9_]+)\.\.\.$"
)
KANI_CHECK_SUMMARY = re.compile(r"(?m)^ \*\* 0 of ([1-9][0-9]*) failed$")
PROOF_FUNCTION = re.compile(
    r"(?m)^// Implements: (TC-\d{3})\n#\[kani::proof\]\nfn ([a-z0-9_]+)\(\)"
)


def proof_check_counts(combined: str) -> dict[str, int]:
    """Return each successfully discharged harness's positive obligation count."""
    starts = list(KANI_HARNESS_START.finditer(combined))
    counts: dict[str, int] = {}
    for index, start in enumerate(starts):
        end = starts[index + 1].start() if index + 1 < len(starts) else len(combined)
        block = combined[start.end() : end]
        summary = KANI_CHECK_SUMMARY.search(block)
        if (
            summary is None
            or "VERIFICATION:- SUCCESSFUL" not in block
            or start.group(1) in counts
        ):
            return {}
        counts[start.group(1)] = int(summary.group(1))
    return counts


def validate_kani_success(combined: str) -> bool:
    """Require the exact harness census and positive obligation floors."""
    expected_summary = (
        f"Complete - {len(EXPECTED_KANI_HARNESSES)} successfully verified harnesses, "
        f"0 failures, {len(EXPECTED_KANI_HARNESSES)} total."
    )
    checks = proof_check_counts(combined)
    return (
        EXPECTED_KANI_VERSION in combined
        and expected_summary in combined
        and set(checks) == set(EXPECTED_KANI_HARNESSES)
        and all(
            checks[name] >= EXPECTED_KANI_CHECK_FLOORS[name]
            for name in EXPECTED_KANI_HARNESSES
        )
    )


# Implements: NFR-002
def main() -> int:
    try:
        source = KANI_SOURCE.read_text(encoding="utf-8")
    except OSError as error:
        print(f"KANI_CENSUS_STATUS=unavailable; cannot read {KANI_SOURCE}: {error}", file=sys.stderr)
        return 2
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
    if set(EXPECTED_KANI_CHECK_FLOORS) != set(EXPECTED_KANI_HARNESSES):
        print("KANI_CENSUS_FAILED: proof check floors do not match harness census", file=sys.stderr)
        return 1
    for trace_id, name in found:
        expected_trace = (
            "TC-001"
            if name.startswith("tc_001_")
            else "TC-002"
            if name.startswith("tc_002_")
            else "TC-003"
        )
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
