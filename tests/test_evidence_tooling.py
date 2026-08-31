"""Tests for the runtime evidence builder and local JSON Schema validator."""

from __future__ import annotations

import importlib.util
import json
import os
import shutil
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path
from unittest import mock


ROOT = Path(__file__).resolve().parent.parent
BUILDER_PATH = ROOT / "scripts" / "build_evidence_envelope.py"
COLLECTOR_PATH = ROOT / "scripts" / "collect_evidence.sh"
VALIDATOR_PATH = ROOT / "scripts" / "validate_json_schema.py"
VERIFIER_PATH = ROOT / "scripts" / "verify_evidence.py"
ANCHOR_UPDATER_PATH = ROOT / "scripts" / "update_evidence_anchors.py"
COVERAGE_PATH = ROOT / "scripts" / "check_coverage_status.py"
KANI_CENSUS_PATH = ROOT / "scripts" / "check_kani_harnesses.py"


def load_module(name: str, path: Path):
    spec = importlib.util.spec_from_file_location(name, path)
    if spec is None or spec.loader is None:
        raise RuntimeError(f"could not load {name}")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


builder = load_module("runtime_evidence_builder", BUILDER_PATH)
verifier = load_module("runtime_evidence_verifier", VERIFIER_PATH)
anchor_updater = load_module("runtime_anchor_updater", ANCHOR_UPDATER_PATH)
coverage = load_module("runtime_coverage_status", COVERAGE_PATH)


class EvidenceBuilderTests(unittest.TestCase):
    # Trace: TC-007, NFR-002-AC-4
    def test_evidence_tools_have_requirement_ownership(self) -> None:
        ownership_label = "".join(
            chr(code) for code in (73, 109, 112, 108, 101, 109, 101, 110, 116, 115)
        )
        ownership_marker = f"# {ownership_label}: NFR-002"
        plan = (ROOT / "spec" / "assurance" / "MP-001-runtime-measurements.md").read_text(
            encoding="utf-8"
        )
        for path in (
            BUILDER_PATH,
            COLLECTOR_PATH,
            VALIDATOR_PATH,
            VERIFIER_PATH,
            ANCHOR_UPDATER_PATH,
            COVERAGE_PATH,
            KANI_CENSUS_PATH,
        ):
            self.assertIn(ownership_marker, path.read_text(encoding="utf-8"), path.name)
            self.assertIn(f"`scripts/{path.name}`", plan, path.name)

    # Trace: TC-007, NFR-002-AC-4
    def test_pgm01_pin_matches_vendored_schema_and_planning(self) -> None:
        self.assertEqual(
            builder.verified_pgm01_schema_digest(),
            builder.PGM01_ENVELOPE_SCHEMA_DIGEST,
        )
        self.assertEqual(
            builder.verified_pgm01_revision(), builder.PGM01_CANDIDATE_REVISION
        )
        for relative_path in (
            "planning/pgm-01-reconciliation.md",
            "planning/gap-analysis.md",
        ):
            text = (ROOT / relative_path).read_text(encoding="utf-8")
            self.assertIn(builder.PGM01_CANDIDATE_REVISION, text, relative_path)
            self.assertIn(builder.PGM01_ENVELOPE_SCHEMA_DIGEST, text, relative_path)

    # Trace: TC-007, NFR-002-AC-4
    def test_pgm01_pin_mismatch_fails_closed(self) -> None:
        with mock.patch.object(builder, "PGM01_ENVELOPE_SCHEMA_DIGEST", "0" * 64):
            with self.assertRaisesRegex(ValueError, "schema digest mismatch"):
                builder.verified_pgm01_schema_digest()
        with mock.patch.object(builder, "PGM01_CANDIDATE_REVISION", "0" * 40):
            with self.assertRaisesRegex(ValueError, "commit identity mismatch"):
                builder.verified_pgm01_revision()

    # Trace: TC-007, NFR-002-AC-4
    def test_build_preserves_roles_digests_extensions_and_pin(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            evidence_dir = Path(directory) / "runtime-v01-fixture"
            evidence_dir.mkdir()
            self.write_fixture_inputs(evidence_dir)

            builder.build(evidence_dir)

            collection_input = self.read_json(evidence_dir / "collection-input.json")
            manifest = self.read_json(evidence_dir / "evidence-manifest.json")
            envelope = self.read_json(evidence_dir / "evidence-envelope.json")

            self.assertEqual(collection_input["sourceRevision"], "a" * 40)
            self.assertEqual(
                collection_input["pgm01"]["candidateRevision"],
                builder.PGM01_CANDIDATE_REVISION,
            )
            self.assertEqual(
                collection_input["pgm01"]["envelopeSchemaDigest"]["value"],
                builder.PGM01_ENVELOPE_SCHEMA_DIGEST,
            )
            self.assertEqual(manifest["sourceRevision"], "a" * 40)
            self.assertIn(
                {"name": "kani", "status": "skipped-unavailable"},
                manifest["outcomes"],
            )
            self.assertEqual(envelope["inputs"][0]["role"], "evidence-collection-input")
            self.assertEqual(envelope["outputs"][0]["role"], "runtime-evidence-manifest")
            self.assertEqual(
                envelope["inputs"][0]["contentDigest"]["value"],
                builder.sha256_file(evidence_dir / "collection-input.json"),
            )
            self.assertEqual(
                envelope["outputs"][0]["contentDigest"]["value"],
                builder.sha256_file(evidence_dir / "evidence-manifest.json"),
            )
            extension = envelope["extensions"]["dev.agent-ix.runtime"]
            self.assertEqual(extension["componentClass"], "linked-runtime")
            self.assertEqual(
                extension["pgm01CandidateRevision"],
                builder.PGM01_CANDIDATE_REVISION,
            )
            self.assertEqual(
                extension["envelopeSchemaDigest"],
                builder.PGM01_ENVELOPE_SCHEMA_DIGEST,
            )
            self.assertEqual(envelope["parametersDigest"]["value"], builder.hash_parameter_files())

    # Trace: TC-007, NFR-002-AC-4
    def test_build_records_failed_and_missing_commands_without_a_pass_claim(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            evidence_dir = Path(directory) / "runtime-v01-fixture"
            evidence_dir.mkdir()
            self.write_fixture_inputs(evidence_dir)
            (evidence_dir / "clippy.status.txt").write_text("101\n", encoding="utf-8")
            (evidence_dir / "fmt.status.txt").unlink()

            builder.build(evidence_dir)

            manifest = self.read_json(evidence_dir / "evidence-manifest.json")
            envelope = self.read_json(evidence_dir / "evidence-envelope.json")
            outcomes = {item["name"]: item["status"] for item in manifest["outcomes"]}
            self.assertEqual(outcomes["clippy"], "failed")
            self.assertEqual(outcomes["fmt"], "inconclusive")
            self.assertEqual(envelope["result"]["status"], "inconclusive")
            self.assertNotIn("all executed", envelope["result"]["summary"])

    # Trace: TC-007, NFR-002-AC-4
    def test_passed_status_contradiction_is_retained_as_failure(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            evidence_dir = Path(directory) / "runtime-v01-fixture"
            evidence_dir.mkdir()
            self.write_fixture_inputs(evidence_dir)
            (evidence_dir / "test-all.stdout").write_text(
                "test result: FAILED. 0 passed; 7 failed\n", encoding="utf-8"
            )

            builder.build(evidence_dir)
            manifest = self.read_json(evidence_dir / "evidence-manifest.json")
            envelope = self.read_json(evidence_dir / "evidence-envelope.json")
            outcomes = {item["name"]: item["status"] for item in manifest["outcomes"]}
            self.assertEqual(outcomes["test-all"], "failed")
            self.assertEqual(envelope["result"]["status"], "inconclusive")

    # Trace: TC-007, NFR-002-AC-4
    def test_kani_pass_requires_numeric_success_complete_summary_and_every_harness(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            evidence_dir = Path(directory)
            self.write_fixture_inputs(evidence_dir)
            (evidence_dir / "kani-status.txt").write_text("passed\n", encoding="utf-8")
            (evidence_dir / "kani-version.txt").write_text(
                builder.EXPECTED_KANI_VERSION + "\n", encoding="utf-8"
            )
            transcript = "Kani Rust Verifier 0.67.0 (cargo plugin)\n" + "\n".join(
                f"kani_proofs::{name} SUCCESSFUL"
                for name in builder.EXPECTED_KANI_HARNESSES
            )
            transcript += (
                f"\nComplete - {len(builder.EXPECTED_KANI_HARNESSES)} successfully verified "
                f"harnesses, 0 failures, {len(builder.EXPECTED_KANI_HARNESSES)} total.\n"
            )
            (evidence_dir / "kani.stdout").write_text(transcript, encoding="utf-8")
            outcomes = {item["name"]: item["status"] for item in builder.command_outcomes(evidence_dir)}
            self.assertEqual(outcomes["kani"], "passed")
            (evidence_dir / "kani.stdout").write_text(
                "VERIFICATION:- FAILED\nComplete - 0 successfully verified harnesses, 6 failures, 6 total.\n",
                encoding="utf-8",
            )
            outcomes = {item["name"]: item["status"] for item in builder.command_outcomes(evidence_dir)}
            self.assertEqual(outcomes["kani"], "failed")

    # Trace: TC-007, NFR-002-AC-4
    def test_collector_and_declared_command_sets_agree(self) -> None:
        collector = COLLECTOR_PATH.read_text(encoding="utf-8").split(
            'quire provenance --pretty >"$evidence_dir/quire-provenance.json"', 1
        )[1]
        collected = set(
            __import__("re").findall(
                r"(?m)^\s*run_and_retain ([a-z0-9-]+)(?: |$)", collector
            )
        )
        declared = {transcript for _, transcript in builder.COMMAND_TRANSCRIPTS}
        self.assertEqual(collected, declared)

    # Trace: TC-007, NFR-002-AC-4
    def test_every_declared_command_has_contradiction_markers(self) -> None:
        declared = {name for name, _ in builder.COMMAND_TRANSCRIPTS}
        self.assertEqual(declared, set(builder.PASS_CONTRADICTION_MARKERS))

    # Trace: TC-007, NFR-002-AC-4
    def test_skipped_outcome_forces_pending_result_and_limitation(self) -> None:
        status, _, limitations = builder.summarize_outcomes(
            [{"name": "pgm01-envelope", "status": "skipped-unavailable"}]
        )
        self.assertEqual(status, "pending")
        self.assertEqual(
            limitations,
            ["skipped-unavailable runtime outcome: pgm01-envelope"],
        )

    # Trace: TC-007, NFR-002-AC-4
    def test_validator_transcript_exclusions_are_explicitly_named(self) -> None:
        expected = {
            "pgm01-pinned-schema",
            "input-schema",
            "manifest-schema",
            "pgm01-schema",
            "pgm01-envelope",
        }
        self.assertEqual(set(builder.VALIDATOR_TRANSCRIPTS), expected)
        declared = {transcript for _, transcript in builder.COMMAND_TRANSCRIPTS}
        self.assertTrue(expected.issubset(declared))

    # Trace: TC-007, NFR-002-AC-4
    def test_collector_fail_closed_self_test(self) -> None:
        completed = subprocess.run(
            ["bash", str(COLLECTOR_PATH), "--self-test"],
            check=False,
            capture_output=True,
            text=True,
        )
        self.assertEqual(completed.returncode, 0, completed.stderr)
        self.assertIn("collector fail-closed self-test passed", completed.stdout)

    # Trace: TC-007, NFR-002-AC-4
    def test_make_graph_requires_real_coverage_kani_and_verification_gates(self) -> None:
        database = subprocess.run(
            ["make", "-qp", "-f", str(ROOT / "Makefile")],
            cwd=ROOT,
            check=False,
            capture_output=True,
            text=True,
        )
        ci_line = next(line for line in database.stdout.splitlines() if line.startswith("ci:"))
        prerequisites = set(ci_line.removeprefix("ci:").split())
        self.assertTrue({"coverage", "kani", "verify-evidence"}.issubset(prerequisites))

        with tempfile.TemporaryDirectory() as directory:
            python_link = Path(directory) / "python3"
            python_link.symlink_to(sys.executable)
            environment = os.environ.copy()
            environment["PATH"] = directory
            make = shutil.which("make")
            self.assertIsNotNone(make)
            unavailable = subprocess.run(
                [str(make), "-f", str(ROOT / "Makefile"), "kani"],
                cwd=ROOT,
                env=environment,
                check=False,
                capture_output=True,
                text=True,
            )
        self.assertNotEqual(unavailable.returncode, 0)
        self.assertIn("cargo-kani is required", unavailable.stderr)

    # Trace: TC-007, NFR-002-AC-4
    def test_kani_census_and_local_coverage_classifier_are_executable(self) -> None:
        for path, expected in (
            (KANI_CENSUS_PATH, "6 declared and trace-bound Kani harnesses"),
            (COVERAGE_PATH, '"statusLies": 0'),
        ):
            completed = subprocess.run(
                [sys.executable, str(path)],
                cwd=ROOT,
                check=False,
                capture_output=True,
                text=True,
            )
            self.assertEqual(completed.returncode, 0, completed.stderr)
            self.assertIn(expected, completed.stdout)

    # Trace: TC-007, NFR-002-AC-4
    def test_local_coverage_classifier_rejects_unbacked_and_false_missing_rows(self) -> None:
        report = {
            "status_lies": [],
            "totals": {"backed": 1, "total": 1},
            "unbacked_rows": [],
        }
        self.assertEqual(coverage.coverage_contradictions(["✅ Complete"], report), [])
        self.assertTrue(
            coverage.coverage_contradictions(["❌ Missing"], report),
        )
        report["unbacked_rows"] = [{"id": "FR-001"}]
        report["totals"] = {"backed": 0, "total": 1}
        self.assertTrue(
            coverage.coverage_contradictions(["✅ Complete"], report),
        )

    @staticmethod
    def read_json(path: Path):
        return json.loads(path.read_text(encoding="utf-8"))

    @staticmethod
    def write_fixture_inputs(evidence_dir: Path) -> None:
        values = {
            "source-revision.txt": "a" * 40 + "\n",
            "source-state.txt": "clean\n",
            "kani-status.txt": "skipped-unavailable\n",
            "kani-version.txt": "skipped-unavailable\n",
            "cargo-version.txt": "cargo 1.94.1\n",
            "jsonschema-version.txt": "3.2.0\n",
            "python-version.txt": "Python 3.10.12\n",
            "rustc-version.txt": "rustc 1.94.1\nhost: x86_64-unknown-linux-gnu\n",
            "msrv-rustc-version.txt": "rustc 1.75.0\nhost: x86_64-unknown-linux-gnu\n",
            "size-version.txt": "GNU size 2.38\n",
            "pgm01-schema-path.txt": "/tmp/quire-contract-ir/schemas/derivation-evidence-envelope-v1.schema.json\n",
            "pgm01-schema-sha256.txt": builder.PGM01_ENVELOPE_SCHEMA_DIGEST + "\n",
            "pgm01-validator-path.txt": "/tmp/quire-contract-ir/scripts/validate_governance.py\n",
            "pgm01-validator-sha256.txt": "b" * 64 + "\n",
            "pgm01-revision.txt": builder.PGM01_CANDIDATE_REVISION + "\n",
        }
        for name, value in values.items():
            (evidence_dir / name).write_text(value, encoding="utf-8")
        for _, transcript in builder.COMMAND_TRANSCRIPTS:
            (evidence_dir / f"{transcript}.status.txt").write_text(
                "0\n", encoding="utf-8"
            )
            (evidence_dir / f"{transcript}.stdout").write_text("", encoding="utf-8")
            (evidence_dir / f"{transcript}.stderr").write_text("", encoding="utf-8")
        (evidence_dir / "metadata.stdout").write_text(
            json.dumps(
                {"packages": [{"name": "quire-contract-runtime", "version": "0.1.0"}]}
            ),
            encoding="utf-8",
        )
        (evidence_dir / "quire-provenance.json").write_text(
            json.dumps({"cli": {"version": "0.31.0"}}),
            encoding="utf-8",
        )


class SchemaValidatorTests(unittest.TestCase):
    # Trace: TC-007, NFR-002-AC-4
    def test_validator_accepts_valid_and_reports_invalid_path(self) -> None:
        schema = {
            "$schema": "http://json-schema.org/draft-07/schema#",
            "type": "object",
            "additionalProperties": False,
            "required": ["value"],
            "properties": {"value": {"type": "integer"}},
        }
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            schema_path = root / "schema.json"
            instance_path = root / "instance.json"
            schema_path.write_text(json.dumps(schema), encoding="utf-8")

            instance_path.write_text('{"value": 1}', encoding="utf-8")
            accepted = self.run_validator(schema_path, instance_path)
            self.assertEqual(accepted.returncode, 0, accepted.stderr)
            self.assertEqual(json.loads(accepted.stdout), {"errors": [], "valid": True})

            instance_path.write_text('{"value": "wrong"}', encoding="utf-8")
            rejected = self.run_validator(schema_path, instance_path)
            self.assertEqual(rejected.returncode, 1, rejected.stderr)
            result = json.loads(rejected.stdout)
            self.assertFalse(result["valid"])
            self.assertEqual(result["errors"][0]["path"], "$.value")

    def test_validator_rejects_invalid_date_time_format(self) -> None:
        schema = {
            "$schema": "http://json-schema.org/draft-07/schema#",
            "type": "object",
            "properties": {"recordedAt": {"type": "string", "format": "date-time"}},
        }
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            schema_path = root / "schema.json"
            instance_path = root / "instance.json"
            schema_path.write_text(json.dumps(schema), encoding="utf-8")
            instance_path.write_text('{"recordedAt":"NOT-A-TIMESTAMP"}', encoding="utf-8")
            rejected = self.run_validator(schema_path, instance_path)
            self.assertEqual(rejected.returncode, 1, rejected.stderr)

    def test_validator_rejects_an_undeclared_format_checker(self) -> None:
        schema = {
            "$schema": "http://json-schema.org/draft-07/schema#",
            "type": "string",
            "format": "runtime-format-with-no-checker",
        }
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            schema_path = root / "schema.json"
            instance_path = root / "instance.json"
            schema_path.write_text(json.dumps(schema), encoding="utf-8")
            instance_path.write_text('"value"', encoding="utf-8")
            rejected = self.run_validator(schema_path, instance_path)
            self.assertNotEqual(rejected.returncode, 0)
            self.assertIn("format checkers are unavailable", rejected.stderr)

    @staticmethod
    def run_validator(schema_path: Path, instance_path: Path) -> subprocess.CompletedProcess[str]:
        return subprocess.run(
            [sys.executable, str(VALIDATOR_PATH), str(schema_path), str(instance_path)],
            check=False,
            capture_output=True,
            text=True,
        )


class EvidenceVerifierTests(unittest.TestCase):
    # Trace: TC-007, NFR-002-AC-4
    def test_checksum_census_rejects_additions_and_listed_symlinks(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            record = Path(directory) / "runtime-v01-fixture"
            record.mkdir()
            retained = record / "retained.txt"
            retained.write_text("retained\n", encoding="utf-8")
            (record / "sha256sums.txt").write_text(
                f"{verifier.sha256_file(retained)}  retained.txt\n", encoding="utf-8"
            )
            self.assertEqual(verifier.verify_checksums(record), 1)
            extra = record / "extra.txt"
            extra.write_text("extra\n", encoding="utf-8")
            with self.assertRaisesRegex(verifier.EvidenceError, "checksum census mismatch"):
                verifier.verify_checksums(record)
            extra.unlink()
            target = Path(directory) / "outside.txt"
            target.write_text("retained\n", encoding="utf-8")
            retained.unlink()
            retained.symlink_to(target)
            with self.assertRaisesRegex(verifier.EvidenceError, "symlink is not allowed"):
                verifier.verify_checksums(record)

    # Trace: TC-007, NFR-002-AC-4
    def test_outcome_census_rejects_an_unknown_retained_gate(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            record = Path(directory)
            EvidenceBuilderTests.write_fixture_inputs(record)
            builder.build(record)
            manifest = EvidenceBuilderTests.read_json(record / "evidence-manifest.json")
            verifier.verify_outcome_census(record, manifest)
            (record / "rogue.status.txt").write_text("0\n", encoding="utf-8")
            with self.assertRaisesRegex(verifier.EvidenceError, "outcome census mismatch"):
                verifier.verify_outcome_census(record, manifest)

    # Trace: TC-007, NFR-002-AC-4
    def test_nonexistent_source_revision_fails_binding(self) -> None:
        failed = subprocess.CompletedProcess(["git"], 1, b"", b"missing")
        with mock.patch.object(verifier, "git_result", return_value=failed):
            with self.assertRaisesRegex(verifier.EvidenceError, "does not exist"):
                verifier.verify_source_binding("0" * 40)

    # Trace: TC-007, NFR-002-AC-4
    def test_unavailable_and_failed_channels_survive_in_status_json(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            status = Path(directory) / "status.json"
            with mock.patch.object(verifier, "VERIFICATION_STATUS", status):
                with mock.patch.object(
                    verifier,
                    "verify_anchors",
                    side_effect=verifier.VerificationUnavailable("missing dependency"),
                ):
                    self.assertEqual(verifier.main(), 2)
                    self.assertEqual(json.loads(status.read_text())["status"], "unavailable")
                with mock.patch.object(
                    verifier,
                    "verify_anchors",
                    side_effect=verifier.EvidenceError("tampered record"),
                ):
                    self.assertEqual(verifier.main(), 1)
                    self.assertEqual(json.loads(status.read_text())["status"], "failed")

    # Trace: TC-007, NFR-002-AC-4
    def test_anchor_file_is_generated_from_the_complete_evidence_census(self) -> None:
        self.assertEqual(
            (ROOT / "evidence" / "ANCHORS").read_text(encoding="utf-8"),
            anchor_updater.rendered_anchors(),
        )


if __name__ == "__main__":
    unittest.main()
