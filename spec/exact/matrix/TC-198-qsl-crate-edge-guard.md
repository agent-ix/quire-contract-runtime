---
id: TC-198
title: "Fail the build on a dependency on a guarded QSL crate"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-runtime/FR-275
    type: verifies
---
# TC-198: Fail the build on a dependency on a guarded QSL crate

## Description

Check that `deny.toml` bans every QSL workspace crate listed in FR-275 under Guarded edges (all but
`quire-exact`) and that `make deny` fails with the bans diagnostic when any of them enters the
dependency graph. The negative cases run in a scratch copy of the workspace that is never committed.

## Test Procedure

1. Read `deny.toml`; expect a `[bans]` `deny` entry for each listed crate (FR-275-AC-7).
2. Run `make deny` on the unmodified tree; expect success.
3. In a scratch copy, add `qsl-eval` as a normal dependency from the QSL repository; run
   `make deny`; expect a non-zero exit that reports cargo-deny's `banned` error for `qsl-eval`. A
   licence error alone does not satisfy this step: the `banned` line must be present. Repeat with
   `qsl-replay`, `qsl-semantics` and `quire-spec-language`. Each failure is FR-275-AC-8.
4. Repeat step 3 with each of those crates as a dev dependency; expect the same `banned` error.
5. Compare the list with QSL's current workspace members; report any member missing from the list
   and not named `quire-exact` or a residue crate QSL-358 names.

## Expected Results

Each addition fails `make deny` with the `banned` error naming the crate; the unmodified tree
passes; the list covers every QSL workspace member but the allowed crates.
