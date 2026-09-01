#!/usr/bin/env bash
set -euo pipefail

# Implements: NFR-002
run_and_retain() {
  local name="$1"
  shift
  set +e
  "$@" >"$evidence_dir/$name.stdout" 2>"$evidence_dir/$name.stderr"
  local status=$?
  set -e
  echo "$status" >"$evidence_dir/$name.status.txt"
  if (( status != 0 )); then
    collection_failed=1
  fi
  return 0
}

record_status_word() {
  local numeric_status="$1"
  local word_status="$2"
  if [[ "$(<"$numeric_status")" == 0 ]]; then
    echo passed >"$word_status"
  else
    echo failed >"$word_status"
  fi
}

envelope_matches_sha256() {
  local expected="$1"
  [[ "$(sha256sum "$evidence_dir/evidence-envelope.json")" == "$expected" ]]
}

if [[ "${1:-}" == "--self-test" ]]; then
  self_test_dir="$(mktemp -d)"
  trap 'rm -rf -- "$self_test_dir"' EXIT
  evidence_dir="$self_test_dir"
  collection_failed=0

  run_and_retain passing true
  [[ "$(<"$evidence_dir/passing.status.txt")" == 0 ]]
  (( collection_failed == 0 ))

  run_and_retain failing false
  [[ "$(<"$evidence_dir/failing.status.txt")" != 0 ]]
  (( collection_failed == 1 ))

  record_status_word "$evidence_dir/passing.status.txt" "$evidence_dir/passing-word.txt"
  record_status_word "$evidence_dir/failing.status.txt" "$evidence_dir/failing-word.txt"
  [[ "$(<"$evidence_dir/passing-word.txt")" == passed ]]
  [[ "$(<"$evidence_dir/failing-word.txt")" == failed ]]

  echo stable >"$evidence_dir/evidence-envelope.json"
  self_test_sha256="$(sha256sum "$evidence_dir/evidence-envelope.json")"
  envelope_matches_sha256 "$self_test_sha256"
  echo changed >>"$evidence_dir/evidence-envelope.json"
  if envelope_matches_sha256 "$self_test_sha256"; then
    echo "collector fixed-point self-test accepted a changed envelope" >&2
    exit 1
  fi

  echo "collector fail-closed self-test passed"
  exit 0
fi

if [[ $# -gt 0 ]]; then
  final_evidence_dir="$1"
else
  evidence_revision="$(git rev-parse --short=12 HEAD)"
  evidence_timestamp="$(date -u +%Y%m%dT%H%M%SZ)"
  final_evidence_dir="evidence/runtime-v01-${evidence_revision}-${evidence_timestamp}"
fi
if [[ -e "$final_evidence_dir" ]]; then
  echo "refusing to overwrite retained evidence: $final_evidence_dir" >&2
  exit 2
fi
if [[ -n "$(git status --porcelain --untracked-files=all)" ]]; then
  echo "refusing to collect evidence from a modified or untracked source tree" >&2
  exit 2
fi
source_state=clean
evidence_dir="target/evidence-staging/$(basename "$final_evidence_dir")"
if [[ -e "$evidence_dir" ]]; then
  echo "refusing to overwrite staged evidence: $evidence_dir" >&2
  exit 2
fi
trusted_home="$(/usr/bin/python3 -c 'import os,pwd; print(pwd.getpwuid(os.getuid()).pw_dir)')"
export HOME="$trusted_home"
export CARGO_HOME="$trusted_home/.cargo"
export RUSTUP_HOME="$trusted_home/.rustup"
export CARGO_TARGET_DIR="$PWD/target"
export PYTHONPYCACHEPREFIX="$CARGO_TARGET_DIR/pycache"
export PATH="$trusted_home/.cargo/bin:$trusted_home/.npm-global/bin:/usr/bin:/bin"
: "${PGM01_SCHEMA:?PGM01_SCHEMA must name the exact merged PGM-01 schema}"
: "${PGM01_VALIDATOR:?PGM01_VALIDATOR must name the exact merged PGM-01 validator}"
pgm01_schema_path="$(realpath "$PGM01_SCHEMA")"
pgm01_validator_path="$(realpath "$PGM01_VALIDATOR")"
pgm01_repo="$(git -C "$(dirname "$pgm01_validator_path")" rev-parse --show-toplevel)"
if [[ "$(git -C "$pgm01_repo" rev-parse HEAD)" != 7dac9d8c19952412b56a0347387666e2ca81e01d ]]; then
  echo "PGM-01 checkout must be exact merged revision 7dac9d8c19952412b56a0347387666e2ca81e01d" >&2
  exit 2
fi
if [[ -n "$(git -C "$pgm01_repo" status --porcelain --untracked-files=all)" ]]; then
  echo "PGM-01 checkout must be clean" >&2
  exit 2
fi
if [[ "$(sha256sum "$pgm01_validator_path" | cut -d' ' -f1)" != "$(sha256sum schemas/pgm01-validate-governance.py | cut -d' ' -f1)" ]]; then
  echo "PGM-01 validator differs from the vendored verification copy" >&2
  exit 2
fi
IFS= read -r expected_pgm01_validator_blob <schemas/pgm01-validator-blob.txt
if [[ "$(git -C "$pgm01_repo" rev-parse HEAD:scripts/validate_governance.py)" != "$expected_pgm01_validator_blob" ]]; then
  echo "PGM-01 validator blob differs from the committed upstream anchor" >&2
  exit 2
fi
if ! python3 -c 'from scripts.validate_json_schema import checked_format_checker; checked_format_checker()' >/dev/null 2>&1; then
  echo "the exact requirements-evidence.txt schema packages are required" >&2
  exit 2
fi
mkdir -p "$(dirname "$evidence_dir")" "$evidence_dir"
collection_failed=0

record_tool_identity() {
  local name="$1"
  local path="$2"
  if [[ ! -f "$path" ]]; then
    echo "required tool is unavailable: $name at $path" >&2
    exit 2
  fi
  echo "$path" >"$evidence_dir/$name-path.txt"
  sha256sum "$path" | cut -d' ' -f1 >"$evidence_dir/$name-sha256.txt"
}

stable_cargo="$(rustup which --toolchain stable cargo)"
stable_rustc="$(rustup which --toolchain stable rustc)"
msrv_rustc="$(rustup which --toolchain 1.75.0 rustc)"
record_tool_identity cargo "$stable_cargo"
record_tool_identity cargo-kani "$trusted_home/.cargo/bin/cargo-kani"
record_tool_identity make /usr/bin/make
record_tool_identity python /usr/bin/python3
record_tool_identity quire "$trusted_home/.npm-global/bin/quire"
record_tool_identity rustc "$stable_rustc"
record_tool_identity msrv-rustc "$msrv_rustc"
record_tool_identity size /usr/bin/size

git rev-parse HEAD >"$evidence_dir/source-revision.txt"
echo "$source_state" >"$evidence_dir/source-state.txt"
rustc --version --verbose >"$evidence_dir/rustc-version.txt"
rustc +1.75.0 --version --verbose >"$evidence_dir/msrv-rustc-version.txt"
size --version >"$evidence_dir/size-version.txt"
cargo --version --verbose >"$evidence_dir/cargo-version.txt"
python3 --version >"$evidence_dir/python-version.txt"
python3 -c 'import jsonschema; print(jsonschema.__version__)' >"$evidence_dir/jsonschema-version.txt"
python3 -m pip freeze --all >"$evidence_dir/python-packages.txt"
sha256sum "$pgm01_schema_path" | cut -d' ' -f1 >"$evidence_dir/pgm01-schema-sha256.txt"
sha256sum "$pgm01_validator_path" | cut -d' ' -f1 >"$evidence_dir/pgm01-validator-sha256.txt"
git -C "$pgm01_repo" rev-parse HEAD >"$evidence_dir/pgm01-revision.txt"
quire provenance --pretty >"$evidence_dir/quire-provenance.json"
run_and_retain ci-guard /usr/bin/python3 scripts/check_failure_propagation.py
run_and_retain kani-census /usr/bin/python3 scripts/check_kani_harnesses.py
run_and_retain evidence-tool /usr/bin/python3 scripts/run_evidence_tests.py
run_and_retain quire-validate \
  bash -c "quire validate --scope . 'spec/**/*.md' 'planning/**/*.md' 'plan/**/*.md' && echo QUIRE_VALIDATION_PASSED"
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
run_and_retain coverage python3 scripts/check_coverage_status.py

if [[ -f "$trusted_home/.cargo/bin/cargo-kani" ]]; then
  cargo kani --version >"$evidence_dir/kani-version.txt"
  run_and_retain kani make kani
  record_status_word "$evidence_dir/kani.status.txt" "$evidence_dir/kani-status.txt"
else
  echo skipped-unavailable >"$evidence_dir/kani-version.txt"
  echo skipped-unavailable >"$evidence_dir/kani-status.txt"
fi

run_schema_validators() {
  run_and_retain pgm01-pinned-schema \
    python3 scripts/validate_json_schema.py \
    schemas/pgm01-derivation-evidence-envelope-v1.schema.json \
    "$evidence_dir/evidence-envelope.json"
  run_and_retain input-schema \
    python3 scripts/validate_json_schema.py \
    schemas/runtime-evidence-input-v1.schema.json "$evidence_dir/collection-input.json"
  run_and_retain manifest-schema \
    python3 scripts/validate_json_schema.py \
    schemas/runtime-evidence-manifest-v1.schema.json "$evidence_dir/evidence-manifest.json"

  run_and_retain pgm01-schema \
    python3 scripts/validate_json_schema.py \
    "$pgm01_schema_path" "$evidence_dir/evidence-envelope.json"
  record_status_word \
    "$evidence_dir/pgm01-schema.status.txt" \
    "$evidence_dir/pgm01-schema-status.txt"

  run_and_retain pgm01-envelope \
    python3 "$pgm01_validator_path" --fixture "$evidence_dir/evidence-envelope.json"
  record_status_word \
    "$evidence_dir/pgm01-envelope.status.txt" \
    "$evidence_dir/pgm01-envelope-status.txt"
}

# Build once to create validator inputs, validate them, then rebuild with those
# retained outcomes. Revalidating the final form ensures the accepted document
# is exactly the deterministic form that will be checksummed.
python3 scripts/build_evidence_envelope.py "$evidence_dir"
run_schema_validators
python3 scripts/build_evidence_envelope.py "$evidence_dir"
run_schema_validators
validated_envelope_sha256="$(sha256sum "$evidence_dir/evidence-envelope.json")"
python3 scripts/build_evidence_envelope.py "$evidence_dir"
if ! envelope_matches_sha256 "$validated_envelope_sha256"; then
  echo "evidence envelope changed after final validation" >&2
  collection_failed=1
fi

(
  cd "$evidence_dir"
  find . -maxdepth 1 -type f ! -name sha256sums.txt -print0 \
    | sort -z \
    | xargs -0 sha256sum >sha256sums.txt
)

if [[ "$(python3 -c 'import json, sys; print(json.load(open(sys.argv[1]))["result"]["status"])' "$evidence_dir/evidence-envelope.json")" != conclusive ]]; then
  collection_failed=1
fi

mkdir -p "$(dirname "$final_evidence_dir")"
mv "$evidence_dir" "$final_evidence_dir"
evidence_dir="$final_evidence_dir"

if (( collection_failed != 0 )); then
  echo "one or more retained evidence commands failed" >&2
  exit 1
fi

python3 -c '
import pathlib, sys
from scripts.verify_evidence import verify_record
verify_record(pathlib.Path(sys.argv[1]))
' "$evidence_dir"
echo "collected and verified $evidence_dir; update evidence/ANCHORS as a separate review-boundary step"
