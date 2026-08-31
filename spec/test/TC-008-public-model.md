---
id: TC-008
title: "Inspect the provenance-bearing public model"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-runtime/FR-001
    type: verifies
  - target: ix://agent-ix/quire-contract-runtime/FR-004
    type: verifies
  - target: ix://agent-ix/quire-contract-runtime/NFR-002
    type: verifies
---
# TC-008: Inspect the provenance-bearing public model

## Description

Verify downstream code cannot turn rejection into success by mutating evidence counters and cannot
silently exhaustively match public data enums that may gain future states.

## Test Procedure

Run the `tc_008_evidence_model_is_non_exhaustive_and_opaque` source-policy test and all five
compile-fail doctests attached to the public non-exhaustive enums. The source-policy test recursively
parses every shipped runtime source file, including modules attached by `#[path]`, and fails on any
crate-level public-surface drift, extra accounting inherent blocks (including private-alias and
cross-file blocks), extra trait implementations (including reference self types), extra public
functions mentioning direct or aliased accounting types, or unexpected macros. Inspect the generated
public API documentation with warnings denied.

## Expected Results

All report and counter fields remain private, read-only accessors expose complete counts, no
crate-provided public API outside the reviewed adapter can reset those types, and every downstream
exhaustive match is rejected by the compiler because each governed enum is `#[non_exhaustive]`.
