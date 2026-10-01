---
id: TC-001
title: "Preserve tri-state verdict identity"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-runtime/FR-001
    type: verifies
---
# TC-001: Preserve tri-state verdict identity

## Description

Verify every verdict retains its identity, execution point, observations, and failure meaning.

## Test Procedure

Construct each public verdict variant with fixed borrowed records, inspect it through accessors and
pattern matching, and compile the default crate without optional features.

## Expected Results

All identities and observations match exactly; failure and rejection remain distinct from pass.

