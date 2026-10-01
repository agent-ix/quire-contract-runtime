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

Check that `deny.toml` bans `qsl-eval`, `qsl-replay` and `quire-spec-language` and that `make deny`
fails when any of them enters the dependency graph. The negative cases run in a scratch copy of the
workspace that is never committed.

## Test Procedure

1. Read `deny.toml`; expect a `[bans]` `deny` entry for each of the three crates (FR-275-AC-7).
2. Run `make deny` on the unmodified tree; expect success.
3. In a scratch copy, add `qsl-eval` as a normal dependency from the QSL repository; run
   `make deny`; expect a non-zero exit that names `qsl-eval` as banned. Repeat with `qsl-replay` and
   with `quire-spec-language`. Each failure is FR-275-AC-8.
4. Repeat step 3 with each crate as a dev dependency; expect the same failure.

## Expected Results

Each of the six additions fails `make deny` and names the banned crate; the unmodified tree passes.
