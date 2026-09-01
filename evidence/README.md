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
revision, digests, and validator Git-blob identity are retained. External checkout paths are
collection-time inputs and are deliberately absent from the sealed record.

`evidence/ANCHORS` is the complete committed census of authoritative records and supporting evidence
content. Run `make verify-evidence` to check the root anchors, record checksums, manifest artifacts,
schema formats, artifact links, source identity, and re-derived outcomes. The current authoritative
record is `runtime-v01-70cc6a0f9260-20260901T004020Z`.

Collection verifies a new record directly but does not rewrite `evidence/ANCHORS`. Retiring the old
record, updating the authoritative record named above, regenerating anchors, and running the full
verifier are explicit review-boundary steps. The anchor updater rejects silent top-level removals,
and both updater and verifier enforce a digest for every retained historical record through
`evidence/HISTORY`, in addition to the minimum retained-history census.
Collection is staged beneath `target/evidence-staging/` and published into `evidence/` only after
the flat record is sealed, so the evidence-tool census cannot mistake an in-progress record for a
new authority.

Candidate output supports the human release decision described by `spec/assurance/MP-001`; it is not
itself a release approval.

Failed or superseded records live beneath `evidence/historical/` with an explicit disposition. They
remain immutable diagnostic records and must not be read as current-candidate assurance evidence.
`historical/DISPOSITIONS` is an exact census: every retained record has one classification and an
envelope-status value that the verifier compares with that record's own verdict. Per-record
sidecars remain additional explanation where present.
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

`historical/retired-round5-staging-fix/` retains the first Round 5 attempt. It correctly failed when
the strengthened anchor updater observed the collector's incomplete top-level record; that failure
led to the out-of-census staging boundary described above.

`historical/retired-round5-schema-fix/` retains the next Round 5 attempt. Its input-schema validator
correctly rejected the newly added MSRV compiler identity until that eighth tool was admitted by the
closed schema and covered by generated-instance validation.

`historical/retired-round5-pycache-fix/` retains the subsequent conclusive command collection. Its
final verifier correctly rejected ignored Python bytecode caches; Python cache output is now routed
beneath `target/`, keeping the ignored-file source-input check strict and self-consistent.

`historical/retired-round5-review-closure/` retains the preceding authoritative record. It predates
the portable validator, actual compiler-toolchain digests, retained behavioral-control floors,
ignored-input enforcement, collector command binding, and enforced historical dispositions.

`historical/retired-round6-pycache-fix/` retains the first Round 6 collection attempt. Every command
was conclusive, but final source verification correctly rejected Python bytecode generated before
collection; those caches were moved outside the repository before the clean recollection.
