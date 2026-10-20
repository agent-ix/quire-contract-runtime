---
id: SR-13431
title: "Base specification review of RT enum owner cutover"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-runtime; spec/assurance/AD-004-runtime-crate-layout.md, spec/exact/functional/FR-012-carried-compiler-vocabulary.md, spec/exact/functional/FR-275-single-exact-kernel.md, spec/exact/matrix/TC-033-carried-compiler-vocabulary.md, spec/exact/matrix/TC-035-boxed-debug-parity.md, spec/exact/matrix/tests.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-runtime/FR-012
    type: reviews
---

## Summary

Ticket: IR-882. Checked the changed requirement, assurance, and test-case text for consistency with deleting RT's enum implementation and consuming QSV directly. The FR-012 cause-owner split still calls one QSV-raised case RT-raised.

## Verdict

CONDITIONAL. Correct the FR-012 owner description before merge; remaining specification findings are in SR-13432 and SR-13433.

## Examined

FR-012-AC-4 (examined): thirteen graph causes are raised by `UnitGraph::admit`, `UndeclaredCase` by reading a case against its declaration, and malformed enum lists by `EnumDeclaration::new`. FR-275-AC-2 (examined): no RT duplicate owner or adapter. FR-275-AC-14/15 (examined): preserve test-case artifacts and planned evidence rows. FR-007-AC-13 (examined): nine boxed Debug renderings. AD-004 step 4 and TC-033/035 (examined): ownership and evidence procedures.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-012 still classifies `UndeclaredCase` as raised by RT, though the only enum member path now calls QSV directly | spec/exact/functional/FR-012-carried-compiler-vocabulary.md:61 |

## Detail

The changed enum-admission bullet correctly names QSV, but the preceding "Raised by the runtime over a graph it is handed" bullet includes undeclared case. The changed TC-033 procedure and `tests/exact_vocabulary.rs` now call QSV `EnumDeclaration::member` directly. Move `UndeclaredCase` to the QSV owner group or otherwise state the boundary accurately.

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | FR-012 explicitly states QSV raises `UndeclaredCase` through `EnumDeclaration::member`, and RT tests consume that owner path directly. |
