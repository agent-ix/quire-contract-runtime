#!/usr/bin/env python3
"""Bind AA-001's automated sufficiency boundary to authoritative evidence."""

from __future__ import annotations

import json
import re
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parent.parent
SCRIPTS = ROOT / "scripts"
if str(SCRIPTS) not in sys.path:
    sys.path.insert(0, str(SCRIPTS))

import verify_evidence as verifier


ARGUMENT = ROOT / "spec" / "assurance" / "AA-001-runtime-argument.md"


def declared_integer(text: str, field: str) -> int:
    match = re.search(rf"(?m)^  {re.escape(field)}: ([0-9]+)$", text)
    if match is None:
        raise ValueError(f"AA-001 does not declare {field}")
    return int(match.group(1))


def declared_text(text: str, field: str) -> str:
    match = re.search(rf"(?m)^  {re.escape(field)}: ([A-Za-z0-9/._-]+)$", text)
    if match is None:
        raise ValueError(f"AA-001 does not declare {field}")
    return match.group(1)


# Implements: NFR-002
def main() -> int:
    try:
        text = ARGUMENT.read_text(encoding="utf-8")
        record_count = declared_integer(text, "authoritative_records")
        outcome_count = declared_integer(text, "outcomes")
        required_status = declared_text(text, "required_result")
        anchor = declared_text(text, "anchor")
        if anchor != "evidence/ANCHORS":
            raise ValueError(f"AA-001 names unexpected anchor {anchor}")
        if declared_text(text, "record_selection") != "evidence/README.md":
            raise ValueError("AA-001 does not bind authoritative record selection")
        if declared_text(text, "checksum_binding") != "sha256sums.txt":
            raise ValueError("AA-001 does not bind the record checksum census")
        if declared_text(text, "history_anchor") != "evidence/HISTORY":
            raise ValueError("AA-001 does not bind per-record historical anchors")
        records = verifier.verify_anchors()
        verification_status = json.loads(
            verifier.VERIFICATION_STATUS.read_text(encoding="utf-8")
        )
        if verification_status.get("status") != "passed" or verification_status.get(
            "exitCode"
        ) != 0:
            raise ValueError("the immediately preceding evidence verification did not pass")
        if len(records) != record_count:
            raise ValueError(
                f"AA-001 expects {record_count} authoritative record, found {len(records)}"
            )
        record = records[0]
        verifier.verify_checksums(record)
        manifest = json.loads((record / "evidence-manifest.json").read_text(encoding="utf-8"))
        envelope = json.loads((record / "evidence-envelope.json").read_text(encoding="utf-8"))
        if envelope["result"]["status"] != required_status:
            raise ValueError(
                f"AA-001 requires {required_status}, record reports "
                f"{envelope['result']['status']}"
            )
        if len(manifest["outcomes"]) != outcome_count:
            raise ValueError(
                f"AA-001 expects {outcome_count} outcomes, found {len(manifest['outcomes'])}"
            )
    except (OSError, ValueError, KeyError, json.JSONDecodeError) as error:
        print(f"ASSURANCE_ANCHOR_FAILED: {error}", file=sys.stderr)
        return 1
    print(
        f"AA-001 binds {record.name}: {outcome_count} outcomes, result {required_status}"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
