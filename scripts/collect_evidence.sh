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
if ! python3 -c 'import jsonschema' >/dev/null 2>&1; then
  echo "jsonschema is required for evidence collection" >&2
  exit 2
fi
mkdir -p "$evidence_dir"

run_and_retain() {
  local name="$1"
  shift
  "$@" >"$evidence_dir/$name.stdout" 2>"$evidence_dir/$name.stderr"
}

git rev-parse HEAD >"$evidence_dir/source-revision.txt"
echo "$source_state" >"$evidence_dir/source-state.txt"
rustc --version --verbose >"$evidence_dir/rustc-version.txt"
rustc +1.75.0 --version --verbose >"$evidence_dir/msrv-rustc-version.txt"
size --version >"$evidence_dir/size-version.txt"
cargo --version --verbose >"$evidence_dir/cargo-version.txt"
python3 --version >"$evidence_dir/python-version.txt"
python3 -c 'import jsonschema; print(jsonschema.__version__)' >"$evidence_dir/jsonschema-version.txt"
quire provenance --pretty >"$evidence_dir/quire-provenance.json"
run_and_retain quire-validate \
  quire validate --scope . 'spec/**/*.md' 'planning/**/*.md' 'plan/**/*.md'
run_and_retain fmt cargo fmt --all -- --check
run_and_retain clippy make lint
run_and_retain test-core cargo test --no-default-features
run_and_retain test-alloc cargo test --features alloc
run_and_retain test-std cargo test --features std
run_and_retain test-all cargo test --all-features
run_and_retain test-footprint cargo test -p quire-contract-runtime-footprint
run_and_retain msrv cargo +1.75.0 check --all-targets --all-features
run_and_retain deny cargo deny check licenses
run_and_retain unsafe-audit bash scripts/check_unsafe_comments.sh
run_and_retain panic-audit bash scripts/check_panic_surface.sh
run_and_retain metadata cargo metadata --format-version 1 --no-default-features
run_and_retain default-dependencies cargo tree --edges normal --no-default-features
run_and_retain release-build cargo build --release --lib --no-default-features
run_and_retain linked-footprint make size
run_and_retain layout cargo run --release --example layout --no-default-features
run_and_retain rustdoc env RUSTDOCFLAGS=-Dwarnings make doc
run_and_retain rlib-size-observation \
  bash scripts/measure_rlib_size.sh "${CARGO_TARGET_DIR:-target}/release/deps"

if command -v cargo-kani >/dev/null 2>&1; then
  run_and_retain kani cargo kani
  echo passed >"$evidence_dir/kani-status.txt"
else
  echo skipped-unavailable >"$evidence_dir/kani-status.txt"
fi

python3 scripts/build_evidence_envelope.py "$evidence_dir"
run_and_retain input-schema \
  python3 scripts/validate_json_schema.py \
  schemas/runtime-evidence-input-v1.schema.json "$evidence_dir/collection-input.json"
run_and_retain manifest-schema \
  python3 scripts/validate_json_schema.py \
  schemas/runtime-evidence-manifest-v1.schema.json "$evidence_dir/evidence-manifest.json"

if [[ -n "${PGM01_SCHEMA:-}" ]]; then
  run_and_retain pgm01-schema \
    python3 scripts/validate_json_schema.py \
    "$PGM01_SCHEMA" "$evidence_dir/evidence-envelope.json"
  echo passed >"$evidence_dir/pgm01-schema-status.txt"
else
  echo skipped-unavailable >"$evidence_dir/pgm01-schema-status.txt"
fi

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
