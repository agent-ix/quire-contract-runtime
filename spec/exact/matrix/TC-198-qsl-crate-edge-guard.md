---
id: TC-198
title: "Fail the build on a dependency from the QSL Git source"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-runtime/FR-275
    type: verifies
---
# TC-198: Fail the build on a dependency from the QSL Git source

## Description

Check that `deny.toml` admits only the shared crates' own Git repositories and that `make deny`
fails with the sources diagnostic when a crate from the QSL Git repository enters the
dependency graph. The negative cases run in a scratch copy of the workspace that is never committed.

## Test Procedure

1. Read `deny.toml`; expect `unknown-git = "deny"`, own-repository `allow-git` entries for
   `quire-exact`, `quire-semantic-value` and `quire-canonical`, and no QSL Git source (FR-275-AC-7).
2. Run `make deny` on the unmodified tree; expect success.
3. In a scratch copy, add `qsl-eval` as a normal dependency from the QSL repository; run
   `make deny`; expect a non-zero exit that reports cargo-deny's `source-not-allowed` error for
   the QSL Git source. A licence error alone does not satisfy this step: that source's diagnostic
   must be present. Repeat with
   `qsl-replay`, `qsl-semantics` and `quire-spec-language`. Each failure is FR-275-AC-8.
4. Repeat step 3 with each of those crates as a dev dependency and as a build dependency; expect
   the same source error. `make deny-mutations` (`scripts/check_deny_bans.sh`) runs steps 2 to 4
   in scratch copies and requires the QSL source diagnostic for each case.

## Expected Results

Each addition fails `make deny` with the QSL Git source error; the unmodified tree passes.
