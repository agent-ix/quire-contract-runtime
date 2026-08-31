#!/usr/bin/env bash
set -euo pipefail

readonly target='thumbv7em-none-eabi'
readonly minimum_bytes=500
readonly limit_bytes=4096
artifact_path="${1:-target/footprint-msrv/$target/release/libquire_contract_runtime_footprint.a}"

if ! command -v size >/dev/null 2>&1; then
  echo "size is required for the linked-footprint audit" >&2
  exit 2
fi
if ! command -v objdump >/dev/null 2>&1; then
  echo "objdump is required for the linked-footprint audit" >&2
  exit 2
fi
if [[ ! -f "$artifact_path" ]]; then
  echo "footprint staticlib not found at $artifact_path" >&2
  exit 2
fi

set +e
section_bytes="$(size -A "$artifact_path" 2>/dev/null | awk '
  /[(]ex .*libquire_contract_runtime_footprint[.]a[)]:$/ {
    include = ($1 ~ /quire_contract_runtime/ && $1 !~ /[.]core-/)
    next
  }
  include && ($1 ~ /^[.]text([.]|$)/ || $1 ~ /^[.]rodata([.]|$)/) {
    total += $2
    found = 1
  }
  END { if (!found) exit 2; print total }
')"
status=$?
set -e
if [[ "$status" -ne 0 || ! "$section_bytes" =~ ^[0-9]+$ ]]; then
  echo "could not measure linked .text/.rodata sections" >&2
  exit 2
fi

set +e
panic_references="$(objdump -r "$artifact_path" 2>/dev/null | awk '
  /:     file format/ {
    include = ($1 ~ /quire_contract_runtime/ && $1 !~ /[.]core-/)
    next
  }
  include && /rust_begin_unwind|panic_bounds_check|core.*panicking|slice.*index.*fail/ {
    print
  }
')"
panic_status=$?
set -e
if [[ "$panic_status" -ne 0 ]]; then
  echo "could not inspect linked footprint relocations" >&2
  exit 2
fi
if [[ -n "$panic_references" ]]; then
  printf '%s\n' "$panic_references" >&2
  echo "linked footprint retains a panic path" >&2
  exit 1
fi

printf 'artifact=%s target=%s sections=.text+.rodata bytes=%s minimum=%s limit=%s panic_references=0\n' \
  "$artifact_path" "$target" "$section_bytes" "$minimum_bytes" "$limit_bytes"
if (( section_bytes < minimum_bytes )); then
  echo "linked release footprint fell below the fixed population floor" >&2
  exit 1
fi
if (( section_bytes > limit_bytes )); then
  echo "linked release footprint exceeds the fixed byte ceiling" >&2
  exit 1
fi
