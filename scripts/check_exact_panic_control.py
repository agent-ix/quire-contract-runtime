#!/usr/bin/env python3
"""Require a reachable RT-owned injected panic to fail the exact relocation gate."""

import io
import os
import subprocess
import sys
import tarfile
import tempfile
from pathlib import Path


def run(*argv: str, cwd: Path, env: dict[str, str] | None = None) -> subprocess.CompletedProcess[str]:
    return subprocess.run(argv, cwd=cwd, env=env, text=True, capture_output=True, check=False)


def main() -> None:
    if len(sys.argv) != 2:
        raise SystemExit("usage: check_exact_panic_control.py TARGET_DIR")
    root = Path(__file__).resolve().parent.parent
    target = (root / sys.argv[1]).resolve()
    target.mkdir(parents=True, exist_ok=True)
    archive = subprocess.run(["git", "archive", "HEAD"], cwd=root, capture_output=True, check=True)

    with tempfile.TemporaryDirectory(prefix="exact-panic-control-", dir=target) as temp:
        scratch = Path(temp)
        with tarfile.open(fileobj=io.BytesIO(archive.stdout), mode="r:") as source:
            source.extractall(scratch, filter="data")
        scalar = scratch / "src/scalar/division.rs"
        content = scalar.read_text()
        anchor = "    items\n        .iter()"
        if content.count(anchor) != 1:
            raise SystemExit("exact panic control: scalar insertion point absent or ambiguous")
        scalar.write_text(
            content.replace(
                anchor,
                '    if core::hint::black_box(!items.is_empty()) { panic!("TC-206 injected panic"); }\n' + anchor,
            )
        )
        env = os.environ.copy()
        env["CARGO_TARGET_DIR"] = str(target)
        build = run(
            "cargo", "+1.98.1", "build", "--locked", "--profile", "exact-panic",
            "--manifest-path", "measurement/exact-panic/Cargo.toml", "--target",
            "thumbv7em-none-eabi", cwd=scratch, env=env,
        )
        if build.returncode:
            raise SystemExit(f"exact panic control: scratch build failed:\n{build.stderr[-4000:]}")
        artifact = target / "thumbv7em-none-eabi/exact-panic/libquire_contract_runtime_exact_panic.a"
        check = run("python3", "scripts/check_exact_panic.py", str(artifact), cwd=scratch)
        if check.returncode == 0 or "RT-owned panic relocations:" not in check.stderr:
            raise SystemExit(
                "exact panic control: injected panic did not trigger the RT relocation verdict:\n"
                + f"build: {build.stderr[-2000:]}\ncheck status: {check.returncode}\n"
                + check.stderr[-2000:]
            )
        print("exact panic control: reachable RT-owned panic relocation detected")


if __name__ == "__main__":
    main()
