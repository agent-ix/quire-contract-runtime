---
id: TC-005
title: "Build the feature matrix"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-runtime/NFR-001
    type: verifies
---
# TC-005: Build the feature matrix

## Description

Verify the core builds alone and every declared optional feature combination builds explicitly.

## Test Procedure

Run tests with no default features, with `alloc`, with `std`, and with all features.

## Expected Results

Every supported combination compiles and passes; proptest symbols are absent from the default API.

