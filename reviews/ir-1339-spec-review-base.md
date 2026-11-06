---
id: SR-20261104024430
title: IR-1339 PR 208 spec-review/base of RT unit identity matrix rows
type: SpecReview
analysis: base
scope: 'agent-ix/quire-contract-runtime; PR #208 at 5b67fe1a220aff453eca7357f16401cd79f25805;
  spec/exact/matrix/tests.md lines 40, 92, 121'
review_set: subset
---

## Summary

Checked the three matrix claims against merged QSL #2422 and integration #83 IT-016 tests. Structural identity, strict-read byte scope, retained-context refusal, and owner boundary are stated accurately. The matrix rows remain Planned for TC-026, U25 1 J two-step, and remaining RT consumer evidence. No new requirement statement or relationship edge was added; no further spec sub-analysis applies.

## Verdict

**PASS.** No defects found in the scoped matrix diff. This review supports IR-1339 closure for the owner/integration identity routes; it does not claim the remaining RT consumer evidence.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Coverage

Examined FR-008-AC-5/6, FR-275-AC-23 and TC-201 status rows only. Source anchors: QSL merged #2422 `tests/it/producer_unit_identity.rs`; integration merged #83 `tests/producer_origin_identity.rs`, `tests/producer_origin_unit_identity.rs` and IT-016. Quire validation of the changed matrix file passed. No Cargo, full suite or unrelated spec review ran.

## Examined scope

```yaml
scope:
- id: FR-008-AC-5/6
  path: spec/exact/matrix/tests.md
  role: examined
  excerpt: "| FR-008 | FR-008-AC-5, FR-008-AC-6 | TC-026; TC-096 (`quire-integration`)\
    \ | \U0001F6A7 TC-096 `admitted_composites_with_different_declarations_compare_unequal`\
    \ and `admitted_enum_cases_compare_by_variant_identity` restore moved RT consumer\
    \ checks; other equality and reference cases remain open. IR-1338 owns reachable\
    \ admitted-QSL agreement. QSL-1720 rules a declared root and a derived one-term\
    \ compound as distinct structural `UnitId` arms; TC-096 `admitted_declared_root_differs_from_its_derived_compound_arm`\
    \ checks that distinction using touched roots in strict-read bytes. TC-096 separately\
    \ reaches th"
- id: FR-275-AC-23
  path: spec/exact/matrix/tests.md
  role: examined
  excerpt: "| FR-275 | FR-275-AC-23 | TC-201, TC-197; TC-096 (`quire-integration`)\
    \ | \U0001F6A7 TC-096 checks producer-origin exact/non-exact handoff and a compound-to-declared\
    \ second conversion. Its strict-read touched-root control distinguishes declared\
    \ and derived one-term compound `UnitId` arms; the former equal-byte wording is\
    \ disposed by QSL-1720 structural identity. Its separate `retained_qsl_roots_refuse_only_at_exact_identity_handoff`\
    \ control reaches the actual oversized `Identity(InputBytes)` refusal through\
    \ 32 genuine checked unused roots retained by QSL `AdmittedContext`, without claiming\
    \ those roots o"
- id: TC-201
  path: spec/exact/matrix/tests.md
  role: examined
  excerpt: "| TC-201 | Convert a quantity exactly between units of one dimension |\
    \ Integration | P0 | FR-007-AC-20, FR-007-AC-21, FR-007-AC-22, FR-007-AC-23, FR-007-AC-24,\
    \ FR-007-AC-25, FR-275-AC-23 | \U0001F6A7 The moved QSV quantity conversion and\
    \ handoff controls run in quire-integration TC-096 with admitted identities. QSL-1720\
    \ rules the old equal-byte declared/compound case as distinct structural `UnitId`\
    \ arms; TC-096 checks that distinction from touched roots present in strict-read\
    \ bytes. A separate authentic retained-`AdmittedContext` control with 32 checked\
    \ unused roots reaches the actual oversized `Identi"
bindings: []
```
