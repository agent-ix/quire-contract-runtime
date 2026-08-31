#!/usr/bin/env python3
"""Build the PGM-01 evidence input, manifest, and canonical envelope."""

from __future__ import annotations

import datetime as dt
import hashlib
import json
import platform
import sys
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parent.parent
PGM01_CANDIDATE_REVISION = "7dac9d8c19952412b56a0347387666e2ca81e01d"
PGM01_ENVELOPE_SCHEMA_DIGEST = (
    "0946e235e9e4b0fa79e9b9ec27ae157b303c17de0a9408d3cc04968fb7152256"
)
PGM01_ENVELOPE_SCHEMA = (
    ROOT / "schemas" / "pgm01-derivation-evidence-envelope-v1.schema.json"
)
PGM01_COMMIT_OBJECT = ROOT / "schemas" / "pgm01-merged-commit.txt"
INPUT_SCHEMA = ROOT / "schemas" / "runtime-evidence-input-v1.schema.json"
MANIFEST_SCHEMA = ROOT / "schemas" / "runtime-evidence-manifest-v1.schema.json"
COLLECTOR = ROOT / "scripts" / "collect_evidence.sh"
BUILDER = Path(__file__).resolve()
SCHEMA_VALIDATOR = ROOT / "scripts" / "validate_json_schema.py"
COMMAND_TRANSCRIPTS = (
    ("quire-validate", "quire-validate"),
    ("fmt", "fmt"),
    ("clippy", "clippy"),
    ("test-core", "test-core"),
    ("test-alloc", "test-alloc"),
    ("test-std", "test-std"),
    ("test-all", "test-all"),
    ("test-footprint", "test-footprint"),
    ("msrv", "msrv"),
    ("deny", "deny"),
    ("unsafe-audit", "unsafe-audit"),
    ("panic-audit", "panic-audit"),
    ("metadata", "metadata"),
    ("default-dependencies", "default-dependencies"),
    ("release-build", "release-build"),
    ("layout", "layout"),
    ("rustdoc", "rustdoc"),
    ("linked-footprint", "linked-footprint"),
    ("rlib-size-observation", "rlib-size-observation"),
    ("pgm01-pinned-schema", "pgm01-pinned-schema"),
    ("input-schema", "input-schema"),
    ("manifest-schema", "manifest-schema"),
    ("pgm01-schema", "pgm01-schema"),
    ("pgm01-envelope", "pgm01-envelope"),
)
VALIDATOR_TRANSCRIPTS = (
    "pgm01-pinned-schema",
    "input-schema",
    "manifest-schema",
    "pgm01-schema",
    "pgm01-envelope",
)
PASS_CONTRADICTION_MARKERS = {
    "clippy": ("error: could not compile",),
    "test-core": ("test result: FAILED", "error: test failed"),
    "test-alloc": ("test result: FAILED", "error: test failed"),
    "test-std": ("test result: FAILED", "error: test failed"),
    "test-all": ("test result: FAILED", "error: test failed"),
    "test-footprint": ("test result: FAILED", "error: test failed"),
    "msrv": ("error: could not compile",),
    "release-build": ("error: could not compile",),
}


def sha256_bytes(value: bytes) -> str:
    return hashlib.sha256(value).hexdigest()


def sha256_file(path: Path) -> str:
    return sha256_bytes(path.read_bytes())


def digest(value: str) -> dict[str, str]:
    return {"algorithm": "sha256", "value": value}


def write_json(path: Path, value: Any) -> None:
    path.write_text(json.dumps(value, indent=2, sort_keys=True) + "\n", encoding="utf-8")


def verified_pgm01_schema_digest() -> str:
    """Return the vendored PGM-01 schema digest, failing on pin drift."""
    actual = sha256_file(PGM01_ENVELOPE_SCHEMA)
    if actual != PGM01_ENVELOPE_SCHEMA_DIGEST:
        raise ValueError(
            "vendored PGM-01 envelope schema digest mismatch: "
            f"expected {PGM01_ENVELOPE_SCHEMA_DIGEST}, got {actual}"
        )
    return actual


def verified_pgm01_revision() -> str:
    """Verify that the vendored raw Git commit object has the pinned object identity."""
    # The repository text fixture carries a conventional terminal newline; the
    # upstream commit message does not, so canonicalize that transport detail.
    content = PGM01_COMMIT_OBJECT.read_bytes().removesuffix(b"\n")
    header = f"commit {len(content)}\0".encode()
    actual = hashlib.sha1(header + content, usedforsecurity=False).hexdigest()
    if actual != PGM01_CANDIDATE_REVISION:
        raise ValueError(
            "vendored PGM-01 commit identity mismatch: "
            f"expected {PGM01_CANDIDATE_REVISION}, got {actual}"
        )
    return actual


def command_outcomes(evidence_dir: Path, kani_status: str) -> list[dict[str, str]]:
    outcomes = []
    for name, transcript in COMMAND_TRANSCRIPTS:
        status_path = evidence_dir / f"{transcript}.status.txt"
        availability_path = evidence_dir / f"{transcript}-status.txt"
        availability = (
            availability_path.read_text(encoding="utf-8").strip()
            if availability_path.exists()
            else None
        )
        if availability == "skipped-unavailable":
            outcomes.append({"name": name, "status": availability})
            continue
        if not status_path.exists():
            status = "inconclusive"
        else:
            try:
                exit_status = int(status_path.read_text(encoding="utf-8").strip())
            except ValueError as error:
                raise ValueError(f"invalid exit status in {status_path}") from error
            if exit_status == 0:
                transcript_paths = (
                    evidence_dir / f"{transcript}.stdout",
                    evidence_dir / f"{transcript}.stderr",
                )
                if not all(path.exists() for path in transcript_paths):
                    status = "inconclusive"
                else:
                    combined = "\n".join(
                        path.read_text(encoding="utf-8", errors="replace")
                        for path in transcript_paths
                    )
                    contradiction = next(
                        (
                            marker
                            for marker in PASS_CONTRADICTION_MARKERS.get(name, ())
                            if marker in combined
                        ),
                        None,
                    )
                    if contradiction is not None:
                        raise ValueError(
                            f"passed status for {name} contradicts retained transcript: "
                            f"{contradiction}"
                        )
                    status = "passed"
            else:
                status = "failed"
        outcomes.append({"name": name, "status": status})
    outcomes.append({"name": "kani", "status": kani_status})
    return outcomes


def hash_parameter_files() -> str:
    paths = (
        ROOT / "Cargo.toml",
        ROOT / "Cargo.lock",
        ROOT / "Makefile",
        ROOT / "rust-toolchain.toml",
        ROOT / "measurement" / "footprint" / "Cargo.toml",
        ROOT / "measurement" / "footprint" / "src" / "lib.rs",
        ROOT / "scripts" / "check_linked_footprint.sh",
        ROOT / "scripts" / "measure_rlib_size.sh",
        COLLECTOR,
        BUILDER,
        SCHEMA_VALIDATOR,
        INPUT_SCHEMA,
        MANIFEST_SCHEMA,
        PGM01_ENVELOPE_SCHEMA,
        PGM01_COMMIT_OBJECT,
    )
    state = hashlib.sha256()
    for path in paths:
        state.update(str(path.relative_to(ROOT)).encode("utf-8"))
        state.update(b"\0")
        state.update(path.read_bytes())
        state.update(b"\0")
    return state.hexdigest()


# Implements: NFR-002
def build(evidence_dir: Path) -> None:
    pgm01_schema_digest = verified_pgm01_schema_digest()
    verified_pgm01_revision()
    evidence_dir = evidence_dir.resolve()
    invocation_directory = (
        str(evidence_dir.relative_to(ROOT))
        if evidence_dir.is_relative_to(ROOT)
        else str(evidence_dir)
    )
    revision = (evidence_dir / "source-revision.txt").read_text(encoding="utf-8").strip()
    source_state = (evidence_dir / "source-state.txt").read_text(encoding="utf-8").strip()
    kani_status = (evidence_dir / "kani-status.txt").read_text(encoding="utf-8").strip()
    metadata = json.loads((evidence_dir / "metadata.stdout").read_text(encoding="utf-8"))
    package = next(
        item for item in metadata["packages"] if item["name"] == "quire-contract-runtime"
    )
    recorded_at_path = evidence_dir / "recorded-at.txt"
    if recorded_at_path.exists():
        recorded_at = recorded_at_path.read_text(encoding="utf-8").strip()
    else:
        recorded_at = (
            dt.datetime.now(dt.timezone.utc)
            .replace(microsecond=0)
            .isoformat()
            .replace("+00:00", "Z")
        )
        recorded_at_path.write_text(recorded_at + "\n", encoding="utf-8")

    collection_input = {
        "schemaVersion": "quire.runtime-evidence-input/v1",
        "sourceRevision": revision,
        "sourceState": source_state,
        "commands": [
            "quire validate --scope . 'spec/**/*.md' 'planning/**/*.md' 'plan/**/*.md'",
            "python3 scripts/validate_json_schema.py schemas/runtime-evidence-input-v1.schema.json collection-input.json",
            "python3 scripts/validate_json_schema.py schemas/runtime-evidence-manifest-v1.schema.json evidence-manifest.json",
            "python3 scripts/validate_json_schema.py schemas/pgm01-derivation-evidence-envelope-v1.schema.json evidence-envelope.json",
            "python3 scripts/validate_json_schema.py $PGM01_SCHEMA evidence-envelope.json (when available)",
            "cargo fmt --all -- --check",
            "make lint",
            "cargo test --no-default-features",
            "cargo test --features alloc",
            "cargo test --features std",
            "cargo test --all-features",
            "cargo test -p quire-contract-runtime-footprint",
            "cargo +1.75.0 check --all-targets --all-features",
            "cargo deny check licenses",
            "bash scripts/check_unsafe_comments.sh",
            "bash scripts/check_panic_surface.sh",
            "cargo metadata --format-version 1 --no-default-features",
            "cargo tree --edges normal --no-default-features",
            "cargo build --release --lib --no-default-features",
            "make size",
            "bash scripts/measure_rlib_size.sh $CARGO_TARGET_DIR/release/deps",
            "cargo run --release --example layout --no-default-features",
            "RUSTDOCFLAGS=-Dwarnings make doc",
            "cargo kani (when available)",
        ],
        "tools": {
            "cargo": (evidence_dir / "cargo-version.txt")
            .read_text(encoding="utf-8")
            .splitlines()[0],
            "jsonschema": (evidence_dir / "jsonschema-version.txt")
            .read_text(encoding="utf-8")
            .strip(),
            "python": (evidence_dir / "python-version.txt")
            .read_text(encoding="utf-8")
            .strip(),
            "quire": json.loads(
                (evidence_dir / "quire-provenance.json").read_text(encoding="utf-8")
            )["cli"]["version"],
            "rustc": (evidence_dir / "rustc-version.txt")
            .read_text(encoding="utf-8")
            .splitlines()[0],
            "rust-msrv": (evidence_dir / "msrv-rustc-version.txt")
            .read_text(encoding="utf-8")
            .splitlines()[0],
            "size": (evidence_dir / "size-version.txt")
            .read_text(encoding="utf-8")
            .splitlines()[0],
        },
        "pgm01": {
            "policy": "ix://agent-ix/quire-contract-ir/PGM-01",
            "candidateRevision": PGM01_CANDIDATE_REVISION,
            "envelopeSchema": "quire.derivation-evidence/v1",
            "envelopeSchemaDigest": digest(pgm01_schema_digest),
        },
    }
    input_path = evidence_dir / "collection-input.json"
    write_json(input_path, collection_input)

    excluded = {
        "collection-input.json",
        "evidence-envelope.json",
        "evidence-manifest.json",
        "pgm01-envelope.stderr",
        "pgm01-envelope.stdout",
        "pgm01-envelope-status.txt",
        "sha256sums.txt",
    }
    for transcript in VALIDATOR_TRANSCRIPTS:
        excluded.update(
            {
                f"{transcript}.status.txt",
                f"{transcript}.stderr",
                f"{transcript}.stdout",
                f"{transcript}-status.txt",
            }
        )
    entries = []
    for path in sorted(evidence_dir.iterdir(), key=lambda item: item.name):
        if path.is_file() and path.name not in excluded:
            entries.append(
                {
                    "path": path.name,
                    "sha256": sha256_file(path),
                    "size": path.stat().st_size,
                }
            )

    limitations = [
        "current main-based candidate has no deliberately dispatched remote CI run",
        "merged PGM-01 revision was integrated under a bounded admin exception without protected checks; that exception excludes runtime release qualification",
        "CODEOWNER approval and the human source-release decision are pending",
    ]
    if kani_status != "passed":
        limitations.append(f"local Kani status is {kani_status}")

    outcomes = command_outcomes(evidence_dir, kani_status)
    manifest = {
        "schemaVersion": "quire.runtime-evidence-manifest/v1",
        "sourceRevision": revision,
        "collectedAt": recorded_at,
        "outcomes": outcomes,
        "artifacts": entries,
        "limitations": limitations,
    }
    manifest_path = evidence_dir / "evidence-manifest.json"
    write_json(manifest_path, manifest)

    failed = [item["name"] for item in outcomes if item["status"] == "failed"]
    inconclusive = [
        item["name"] for item in outcomes if item["status"] == "inconclusive"
    ]
    if failed:
        result_status = "inconclusive"
        result_summary = f"{len(failed)} locally collected runtime checks failed"
    elif inconclusive:
        result_status = "pending"
        result_summary = f"{len(inconclusive)} local runtime check outcomes are inconclusive"
    elif kani_status == "passed":
        result_status = "conclusive"
        result_summary = "all locally collected runtime checks, including Kani, passed"
    else:
        result_status = "pending"
        result_summary = (
            "all executed local runtime checks passed; " f"Kani is {kani_status}"
        )
    envelope = {
        "schemaVersion": "quire.derivation-evidence/v1",
        "recordId": evidence_dir.name,
        "recordedAt": recorded_at,
        "producer": {
            "name": "quire-contract-runtime-evidence-collector",
            "version": package["version"],
            "sourceRevision": revision,
            "executableDigest": digest(sha256_file(COLLECTOR)),
            "invocation": ["scripts/collect_evidence.sh", invocation_directory],
        },
        "inputs": [
            {
                "role": "evidence-collection-input",
                "uri": "collection-input.json",
                "mediaType": "application/json",
                "schema": {
                    "id": "quire.runtime-evidence-input",
                    "version": "v1",
                    "digest": digest(sha256_file(INPUT_SCHEMA)),
                },
                "contentDigest": digest(sha256_file(input_path)),
            }
        ],
        "backend": {
            "kind": "none",
            "reason": (
                "deterministic evidence packaging; invoked tools are identified "
                "in the input and manifest"
            ),
        },
        "outputs": [
            {
                "role": "runtime-evidence-manifest",
                "uri": "evidence-manifest.json",
                "mediaType": "application/json",
                "schema": {
                    "id": "quire.runtime-evidence-manifest",
                    "version": "v1",
                    "digest": digest(sha256_file(MANIFEST_SCHEMA)),
                },
                "contentDigest": digest(sha256_file(manifest_path)),
            }
        ],
        "parametersDigest": digest(hash_parameter_files()),
        "environment": {
            "targetTriple": next(
                line.split(": ", 1)[1]
                for line in (evidence_dir / "rustc-version.txt")
                .read_text(encoding="utf-8")
                .splitlines()
                if line.startswith("host: ")
            ),
            "operatingSystem": platform.platform(),
            "toolchain": collection_input["tools"]["rustc"],
            "dependenciesDigest": digest(sha256_file(ROOT / "Cargo.lock")),
        },
        "provenance": {
            "repository": "https://github.com/agent-ix/quire-contract-runtime",
            "sourceRevision": revision,
            "candidateRevision": revision,
            "contributionMethod": "agent-assisted",
            "reviewers": ["@kreneskyp"],
        },
        "result": {
            "status": result_status,
            "summary": result_summary,
            "requirementRefs": ["PGM-01-R08", "PGM-01-R09", "MP-001"],
        },
        "extensions": {
            "dev.agent-ix.runtime": {
                "componentClass": "linked-runtime",
                "envelopeSchemaDigest": pgm01_schema_digest,
                "pgm01CandidateRevision": PGM01_CANDIDATE_REVISION,
                "reviewState": "pending",
                "sourceState": source_state,
            }
        },
    }
    write_json(evidence_dir / "evidence-envelope.json", envelope)


def main() -> int:
    if len(sys.argv) != 2:
        print("usage: build_evidence_envelope.py EVIDENCE_DIR", file=sys.stderr)
        return 2
    evidence_dir = Path(sys.argv[1])
    build(evidence_dir)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
