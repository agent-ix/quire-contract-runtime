---
id: TC-009
title: "Classify every shared pin through the packaged compatibility matrix"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-runtime/FR-005
    type: verifies
---
# TC-009: Classify every shared pin through the packaged compatibility matrix

## Description

Verify that the four shared components are classified by `engineering_assurance.compatibility` and
not by a local restatement of the matrix, that every artifact this repository reads out of the
pinned release still hashes to the digest `assurance/pins.json` records, that at least one artifact
carries a digest so the check has a population to run over, and that the internal npm mirror is
named nowhere in the repository.

## Test Procedure

Run `scripts/check_shared_pins.py --json` under the pinned assurance interpreter and read its
report. Then call `artifact_digest_mismatches` with every recorded digest falsified, requiring the
pinned population to be non-empty and every falsified digest to be reported. Then call
`mirror_references` with a pins document carrying an injected mirror URL, so each refusal is observed
rather than assumed.

## Expected Results

Four components, every one `compatible`; no artifact digest mismatch; no mirror reference; the
acceptance state reported and never recorded locally; at least one digest-pinned artifact; every
falsified digest detected; and the injected mirror URL detected.
