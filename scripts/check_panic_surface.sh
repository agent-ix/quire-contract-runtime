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
  assert_tests_suffix_files_are_cfg_test_modules "$root" ${tests[@]+"${tests[@]}"}
}

# The two lists above are derived from the `*_tests.rs` suffix alone, which makes it the sole,
# implicit key that excludes a file from runtime_pattern: nothing else has to change for a file
# wearing that suffix to be held out of the stricter scan. A production module named
# `src/slice_tests.rs` containing real `split_at`/`copy_from_slice`/... calls would go unaudited
# under runtime_pattern, checked only against verification_pattern's narrower set -- an exemption
# that used to require a visible edit to this script's exclude list and now requires none (IR-41).
#
# This asserts every discovered `*_tests.rs` file is referenced by a #[cfg(test)]-gated `mod
# <stem>;` (the implicit-path form) or `#[path = "...<basename>"]` (the explicit-path form)
# declaration somewhere in the same tree -- so a file cannot wear the suffix for free; it must
# actually be compiled as a #[cfg(test)] module by something. This does not fully close the gap
# (a file could still be a *_tests.rs module that is itself real production code mistakenly
# behind #[cfg(test)]), but it converts "any file with this suffix" into "a file this tree
# actually treats as a test", which a stray production module would not be.
assert_tests_suffix_files_are_cfg_test_modules() {
  local root="$1"
  shift
  local rust_files=()
  local file stem basename

  while IFS= read -r file; do
    rust_files+=("$file")
  done < <(find "$root" -type f -name '*.rs')

  for file in "$@"; do
    basename="${file##*/}"
    stem="${basename%.rs}"
    if ! awk -v stem="$stem" -v base="$basename" '
      FNR == 1 { gate = -100 }
      /#\[cfg\(test\)\]/ { gate = FNR }
      $0 ~ ("mod[ \t]+" stem "[ \t]*;") && (FNR - gate) <= 2 { found = 1 }
      $0 ~ ("#\\[path[ \t]*=[ \t]*\"[^\"]*" base "\"\\]") && (FNR - gate) <= 2 { found = 1 }
      END { exit !found }
    ' "${rust_files[@]}"; then
      echo "no #[cfg(test)]-gated mod or #[path] declaration references $file" >&2
      exit 1
    fi
  done
}

scan_tree src
scan_tree measurement/footprint/src

# The whole verification tree is held to the verification pattern: it is compiled
# only under cfg(kani) and asserting is its purpose.
scan verification "$verification_pattern"

echo "runtime and verification panic-surface audit passed"
