# Retained evidence

Run `scripts/collect_evidence.sh` from the repository root. By default it creates a new
revision-and-UTC-timestamp-scoped directory and refuses to overwrite an existing record. Each output
preserves stdout and stderr separately, along with source/tool identities and SHA-256 digests. Kani
and the pinned PGM-01 schema and validator are mandatory for conclusive collection. An unavailable
tool or skipped command leaves the result pending; it is never represented as successful evidence.

The collector emits `evidence-envelope.json` with the canonical
`quire.derivation-evidence/v1` identity from PGM-01, plus separately versioned collection-input and
manifest schemas. The collector requires the exact packages in `requirements-evidence.txt`, applies
Draft 7 format validation, and records the Python, package, Rust, and Kani identities. Set
`PGM01_SCHEMA` to the reviewed IR repository's envelope schema and `PGM01_VALIDATOR` to its
`scripts/validate_governance.py`; both files must come from the exact merged PGM-01 revision and their
paths, revisions, and digests are retained.

`evidence/ANCHORS` is the complete committed census of authoritative records and supporting evidence
content. Run `make verify-evidence` to check the root anchors, record checksums, manifest artifacts,
schema formats, artifact links, source identity, and re-derived outcomes. The current authoritative
record is `runtime-v01-cc48ce2ff505-20260831T190534Z`.

Candidate output supports the human release decision described by `spec/assurance/MP-001`; it is not
itself a release approval.

Failed or superseded records live beneath `evidence/historical/` with an explicit disposition. They
remain immutable diagnostic records and must not be read as current-candidate assurance evidence.
The prior `runtime-v01-f3f1c28d1703-20260831T174552Z` record is retained under
`historical/retired-pre-head-binding/`: its outcomes remain intact, but it predates exact HEAD
binding and the repository-owned coverage-status classifier introduced by the current record.
