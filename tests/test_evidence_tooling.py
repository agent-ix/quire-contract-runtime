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
KANI_MUTATIONS_PATH = ROOT / "scripts" / "check_kani_mutations.py"
FAILURE_PROPAGATION_PATH = ROOT / "scripts" / "check_failure_propagation.py"
KANI_RUNNER_PATH = ROOT / "scripts" / "run_kani_gate.py"
EVIDENCE_TEST_RUNNER_PATH = ROOT / "scripts" / "run_evidence_tests.py"
ASSURANCE_CHECKER_PATH = ROOT / "scripts" / "check_assurance_anchor.py"


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
assurance = load_module("runtime_assurance_anchor", ASSURANCE_CHECKER_PATH)
failure_propagation = load_module("runtime_failure_propagation", FAILURE_PROPAGATION_PATH)
kani_runner = load_module("runtime_kani_runner", KANI_RUNNER_PATH)
evidence_test_runner = load_module("runtime_evidence_test_runner", EVIDENCE_TEST_RUNNER_PATH)


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
            KANI_MUTATIONS_PATH,
            FAILURE_PROPAGATION_PATH,
            KANI_RUNNER_PATH,
            EVIDENCE_TEST_RUNNER_PATH,
            ASSURANCE_CHECKER_PATH,
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
    def test_zero_exit_without_positive_corroboration_is_inconclusive(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            evidence_dir = Path(directory)
            self.write_fixture_inputs(evidence_dir)
            (evidence_dir / "test-all.stdout").write_text("", encoding="utf-8")

            outcomes = {
                item["name"]: item["status"]
                for item in builder.command_outcomes(evidence_dir)
            }
            self.assertEqual(outcomes["test-all"], "inconclusive")

    # Trace: TC-007, NFR-002-AC-4
    def test_evidence_suite_distinguishes_negative_fixtures_from_suite_failure(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            evidence_dir = Path(directory)
            self.write_fixture_inputs(evidence_dir)
            (evidence_dir / "evidence-tool.stderr").write_text(
                "KANI_FAILED: intentional negative control\n",
                encoding="utf-8",
            )
            outcomes = {
                item["name"]: item["status"]
                for item in builder.command_outcomes(evidence_dir)
            }
            self.assertEqual(outcomes["evidence-tool"], "passed")
            (evidence_dir / "evidence-tool.stderr").write_text(
                "\nFAILED (failures=1)\n", encoding="utf-8"
            )
            outcomes = {
                item["name"]: item["status"]
                for item in builder.command_outcomes(evidence_dir)
            }
            self.assertEqual(outcomes["evidence-tool"], "failed")

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
                f"Checking harness kani_proofs::{name}...\n"
                f"SUMMARY:\n ** 0 of {builder.EXPECTED_KANI_CHECK_FLOORS[name]} failed\n"
                "VERIFICATION:- SUCCESSFUL"
                for name in builder.EXPECTED_KANI_HARNESSES
            )
            transcript += (
                f"\nComplete - {len(builder.EXPECTED_KANI_HARNESSES)} successfully verified "
                f"harnesses, 0 failures, {len(builder.EXPECTED_KANI_HARNESSES)} total.\n"
            )
            (evidence_dir / "kani.stdout").write_text(transcript, encoding="utf-8")
            outcomes = {item["name"]: item["status"] for item in builder.command_outcomes(evidence_dir)}
            self.assertEqual(outcomes["kani"], "passed")
            self.assertEqual(
                builder.kani_proof_checks(transcript),
                {
                    name: builder.EXPECTED_KANI_CHECK_FLOORS[name]
                    for name in builder.EXPECTED_KANI_HARNESSES
                },
            )
            hollow = transcript.replace(
                f" ** 0 of {builder.EXPECTED_KANI_CHECK_FLOORS[builder.EXPECTED_KANI_HARNESSES[0]]} failed",
                " ** 0 of 0 failed",
            )
            self.assertFalse(builder.validate_kani_success(hollow))
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
        special = {"fmt", "metadata", "kani"}
        self.assertEqual(declared - special, set(builder.PASS_CORROBORATION_PATTERNS))

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

        makefile = (ROOT / "Makefile").read_text(encoding="utf-8")
        self.assertIn("scripts/run_kani_gate.py", makefile)

    # Trace: TC-007, NFR-002-AC-4
    def test_evidence_test_runner_rejects_a_missing_suite(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            (root / "tests").mkdir()
            with mock.patch.object(evidence_test_runner, "ROOT", root):
                self.assertEqual(evidence_test_runner.main(), 1)

    # Trace: TC-007, NFR-002-AC-4
    def test_live_kani_gate_rejects_zero_exit_without_proof_floors(self) -> None:
        hollow = subprocess.CompletedProcess(["cargo", "kani"], 0, "success\n", "")
        with (
            mock.patch.object(Path, "is_file", return_value=True),
            mock.patch.object(kani_runner.subprocess, "run", return_value=hollow),
        ):
            self.assertEqual(kani_runner.main(), 1)

    # Trace: TC-007, NFR-002-AC-4
    def test_make_inspection_rejects_prefixed_makeflags_and_backgrounding(self) -> None:
        original = (ROOT / "Makefile").read_text(encoding="utf-8")
        for mutation, expected in (
            ("\noverride export MAKEFLAGS = -i\n", "MAKEFLAGS"),
            (
                original.replace(
                    "\t$(PYTHON) scripts/check_coverage_status.py",
                    "\t$(PYTHON) scripts/check_coverage_status.py &",
                ),
                "shell control",
            ),
        ):
            with tempfile.TemporaryDirectory() as directory:
                makefile = Path(directory) / "Makefile"
                makefile.write_text(
                    original + mutation if mutation.startswith("\n") else mutation,
                    encoding="utf-8",
                )
                errors = failure_propagation.inspect(makefile)
            self.assertTrue(any(expected in error for error in errors), errors)

    # Trace: TC-007, NFR-002-AC-4
    def test_kani_census_and_local_coverage_classifier_are_executable(self) -> None:
        for path, expected in (
            (KANI_CENSUS_PATH, "7 declared and trace-bound Kani harnesses"),
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
    def test_kani_census_process_rejects_a_mutated_fixture_tree(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            source = root / "verification" / "kani.rs"
            source.parent.mkdir(parents=True)
            text = (ROOT / "verification" / "kani.rs").read_text(encoding="utf-8")
            source.write_text(
                text.replace(
                    "fn tc_003_option_helpers_preserve_definedness()",
                    "fn removed_option_helper_harness()",
                ),
                encoding="utf-8",
            )
            environment = os.environ.copy()
            environment["QUIRE_RUNTIME_REPO_ROOT"] = str(root)
            rejected = subprocess.run(
                [sys.executable, str(KANI_CENSUS_PATH)],
                env=environment,
                check=False,
                capture_output=True,
                text=True,
            )
        self.assertEqual(rejected.returncode, 1)
        self.assertIn("KANI_CENSUS_FAILED", rejected.stderr)

    # Trace: TC-007, NFR-002-AC-4
    def test_coverage_process_rejects_a_mutated_matrix_fixture(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            matrix = root / "spec" / "test-matrix.md"
            matrix.parent.mkdir(parents=True)
            text = (ROOT / "spec" / "test-matrix.md").read_text(encoding="utf-8")
            matrix.write_text(
                text.replace(
                    "| FR-004 | FR-004-AC-3 | TC-008 | ✅ Complete |\n", ""
                ),
                encoding="utf-8",
            )
            environment = os.environ.copy()
            environment["QUIRE_RUNTIME_REPO_ROOT"] = str(root)
            rejected = subprocess.run(
                [sys.executable, str(COVERAGE_PATH)],
                env=environment,
                check=False,
                capture_output=True,
                text=True,
            )
        self.assertEqual(rejected.returncode, 1)
        self.assertIn("row census", rejected.stderr)

    # Trace: TC-007, NFR-002-AC-4
    def test_anchor_updater_process_rejects_an_implicit_removal(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            evidence = root / "evidence"
            for required in anchor_updater.REQUIRED_HISTORICAL_DIRECTORIES:
                (evidence / "historical" / required).mkdir(parents=True)
            records = evidence / "historical" / "retired-pre-head-binding"
            for index in range(anchor_updater.MINIMUM_HISTORICAL_RECORDS):
                record = records / f"record-{index:02}"
                record.mkdir()
                (record / "evidence-envelope.json").write_text("{}\n", encoding="utf-8")
            support = evidence / "support.txt"
            support.write_text("retained\n", encoding="utf-8")
            environment = os.environ.copy()
            environment["QUIRE_RUNTIME_REPO_ROOT"] = str(root)
            accepted = subprocess.run(
                [sys.executable, str(ANCHOR_UPDATER_PATH)],
                env=environment,
                check=False,
                capture_output=True,
                text=True,
            )
            self.assertEqual(accepted.returncode, 0, accepted.stderr)
            support.unlink()
            rejected = subprocess.run(
                [sys.executable, str(ANCHOR_UPDATER_PATH)],
                env=environment,
                check=False,
                capture_output=True,
                text=True,
            )
        self.assertNotEqual(rejected.returncode, 0)
        self.assertIn("refusing to remove committed evidence anchors", rejected.stderr)

    # Trace: TC-007, NFR-002-AC-4
    def test_anchor_updater_rejects_a_historical_record_removal(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            evidence = root / "evidence"
            for required in anchor_updater.REQUIRED_HISTORICAL_DIRECTORIES:
                (evidence / "historical" / required).mkdir(parents=True)
            records = evidence / "historical" / "retired-pre-head-binding"
            created = []
            for index in range(anchor_updater.MINIMUM_HISTORICAL_RECORDS):
                record = records / f"record-{index:02}"
                record.mkdir()
                (record / "evidence-envelope.json").write_text("{}\n", encoding="utf-8")
                created.append(record)
            environment = os.environ.copy()
            environment["QUIRE_RUNTIME_REPO_ROOT"] = str(root)
            accepted = subprocess.run(
                [sys.executable, str(ANCHOR_UPDATER_PATH)],
                env=environment,
                check=False,
                capture_output=True,
                text=True,
            )
            self.assertEqual(accepted.returncode, 0, accepted.stderr)
            shutil.rmtree(created[-1])
            replacement = records / "replacement"
            replacement.mkdir()
            (replacement / "evidence-envelope.json").write_text("{}\n", encoding="utf-8")
            rejected = subprocess.run(
                [sys.executable, str(ANCHOR_UPDATER_PATH)],
                env=environment,
                check=False,
                capture_output=True,
                text=True,
            )
        self.assertEqual(rejected.returncode, 1)
        self.assertIn("historical records", rejected.stderr)

    # Trace: TC-007, NFR-002-AC-4
    def test_failure_propagation_process_rejects_make_controls_and_shadowed_tools(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            makefile = root / "Makefile"
            makefile.write_text(
                (ROOT / "Makefile").read_text(encoding="utf-8") + "\n.IGNORE:\n",
                encoding="utf-8",
            )
            rejected = subprocess.run(
                [
                    sys.executable,
                    str(FAILURE_PROPAGATION_PATH),
                    "--makefile",
                    str(makefile),
                    "--static-only",
                ],
                check=False,
                capture_output=True,
                text=True,
            )
            self.assertEqual(rejected.returncode, 1)
            self.assertIn("global recipe-control directive", rejected.stderr)

            cargo = root / "cargo"
            cargo.write_text("#!/bin/sh\necho 'cargo 1.94.1'\n", encoding="utf-8")
            cargo.chmod(0o755)
            environment = os.environ.copy()
            environment["PATH"] = f"{root}:/usr/bin"
            rejected = subprocess.run(
                [sys.executable, str(FAILURE_PROPAGATION_PATH), "--inspect-only"],
                env=environment,
                check=False,
                capture_output=True,
                text=True,
            )
        self.assertEqual(rejected.returncode, 1)
        self.assertIn("cargo must resolve", rejected.stderr)

    # Trace: TC-007, NFR-002-AC-4
    def test_assurance_anchor_consumes_the_verifier_status_and_record_verdict(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            argument = root / "AA-001.md"
            argument.write_text(
                "evidence_binding:\n"
                "  anchor: evidence/ANCHORS\n"
                "  history_anchor: evidence/HISTORY\n"
                "  record_selection: evidence/README.md\n"
                "  checksum_binding: sha256sums.txt\n"
                "  authoritative_records: 1\n"
                "  outcomes: 26\n"
                "  required_result: conclusive\n",
                encoding="utf-8",
            )
            record = root / "runtime-v01-aaaaaaaaaaaa-20260831T000000Z"
            record.mkdir()
            manifest = {"outcomes": [{"name": f"gate-{index}"} for index in range(26)]}
            envelope = {"result": {"status": "conclusive"}}
            for name, value in (
                ("evidence-manifest.json", manifest),
                ("evidence-envelope.json", envelope),
            ):
                (record / name).write_text(json.dumps(value) + "\n", encoding="utf-8")
            (record / "sha256sums.txt").write_text(
                "".join(
                    f"{verifier.sha256_file(record / name)}  {name}\n"
                    for name in ("evidence-envelope.json", "evidence-manifest.json")
                ),
                encoding="utf-8",
            )
            status = root / "verification-status.json"
            status.write_text(
                '{"exitCode": 0, "message": "verified", "status": "passed"}\n',
                encoding="utf-8",
            )
            with (
                mock.patch.object(assurance, "ARGUMENT", argument),
                mock.patch.object(assurance.verifier, "VERIFICATION_STATUS", status),
                mock.patch.object(assurance.verifier, "verify_anchors", return_value=[record]),
            ):
                self.assertEqual(assurance.main(), 0)
                status.write_text(
                    '{"exitCode": 1, "message": "failed", "status": "failed"}\n',
                    encoding="utf-8",
                )
                self.assertEqual(assurance.main(), 1)

    # Trace: TC-007, NFR-002-AC-4
    def test_coverage_and_kani_census_use_the_unavailable_exit_channel(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            matrix = root / "spec" / "test-matrix.md"
            matrix.parent.mkdir(parents=True)
            shutil.copy2(ROOT / "spec" / "test-matrix.md", matrix)
            environment = os.environ.copy()
            environment["QUIRE_RUNTIME_REPO_ROOT"] = str(root)
            environment["PATH"] = directory
            coverage_unavailable = subprocess.run(
                [sys.executable, str(COVERAGE_PATH)],
                env=environment,
                check=False,
                capture_output=True,
                text=True,
            )
            kani_unavailable = subprocess.run(
                [sys.executable, str(KANI_CENSUS_PATH)],
                env=environment,
                check=False,
                capture_output=True,
                text=True,
            )
        self.assertEqual(coverage_unavailable.returncode, 2)
        self.assertIn("COVERAGE_STATUS=unavailable", coverage_unavailable.stderr)
        self.assertEqual(kani_unavailable.returncode, 2)
        self.assertIn("KANI_CENSUS_STATUS=unavailable", kani_unavailable.stderr)

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

    # Trace: TC-007, NFR-002-AC-4
    def test_matrix_integrity_rejects_empty_and_unknown_test_citations(self) -> None:
        original = (ROOT / "spec" / "test-matrix.md").read_text(encoding="utf-8")
        with tempfile.TemporaryDirectory() as directory:
            matrix = Path(directory) / "test-matrix.md"
            with mock.patch.object(coverage, "MATRIX", matrix):
                matrix.write_text(
                    original.replace(
                        "| FR-004 | FR-004-AC-3 | TC-008 | ✅ Complete |",
                        "| FR-004 | FR-004-AC-3 |  | ✅ Complete |",
                    ),
                    encoding="utf-8",
                )
                with self.assertRaisesRegex(ValueError, "no test citation"):
                    coverage.functional_rows()
                matrix.write_text(
                    original.replace(
                        "| FR-004 | FR-004-AC-3 | TC-008 | ✅ Complete |",
                        "| FR-004 | FR-004-AC-3 | TC-999 | ✅ Complete |",
                    ),
                    encoding="utf-8",
                )
                with self.assertRaisesRegex(ValueError, "unknown tests"):
                    coverage.functional_rows()

    # Trace: TC-007, NFR-002-AC-4
    def test_coverage_process_finds_cfg_attr_ignore_anywhere_in_repository(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            matrix = root / "spec" / "test-matrix.md"
            matrix.parent.mkdir(parents=True)
            shutil.copy2(ROOT / "spec" / "test-matrix.md", matrix)
            harness = root / "harness" / "operators_impl.rs"
            harness.parent.mkdir(parents=True)
            harness.write_text(
                "// Trace: TC-002\n"
                + "// deliberately distant trace context\n" * 15
                + "#[cfg_attr(all(), ignore)]\n#[test]\nfn tc_002_hidden() {}\n",
                encoding="utf-8",
            )
            environment = os.environ.copy()
            environment["QUIRE_RUNTIME_REPO_ROOT"] = str(root)
            rejected = subprocess.run(
                [sys.executable, str(COVERAGE_PATH)],
                env=environment,
                check=False,
                capture_output=True,
                text=True,
            )
        self.assertEqual(rejected.returncode, 1)
        self.assertIn("ignored trace-bearing test", rejected.stderr)

    # Trace: TC-007, NFR-002-AC-4
    def test_coverage_rejects_cfg_any_and_nested_target_hiding(self) -> None:
        for relative in ("harness/disabled.rs", "harness/target/disabled.rs"):
            with tempfile.TemporaryDirectory() as directory:
                root = Path(directory)
                matrix = root / "spec" / "test-matrix.md"
                matrix.parent.mkdir(parents=True)
                shutil.copy2(ROOT / "spec" / "test-matrix.md", matrix)
                shutil.copytree(ROOT / "verification", root / "verification")
                hidden = root / relative
                hidden.parent.mkdir(parents=True, exist_ok=True)
                hidden.write_text(
                    "// Trace: TC-002\n#[cfg(any())]\n#[test]\nfn tc_002_hidden() {}\n",
                    encoding="utf-8",
                )
                environment = os.environ.copy()
                environment["QUIRE_RUNTIME_REPO_ROOT"] = str(root)
                rejected = subprocess.run(
                    [sys.executable, str(COVERAGE_PATH)],
                    env=environment,
                    check=False,
                    capture_output=True,
                    text=True,
                )
            self.assertEqual(rejected.returncode, 1)
            self.assertIn("cfg(any())", rejected.stderr)

    # Trace: TC-007, NFR-002-AC-4
    def test_shell_audits_reject_injected_unsafe_and_panic_surfaces(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            (root / "scripts").mkdir()
            (root / "scripts" / "unsafe_comment_baseline.txt").write_text("", encoding="utf-8")
            (root / "src").mkdir()
            (root / "src" / "bad.rs").write_text("fn bad() { unsafe { bad(); } }\n", encoding="utf-8")
            unsafe = subprocess.run(
                ["/usr/bin/bash", str(ROOT / "scripts" / "check_unsafe_comments.sh")],
                cwd=root,
                check=False,
                capture_output=True,
                text=True,
            )
            self.assertEqual(unsafe.returncode, 1)
            for relative in (
                "src/accounting_tests.rs",
                "verification/empty.rs",
                "measurement/footprint/src/lib.rs",
                "measurement/footprint/src/population_tests.rs",
            ):
                path = root / relative
                path.parent.mkdir(parents=True, exist_ok=True)
                path.write_text("// clean\n", encoding="utf-8")
            (root / "src" / "bad.rs").write_text("fn bad() { panic!(); }\n", encoding="utf-8")
            panic = subprocess.run(
                ["/usr/bin/bash", str(ROOT / "scripts" / "check_panic_surface.sh")],
                cwd=root,
                check=False,
                capture_output=True,
                text=True,
            )
            self.assertEqual(panic.returncode, 1)
            self.assertIn("intentional panic surface", panic.stderr)

    # Trace: TC-007, NFR-002-AC-4
    def test_control_helpers_are_behaviorally_exercised(self) -> None:
        original = (ROOT / "Makefile").read_text(encoding="utf-8")
        with tempfile.TemporaryDirectory() as directory:
            makefile = Path(directory) / "Makefile"
            makefile.write_text(original, encoding="utf-8")
            swallowed = subprocess.CompletedProcess(["make"], 0)
            with mock.patch.object(
                failure_propagation.subprocess, "run", return_value=swallowed
            ):
                errors = failure_propagation.probe_command_positions(makefile)
            self.assertGreater(len(errors), 0)
            record = Path(directory) / "record"
            record.mkdir()
            (record / "value").write_text("one\n", encoding="utf-8")
            first = verifier.tree_digest(record)
            (record / "value").write_text("two\n", encoding="utf-8")
            self.assertNotEqual(first, verifier.tree_digest(record))

    # Trace: TC-007, NFR-002-AC-4
    def test_coverage_accepts_upstream_status_classifier_repair(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            matrix = root / "spec" / "test-matrix.md"
            matrix.parent.mkdir(parents=True)
            shutil.copy2(ROOT / "spec" / "test-matrix.md", matrix)
            shutil.copytree(ROOT / "verification", root / "verification")
            executable = root / "quire"
            executable.write_text(
                "#!/bin/sh\n"
                "echo '{\"unbacked_rows\":[],\"status_lies\":[],"
                "\"diagnostics\":[],\"totals\":{\"backed\":28,\"total\":28}}'\n",
                encoding="utf-8",
            )
            executable.chmod(0o755)
            environment = os.environ.copy()
            environment["QUIRE_RUNTIME_REPO_ROOT"] = str(root)
            environment["PATH"] = directory
            accepted = subprocess.run(
                [sys.executable, str(COVERAGE_PATH)],
                env=environment,
                check=False,
                capture_output=True,
                text=True,
            )
        self.assertEqual(accepted.returncode, 0, accepted.stderr)
        self.assertIn("COVERAGE_STATUS_NOTICE", accepted.stderr)

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
        for name in ("cargo", "cargo-kani", "make", "python", "quire", "rustc", "size"):
            values[f"{name}-path.txt"] = f"/trusted/{name}\n"
            values[f"{name}-sha256.txt"] = "c" * 64 + "\n"
        for name, value in values.items():
            (evidence_dir / name).write_text(value, encoding="utf-8")
        successful_stdout = {
            "ci-guard": "all 15 mandatory local-check targets propagate failures\n",
            "kani-census": "verified 7 declared and trace-bound Kani harnesses\n",
            "evidence-tool": "verified 44 evidence-tool behavioral tests\n",
            "quire-validate": "QUIRE_VALIDATION_PASSED\n",
            "clippy": "Finished `dev` profile\n",
            "test-core": "test result: ok. 17 passed\n",
            "test-alloc": "test result: ok. 17 passed\n",
            "test-std": "test result: ok. 17 passed\n",
            "test-all": "test result: ok. 19 passed\n",
            "test-footprint": "test result: ok. 1 passed\n",
            "msrv": "Finished dev profile\n",
            "deny": "licenses ok\n",
            "unsafe-audit": "unsafe audit passed\n",
            "panic-audit": "runtime and verification panic-surface audit passed\n",
            "default-dependencies": "quire-contract-runtime v0.1.0\n",
            "release-build": "Finished `release` profile\n",
            "layout": "CampaignCounts=32\n",
            "rustdoc": "Generated /tmp/doc/quire_contract_runtime/index.html\n",
            "linked-footprint": "bytes=907 panic_references=0\n",
            "rlib-size-observation": "bytes=1 enforcement=observation-only\n",
            "coverage": '{"statusLies": 0}\n',
            "pgm01-pinned-schema": '{"valid": true}\n',
            "input-schema": '{"valid": true}\n',
            "manifest-schema": '{"valid": true}\n',
            "pgm01-schema": '{"valid": true}\n',
            "pgm01-envelope": '{"valid": true}\n',
        }
        for _, transcript in builder.COMMAND_TRANSCRIPTS:
            (evidence_dir / f"{transcript}.status.txt").write_text(
                "0\n", encoding="utf-8"
            )
            (evidence_dir / f"{transcript}.stdout").write_text(
                successful_stdout.get(transcript, ""), encoding="utf-8"
            )
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
    def test_record_identity_and_conclusive_verdict_are_mandatory(self) -> None:
        record = Path("runtime-v01-aaaaaaaaaaaa-20260831T000000Z")
        envelope = {"recordId": record.name, "result": {"status": "conclusive"}}
        verifier.verify_record_identity(record, envelope)
        verifier.verify_conclusive_result(record, envelope)
        with self.assertRaisesRegex(verifier.EvidenceError, "record identity mismatch"):
            verifier.verify_record_identity(
                record, {"recordId": "runtime-v01-clone-20260831T000000Z"}
            )
        with self.assertRaisesRegex(verifier.EvidenceError, "not conclusive"):
            verifier.verify_conclusive_result(
                record, {"result": {"status": "inconclusive"}}
            )

    # Trace: TC-007, NFR-002-AC-4
    def test_anchor_root_symlink_is_rejected(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            target = root / "outside-anchors"
            target.write_text("# external\n", encoding="utf-8")
            anchor = root / "ANCHORS"
            anchor.symlink_to(target)
            with mock.patch.object(verifier, "ANCHORS", anchor):
                with self.assertRaisesRegex(verifier.EvidenceError, "must not be a symlink"):
                    verifier.verify_anchors()

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
    def test_source_binding_ignores_only_committed_repository_rules(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            subprocess.run(["git", "init", "-q"], cwd=root, check=True)
            (root / ".gitignore").write_text("/target\n__pycache__/\n*.pyc\n", encoding="utf-8")
            (root / "tracked.txt").write_text("tracked\n", encoding="utf-8")
            subprocess.run(["git", "add", ".gitignore", "tracked.txt"], cwd=root, check=True)
            subprocess.run(
                [
                    "git",
                    "-c",
                    "user.name=Runtime Test",
                    "-c",
                    "user.email=runtime@example.invalid",
                    "commit",
                    "-q",
                    "-m",
                    "fixture",
                ],
                cwd=root,
                check=True,
            )
            revision = subprocess.run(
                ["git", "rev-parse", "HEAD"],
                cwd=root,
                check=True,
                capture_output=True,
                text=True,
            ).stdout.strip()
            (root / "target").mkdir()
            (root / "target" / "ignored").write_text("ignored\n", encoding="utf-8")
            hidden = root / ".cargo"
            hidden.mkdir()
            (hidden / ".gitignore").write_text("*\n", encoding="utf-8")
            (hidden / "config.toml").write_text(
                '[build]\nrustflags=["--cfg","smuggled"]\n', encoding="utf-8"
            )
            with mock.patch.object(verifier, "ROOT", root):
                with self.assertRaisesRegex(verifier.EvidenceError, "config.toml"):
                    verifier.verify_source_binding(revision)

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
