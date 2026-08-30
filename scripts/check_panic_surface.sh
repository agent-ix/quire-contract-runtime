#!/usr/bin/env bash
set -euo pipefail

pattern='panic!|unreachable!|todo!|unimplemented!|assert!|assert_eq!|assert_ne!|[.]unwrap\(|[.]expect\('
if rg -n "$pattern" src --glob '*.rs'; then
  echo "intentional panic surface found in runtime source" >&2
  exit 1
fi

echo "runtime panic-surface audit passed"

