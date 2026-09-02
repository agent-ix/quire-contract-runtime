---
id: TC-011
title: "Bind the sealed record's impact snapshot to the Quire static export"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-runtime/FR-005
    type: verifies
---
# TC-011: Bind the sealed record's impact snapshot to the Quire static export

## Description

Verify that the impact snapshot in the sealed change-assurance record is the SHA-256 of the Quire
static export for the candidate revision, that the export is a populated document naming every
requirement this repository declares, and that the chain attested it `passed` rather than
`not_computed`.

## Test Procedure

Read the chain report's `impact_snapshot_digest` and `quire_export`, hash the exported file
independently, and compare. Assert the export text names FR-001 through FR-005, NFR-001, NFR-002,
and StR-001.

## Expected Results

The digests agree, the export is a non-empty object naming every requirement, and its attested
result is `passed`. An empty document has a digest too, so the content is asserted and not only the
binding.
