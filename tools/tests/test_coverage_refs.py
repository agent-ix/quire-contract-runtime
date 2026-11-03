"""Fail-closed source-reference controls; no authoritative owner tests are duplicated here."""

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
        for filename, content in files.items():
            path = root / filename
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_text(content)
        subprocess.run(["git", "-C", str(root), "add", "."], check=True)
        origin = f"https://github.com/agent-ix/{'quire-integration' if name == 'integration' else name}"
        subprocess.run(["git", "-C", str(root), "remote", "add", "origin", origin], check=True)
        self.commit(root)
        return root

    def commit(self, root, update_remote=True):
        subprocess.run(["git", "-C", str(root), "add", "."], check=True)
        subprocess.run(
            ["git", "-C", str(root), "-c", "user.name=Coverage test",
             "-c", "user.email=coverage@example.invalid", "-c", "commit.gpgsign=false",
             "-c", "core.hooksPath=/dev/null", "commit", "-qm", "fixture"],
            check=True,
        )
        if update_remote:
            subprocess.run(["git", "-C", str(root), "update-ref",
                            "refs/remotes/origin/main", "HEAD"], check=True)

    def test_scope_borrows_both_sources_and_removes_only_its_references(self):
        root = self.repository("runtime", {"src/lib.rs": "pub fn sample() {}\n"})
        (root / "spec").mkdir()
        sibling = self.repository("integration", {str(path): "fn sample() {}\n" for path in coverage_refs.EXTERNAL_TESTS})
        sources = coverage_refs.main_test_sources(sibling)
        with coverage_refs.reference_scope(root, sources) as scope:
            local = scope / "src/lib.rs"
            external = scope / "external/quire-integration" / coverage_refs.EXTERNAL_TESTS[0]
            self.assertTrue(local.is_symlink())
            self.assertTrue(external.is_symlink())
            self.assertTrue(local.samefile(root / "src/lib.rs"))
            self.assertTrue(external.samefile(sources[0][1]))
            for relative, source in sources:
                self.assertTrue((scope / "external/quire-integration" / relative).samefile(source))
            self.assertTrue((scope / "spec").samefile(root / "spec"))
        self.assertFalse(scope.exists())
        self.assertEqual(sources[0][1].read_text(), "fn sample() {}\n")

    def test_feature_branch_and_modified_main_source_are_refused(self):
        sibling = self.repository("integration", {str(path): "fn sample() {}\n" for path in coverage_refs.EXTERNAL_TESTS})
        subprocess.run(["git", "-C", str(sibling), "switch", "-qc", "feature"], check=True)
        with self.assertRaises(ValueError):
            coverage_refs.main_test_sources(sibling)
        subprocess.run(["git", "-C", str(sibling), "switch", "-q", "main"], check=True)
        (sibling / coverage_refs.EXTERNAL_TESTS[0]).write_text("fn changed() {}\n")
        with self.assertRaises(ValueError):
            coverage_refs.main_test_sources(sibling)

    def test_a_later_invalid_source_refuses_the_entire_input(self):
        sibling = self.repository("integration", {str(path): "fn sample() {}\n" for path in coverage_refs.EXTERNAL_TESTS})
        (sibling / coverage_refs.EXTERNAL_TESTS[-1]).write_text("fn changed() {}\n")
        with self.assertRaisesRegex(ValueError, "committed main"):
            coverage_refs.main_test_sources(sibling)

    def test_absent_tracked_main_test_does_not_fall_back_to_untracked_file(self):
        sibling = self.repository("integration", {"README.md": "fixture\n"})
        (sibling / "tests").mkdir()
        (sibling / coverage_refs.EXTERNAL_TESTS[0]).write_text("fn untracked() {}\n")
        with self.assertRaises(ValueError):
            coverage_refs.main_test_sources(sibling)

    def test_tracked_symlink_is_not_authoritative_file_content(self):
        sibling = self.repository("integration", {str(path): "fn sample() {}\n" for path in coverage_refs.EXTERNAL_TESTS})
        external = self.root / "outside.rs"
        external.write_text("fn outside() {}\n")
        source = sibling / coverage_refs.EXTERNAL_TESTS[0]
        source.unlink()
        source.symlink_to(external)
        self.commit(sibling)
        with self.assertRaisesRegex(ValueError, "regular file"):
            coverage_refs.main_test_sources(sibling)

    def test_assume_unchanged_does_not_hide_modified_source(self):
        sibling = self.repository("integration", {str(path): "fn sample() {}\n" for path in coverage_refs.EXTERNAL_TESTS})
        subprocess.run(["git", "-C", str(sibling), "update-index", "--assume-unchanged",
                        coverage_refs.EXTERNAL_TESTS[0].as_posix()], check=True)
        (sibling / coverage_refs.EXTERNAL_TESTS[0]).write_text("fn hidden_change() {}\n")
        subprocess.run(["git", "-C", str(sibling), "diff", "--quiet", "HEAD"], check=True)
        with self.assertRaisesRegex(ValueError, "committed main"):
            coverage_refs.main_test_sources(sibling)

    def test_unrelated_repository_on_main_is_refused(self):
        sibling = self.repository("unrelated", {str(path): "fn sample() {}\n" for path in coverage_refs.EXTERNAL_TESTS})
        with self.assertRaisesRegex(ValueError, "origin"):
            coverage_refs.main_test_sources(sibling)

    def test_local_main_changes_must_also_be_on_origin_main(self):
        sibling = self.repository("integration", {str(path): "fn sample() {}\n" for path in coverage_refs.EXTERNAL_TESTS})
        (sibling / coverage_refs.EXTERNAL_TESTS[0]).write_text("fn local_only() {}\n")
        self.commit(sibling, update_remote=False)
        with self.assertRaisesRegex(ValueError, "committed main"):
            coverage_refs.main_test_sources(sibling)


if __name__ == "__main__":
    unittest.main()
