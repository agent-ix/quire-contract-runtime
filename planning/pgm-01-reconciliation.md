---
id: REV-002
title: "Runtime PGM-01 merged-revision reconciliation"
type: Review
---

# PGM-01 merged-revision reconciliation

Runtime source reviewed: the source commit immediately preceding refreshed evidence.

PGM-01 merged revision: `agent-ix/quire-contract-ir` `main` at
`7dac9d8c19952412b56a0347387666e2ca81e01d` (merged PR #12).

Envelope schema: `quire.derivation-evidence/v1`, SHA-256
`0946e235e9e4b0fa79e9b9ec27ae157b303c17de0a9408d3cc04968fb7152256`.

The merged revision's tree is byte-identical to reviewed PR #12 head
`d8d376d887c40255e87ef9656bc0faf79216b321`, so the reviewed schema and policy content are
unchanged. The complete policy release check on merged `main` passes
21/21 Quire documents, 28/28 backed criteria, the 13/13 Draft 7 corpus, and all seven schema/format
mutation probes, seven Python tests, and 14 Rust tests. The runtime envelope is accepted by both the
merged schema and its custom validator with zero errors. PR #12 was merged under the
operator-authorized bounded admin exception; that decision is recorded as governance provenance and
is not converted into a runtime release claim.

The merged revision's release-only verifier also matches its unique retained record across 66/66 complete
HEAD/worktree inputs and 5/5 committed outputs, including the published PGM evidence-manifest schema.

The exact merged schema and raw signed Git commit object are vendored at
`schemas/pgm01-derivation-evidence-envelope-v1.schema.json` and
`schemas/pgm01-merged-commit.txt`. The evidence builder derives the schema SHA-256 and canonical Git
commit SHA-1, fails on disagreement with either executable pin, and TC-007 asserts that the executable
revision and digest agree with this reconciliation and `planning/gap-analysis.md`.

| Policy requirement | Runtime disposition | Evidence or remaining gate |
|---|---|---|
| PGM-01-R01 schema compatibility | canonical policy URI and explicit v1 identities adopted; unknown-version handling applies to evidence consumers, while the runtime public API has no serialized wire boundary | `spec/index.md`; versioned local evidence schemas |
| PGM-01-R02 exact pins | source, candidate, tools, environment, dependencies, parameters, and schema digest recorded | canonical envelope and collection input |
| PGM-01-R03 release order | runtime is an independent initial source-tag root | merged PGM-01 revision pinned; human tag decision remains open |
| PGM-01-R04 licensing/provenance | crate and local schemas are `MIT OR Apache-2.0`; Cargo publication is disabled | Cargo metadata, schema notices, license audit |
| PGM-01-R05 clean-room grammar | not applicable: this repository contains no grammar/parser implementation or imported grammar fixtures | repository and provenance inspection |
| PGM-01-R06 human authority | agent-assisted method and `@kreneskyp` reviewer identity recorded; automation leaves review and release pending | envelope provenance, CODEOWNERS, protected branch |
| PGM-01-R07 classification | component classified as `linked-runtime`; consuming projects must verify their deployed target | envelope extension and assurance profile |
| PGM-01-R08 common envelope | JSON envelope uses every required core field and preserves the executed local Kani result | exact merged-main validator and Kani output retained with refreshed evidence |
| PGM-01-R09 retention/release | timestamped manifest records outcomes, artifacts, limitations, and checksums; release decision remains open | evidence bundle and `planning/release-decision.md` |
| PGM-01-R10 qualification boundary | source release provides support only and confers no project validation, accreditation, or certification | master requirements, assurance artifacts, README |

No runtime semantic API change is required by the merged PGM-01 policy. The pre-PGM ad hoc text envelope
is replaced by the canonical JSON record. Merged-main local Kani and both policy validators pass; a
hosted run and the human source-release decision remain open.
