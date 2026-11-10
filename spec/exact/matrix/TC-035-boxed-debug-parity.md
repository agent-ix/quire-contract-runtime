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
hand-written impl that lists each `*Fields` field explicitly. A field added to a `*Fields` struct but not to its `Debug` impl would disappear from the rendering.
The kernel move retired RT's boxed scalar and enum implementations. The retained RT evidence is
`tests/exact_debug_parity.rs` (`--features exact`) for `CompoundUnit`; owner evidence for the
remaining types requires mapping.

## Test Procedure

1. Construct one instance of each of the nine boxed structs through its public API.
2. Render each with `{:?}` and compare the result to a fixed string listing every field the
   corresponding `*Fields` struct declares, in declaration order.

## Expected Results

Every rendering matches its pinned string exactly. A field added to a `*Fields` struct without a
matching addition to its `Debug` impl changes the rendering and fails the comparison.
