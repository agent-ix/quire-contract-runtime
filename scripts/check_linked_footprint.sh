#!/usr/bin/env bash
# Measure the linked thumbv7em-none-eabi release footprint against MP-001's
# governed floor and ceiling, and its panic-relocation requirement.
#
# With --json this is the PROOF-footprint producer and emits
# runtime.footprint/v1 on stdout. Without it, it is the human gate.
#
# The measurement itself is unchanged from the one MP-001 governs: the crate's
# own .text and .rodata sections in the linked staticlib, and the relocations
# that would drag in a panic path. What --json adds is a structured result, so
# that no downstream consumer has to read the human line to learn the answer.
#
# Exit status
#   gate mode : 0 inside the governed band with no panic relocation, 1 outside
#               it, 2 when the measurement could not be taken at all
#   --json    : 0 whenever a document was produced, whatever it says
set -euo pipefail

readonly target='thumbv7em-none-eabi'
readonly minimum_bytes=500
readonly limit_bytes=4096
readonly protocol='runtime.footprint/v1'

json_output=0
if [[ "${1:-}" == '--json' ]]; then
  json_output=1
  shift
fi
artifact_path="${1:-target/footprint-msrv/$target/release/libquire_contract_runtime_footprint.a}"

emit_unavailable() {
  local reason="$1"
  if (( json_output )); then
    printf '{\n  "protocol": "%s",\n  "artifact": "%s",\n  "target": "%s",\n  "entries": [\n' \
      "$protocol" "$artifact_path" "$target"
    printf '    {"symbol": "linked-text-rodata-bytes", "outcome": "unavailable", "traceIds": ["NFR-001", "MP-001"], "detail": "%s"},\n' "$reason"
    printf '    {"symbol": "panic-relocations", "outcome": "unavailable", "traceIds": ["NFR-002", "MP-001"], "detail": "%s"}\n' "$reason"
    printf '  ]\n}\n'
    exit 0
  fi
  echo "$reason" >&2
  exit 2
}

if ! command -v size >/dev/null 2>&1; then
  emit_unavailable "size is required for the linked-footprint audit"
fi
if ! command -v objdump >/dev/null 2>&1; then
  emit_unavailable "objdump is required for the linked-footprint audit"
fi
if [[ ! -f "$artifact_path" ]]; then
  emit_unavailable "footprint staticlib not found at $artifact_path"
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
  emit_unavailable "could not measure linked .text/.rodata sections"
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
  emit_unavailable "could not inspect linked footprint relocations"
fi

panic_count=0
if [[ -n "$panic_references" ]]; then
  panic_count="$(printf '%s\n' "$panic_references" | wc -l | tr -d ' ')"
fi

size_outcome='pass'
if (( section_bytes < minimum_bytes || section_bytes > limit_bytes )); then
  size_outcome='fail'
fi
panic_outcome='pass'
if (( panic_count > 0 )); then
  panic_outcome='fail'
fi

if (( json_output )); then
  printf '{\n  "protocol": "%s",\n  "artifact": "%s",\n  "target": "%s",\n  "entries": [\n' \
    "$protocol" "$artifact_path" "$target"
  printf '    {"symbol": "linked-text-rodata-bytes", "outcome": "%s", "traceIds": ["NFR-001", "MP-001"], "measured": %s, "minimum": %s, "limit": %s},\n' \
    "$size_outcome" "$section_bytes" "$minimum_bytes" "$limit_bytes"
  printf '    {"symbol": "panic-relocations", "outcome": "%s", "traceIds": ["NFR-002", "MP-001"], "measured": %s, "limit": 0}\n' \
    "$panic_outcome" "$panic_count"
  printf '  ]\n}\n'
  exit 0
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
