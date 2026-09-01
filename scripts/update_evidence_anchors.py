#!/usr/bin/env python3
"""Regenerate the complete committed runtime evidence anchor census."""

from __future__ import annotations

import argparse
import hashlib
import os
import sys
from pathlib import Path


ROOT = Path(os.environ.get("QUIRE_RUNTIME_REPO_ROOT", Path(__file__).resolve().parent.parent))
EVIDENCE_ROOT = ROOT / "evidence"
ANCHORS = EVIDENCE_ROOT / "ANCHORS"
HISTORY_ANCHORS = EVIDENCE_ROOT / "HISTORY"
MINIMUM_HISTORICAL_RECORDS = 34
REQUIRED_HISTORICAL_DIRECTORIES = {
    "retired-pre-head-binding",
    "retired-pre-verifier",
}


def sha256_file(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def tree_digest(root: Path) -> str:
    state = hashlib.sha256()
    for path in sorted(root.rglob("*"), key=lambda item: item.as_posix()):
        if path.is_symlink():
            raise ValueError(f"symlink is not allowed in retained evidence: {path}")
        relative = path.relative_to(root).as_posix()
        if path.is_dir():
            kind = b"d"
        elif path.is_file():
            kind = b"f"
        else:
            raise ValueError(f"unsupported retained-evidence entry: {path}")
        state.update(kind + b"\0" + relative.encode("utf-8") + b"\0")
        if path.is_file():
            state.update(bytes.fromhex(sha256_file(path)))
        state.update(b"\0")
    return state.hexdigest()


def rendered_anchors() -> str:
    history = EVIDENCE_ROOT / "historical"
    if not history.is_dir() or history.is_symlink():
        raise ValueError("retained evidence history is absent or unsafe")
    directories = {
        path.name for path in history.iterdir() if path.is_dir() and not path.is_symlink()
    }
    missing = REQUIRED_HISTORICAL_DIRECTORIES - directories
    record_count = sum(
        1 for path in history.rglob("evidence-envelope.json") if path.is_file()
    )
    if missing or record_count < MINIMUM_HISTORICAL_RECORDS:
        raise ValueError(
            "retained evidence history census regressed: "
            f"missing={sorted(missing)}, records={record_count}, "
            f"minimum={MINIMUM_HISTORICAL_RECORDS}"
        )
    entries: list[tuple[Path, str]] = []
    for path in EVIDENCE_ROOT.iterdir():
        if path == ANCHORS:
            continue
        if path.is_symlink():
            raise ValueError(f"symlink is not allowed in retained evidence: {path}")
        if path.is_dir() and path.name.startswith("runtime-v01-"):
            target = path / "sha256sums.txt"
            if not (path / "evidence-envelope.json").is_file() or not target.is_file():
                raise ValueError(f"incomplete authoritative evidence directory: {path}")
            entries.append((target, sha256_file(target)))
        elif path.is_dir():
            entries.append((path, tree_digest(path)))
        elif path.is_file():
            entries.append((path, sha256_file(path)))
        else:
            raise ValueError(f"unsupported evidence entry: {path}")
    lines = ["# Complete SHA-256 census for committed runtime evidence."]
    lines.extend(
        f"{digest}  {path.relative_to(ROOT).as_posix()}"
        for path, digest in sorted(entries, key=lambda item: item[0].as_posix())
    )
    return "\n".join(lines) + "\n"


def rendered_history_anchors() -> str:
    history = EVIDENCE_ROOT / "historical"
    records = sorted(
        path.parent
        for path in history.rglob("evidence-envelope.json")
        if path.is_file()
    )
    if len(records) < MINIMUM_HISTORICAL_RECORDS:
        raise ValueError(
            f"retained evidence history census regressed: records={len(records)}, "
            f"minimum={MINIMUM_HISTORICAL_RECORDS}"
        )
    lines = ["# SHA-256 census for every retained historical record directory."]
    lines.extend(
        f"{tree_digest(record)}  {record.relative_to(ROOT).as_posix()}"
        for record in records
    )
    return "\n".join(lines) + "\n"


def anchor_targets(text: str) -> dict[str, str]:
    return {
        line.split("  ", 1)[1]: line.split("  ", 1)[0]
        for line in text.splitlines()
        if line and not line.startswith("#")
    }


# Implements: NFR-002
def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--allow-removals",
        action="store_true",
        help="permit an explicitly reviewed reduction of the anchored top-level census",
    )
    args = parser.parse_args()
    try:
        rendered_history = rendered_history_anchors()
        previous_history = (
            anchor_targets(HISTORY_ANCHORS.read_text(encoding="utf-8"))
            if HISTORY_ANCHORS.exists()
            else {}
        )
        proposed_history = anchor_targets(rendered_history)
        removed_history = sorted(set(previous_history) - set(proposed_history))
        if removed_history and not args.allow_removals:
            raise ValueError(
                "refusing to remove committed historical records without "
                f"--allow-removals: {removed_history}"
            )
        HISTORY_ANCHORS.write_text(rendered_history, encoding="utf-8")
        rendered = rendered_anchors()
        previous = (
            anchor_targets(ANCHORS.read_text(encoding="utf-8")) if ANCHORS.exists() else {}
        )
        proposed = anchor_targets(rendered)
        removed = sorted(set(previous) - set(proposed))
        if removed and not args.allow_removals:
            raise ValueError(
                f"refusing to remove committed evidence anchors without --allow-removals: {removed}"
            )
        ANCHORS.write_text(rendered, encoding="utf-8")
    except (OSError, ValueError) as error:
        print(f"EVIDENCE_ANCHOR_UPDATE_FAILED: {error}", file=sys.stderr)
        return 1
    changed = sorted(
        path for path in set(previous) & set(proposed) if previous[path] != proposed[path]
    )
    print(
        f"updated {ANCHORS.relative_to(ROOT)}: "
        f"added={sorted(set(proposed) - set(previous))}, removed={removed}, changed={changed}"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
