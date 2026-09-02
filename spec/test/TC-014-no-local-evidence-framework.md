---
id: TC-014
title: "Prove no generic evidence machinery remains and the frozen schemas bind nothing"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-runtime/FR-005
    type: verifies
---
# TC-014: Prove no generic evidence machinery remains and the frozen schemas bind nothing

## Description

Verify that the collector, envelope builder, verifier, anchor writer, recipe-failure policer, tool
identity lock, and generic traceability checker are gone by name; that the four frozen schema
artifacts are still present and byte-identical, because retained records name each of them by
SHA-256; and that no source file, Make target, or workflow step references any of them.

## Test Procedure

Assert the absence of each removed path. Hash each frozen artifact and compare against its pinned
digest. Walk every source file under `scripts`, `tests`, `src`, `verification`, `measurement`,
`spec`, and `.github`, plus `Makefile`, `Cargo.toml`, and `requirements-assurance.txt`, and assert
that none names a frozen schema. Assert the census inspected enough files to be meaningful.

## Expected Results

Every removed path is absent, every frozen artifact is unchanged, no source references one, the
Makefile carries no self-attestation target, and the census size is large enough that the claim
could not pass by inspecting nothing.
