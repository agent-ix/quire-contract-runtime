---
id: TC-014
title: "Prove no generic evidence machinery remains"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-runtime/FR-005
    type: verifies
---
# TC-014: Prove no generic evidence machinery remains

## Description

Verify that the collector, envelope builder, verifier, anchor writer, recipe-failure policer, tool
identity lock, and generic traceability checker are gone by name; that the retained-evidence tree,
its reader, its fixtures, and the schema family frozen only because those records named it are gone
by name too; and that no source file, Make target, or workflow step references any of them.

## Test Procedure

Assert the absence of each removed path. Walk the repository root, excluding rather than listing the
directories to look in, and collect every code, configuration and workflow file — by extension, plus
the extensionless `Makefile`, which is the one file a reintroduced Make target could live in. Assert
that none names the deleted reader, its fixture directory, the retained-evidence tree, the deleted
schema directory, or the mapping symbols they used. Assert the census inspected enough files to be
meaningful.

## Expected Results

Every removed path is absent, no source references one, the Makefile carries no self-attestation
target, and the census size is large enough that the claim could not pass by inspecting nothing.
