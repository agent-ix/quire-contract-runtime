# Retained evidence

Run `scripts/collect_evidence.sh` from the repository root. By default it creates a new
revision-and-UTC-timestamp-scoped directory and refuses to overwrite an existing record. Each output
preserves stdout and stderr separately, along with source/tool identities and SHA-256 digests. A
missing optional Kani installation is recorded as `skipped-unavailable`; it is never represented as
successful proof evidence.

The collector emits `evidence-envelope.json` with the canonical
`quire.derivation-evidence/v1` identity from PGM-01, plus separately versioned collection-input and
manifest schemas. The collector gates those local schemas with the installed `jsonschema` Draft 7
implementation and records both its version and the Python version. Set `PGM01_SCHEMA` to the reviewed IR repository's envelope schema and
`PGM01_VALIDATOR` to its `scripts/validate_governance.py` to retain independent schema and custom
validator results. An absent optional PGM-01 gate is recorded as `skipped-unavailable`, not passed.

Candidate output supports the human release decision described by `spec/assurance/MP-001`; it is not
itself a release approval.
