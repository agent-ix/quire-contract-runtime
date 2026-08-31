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
EVIDENCE_VERIFIER = ROOT / "scripts" / "verify_evidence.py"
EVIDENCE_REQUIREMENTS = ROOT / "requirements-evidence.txt"
EXPECTED_KANI_HARNESSES = (
    "tc_002_boolean_truth_tables",
    "tc_003_campaign_counts_total_saturates",
    "tc_003_checked_i8_arithmetic_matches_primitives",
    "tc_003_i32_division_boundaries_are_undefined",
    "tc_003_option_helpers_preserve_definedness",
    "tc_003_slice_index_is_defined_exactly_in_bounds",
)
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
    ("coverage", "coverage"),
    ("kani", "kani"),
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
    "quire-validate": ('"valid": false', "validation failed"),
    "fmt": ("Diff in ",),
    "clippy": ("error: could not compile",),
    "test-core": ("test result: FAILED", "error: test failed"),
    "test-alloc": ("test result: FAILED", "error: test failed"),
    "test-std": ("test result: FAILED", "error: test failed"),
    "test-all": ("test result: FAILED", "error: test failed"),
    "test-footprint": ("test result: FAILED", "error: test failed"),
    "msrv": ("error: could not compile",),
    "release-build": ("error: could not compile",),
    "deny": ("error:", "FAILED"),
    "unsafe-audit": ("unsafe audit failed", "missing // SAFETY:"),
    "panic-audit": ("panic surface audit failed",),
    "metadata": ("error:", "error["),
    "default-dependencies": ("error:",),
    "layout": ("panicked at",),
    "rustdoc": ("error: could not document",),
    "linked-footprint": ("linked footprint check failed",),
    "rlib-size-observation": ("error:",),
    "coverage": ("claims `", "COVERAGE_STATUS_CONTRADICTION"),
    "kani": ("VERIFICATION:- FAILED", "UNSUCCESSFUL"),
    "pgm01-pinned-schema": ('"valid": false',),
    "input-schema": ('"valid": false',),
    "manifest-schema": ('"valid": false',),
    "pgm01-schema": ('"valid": false',),
    "pgm01-envelope": ('"valid": false', "governance validation error:"),
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


def validate_kani_success(combined: str) -> bool:
    expected_summary = (
        f"Complete - {len(EXPECTED_KANI_HARNESSES)} successfully verified harnesses, "
        f"0 failures, {len(EXPECTED_KANI_HARNESSES)} total."
    )
    return expected_summary in combined and all(
        f"kani_proofs::{name}" in combined for name in EXPECTED_KANI_HARNESSES
    )


def command_outcomes(evidence_dir: Path) -> list[dict[str, str]]:
    outcomes = []
    numeric = {
        path.name.removesuffix(".status.txt")
        for path in evidence_dir.glob("*.status.txt")
    }
    availability = {
        path.name.removesuffix("-status.txt")
        for path in evidence_dir.glob("*-status.txt")
        if not path.name.endswith(".status.txt")
    }
    required = {transcript for _, transcript in COMMAND_TRANSCRIPTS}
    for transcript in sorted(required | numeric | availability):
        name = transcript
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
                        status = "failed"
                    elif name == "kani" and not validate_kani_success(combined):
                        status = "failed"
                    else:
                        status = "passed"
            else:
                status = "failed"
        outcomes.append({"name": name, "status": status})
    return outcomes


def summarize_outcomes(
    outcomes: list[dict[str, str]],
) -> tuple[str, str, list[str]]:
    failed = sorted(item["name"] for item in outcomes if item["status"] == "failed")
    inconclusive = sorted(
        item["name"] for item in outcomes if item["status"] == "inconclusive"
    )
    skipped = sorted(
        item["name"]
        for item in outcomes
        if item["status"] == "skipped-unavailable"
    )
    limitations = [
        *(f"failed runtime outcome: {name}" for name in failed),
        *(f"inconclusive runtime outcome: {name}" for name in inconclusive),
        *(f"skipped-unavailable runtime outcome: {name}" for name in skipped),
    ]
    if failed:
        return "inconclusive", f"{len(failed)} locally collected runtime checks failed", limitations
    if inconclusive:
        return "pending", f"{len(inconclusive)} runtime outcomes are inconclusive", limitations
    if skipped:
        return "pending", f"{len(skipped)} runtime checks were skipped-unavailable", limitations
    return "conclusive", "all retained local runtime checks, including Kani, passed", limitations


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
        EVIDENCE_VERIFIER,
        EVIDENCE_REQUIREMENTS,
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
    recorded_pgm01_revision = (evidence_dir / "pgm01-revision.txt").read_text(
        encoding="utf-8"
    ).strip()
    if recorded_pgm01_revision != PGM01_CANDIDATE_REVISION:
        raise ValueError(
            f"PGM-01 checkout mismatch: expected {PGM01_CANDIDATE_REVISION}, "
            f"got {recorded_pgm01_revision}"
        )
    recorded_pgm01_schema_digest = (evidence_dir / "pgm01-schema-sha256.txt").read_text(
        encoding="utf-8"
    ).strip()
    if recorded_pgm01_schema_digest != pgm01_schema_digest:
        raise ValueError(
            f"external PGM-01 schema mismatch: expected {pgm01_schema_digest}, "
            f"got {recorded_pgm01_schema_digest}"
        )
    invocation_directory = (
        str(evidence_dir.relative_to(ROOT))
        if evidence_dir.is_relative_to(ROOT)
        else str(evidence_dir)
    )
    revision = (evidence_dir / "source-revision.txt").read_text(encoding="utf-8").strip()
    source_state = (evidence_dir / "source-state.txt").read_text(encoding="utf-8").strip()
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
            "quire coverage --scope . --strict",
        ],
        "tools": {
            "cargo": (evidence_dir / "cargo-version.txt")
            .read_text(encoding="utf-8")
            .splitlines()[0],
            "jsonschema": (evidence_dir / "jsonschema-version.txt")
            .read_text(encoding="utf-8")
            .strip(),
            "kani": (evidence_dir / "kani-version.txt")
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
            "schemaPath": (evidence_dir / "pgm01-schema-path.txt")
            .read_text(encoding="utf-8")
            .strip(),
            "schemaDigest": digest(recorded_pgm01_schema_digest),
            "validatorPath": (evidence_dir / "pgm01-validator-path.txt")
            .read_text(encoding="utf-8")
            .strip(),
            "validatorDigest": digest(
                (evidence_dir / "pgm01-validator-sha256.txt")
                .read_text(encoding="utf-8")
                .strip()
            ),
            "validatorRevision": recorded_pgm01_revision,
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

    outcomes = command_outcomes(evidence_dir)
    result_status, result_summary, outcome_limitations = summarize_outcomes(outcomes)
    limitations = [
        "current main-based candidate has no deliberately dispatched remote CI run",
        "merged PGM-01 revision was integrated under a bounded admin exception without protected checks; that exception excludes runtime release qualification",
        "CODEOWNER approval and the human source-release decision are pending",
        "Kani proofs cover the dispatch layer and bounded requirements named by each harness; they do not establish semantics outside that declared scope",
        *outcome_limitations,
    ]
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
