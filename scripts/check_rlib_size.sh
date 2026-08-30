#!/usr/bin/env bash
set -euo pipefail

artifact_dir="${1:-${CARGO_TARGET_DIR:-target}/release/deps}"
limit_bytes="${RLIB_SIZE_LIMIT_BYTES:-262144}"

if [[ ! "$limit_bytes" =~ ^[0-9]+$ ]]; then
  echo "RLIB_SIZE_LIMIT_BYTES must be a non-negative integer" >&2
  exit 2
fi

rlib_path="$({
  find "$artifact_dir" -maxdepth 1 -type f -name 'libquire_contract_runtime-*.rlib' \
    -printf '%T@ %p\n' 2>/dev/null || true
} | sort -nr | head -n 1 | cut -d' ' -f2-)"

if [[ -z "$rlib_path" ]]; then
  echo "release rlib not found beneath $artifact_dir" >&2
  exit 2
fi

size_bytes="$(wc -c <"$rlib_path")"
printf 'artifact=%s bytes=%s limit=%s\n' "$rlib_path" "$size_bytes" "$limit_bytes"

if (( size_bytes > limit_bytes )); then
  echo "release rlib exceeds the configured byte ceiling" >&2
  exit 1
fi
