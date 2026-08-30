#!/usr/bin/env bash
set -euo pipefail

pattern='panic!|unreachable!|todo!|unimplemented!|assert!|assert_eq!|assert_ne!|[.]unwrap\(|[.]expect\('
if ! command -v grep >/dev/null 2>&1; then
  echo "grep is required for the panic-surface audit" >&2
  exit 2
fi

set +e
grep -R -n -E --include='*.rs' "$pattern" src
status=$?
set -e

case "$status" in
  0)
    echo "intentional panic surface found in runtime source" >&2
    exit 1
    ;;
  1)
    ;;
  *)
    echo "panic-surface audit could not run (grep exit $status)" >&2
    exit 2
    ;;
esac

echo "runtime panic-surface audit passed"
