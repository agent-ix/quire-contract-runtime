---
id: TC-035
title: "Pin boxed value and type structs' hand-written Debug rendering"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-runtime/FR-007
    type: verifies
---
# TC-035: Pin boxed value and type structs' hand-written Debug rendering

## Description

`Rational`, `RationalDomain`, `Decimal`, `DecimalType`, `Quantity`, `Text`, `EnumValue`,
`ObjectReference` and `CompoundUnit` each hold their fields behind one `Box` (a Kani-provability
layout constraint; see `AD-002`'s risk section), which turned their derived `Debug` into a
hand-written impl that lists each `*Fields` field explicitly. A field added to a `*Fields` struct
but not to its `Debug` impl would disappear from the rendering with nothing to notice, since
FR-007-AC-6's own Debug-parity oracle (equal renderings against the pinned authority) lives in
`conformance/qsl-agreement`, which does not compile independently of this concern.
Evidence: `tests/exact_debug_parity.rs` (`--features exact`).

This is not FR-007-AC-6's oracle: it makes no comparison against the pinned authority, and it
covers only these nine boxed structs, not the shared corpus. It is a same-tree regression pin.

## Test Procedure

1. Construct one instance of each of the nine boxed structs through its public API.
2. Render each with `{:?}` and compare the result to a fixed string listing every field the
   corresponding `*Fields` struct declares, in declaration order.

## Expected Results

Every rendering matches its pinned string exactly. A field added to a `*Fields` struct without a
matching addition to its `Debug` impl changes the rendering and fails the comparison.
