#!/usr/bin/env python3
"""Verify the anchored runtime evidence set and re-derive its claims."""

from __future__ import annotations

import hashlib
import json
import re
import subprocess
import sys
from pathlib import Path
from typing import Any

from jsonschema import Draft7Validator

SCRIPTS = Path(__file__).resolve().parent
if str(SCRIPTS) not in sys.path:
    sys.path.insert(0, str(SCRIPTS))

from build_evidence_envelope import (
    COMMAND_TRANSCRIPTS,
    EXPECTED_KANI_CHECK_FLOORS,
    EXPECTED_KANI_HARNESSES,
    command_outcomes,
    kani_proof_checks,
    summarize_outcomes,
)
from validate_json_schema import checked_format_checker


ROOT = SCRIPTS.parent
EVIDENCE_ROOT = ROOT / "evidence"
ANCHORS = EVIDENCE_ROOT / "ANCHORS"
ENVELOPE_SCHEMA = ROOT / "schemas" / "pgm01-derivation-evidence-envelope-v1.schema.json"
MANIFEST_SCHEMA = ROOT / "schemas" / "runtime-evidence-manifest-v1.schema.json"
VERIFICATION_STATUS = ROOT / "target" / "evidence-verification-status.json"
CHECKSUM_LINE = re.compile(r"^([0-9a-f]{64})  (.+)$")
REVISION = re.compile(r"^[0-9a-f]{40}$")
AUTHORITATIVE_RECORD = re.compile(r"runtime-v01-[0-9a-f]{12}-[0-9]{8}T[0-9]{6}Z")
MINIMUM_HISTORICAL_RECORDS = 29
REQUIRED_HISTORICAL_DIRECTORIES = {
    "retired-pre-head-binding",
    "retired-pre-verifier",
}
PARAMETER_PATHS = (
    "Cargo.toml",
    "Cargo.lock",
    "Makefile",
    "rust-toolchain.toml",
    "measurement/footprint/Cargo.toml",
    "measurement/footprint/src/lib.rs",
    "scripts/check_linked_footprint.sh",
    "scripts/measure_rlib_size.sh",
    "scripts/collect_evidence.sh",
    "scripts/build_evidence_envelope.py",
    "scripts/validate_json_schema.py",
    "scripts/verify_evidence.py",
    "scripts/update_evidence_anchors.py",
    "scripts/check_coverage_status.py",
    "scripts/check_kani_harnesses.py",
    "scripts/check_failure_propagation.py",
    "scripts/run_kani_gate.py",
    "scripts/check_assurance_anchor.py",
    "tests/test_evidence_tooling.py",
    "spec/test-matrix.md",
    "spec/assurance/AA-001-runtime-argument.md",
    "spec/nonfunctional/NFR-002-panic-compatibility-license.md",
    "requirements-evidence.txt",
    "schemas/runtime-evidence-input-v1.schema.json",
    "schemas/runtime-evidence-manifest-v1.schema.json",
    "schemas/pgm01-derivation-evidence-envelope-v1.schema.json",
    "schemas/pgm01-merged-commit.txt",
)


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
    try:
        format_checker = checked_format_checker(schema)
    except RuntimeError as error:
        raise VerificationUnavailable(str(error)) from error
    errors = sorted(
        Draft7Validator(schema, format_checker=format_checker).iter_errors(instance),
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
        if path.is_symlink():
            raise EvidenceError(
                f"symlink is not allowed in retained evidence: {record.name}/{path.name}"
            )
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


def verify_record_identity(record: Path, envelope: dict[str, Any]) -> None:
    if envelope.get("recordId") != record.name or AUTHORITATIVE_RECORD.fullmatch(record.name) is None:
        raise EvidenceError(f"record identity mismatch in {record.name}")


def verify_conclusive_result(record: Path, envelope: dict[str, Any]) -> None:
    if envelope.get("result", {}).get("status") != "conclusive":
        raise EvidenceError(f"authoritative record is not conclusive: {record.name}")


def git_result(arguments: list[str]) -> subprocess.CompletedProcess[bytes]:
    return subprocess.run(
        ["git", *arguments], cwd=ROOT, check=False, capture_output=True
    )


def git_blob(revision: str, relative: str) -> bytes:
    result = git_result(["show", f"{revision}:{relative}"])
    if result.returncode != 0:
        raise EvidenceError(f"cannot read source-bound parameter {relative} at {revision}")
    return result.stdout


def source_parameter_digest(revision: str) -> str:
    state = hashlib.sha256()
    for relative in PARAMETER_PATHS:
        state.update(relative.encode("utf-8"))
        state.update(b"\0")
        state.update(git_blob(revision, relative))
        state.update(b"\0")
    return state.hexdigest()


def verify_source_binding(revision: str) -> None:
    if not REVISION.fullmatch(revision):
        raise EvidenceError(f"invalid source revision: {revision!r}")
    exists = git_result(["cat-file", "-e", f"{revision}^{{commit}}"])
    if exists.returncode != 0:
        raise EvidenceError(f"recorded source revision does not exist: {revision}")
    comparison = git_result(
        ["diff", "--quiet", revision, "HEAD", "--", ".", ":(exclude)evidence"]
    )
    if comparison.returncode == 1:
        raise EvidenceError(
            f"recorded source tree {revision} differs from current HEAD outside evidence/"
        )
    if comparison.returncode != 0:
        raise VerificationUnavailable("cannot compare the recorded source tree with HEAD")
    for arguments in (
        ["diff", "--quiet", "--", ".", ":(exclude)evidence"],
        ["diff", "--cached", "--quiet", "--", ".", ":(exclude)evidence"],
    ):
        worktree = git_result(arguments)
        if worktree.returncode == 1:
            raise EvidenceError("current worktree differs from HEAD outside evidence/")
        if worktree.returncode != 0:
            raise VerificationUnavailable("cannot compare the current worktree with HEAD")
    untracked = git_result(
        [
            "-c",
            "core.excludesFile=/dev/null",
            "ls-files",
            "--others",
            "--exclude-from=.gitignore",
            "-z",
        ]
    )
    if untracked.returncode != 0:
        raise VerificationUnavailable("cannot enumerate untracked worktree inputs")
    outside_evidence = sorted(
        value.decode("utf-8")
        for value in untracked.stdout.split(b"\0")
        if value and not value.decode("utf-8").startswith("evidence/")
    )
    if outside_evidence:
        raise EvidenceError(
            f"untracked worktree inputs exist outside evidence/: {outside_evidence}"
        )


def verify_outcome_census(record: Path, manifest: dict[str, Any]) -> None:
    declared = {transcript for _, transcript in COMMAND_TRANSCRIPTS}
    retained = {
        path.name.removesuffix(".status.txt")
        for path in record.glob("*.status.txt")
    }
    retained.update(
        path.name.removesuffix("-status.txt")
        for path in record.glob("*-status.txt")
        if not path.name.endswith(".status.txt")
        and path.read_text(encoding="utf-8").strip() == "skipped-unavailable"
    )
    recorded = [item.get("name") for item in manifest.get("outcomes", [])]
    if len(recorded) != len(set(recorded)):
        raise EvidenceError(f"duplicate declared outcome in {record.name}")
    if retained != declared or set(recorded) != declared:
        raise EvidenceError(
            f"outcome census mismatch in {record.name}: "
            f"retained={sorted(retained)}, declared={sorted(declared)}, "
            f"manifest={sorted(str(item) for item in recorded)}"
        )


def verify_anchors() -> list[Path]:
    if ANCHORS.is_symlink():
        raise EvidenceError("evidence/ANCHORS must not be a symlink")
    if not ANCHORS.is_file():
        raise VerificationUnavailable("committed evidence/ANCHORS is missing")
    history = EVIDENCE_ROOT / "historical"
    if not history.is_dir() or history.is_symlink():
        raise EvidenceError("retained evidence history is absent or unsafe")
    history_directories = {
        path.name for path in history.iterdir() if path.is_dir() and not path.is_symlink()
    }
    missing_history = REQUIRED_HISTORICAL_DIRECTORIES - history_directories
    historical_records = sum(
        1 for path in history.rglob("evidence-envelope.json") if path.is_file()
    )
    if missing_history or historical_records < MINIMUM_HISTORICAL_RECORDS:
        raise EvidenceError(
            "retained evidence history census regressed: "
            f"missing={sorted(missing_history)}, records={historical_records}, "
            f"minimum={MINIMUM_HISTORICAL_RECORDS}"
        )
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
        if path.is_dir() and path.name.startswith("runtime-v01-"):
            if not (path / "evidence-envelope.json").is_file():
                raise EvidenceError(f"incomplete authoritative evidence directory: {path.name}")
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
    records = sorted(
        path.parent
        for path in expected
        if path.name == "sha256sums.txt" and path.parent.name.startswith("runtime-v01-")
    )
    if not records:
        raise VerificationUnavailable("no authoritative records are anchored")
    if len(records) != 1:
        raise EvidenceError(f"expected exactly one authoritative record, found {len(records)}")
    readme = (EVIDENCE_ROOT / "README.md").read_text(encoding="utf-8")
    declared = re.search(r"current authoritative\s+record is `([^`]+)`", readme)
    if declared is None or declared.group(1) != records[0].name:
        raise EvidenceError("evidence README authoritative record does not match ANCHORS")
    return records


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
    verify_record_identity(record, envelope)
    revision = (record / "source-revision.txt").read_text(encoding="utf-8").strip()
    identities = {
        revision,
        manifest["sourceRevision"],
        envelope["producer"]["sourceRevision"],
        envelope["provenance"]["sourceRevision"],
    }
    if len(identities) != 1:
        raise EvidenceError(f"source revision mismatch in {record.name}: {sorted(identities)}")
    verify_source_binding(revision)
    if envelope["parametersDigest"]["value"] != source_parameter_digest(revision):
        raise EvidenceError(f"parameters digest mismatch in {record.name}")
    if envelope["producer"]["executableDigest"]["value"] != hashlib.sha256(
        git_blob(revision, "scripts/collect_evidence.sh")
    ).hexdigest():
        raise EvidenceError(f"collector executable digest mismatch in {record.name}")
    if envelope["environment"]["dependenciesDigest"]["value"] != hashlib.sha256(
        git_blob(revision, "Cargo.lock")
    ).hexdigest():
        raise EvidenceError(f"dependency lock digest mismatch in {record.name}")
    verify_outcome_census(record, manifest)
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
    verify_conclusive_result(record, envelope)
    combined_kani = (record / "kani.stdout").read_text(encoding="utf-8") + "\n" + (
        record / "kani.stderr"
    ).read_text(encoding="utf-8")
    checks = kani_proof_checks(combined_kani)
    declared_checks = {
        item["harness"]: item["checks"] for item in manifest["kaniProofChecks"]
    }
    if (
        checks != declared_checks
        or set(checks) != set(EXPECTED_KANI_HARNESSES)
        or any(
            checks[name] < EXPECTED_KANI_CHECK_FLOORS[name]
            for name in EXPECTED_KANI_HARNESSES
        )
    ):
        raise EvidenceError(f"Kani proof-obligation census mismatch in {record.name}")
    return checksums, artifacts


def write_verification_status(status: str, message: str, exit_code: int) -> None:
    VERIFICATION_STATUS.parent.mkdir(parents=True, exist_ok=True)
    VERIFICATION_STATUS.write_text(
        json.dumps(
            {"exitCode": exit_code, "message": message, "status": status},
            sort_keys=True,
        )
        + "\n",
        encoding="utf-8",
    )


def main() -> int:
    try:
        records = verify_anchors()
        if not records:
            raise VerificationUnavailable("no authoritative records are anchored")
        totals = [verify_record(record) for record in records]
    except VerificationUnavailable as error:
        message = f"runtime evidence verification unavailable: {error}"
        write_verification_status("unavailable", message, 2)
        print(message, file=sys.stderr)
        return 2
    except (EvidenceError, KeyError, OSError, RuntimeError, TypeError) as error:
        message = f"runtime evidence verification failed: {error}"
        write_verification_status("failed", message, 1)
        print(message, file=sys.stderr)
        return 1
    message = (
        f"verified {len(records)} authoritative records, "
        f"{sum(item[0] for item in totals)} checksums, "
        f"{sum(item[1] for item in totals)} manifest artifacts"
    )
    write_verification_status("passed", message, 0)
    print(message)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
