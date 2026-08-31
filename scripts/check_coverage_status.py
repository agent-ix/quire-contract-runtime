#!/usr/bin/env python3
"""Enforce runtime traceability status despite the upstream header mismatch."""

from __future__ import annotations

import json
import re
import subprocess
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parent.parent
MATRIX = ROOT / "spec" / "test-matrix.md"
TRACE_ID = re.compile(r"\b(?:TC-\d{3}|(?:N?FR|StR)-\d{3}(?:-(?:AC|VC)-\d+)?)\b")


def ignored_trace_tests() -> list[str]:
    findings = []
    for root_name in ("src", "tests", "verification", "examples"):
        root = ROOT / root_name
        if not root.exists():
            continue
        for path in sorted(root.rglob("*.rs")):
            lines = path.read_text(encoding="utf-8").splitlines()
            for index, line in enumerate(lines):
                if "#[ignore" not in line:
                    continue
                context = "\n".join(lines[max(0, index - 10) : index + 8])
                identifiers = sorted(set(TRACE_ID.findall(context)))
                if identifiers:
                    findings.append(
                        f"{path.relative_to(ROOT)}:{index + 1}: ignored trace-bearing test "
                        + ", ".join(identifiers)
                    )
    return findings


def functional_statuses() -> list[str]:
    lines = MATRIX.read_text(encoding="utf-8").splitlines()
    start = lines.index("## Functional Requirement Coverage")
    rows = []
    for line in lines[start + 1 :]:
        if line.startswith("## "):
            break
        if not line.startswith("|") or line.startswith("|---"):
            continue
        columns = [column.strip() for column in line.strip("|").split("|")]
        if columns[0] == "Functional Req":
            if columns[-1] != "Coverage Status":
                raise ValueError("functional coverage table must end with Coverage Status")
            continue
        rows.append(columns[-1])
    if not rows:
        raise ValueError("functional coverage table has no rows")
    return rows


def coverage_contradictions(statuses: list[str], report: dict[str, object]) -> list[str]:
    findings = []
    unbacked = report.get("unbacked_rows", [])
    status_lies = report.get("status_lies", [])
    totals = report.get("totals", {})
    if not isinstance(totals, dict):
        return ["coverage report totals are absent or malformed"]
    if unbacked or status_lies or totals.get("backed") != totals.get("total"):
        findings.append("coverage rows are not completely backed")
    incomplete = [status for status in statuses if status != "✅ Complete"]
    if incomplete:
        findings.append(
            "fully backed functional rows claim " + ", ".join(sorted(set(incomplete)))
        )
    return findings


# Implements: NFR-002
def main() -> int:
    ignored = ignored_trace_tests()
    if ignored:
        for finding in ignored:
            print(f"COVERAGE_STATUS_CONTRADICTION: {finding}", file=sys.stderr)
        return 1
    try:
        statuses = functional_statuses()
    except ValueError as error:
        print(f"COVERAGE_STATUS_CONTRADICTION: {error}", file=sys.stderr)
        return 1

    completed = subprocess.run(
        ["quire", "coverage", "--scope", ".", "--json", "--strict"],
        cwd=ROOT,
        check=False,
        capture_output=True,
        text=True,
    )
    if completed.returncode != 0:
        sys.stdout.write(completed.stdout)
        sys.stderr.write(completed.stderr)
        return completed.returncode
    try:
        report = json.loads(completed.stdout)
    except json.JSONDecodeError as error:
        print(f"COVERAGE_STATUS_CONTRADICTION: invalid JSON report: {error}", file=sys.stderr)
        return 1

    contradictions = coverage_contradictions(statuses, report)
    if contradictions:
        for contradiction in contradictions:
            print(f"COVERAGE_STATUS_CONTRADICTION: {contradiction}", file=sys.stderr)
        return 1
    totals = report.get("totals", {})
    diagnostics = {
        item.get("reason") for item in report.get("diagnostics", []) if isinstance(item, dict)
    }
    if "status-column-matches-nothing" not in diagnostics:
        print(
            "COVERAGE_STATUS_CONTRADICTION: expected upstream status-column diagnostic is absent; "
            "remove the local compatibility classifier",
            file=sys.stderr,
        )
        return 1
    print(
        json.dumps(
            {
                "classification": "repository-owned Coverage Status compatibility check",
                "functionalRows": len(statuses),
                "statusLies": 0,
                "totals": totals,
            },
            sort_keys=True,
        )
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
