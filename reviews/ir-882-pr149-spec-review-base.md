---
id: SR-13480
title: "IR-882 PR 149 base spec review"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-runtime; spec/assurance/AD-004-runtime-crate-layout.md, spec/exact/functional/FR-012-carried-compiler-vocabulary.md, spec/exact/functional/FR-275-single-exact-kernel.md, spec/exact/matrix/TC-033-carried-compiler-vocabulary.md, spec/exact/matrix/tests.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-runtime/FR-012
    type: reviews
---

## Summary

Ticket: IR-882. Reviewed the changed R-3 status prose, FR-012 ownership account, TC-033 procedure and Test Matrix against the code deletion. The main cutover account is accurate; one adjacent matrix row was not updated.

## Verdict

CONDITIONAL. The stale row contradicts the implemented cutover and should be corrected in this PR.

## Examined scope

- FR-012-AC-7 (examined): After the R-3 cutover, inspection finds no RT-local `PackageCause`, `PackageRefusal` or `PackageRefusalCode` definition, type alias, public re-export or adapter. The RT consumers of selection and package refusals use QSV's definition types directly; `RevisionMismatch`, `DigestDomainMismatch` and `ByteDigestMismatch` are absent from RT's `invalid_package` surface, while FR-012-AC-3's eight selection spellings and `invalid_package` code remain available from QSV.
- FR-012-AC-3 (examined): `SelectionRefusalCode::ALL` is the eight codes in normative check order, `as_str` yields the lock spelling for each, `from_code` resolves exactly those eight spellings and nothing else, and `PackageRefusalCode::as_str` is `invalid_package`.
- TC-033 (examined): The R-3 step is implemented: RT has no local definition refusal vocabulary, its tests import QSV directly, and the three retired causes cannot be paired with `invalid_package`.
- FR-012-AC-3/4/5-matrix (examined): current tests consume QSV's semantic-node refusal and enum declaration types directly and still exercise RT's definition and unit residue; owner evidence for the enum part of AC-4 needs mapping

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The AC-3/4/5 matrix row still says tests exercise RT definition residue, although this PR deletes definition.rs and changes those tests to import QSV. Update that clause to describe only remaining unit residue and the unresolved enum mapping. | spec/exact/matrix/tests.md:69 |

## Dispositions

Round 1, inspected exact PR head 48548fe1ffc6622584d043b1051f3d1bba46266e:

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | The AC-3/4/5 matrix row now says tests consume QSV's definition, semantic-node and enum types directly and still exercise only RT's unit residue. The unresolved enum mapping remains explicit. |
