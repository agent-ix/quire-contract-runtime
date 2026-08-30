#!/usr/bin/env bash
set -euo pipefail

if [[ $# -gt 0 ]]; then
  evidence_dir="$1"
else
  evidence_revision="$(git rev-parse --short=12 HEAD)"
  evidence_timestamp="$(date -u +%Y%m%dT%H%M%SZ)"
  evidence_dir="evidence/runtime-v01-${evidence_revision}-${evidence_timestamp}"
fi
if [[ -e "$evidence_dir" ]]; then
  echo "refusing to overwrite retained evidence: $evidence_dir" >&2
  exit 2
fi
if [[ -n "$(git status --porcelain --untracked-files=all)" ]]; then
  echo "refusing to collect evidence from a modified or untracked source tree" >&2
  exit 2
fi
source_state=clean
mkdir -p "$evidence_dir"

run_and_retain() {
  local name="$1"
  shift
  "$@" >"$evidence_dir/$name.stdout" 2>"$evidence_dir/$name.stderr"
}

git rev-parse HEAD >"$evidence_dir/source-revision.txt"
echo "$source_state" >"$evidence_dir/source-state.txt"
rustc --version --verbose >"$evidence_dir/rustc-version.txt"
cargo --version --verbose >"$evidence_dir/cargo-version.txt"
quire provenance --pretty >"$evidence_dir/quire-provenance.json"
run_and_retain quire-validate quire validate --scope . 'spec/**/*.md' 'planning/**/*.md'
run_and_retain fmt cargo fmt --all -- --check
run_and_retain clippy cargo clippy --all-targets --all-features -- -D warnings
run_and_retain test-core cargo test --no-default-features
run_and_retain test-alloc cargo test --features alloc
run_and_retain test-std cargo test --features std
run_and_retain test-all cargo test --all-features
run_and_retain deny cargo deny check licenses
run_and_retain unsafe-audit bash scripts/check_unsafe_comments.sh
run_and_retain panic-audit bash scripts/check_panic_surface.sh
run_and_retain metadata cargo metadata --format-version 1 --no-default-features
run_and_retain default-dependencies cargo tree --edges normal --no-default-features
run_and_retain release-build cargo build --release --lib --no-default-features
run_and_retain layout cargo run --release --example layout --no-default-features
run_and_retain rustdoc env RUSTDOCFLAGS=-Dwarnings cargo doc --all-features --no-deps

rlib_path="$(find "${CARGO_TARGET_DIR:-target}/release/deps" -maxdepth 1 -type f -name 'libquire_contract_runtime-*.rlib' -print -quit)"
if [[ -z "$rlib_path" ]]; then
  echo "release rlib not found" >"$evidence_dir/rlib-size.stderr"
  exit 1
fi
wc -c "$rlib_path" >"$evidence_dir/rlib-size.stdout"

if command -v cargo-kani >/dev/null 2>&1; then
  run_and_retain kani cargo kani
  echo passed >"$evidence_dir/kani-status.txt"
else
  echo skipped-unavailable >"$evidence_dir/kani-status.txt"
fi

python3 scripts/build_evidence_envelope.py "$evidence_dir"

if [[ -n "${PGM01_VALIDATOR:-}" ]]; then
  run_and_retain pgm01-envelope \
    python3 "$PGM01_VALIDATOR" --fixture "$evidence_dir/evidence-envelope.json"
  echo passed >"$evidence_dir/pgm01-envelope-status.txt"
else
  echo skipped-unavailable >"$evidence_dir/pgm01-envelope-status.txt"
fi

(
  cd "$evidence_dir"
  find . -maxdepth 1 -type f ! -name sha256sums.txt -print0 \
    | sort -z \
    | xargs -0 sha256sum >sha256sums.txt
)
