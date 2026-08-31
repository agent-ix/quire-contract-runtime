#!/usr/bin/env python3
"""Enforce runtime traceability status despite the upstream header mismatch."""

from __future__ import annotations

import json
import os
import re
import subprocess
import sys
from pathlib import Path


ROOT = Path(os.environ.get("QUIRE_RUNTIME_REPO_ROOT", Path(__file__).resolve().parent.parent))
MATRIX = ROOT / "spec" / "test-matrix.md"
TRACE_ID = re.compile(r"\b(?:TC-\d{3}|(?:N?FR|StR)-\d{3}(?:-(?:AC|VC)-\d+)?)\b")
TEST_CITATION = re.compile(r"\b(?:TC|SUITE)-\d{3}\b")
FUNCTION = re.compile(r"(?m)^\s*(?:pub(?:\([^)]*\))?\s+)?(?:async\s+)?fn\s+([a-zA-Z0-9_]+)\s*\(")
ATTRIBUTE = re.compile(r"#\s*\[(.*?)\]", re.DOTALL)
EXPECTED_FUNCTIONAL_ROWS = 8


def rust_sources() -> list[Path]:
    excluded = {".git", "target", "evidence"}
    return sorted(
        path
        for path in ROOT.rglob("*.rs")
        if not any(part in excluded for part in path.relative_to(ROOT).parts)
    )


def ignored_functions(source: str) -> list[tuple[int, str, str]]:
    findings = []
    for function in FUNCTION.finditer(source):
        boundary = max(
            source.rfind("}", 0, function.start()),
            source.rfind(";", 0, function.start()),
        )
        prefix = source[boundary + 1 : function.start()]
        attributes = [match.group(1) for match in ATTRIBUTE.finditer(prefix)]
        ignored = any(
            re.match(r"\s*ignore\b", attribute)
            or (
                re.match(r"\s*cfg_attr\s*\(", attribute)
                and re.search(r"\bignore\b", attribute)
            )
            for attribute in attributes
        )
        trace_bearing = bool(TRACE_ID.search(prefix)) or bool(
            re.match(r"(?:tc|fr|nfr|str)_\d{3}", function.group(1), re.IGNORECASE)
        )
        if ignored and trace_bearing:
            line = source.count("\n", 0, function.start()) + 1
            findings.append((line, function.group(1), prefix))
    return findings


def ignored_trace_tests() -> list[str]:
    findings = []
    for path in rust_sources():
        source = path.read_text(encoding="utf-8")
        for line, name, prefix in ignored_functions(source):
            identifiers = sorted(set(TRACE_ID.findall(prefix)))
            label = ", ".join(identifiers) if identifiers else name
            findings.append(
                f"{path.relative_to(ROOT)}:{line}: ignored trace-bearing test {label}"
            )
    return findings


def functional_rows() -> list[dict[str, str]]:
    lines = MATRIX.read_text(encoding="utf-8").splitlines()
    try:
        start = lines.index("## Functional Requirement Coverage")
    except ValueError as error:
        raise ValueError("functional coverage section is absent") from error
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
        if len(columns) != 4:
            raise ValueError("functional coverage row must have exactly four columns")
        rows.append(
            {
                "requirement": columns[0],
                "criteria": columns[1],
                "tests": columns[2],
                "status": columns[3],
            }
        )
    if len(rows) != EXPECTED_FUNCTIONAL_ROWS:
        raise ValueError(
            f"functional coverage row census is {len(rows)}, expected {EXPECTED_FUNCTIONAL_ROWS}"
        )
    registry = {
        columns[0]
        for line in lines
        if line.startswith("|") and not line.startswith("|---")
        for columns in [[column.strip() for column in line.strip("|").split("|")]]
        if columns and re.fullmatch(r"(?:TC|SUITE)-\d{3}", columns[0])
    }
    for row in rows:
        citations = set(TEST_CITATION.findall(row["tests"]))
        if not citations:
            raise ValueError(
                f"complete functional row {row['requirement']} has no test citation"
            )
        unknown = citations - registry
        if unknown:
            raise ValueError(
                f"functional row {row['requirement']} cites unknown tests {sorted(unknown)}"
            )
    return rows


def functional_statuses() -> list[str]:
    return [row["status"] for row in functional_rows()]


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
        rows = functional_rows()
        statuses = [row["status"] for row in rows]
    except (OSError, ValueError) as error:
        print(f"COVERAGE_STATUS_CONTRADICTION: {error}", file=sys.stderr)
        return 2 if isinstance(error, OSError) else 1

    try:
        completed = subprocess.run(
            ["quire", "coverage", "--scope", ".", "--json", "--strict"],
            cwd=ROOT,
            check=False,
            capture_output=True,
            text=True,
        )
    except FileNotFoundError:
        print("COVERAGE_STATUS=unavailable; quire is required for this gate", file=sys.stderr)
        return 2
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
            "COVERAGE_STATUS_NOTICE: upstream status-column diagnostic is absent; "
            "the repository-owned integrity checks remain active",
            file=sys.stderr,
        )
    print(
        json.dumps(
            {
                "classification": "repository-owned Coverage Status compatibility check",
                "functionalRows": len(rows),
                "statusLies": 0,
                "totals": totals,
            },
            sort_keys=True,
        )
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
