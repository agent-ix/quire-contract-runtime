---
id: TC-012
title: "Read every retained evidence byte through the pinned mapping without moving one"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-runtime/FR-005
    type: verifies
---
# TC-012: Read every retained evidence byte through the pinned mapping without moving one

## Description

Verify that all 42 retained envelopes, current and historical, are read through
`engineering_assurance.verification_semantics.map_pgm01_bytes`; that the mapping's refusal of this
repository's `quire.derivation-evidence/v1` family is reported as it stands; that no retained byte is
written by the run and none differs from what was committed; and that removing any load-bearing
check in the census makes the census go red.

## Test Procedure

Run `scripts/legacy_evidence_view.py --json` and assert the census. Compare its file count against a
recursive walk of `evidence/`. Then run `scripts/legacy_evidence_view.py --mutation-probes`, which
degrades one check at a time and requires the census to notice each degradation.

## Expected Results

42 envelopes, all `incompatible` for an unknown PGM-01 schema version; zero bytes moved during the
run and zero uncommitted differences; at least one positive control accepted as `lossy`; and every
mutation probe detected.
