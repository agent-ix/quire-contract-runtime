---
id: SR-13330
title: "IR-882 PR 140 spec review: AD-004 semantic-node slice, FR-012, FR-275 and matrix edits"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-runtime@2a5119f8dab783682df44a8e235f86cfaceda11c vs origin/main f5c3570; spec/assurance/AD-004-runtime-crate-layout.md, spec/exact/functional/FR-012-carried-compiler-vocabulary.md, spec/exact/functional/FR-275-single-exact-kernel.md, spec/exact/matrix/tests.md"
review_set: subset
---

## Summary

Ticket: IR-882. PR agent-ix/quire-contract-runtime#140. The spec edits are prose and matrix status only: no requirement statement, AC text or relationship edge changes. So no EARS, integrity or dependency sub-analysis applies. `make spec` validation is grammar-clean (235/235).

These hold:

- The new AD-004 step-4 paragraph is consistent with the step-4 split rule and with Q-4. It names the slice's deletion, the absence of an alias or re-export, and the reason `definition.rs` stays. It also states that FR-275-AC-2 and FR-277-AC-1 are not satisfied.
- The `PackageCause` claim was measured against the locked QSV `9fd13752` `src/definition.rs`. QSV has `MissingMember`, `ConflictingDefinition` and `IncompatibleDefinition`. RT additionally has `RevisionMismatch`, `DigestDomainMismatch` and `ByteDigestMismatch`, so "three variants while RT's ... carries six" is true.
- The FR-012 Description and Outputs edits are true at the head. The deleted sentence ("roughly sixty public items across `node.rs` ...") was historical.
- The FR-275 residue table row correctly records that `node.rs` is deleted.

## Examined criteria

- AD-004 step 4 slice paragraph (examined): "The semantic-node refusal vocabulary has no `Value` edge. It can leave first as one compileable step-4 slice: RT deletes `src/exact/node.rs`, uses QSV's `InvalidSemanticGraph` and `SemanticGraphCause` directly ... The locked QSV `PackageCause` has three variants while RT's definition vocabulary carries six; `definition.rs` cannot leave in this slice without dropping three causes."
- AD-004 Routed gaps R-3 (context_only): "R-3 the `PackageCause` variants, or their retirement".
- AD-004 step-4 table row, CG column (context_only): "the different-declaration use sites, the three serializable mirrors and paired CG changes"; and "To CG: the paired PR for each of steps 3 to 5".
- FR-012 Description and Outputs (examined): "`InvalidSemanticGraph` and `SemanticGraphCause` are consumed directly from `quire_semantic_value::semantic_node`."
- FR-275 residue table, Carried compiler vocabulary row (examined): "`definition.rs` remains until QSV supplies the three additional `PackageCause` variants RT still carries".
- spec/exact/matrix/tests.md FR-012-AC-3/4/5 row and TC-197 row (examined).

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The FR-275 residue row says `definition.rs` remains "until QSV supplies the three additional `PackageCause` variants". That commits to one resolution of AD-004 R-3, which routes "the `PackageCause` variants, or their retirement". RT has no raise site for `RevisionMismatch`, `DigestDomainMismatch` or `ByteDigestMismatch` (they are only carried), so retiring them is a live outcome. The row should say "until R-3 is resolved" | spec/exact/functional/FR-275-single-exact-kernel.md:201 |
| FND-002 | low | The new AD-004 slice paragraph does not state this slice's CG side. CG `main` tests name `rt::InvalidSemanticGraph` and `rt::SemanticGraphCause` (quire-contract-codegen `tests/exact_scalar_support/agreement_cases.rs:1078-1172`, `agreement.rs:514-565`), and they must repoint to `quire_semantic_value::semantic_node` when CG's lock moves past this commit. The step-4 row and "To CG: the paired PR for each of steps 3 to 5" make the CG pairing part of the step, but the slice record is silent on it. CG is already on an older RT lock and already names RT items deleted by earlier slices, so this is a record gap, not a new break | spec/assurance/AD-004-runtime-crate-layout.md:407-415 |

## Verdict

**PASS with findings.** The edits are truthful and consistent with AD-004's split rule, Q-4 and the measured QSV owner. Both findings are wording-level, and either can be fixed in this PR or accepted with a reason.

## Dispositions

Round 1 checked fix commit `792755c` at head `792755cdf038c5d36bb7fd1fc7c55baff1b03512`. 238/238 docs are grammar-clean. The round also adds FR-275-AC-22. It is well-formed and scoped to this slice. Its shared `Inspection (TC-197)` verification is what lets the new tag inflate coverage, which is recorded as SR-13329 FND-002.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | The FR-275 residue row now reads "`definition.rs` remains until AD-004 R-3 is resolved" |
| FND-002 | fixed | The AD-004 slice paragraph now records the CG side: "CG main's `tests/exact_scalar_support/agreement_cases.rs` and `agreement.rs` still use `rt::InvalidSemanticGraph` and `rt::SemanticGraphCause`; those imports must move to `quire_semantic_value::semantic_node` when CG's RT lock moves past this slice." |
