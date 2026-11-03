#!/usr/bin/env python3
"""Run strict RT coverage with authoritative sibling test sources by reference."""

import argparse
from contextlib import contextmanager
from pathlib import Path
import subprocess
import sys
import tempfile

SOURCE_SUFFIXES = {".rs", ".py", ".ts", ".tsx", ".js", ".jsx"}
EXTERNAL_TESTS = tuple(Path(name) for name in (
    "tests/producer_origin_identity.rs",
    "tests/producer_origin_quantity_residual.rs",
    "tests/producer_origin_equality.rs",
))


def git_bytes(root, *arguments):
    result = subprocess.run(
        ["git", "-C", str(root), *arguments],
        check=True,
        capture_output=True,
    )
    return result.stdout


def git(root, *arguments):
    return git_bytes(root, *arguments).decode().strip()


def sibling_checkout(root):
    common = Path(git(root, "rev-parse", "--git-common-dir"))
    if not common.is_absolute():
        common = root / common
    return common.resolve().parent.parent / "quire-integration"


def regular_blob(checkout, reference, relative):
    entry = git_bytes(checkout, "ls-tree", "-z", reference, "--", relative.as_posix())
    fields = entry.split(b"\t", 1)[0].split()
    if len(fields) != 3 or fields[0] not in {b"100644", b"100755"} or fields[1] != b"blob":
        raise ValueError(f"{reference} must track the authoritative test as a regular file")
    return git_bytes(checkout, "cat-file", "blob", f"{reference}:{relative.as_posix()}")


def main_test_sources(checkout):
    origin = git(checkout, "config", "--get", "remote.origin.url").lower().rstrip("/").removesuffix(".git")
    if origin not in {
        "https://github.com/agent-ix/quire-integration",
        "git@github.com:agent-ix/quire-integration",
        "ssh://git@github.com/agent-ix/quire-integration",
    }:
        raise ValueError("integration origin must identify agent-ix/quire-integration")
    if git(checkout, "symbolic-ref", "--short", "HEAD") != "main":
        raise ValueError("the integration checkout must be on main")
    sources = []
    for relative in EXTERNAL_TESTS:
        source = checkout / relative
        if source.is_symlink() or not source.is_file():
            raise ValueError(f"authoritative main source must be a regular file: {source}")
        content = source.read_bytes()
        if (content != regular_blob(checkout, "HEAD", relative)
                or content != regular_blob(checkout, "refs/remotes/origin/main", relative)):
            raise ValueError(f"test contents differ from committed main: {relative}; refresh the sibling checkout")
        sources.append((relative, source))
    return sources


@contextmanager
def reference_scope(root, external_sources):
    # Directory entries are references, never copies of either repository's tests.
    with tempfile.TemporaryDirectory(prefix="rt-coverage-references-") as temporary:
        scope = Path(temporary)
        (scope / "spec").symlink_to(root / "spec", target_is_directory=True)
        paths = git(root, "ls-files", "-z").split("\0")
        for name in paths:
            relative = Path(name)
            if not name or (relative.suffix not in SOURCE_SUFFIXES
                            and relative.name not in {".gitignore", ".ignore"}):
                continue
            source = root / relative
            if not source.is_file():
                raise ValueError(f"tracked RT source is absent: {source}")
            reference = scope / relative
            reference.parent.mkdir(parents=True, exist_ok=True)
            reference.symlink_to(source)
        for relative, source in external_sources:
            reference = scope / "external/quire-integration" / relative
            reference.parent.mkdir(parents=True, exist_ok=True)
            reference.symlink_to(source)
        yield scope


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--quire", default="quire")
    parser.add_argument("--integration-dir", type=Path)
    parser.add_argument("--json", action="store_true")
    arguments = parser.parse_args()
    root = Path(__file__).resolve().parent.parent
    try:
        checkout = (arguments.integration_dir or sibling_checkout(root)).resolve()
        sources = main_test_sources(checkout)
        for _, source in sources:
            print(f"coverage source reference: {source} (main, clean)", file=sys.stderr)
        with reference_scope(root, sources) as scope:
            command = [arguments.quire, "coverage", "--scope", str(scope), "--strict"]
            if arguments.json:
                command.append("--json")
            return subprocess.run(command, check=False).returncode
    except (OSError, ValueError, subprocess.CalledProcessError) as error:
        detail = error.stderr.decode().strip() if isinstance(
            error, subprocess.CalledProcessError
        ) else str(error)
        detail = detail or str(error)
        print(f"coverage reference input refused: {detail}", file=sys.stderr)
        return 2


if __name__ == "__main__":
    sys.exit(main())
