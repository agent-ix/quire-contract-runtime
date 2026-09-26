---
id: TC-036
title: "Pin evaluate_integer_arithmetic's direct Outcome construction"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-runtime/FR-007
    type: verifies
---
# TC-036: Pin evaluate_integer_arithmetic's direct Outcome construction

## Description

`evaluate_integer_arithmetic` builds its `Outcome<Integer>` directly rather than through an inner
`Result<Integer, Stop>` round trip: `Result`'s discriminant sits in a niche inside `Stop`'s own tag
that Kani writes over with nondeterministic bytes, so CBMC cannot fold it back on the call path (a
Kani-provability rule with no owning FR or NFR of its own; see `AD-002`'s Risks section). No type
signature distinguishes a direct build from an equivalent `Result`-round-trip rewrite that reaches
the same `Outcome` values, so this is a source check rather than a behavioural one.
Evidence: `tests/exact_arithmetic.rs` (`--features exact`).

## Test Procedure

1. Extract `evaluate_integer_arithmetic`'s own function body from `src/exact/numeric.rs` by its
   signature and its first unindented closing brace.
2. Strip comment lines, then check the body contains no `Result<` type annotation (which a
   `Result<Integer, Stop>` or `Result<_, Stop>` round trip would introduce) and does build
   `Outcome::Completed(result)` directly.

## Expected Results

The function's body never spells a `Result<...>` type, and it constructs `Outcome::Completed`
directly. A rewrite that stages the result through a `Result<Integer, Stop>` (whether via
`Outcome::from_stop` or a `match` on a locally bound `Result`) fails this check.
