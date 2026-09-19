#!/usr/bin/env bash
set -euo pipefail

runtime_pattern='panic!|unreachable!|todo!|unimplemented!|assert!|assert_eq!|assert_ne!|[.]unwrap\(|[.]expect\(|[.]split_at(_mut)?\(|[.]r?chunks(_mut|_exact|_exact_mut)?\(|[.]windows\(|[.]copy_within\(|[.]swap\(|[.]step_by\(|[.](copy|clone)_from_slice\(|[.]swap_with_slice\(|[.]rotate_(left|right)\(|[.]borrow_mut\(|[.]borrow\('
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

# In-tree test modules are held to the verification pattern rather than the
# runtime one: a test may assert, but it may still not panic!, unwrap or expect.
#
# The two lists below — which files to hold out of the runtime scan, and which
# to then scan as tests — are derived from the tree rather than written down.
# When they were maintained by hand they drifted: `src/exact_accounting_tests.rs`
# arrived in #26 while the exclude still named only `accounting_tests.rs`, so
# `make audit-panic` went red on main (#30). Deriving both from one `find` makes
# the two lists incapable of disagreeing, and a new `*_tests.rs` file is picked
# up with no edit here. Adding the file to the exclude alone would have been the
# wrong repair: it would have dropped the file from auditing entirely, which is
# a check that cannot fail rather than a check that passed.
scan_tree() {
  local root="$1"
  local excludes=()
  local tests=()
  local file

  while IFS= read -r file; do
    tests+=("$file")
    excludes+=(--exclude="${file##*/}")
  done < <(find "$root" -type f -name '*_tests.rs' | sort)

  scan "$root" "$runtime_pattern" ${excludes[@]+"${excludes[@]}"}
  for file in ${tests[@]+"${tests[@]}"}; do
    scan "$file" "$verification_pattern"
  done
}

scan_tree src
scan_tree measurement/footprint/src

# The whole verification tree is held to the verification pattern: it is compiled
# only under cfg(kani) and asserting is its purpose.
scan verification "$verification_pattern"

echo "runtime and verification panic-surface audit passed"
