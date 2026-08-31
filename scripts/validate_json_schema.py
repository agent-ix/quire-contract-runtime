#!/usr/bin/env python3
"""Validate one JSON instance against a Draft 7 schema."""

from __future__ import annotations

import json
import importlib.metadata
import sys
from pathlib import Path

import jsonschema
from jsonschema import Draft7Validator, FormatChecker


REQUIRED_PACKAGES = {
    "jsonschema": "3.2.0",
    "rfc3339-validator": "0.1.4",
    "rfc3986-validator": "0.1.1",
}
REQUIRED_FORMATS = {"date-time", "uri", "uri-reference"}


def schema_formats(value: object) -> set[str]:
    if isinstance(value, dict):
        found = {value["format"]} if isinstance(value.get("format"), str) else set()
        for child in value.values():
            found.update(schema_formats(child))
        return found
    if isinstance(value, list):
        found: set[str] = set()
        for child in value:
            found.update(schema_formats(child))
        return found
    return set()


def checked_format_checker(schema: object | None = None) -> FormatChecker:
    for package, expected in REQUIRED_PACKAGES.items():
        try:
            actual = importlib.metadata.version(package)
        except importlib.metadata.PackageNotFoundError as error:
            raise RuntimeError(f"required schema package {package} is not installed") from error
        if actual != expected:
            raise RuntimeError(
                f"expected schema package {package} {expected}, found {actual}"
            )
    if jsonschema.__version__ != REQUIRED_PACKAGES["jsonschema"]:
        raise RuntimeError(
            f"expected jsonschema {REQUIRED_PACKAGES['jsonschema']}, "
            f"found {jsonschema.__version__}"
        )
    checker = FormatChecker()
    required = REQUIRED_FORMATS | (schema_formats(schema) if schema is not None else set())
    missing = required.difference(checker.checkers)
    if missing:
        raise RuntimeError(
            "required JSON Schema format checkers are unavailable: "
            + ", ".join(sorted(missing))
        )
    return checker


def display_path(parts: list[object]) -> str:
    path = "$"
    for part in parts:
        path += f"[{part}]" if isinstance(part, int) else f".{part}"
    return path


# Implements: NFR-002
def main() -> int:
    if len(sys.argv) != 3:
        print("usage: validate_json_schema.py SCHEMA INSTANCE", file=sys.stderr)
        return 2

    schema = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
    instance = json.loads(Path(sys.argv[2]).read_text(encoding="utf-8"))
    checker = checked_format_checker(schema)
    Draft7Validator.check_schema(schema)
    errors = sorted(
        Draft7Validator(schema, format_checker=checker).iter_errors(instance),
        key=lambda error: [str(part) for part in error.absolute_path],
    )
    result = {
        "errors": [
            {"message": error.message, "path": display_path(list(error.absolute_path))}
            for error in errors
        ],
        "valid": not errors,
    }
    print(json.dumps(result, indent=2, sort_keys=True))
    return 0 if not errors else 1


if __name__ == "__main__":
    raise SystemExit(main())
