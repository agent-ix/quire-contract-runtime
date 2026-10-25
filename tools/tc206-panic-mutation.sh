#!/usr/bin/env bash
set -euo pipefail

repo=$(git rev-parse --show-toplevel)
scratch=$(mktemp -d /tmp/tc206-panic-XXXXXXXX)
cleanup() {
  if [ -d "$scratch/tree" ]; then
    rm -rf -- "$scratch/tree/target"
    git -C "$repo" worktree remove --force "$scratch/tree"
  fi
  rmdir -- "$scratch"
}
trap cleanup EXIT

git -C "$repo" worktree add --detach "$scratch/tree" HEAD
python3 - "$scratch/tree/src/scalar/division.rs" <<'PY'
from pathlib import Path
import sys

path = Path(sys.argv[1])
source = path.read_text()
needle = "    items\n        .iter()"
if source.count(needle) != 1:
    raise SystemExit("TC-206 mutation anchor is absent or ambiguous")
path.write_text(source.replace(needle, '    panic!("TC-206 injected runtime panic");\n' + needle, 1))
PY

(
  cd "$scratch/tree"
  cargo +1.98.1 build --locked --release --manifest-path measurement/exact-panic/Cargo.toml \
    --target thumbv7em-none-eabi --target-dir target/exact-panic-msrv
  artifact=target/exact-panic-msrv/thumbv7em-none-eabi/release/libquire_contract_runtime_exact_panic.a
  set +e
  output=$(cargo run --locked --quiet --bin check_exact_panic -- "$artifact" 2>&1)
  status=$?
  set -e
  printf '%s\n' "$output"
  if [ "$status" -eq 0 ] || [[ "$output" != *"runtime panic relocation"* ]]; then
    echo "TC-206 mutation failed to trigger the exact-profile gate" >&2
    exit 1
  fi
  echo "TC-206 injected runtime panic rejected"
)
