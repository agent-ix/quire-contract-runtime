#!/usr/bin/env bash
set -euo pipefail

artifact_dir="${1:-${CARGO_TARGET_DIR:-target}/release/deps}"
rlib_path="$({
  find "$artifact_dir" -maxdepth 1 -type f -name 'libquire_contract_runtime-*.rlib' \
    -printf '%T@ %p\n' 2>/dev/null || true
} | sort -nr | head -n 1 | cut -d' ' -f2-)"

if [[ -z "$rlib_path" ]]; then
  echo "release rlib not found beneath $artifact_dir" >&2
  exit 2
fi

size_bytes="$(wc -c <"$rlib_path")"
printf 'artifact=%s bytes=%s enforcement=observation-only\n' "$rlib_path" "$size_bytes"
