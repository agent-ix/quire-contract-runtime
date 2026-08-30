---
id: REV-002
title: "Runtime PGM-01 candidate reconciliation"
type: Review
---

# PGM-01 candidate reconciliation

Runtime source reviewed: the source commit immediately preceding refreshed candidate evidence.

PGM-01 candidate: `agent-ix/quire-contract-ir#12` at
`0b8669b80f98b6c11954f922b32d9edae8a11983`.

Envelope schema: `quire.derivation-evidence/v1`, SHA-256
`0946e235e9e4b0fa79e9b9ec27ae157b303c17de0a9408d3cc04968fb7152256`.

This is a provisional reconciliation against a review candidate. The identities and conclusions must
be checked again after PGM-01 merges.

| Policy requirement | Runtime disposition | Evidence or remaining gate |
|---|---|---|
| PGM-01-R01 schema compatibility | canonical policy URI and explicit v1 identities adopted; unknown-version handling applies to evidence consumers, while the runtime public API has no serialized wire boundary | `spec/index.md`; versioned local evidence schemas |
| PGM-01-R02 exact pins | source, candidate, tools, environment, dependencies, parameters, and schema digest recorded | canonical envelope and collection input |
| PGM-01-R03 release order | runtime is an independent initial source-tag root | PGM-01 candidate; human tag decision remains open |
| PGM-01-R04 licensing/provenance | crate and local schemas are `MIT OR Apache-2.0`; Cargo publication is disabled | Cargo metadata, schema notices, license audit |
| PGM-01-R05 clean-room grammar | not applicable: this repository contains no grammar/parser implementation or imported grammar fixtures | repository and provenance inspection |
| PGM-01-R06 human authority | agent-assisted method and `@kreneskyp` reviewer identity recorded; automation leaves review and release pending | envelope provenance, CODEOWNERS, protected branch |
| PGM-01-R07 classification | component classified as `linked-runtime`; consuming projects must verify their deployed target | envelope extension and assurance profile |
| PGM-01-R08 common envelope | JSON envelope uses every required core field and preserves pending local Kani state | exact candidate validator output retained with refreshed evidence |
| PGM-01-R09 retention/release | timestamped manifest records outcomes, artifacts, limitations, and checksums; release decision remains open | evidence bundle and `planning/release-decision.md` |
| PGM-01-R10 qualification boundary | source release provides support only and confers no project validation, accreditation, or certification | master requirements, assurance artifacts, README |

No runtime semantic API change is required by the PGM-01 candidate. The pre-PGM ad hoc text envelope
is replaced by the canonical JSON record. A fresh deliberate remote CI dispatch, CODEOWNER approval,
PGM-01 merge, final identity reconciliation, and human source-release decision remain open.
