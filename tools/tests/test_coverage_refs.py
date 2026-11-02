"""Fail-closed source-reference controls; no monitor tests are duplicated here."""

from pathlib import Path
import subprocess
import sys
import tempfile
import unittest

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
import coverage_refs


class CoverageReferences(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory()
        self.addCleanup(self.temporary.cleanup)
        self.root = Path(self.temporary.name)

    def repository(self, name, files):
        root = self.root / name
        root.mkdir()
        subprocess.run(["git", "init", "-q", "-b", "main", str(root)], check=True)
        for name, content in files.items():
            path = root / name
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_text(content)
        subprocess.run(["git", "-C", str(root), "add", "."], check=True)
        subprocess.run(
            ["git", "-C", str(root), "-c", "user.name=Coverage test",
             "-c", "user.email=coverage@example.invalid", "-c", "commit.gpgsign=false",
             "-c", "core.hooksPath=/dev/null", "commit", "-qm", "fixture"],
            check=True,
        )
        return root

    def test_scope_borrows_both_sources_and_removes_only_its_references(self):
        root = self.repository("runtime", {"src/lib.rs": "pub fn sample() {}\n"})
        (root / "spec").mkdir()
        sibling = self.repository("integration", {"tests/monitor_program.rs": "fn sample() {}\n"})
        source = coverage_refs.main_test_source(sibling)
        with coverage_refs.reference_scope(root, source) as scope:
            local = scope / "src/lib.rs"
            external = scope / "external/quire-integration/tests/monitor_program.rs"
            self.assertTrue(local.is_symlink())
            self.assertTrue(external.is_symlink())
            self.assertTrue(local.samefile(root / "src/lib.rs"))
            self.assertTrue(external.samefile(source))
            self.assertTrue((scope / "spec").samefile(root / "spec"))
        self.assertFalse(scope.exists())
        self.assertEqual(source.read_text(), "fn sample() {}\n")

    def test_feature_branch_and_modified_main_source_are_refused(self):
        sibling = self.repository("integration", {"tests/monitor_program.rs": "fn sample() {}\n"})
        subprocess.run(["git", "-C", str(sibling), "switch", "-qc", "feature"], check=True)
        with self.assertRaises(ValueError):
            coverage_refs.main_test_source(sibling)
        subprocess.run(["git", "-C", str(sibling), "switch", "-q", "main"], check=True)
        (sibling / "tests/monitor_program.rs").write_text("fn changed() {}\n")
        with self.assertRaises(subprocess.CalledProcessError):
            coverage_refs.main_test_source(sibling)

    def test_absent_tracked_main_test_does_not_fall_back_to_untracked_file(self):
        sibling = self.repository("integration", {"README.md": "fixture\n"})
        (sibling / "tests").mkdir()
        (sibling / "tests/monitor_program.rs").write_text("fn untracked() {}\n")
        with self.assertRaises(subprocess.CalledProcessError):
            coverage_refs.main_test_source(sibling)


if __name__ == "__main__":
    unittest.main()
