#!/usr/bin/env python3
"""Verify the anchored runtime evidence set and re-derive its claims."""

from __future__ import annotations

import hashlib
import json
import re
import sys
from pathlib import Path
from typing import Any

from jsonschema import Draft7Validator

SCRIPTS = Path(__file__).resolve().parent
if str(SCRIPTS) not in sys.path:
    sys.path.insert(0, str(SCRIPTS))

from build_evidence_envelope import command_outcomes, summarize_outcomes
from validate_json_schema import checked_format_checker


ROOT = SCRIPTS.parent
EVIDENCE_ROOT = ROOT / "evidence"
ANCHORS = EVIDENCE_ROOT / "ANCHORS"
ENVELOPE_SCHEMA = ROOT / "schemas" / "pgm01-derivation-evidence-envelope-v1.schema.json"
MANIFEST_SCHEMA = ROOT / "schemas" / "runtime-evidence-manifest-v1.schema.json"
CHECKSUM_LINE = re.compile(r"^([0-9a-f]{64})  (.+)$")


class EvidenceError(ValueError):
    """Raised when retained evidence is incomplete or inconsistent."""


class VerificationUnavailable(EvidenceError):
    """Raised when the committed verification boundary is unavailable."""


def sha256_file(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def tree_digest(root: Path) -> str:
    state = hashlib.sha256()
    for path in sorted(root.rglob("*"), key=lambda item: item.as_posix()):
        if path.is_symlink():
            raise EvidenceError(f"symlink is not allowed in retained evidence: {path}")
        relative = path.relative_to(root).as_posix()
        if path.is_dir():
            kind = b"d"
        elif path.is_file():
            kind = b"f"
        else:
            raise EvidenceError(f"unsupported retained-evidence entry: {path}")
        state.update(kind + b"\0" + relative.encode("utf-8") + b"\0")
        if path.is_file():
            state.update(bytes.fromhex(sha256_file(path)))
        state.update(b"\0")
    return state.hexdigest()


def safe_root_path(value: str) -> Path:
    relative = Path(value)
    if relative.is_absolute() or not relative.parts or ".." in relative.parts:
        raise EvidenceError(f"unsafe evidence anchor path: {value!r}")
    path = ROOT / relative
    if not path.is_relative_to(EVIDENCE_ROOT):
        raise EvidenceError(f"anchor escapes evidence root: {value!r}")
    return path


def safe_record_path(record: Path, value: str) -> Path:
    relative = Path(value)
    if relative.is_absolute() or not relative.parts or ".." in relative.parts:
        raise EvidenceError(f"unsafe retained-evidence path: {value!r}")
    path = record / relative
    if path.parent != record:
        raise EvidenceError(f"nested authoritative path is not allowed: {value!r}")
    return path


def load_json(path: Path) -> dict[str, Any]:
    try:
        value = json.loads(path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as error:
        raise EvidenceError(f"cannot load {path}: {error}") from error
    if not isinstance(value, dict):
        raise EvidenceError(f"{path} must contain a JSON object")
    return value


def validate_json(instance: dict[str, Any], schema_path: Path, label: str) -> None:
    schema = load_json(schema_path)
    errors = sorted(
        Draft7Validator(schema, format_checker=checked_format_checker()).iter_errors(instance),
        key=lambda error: (list(error.absolute_path), error.message),
    )
    if errors:
        first = errors[0]
        location = "/".join(str(part) for part in first.absolute_path) or "<root>"
        raise EvidenceError(f"{label} schema violation at {location}: {first.message}")


def verify_checksums(record: Path) -> int:
    checksum_path = record / "sha256sums.txt"
    expected: dict[Path, str] = {}
    for line in checksum_path.read_text(encoding="utf-8").splitlines():
        match = CHECKSUM_LINE.fullmatch(line)
        if match is None:
            raise EvidenceError(f"invalid checksum line in {record.name}: {line!r}")
        path = safe_record_path(record, match.group(2))
        if path in expected:
            raise EvidenceError(f"duplicate checksum entry in {record.name}: {path.name}")
        expected[path] = match.group(1)
    actual = {path for path in record.iterdir() if path != checksum_path}
    if set(expected) != actual:
        unlisted = sorted(path.name for path in actual - set(expected))
        absent = sorted(path.name for path in set(expected) - actual)
        raise EvidenceError(
            f"checksum census mismatch in {record.name}: unlisted={unlisted}, absent={absent}"
        )
    for path, digest in expected.items():
        observed = sha256_file(path)
        if observed != digest:
            raise EvidenceError(
                f"checksum mismatch in {record.name}/{path.name}: expected {digest}, got {observed}"
            )
    return len(expected)


def verify_artifacts(record: Path, manifest: dict[str, Any]) -> int:
    seen: set[Path] = set()
    for artifact in manifest["artifacts"]:
        path = safe_record_path(record, artifact["path"])
        if path in seen or not path.is_file():
            raise EvidenceError(f"invalid manifest artifact in {record.name}: {path.name}")
        seen.add(path)
        if path.stat().st_size != artifact["size"] or sha256_file(path) != artifact["sha256"]:
            raise EvidenceError(f"manifest artifact mismatch in {record.name}: {path.name}")
    return len(seen)


def verify_envelope_links(record: Path, envelope: dict[str, Any]) -> None:
    for artifact in [*envelope["inputs"], *envelope["outputs"]]:
        uri = artifact["uri"]
        if "://" in uri:
            continue
        path = safe_record_path(record, uri)
        if not path.is_file() or sha256_file(path) != artifact["contentDigest"]["value"]:
            raise EvidenceError(f"envelope artifact mismatch in {record.name}: {path.name}")


def verify_anchors() -> list[Path]:
    if not ANCHORS.is_file():
        raise VerificationUnavailable("committed evidence/ANCHORS is missing")
    expected: dict[Path, str] = {}
    for line in ANCHORS.read_text(encoding="utf-8").splitlines():
        if not line or line.startswith("#"):
            continue
        match = CHECKSUM_LINE.fullmatch(line)
        if match is None:
            raise EvidenceError(f"invalid evidence anchor line: {line!r}")
        target = safe_root_path(match.group(2))
        if target in expected:
            raise EvidenceError(f"duplicate evidence anchor: {target.relative_to(ROOT)}")
        expected[target] = match.group(1)
    actual: set[Path] = set()
    for path in EVIDENCE_ROOT.iterdir():
        if path == ANCHORS:
            continue
        if path.is_dir() and path.name.startswith("runtime-v01-") and (
            path / "evidence-envelope.json"
        ).is_file():
            actual.add(path / "sha256sums.txt")
        else:
            actual.add(path)
    if set(expected) != actual:
        unanchored = sorted(str(path.relative_to(ROOT)) for path in actual - set(expected))
        absent = sorted(str(path.relative_to(ROOT)) for path in set(expected) - actual)
        raise EvidenceError(
            f"evidence anchor census mismatch: unanchored={unanchored}, absent={absent}"
        )
    for path, digest in expected.items():
        if not path.exists():
            raise EvidenceError(f"anchored evidence target is absent: {path.relative_to(ROOT)}")
        observed = tree_digest(path) if path.is_dir() else sha256_file(path)
        if observed != digest:
            raise EvidenceError(
                f"evidence anchor mismatch for {path.relative_to(ROOT)}: expected {digest}, got {observed}"
            )
    return sorted(
        path.parent
        for path in expected
        if path.name == "sha256sums.txt" and path.parent.name.startswith("runtime-v01-")
    )


# Implements: NFR-002
def verify_record(record: Path) -> tuple[int, int]:
    checksums = verify_checksums(record)
    manifest = load_json(record / "evidence-manifest.json")
    envelope = load_json(record / "evidence-envelope.json")
    validate_json(manifest, MANIFEST_SCHEMA, f"{record.name} manifest")
    validate_json(envelope, ENVELOPE_SCHEMA, f"{record.name} envelope")
    recorded_schema_digest = (record / "pgm01-schema-sha256.txt").read_text(
        encoding="utf-8"
    ).strip()
    if recorded_schema_digest != sha256_file(ENVELOPE_SCHEMA):
        raise EvidenceError(f"PGM-01 schema anchor mismatch in {record.name}")
    artifacts = verify_artifacts(record, manifest)
    verify_envelope_links(record, envelope)
    revision = (record / "source-revision.txt").read_text(encoding="utf-8").strip()
    identities = {
        revision,
        manifest["sourceRevision"],
        envelope["producer"]["sourceRevision"],
        envelope["provenance"]["sourceRevision"],
    }
    if len(identities) != 1:
        raise EvidenceError(f"source revision mismatch in {record.name}: {sorted(identities)}")
    derived = command_outcomes(record)
    if manifest["outcomes"] != derived:
        raise EvidenceError(
            f"outcome value mismatch in {record.name}: derived={derived}, declared={manifest['outcomes']}"
        )
    status, summary, limitations = summarize_outcomes(derived)
    if envelope["result"]["status"] != status or envelope["result"]["summary"] != summary:
        raise EvidenceError(f"derived result mismatch in {record.name}")
    if not set(limitations).issubset(set(manifest["limitations"])):
        raise EvidenceError(f"derived limitations missing in {record.name}")
    return checksums, artifacts


def main() -> int:
    try:
        records = verify_anchors()
        if not records:
            raise VerificationUnavailable("no authoritative records are anchored")
        totals = [verify_record(record) for record in records]
    except VerificationUnavailable as error:
        print(f"runtime evidence verification unavailable: {error}", file=sys.stderr)
        return 2
    except (EvidenceError, KeyError, OSError, TypeError) as error:
        print(f"runtime evidence verification failed: {error}", file=sys.stderr)
        return 1
    print(
        f"verified {len(records)} authoritative records, "
        f"{sum(item[0] for item in totals)} checksums, "
        f"{sum(item[1] for item in totals)} manifest artifacts"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
