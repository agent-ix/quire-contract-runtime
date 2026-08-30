#!/usr/bin/env bash
set -euo pipefail

runtime_pattern='panic!|unreachable!|todo!|unimplemented!|assert!|assert_eq!|assert_ne!|[.]unwrap\(|[.]expect\(|[.]split_at(_mut)?\(|[.]chunks(_mut|_exact|_exact_mut)?\(|[.](copy|clone)_from_slice\(|[.]swap_with_slice\(|[.]rotate_(left|right)\('
verification_pattern='panic!|unreachable!|todo!|unimplemented!|[.]unwrap\(|[.]expect\('
if ! command -v grep >/dev/null 2>&1; then
  echo "grep is required for the panic-surface audit" >&2
  exit 2
fi

scan() {
  local path="$1"
  local pattern="$2"
  shift 2
  local status

  set +e
  grep -R -n -E --include='*.rs' "$@" "$pattern" "$path"
  status=$?
  set -e

  case "$status" in
    0)
      echo "intentional panic surface found beneath $path" >&2
      exit 1
      ;;
    1)
      ;;
    *)
      echo "panic-surface audit could not scan $path (grep exit $status)" >&2
      exit 2
      ;;
  esac
}

scan src "$runtime_pattern" --exclude='accounting_tests.rs'
scan src/accounting_tests.rs "$verification_pattern"
scan verification "$verification_pattern"
scan measurement/footprint/src "$runtime_pattern"

echo "runtime and verification panic-surface audit passed"
