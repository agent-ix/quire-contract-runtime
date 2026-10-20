---
id: SR-13479
title: "IR-882 PR 149 gap analysis"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-runtime; FR-012-AC-7, TC-033, tests/exact_vocabulary.rs, src/exact/mod.rs"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-runtime/FR-012
    type: reviews
---

## Summary

Ticket: IR-882. Planless diff-scope trace review of FR-012-AC-7 through TC-033 and its new tagged test. Current source has no local definition vocabulary, but the new regression test guards only the former fixed file and two exact module strings.

## Verdict

CONDITIONAL. FR-012-AC-7 is true by current source inspection, but its new tagged test is incomplete as future regression evidence. Plan completion: not assessed.

## Examined scope

- FR-012-AC-7 (examined): After the R-3 cutover, inspection finds no RT-local `PackageCause`, `PackageRefusal` or `PackageRefusalCode` definition, type alias, public re-export or adapter. The RT consumers of selection and package refusals use QSV's definition types directly; `RevisionMismatch`, `DigestDomainMismatch` and `ByteDigestMismatch` are absent from RT's `invalid_package` surface, while FR-012-AC-3's eight selection spellings and `invalid_package` code remain available from QSV.
- TC-033 (examined): After the R-3 definition cutover, inspect RT library source for local definitions, aliases, re-exports or adapters of `PackageCause`, `PackageRefusal` and `PackageRefusalCode`; check that RT's selection and package refusal callers import QSV's definition owner directly.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The tagged AC-7 test checks only absence of src/exact/definition.rs and two literal strings in mod.rs. A renamed local enum, alias, re-export or adapter in another RT source file would leave it green, while AC-7 and TC-033 require inspection across RT library source. Extend the existing source walker to reject these forms or keep the matrix status partial. | tests/exact_vocabulary.rs:767-795 |

## Coverage

Plan completion: not assessed. Computed matrix binds FR-012-AC-7 to the new tagged test, but the source-inspection assertion does not cover all required forms.

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-002 | medium | The new ProductionDefinitionInspection::visit_ident rejects every occurrence of PackageCause, PackageRefusal, PackageRefusalCode and SelectionRefusalCode in src/, including a legitimate private `use quire_semantic_value::definition::PackageCause;` or direct type reference. FR-012-AC-7 requires RT consumers to use QSV types directly. Narrow the scanner to local definitions, aliases, public re-exports and adapters while permitting direct private QSV use. | tests/exact_vocabulary.rs:87-161 |

## Dispositions

Round 1, inspected exact PR head 48548fe1ffc6622584d043b1051f3d1bba46266e:

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | The test now recursively visits every src/ Rust file, parses it with syn, and checks definition vocabulary identifiers, aliases, public re-exports and macro tokens. Five synthetic mutation cases fail the scanner. |
| FND-002 | still-open | New this round: unconditional visit_ident also rejects valid direct QSV imports and type uses in production, contrary to AC-7. |

Round 2, inspected exact PR head b8f0dfadf1f1d44aefe83cbe73685dea696cc933:

| FND | outcome | reason |
| --- | --- | --- |
| FND-002 | fixed | The scanner now tests declarations, aliases, public re-exports, wrappers and macro declarations by context; it no longer rejects direct private QSV imports or ordinary signature/holder uses. A positive synthetic owner-use case and the focused tagged test pass (1/1). |

## Current verdict

PASS by the latest dispositions: FND-001 and FND-002 are fixed. Plan completion: not assessed.
