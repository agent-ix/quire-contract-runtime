#!/usr/bin/env bash
# FR-275-AC-8 / TC-198: prove the `deny.toml` bans on QSL crates actually fire.
#
# For each guarded crate, in a scratch copy of the workspace that is never committed, add the
# crate as a normal dependency and as a dev dependency, run `cargo deny`, and require both a
# non-zero exit and cargo-deny's `banned` diagnostic naming the crate. A licence or source
# failure alone does not count. The unmodified tree must pass first.
#
# Usage: scripts/check_deny_bans.sh [crate ...]   (default: the TC-198 step 3 crates)
set -uo pipefail

readonly qsl_git='https://github.com/agent-ix/quire-spec-language'
readonly root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
if [[ $# -gt 0 ]]; then
  crates=("$@")
else
  crates=(qsl-eval qsl-replay qsl-semantics quire-spec-language)
fi

scratch="$(mktemp -d)"
trap 'rm -rf "$scratch"' EXIT

copy_workspace() {
  local dest="$1"
  mkdir -p "$dest"
  (cd "$root" && tar --exclude=./target --exclude=./.git --exclude='*-target' -cf - .) |
    (cd "$dest" && tar -xf -)
}

failures=0

copy_workspace "$scratch/base"
if ! (cd "$scratch/base" && cargo deny --workspace check licenses bans sources >/dev/null 2>&1); then
  echo "unmodified tree: cargo deny failed" >&2
  exit 1
fi
echo "unmodified tree: cargo deny passes"

for crate in "${crates[@]}"; do
  for table in dependencies dev-dependencies; do
    dir="$scratch/$crate-$table"
    copy_workspace "$dir"
    printf '\n[%s.%s]\ngit = "%s"\nbranch = "main"\n' "$table" "$crate" "$qsl_git" >>"$dir/Cargo.toml"
    output="$(cd "$dir" && cargo deny --workspace check licenses bans sources 2>&1)"
    status=$?
    # cargo-deny's banned diagnostic: `error[banned]: crate '<name> = <version>' is explicitly banned`
    if [[ $status -ne 0 ]] && grep -Eq "error\[banned\]: crate '$crate = " <<<"$output"; then
      echo "ok: $crate as $table fails with the banned diagnostic"
    else
      echo "FAIL: $crate as $table (exit $status) did not report error[banned] for $crate" >&2
      failures=$((failures + 1))
    fi
  done
done

exit $((failures > 0))
