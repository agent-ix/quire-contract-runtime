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
record is `runtime-v01-c9ec99b6418a-20260831T231052Z`.

Collection verifies a new record directly but does not rewrite `evidence/ANCHORS`. Retiring the old
record, updating the authoritative record named above, regenerating anchors, and running the full
verifier are explicit review-boundary steps. The anchor updater rejects silent top-level removals,
and both updater and verifier enforce a digest for every retained historical record through
`evidence/HISTORY`, in addition to the minimum retained-history census.

Candidate output supports the human release decision described by `spec/assurance/MP-001`; it is not
itself a release approval.

Failed or superseded records live beneath `evidence/historical/` with an explicit disposition. They
remain immutable diagnostic records and must not be read as current-candidate assurance evidence.
The prior `runtime-v01-f3f1c28d1703-20260831T174552Z` record is retained under
`historical/retired-pre-head-binding/`: its outcomes remain intact, but it predates exact HEAD
binding and the repository-owned coverage-status classifier introduced by the current record.

`historical/retired-round3-control-strengthening/` retains the prior authoritative record. Its
transcripts remain intact, but it predates positive transcript corroboration, per-harness proof
obligation counts, conclusive-verdict enforcement, independent parameter re-derivation, and the
Round 3 Make/coverage/history controls, so it is explicitly non-authoritative.

`historical/failed-round3-collection/` retains the first clean-tree Round 3 collection attempt. It
is inconclusive because Quire rejected an assurance binding placed in structured frontmatter and
the panic audit rejected proof-only `expect` calls; both numeric failures and their transcripts are
preserved rather than rewritten as successful evidence.

`historical/retired-round4-control-strengthening/` retains the preceding authoritative record. It
predates the live test census, trusted-home and executable-digest controls, Kani obligation floors
and mutation campaign, per-lane Rust test counts, and per-record history anchors.

`historical/failed-round4-collection/` retains two fail-closed Round 4 collection attempts: one
observed its own transient staging directory, and one exposed an over-broad transcript
contradiction marker. Their dispositions and original command results are preserved.
