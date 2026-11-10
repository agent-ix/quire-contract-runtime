---
id: TC-035
title: "Pin the consumed CompoundUnit Debug rendering"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-runtime/FR-007
    type: verifies
---
# TC-035: Pin the consumed CompoundUnit Debug rendering

## Description

The runtime consumes QSV's `CompoundUnit`. Preserve a same-tree regression pin for its `Debug`
rendering through `tests/exact_unit_owner.rs` (`--features exact`), the one remaining RT-owned
obligation in FR-007-AC-13.

## Test Procedure

1. Construct a `CompoundUnit` through its public API.
2. Render it with `{:?}` and compare the result to a fixed string listing every field its
   `CompoundUnitFields` struct declares, in declaration order.

## Expected Results

The `CompoundUnit` rendering matches its pinned string exactly. A field added to `CompoundUnitFields`
without a matching addition to its `Debug` impl changes the rendering and fails the comparison.
