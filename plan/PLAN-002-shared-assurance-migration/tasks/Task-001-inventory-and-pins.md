---
id: Task-001
title: "Keep/replace/delete/defer inventory and the accepted pins"
type: Task
status: done
track: A
priority: P0
relationships:
  - target: ix://agent-ix/quire-contract-runtime/FR-005
    type: references
---
# Task-001: Keep/replace/delete/defer inventory and the accepted pins

## Scope

Classify every artifact in the repository before changing any of it, and pin the accepted shared
release without restating the compatibility matrix locally.

## Inventory

**KEEP — domain behaviour this repository owns.** The `no_std` crate and its tri-state terminal
verdict; the `non_exhaustive` public enums and their compile-fail doctests; the panic-free operator
family; saturating campaign accounting; the proptest adapter; the seven Kani harnesses and their
per-harness discharged-obligation floors; the three semantic mutation controls; the harness census;
the panic-surface and unsafe audits; the governed linked-footprint measurement and its floor,
ceiling and zero-panic-relocation requirement; MSRV; rustdoc; cargo-deny.

**REPLACE — generic machinery now owned upstream.** The local evidence envelope, manifest, and
collection input become a Quoin change-assurance record, seven proof attestations, and a
verification receipt. The local tool-identity and parameter-digest framework becomes the attestation
`tool` block. The anchor and history census becomes Quoin's retention. The local traceability status
checker becomes the Quire static export. The PGM-01 envelope validation becomes
`engineering_assurance.verification_semantics.map_pgm01_bytes`.

**DELETE — after parity is observed at one revision.** `build_evidence_envelope.py`,
`collect_evidence.sh`, `verify_evidence.py`, `check_assurance_anchor.py`,
`update_evidence_anchors.py`, `check_failure_propagation.py`, `check_coverage_status.py`,
`validate_json_schema.py`, `evidence_policy.py`, `run_evidence_tests.py`,
`tests/test_evidence_tooling.py`, `requirements-evidence.txt`, and the Makefile's MAKEFLAGS guard,
`override` fence, and self-attestation targets.

**FREEZE — not deleted.** Four artifacts under `schemas/`: the two runtime evidence schemas, the
vendored PGM-01 envelope schema, and the vendored governance validator. Retained records name each
of them by SHA-256; `schemas/README.md` lists the digests and where each reference sits. Deleting one
would break a reference inside bytes this migration must leave untouched. They are named here by
description rather than by filename on purpose: `tests/shared_assurance.rs` asserts that no document
in the executable or specification surface mentions their names, and a plan that named them would
be the first violation of the rule it is describing.

**DEFER — with a linked issue.** The `[status-column-matches-nothing]` gap in
`quire coverage --strict` (`agent-ix/quire-contract-ir#21`); the absence of a
`quire.derivation-evidence/v1` reader in the shared mapping
(`agent-ix/engineering-assurance#21`); the absence of a release carrying the recorded human
acceptance of the compatibility matrix (`agent-ix/engineering-assurance#20`).

## Completion Evidence

`assurance/pins.json` records the release and the digests of the four artifacts read from it.
`scripts/check_shared_pins.py` classifies four components through the packaged matrix and reports
the acceptance state without gating on it.
