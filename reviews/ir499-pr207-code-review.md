---
id: SR-20261104024427
title: "code-review of IR-499 owner text"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-runtime; spec/exact/matrix/tests.md; IR-499"
review_set: base
---

## Summary

PASS: three appended owner clauses only; no findings. Existing partial statuses and evidence text are preserved exactly. No new executable evidence or whole-ticket completion is claimed.

## Coverage

- `FR-008:TC-026-owner` (examined, `spec/exact/matrix/tests.md`): Remaining admitted-QSL equality/reference agreement is owned by IR-1338 for reachable vectors; oversized Identity and equal-byte declared/compound UnitId routes are Planned under IR-1339 / QSL-1720, with RT consumer evidence after those routes admit.
- `FR-008:TC-026-status` (examined, `spec/exact/matrix/tests.md`): | FR-008 | FR-008-AC-5, FR-008-AC-6 | TC-026; TC-096 (`quire-integration`) | 🚧 TC-096 `admitted_composites_with_different_declarations_compare_unequal` and `admitted_enum_cases_compare_by_variant_identity` restore the moved RT consumer checks; owner evidence and the rest of the equality matrix remain open. |
- `FR-275-AC-23:owner` (examined, `spec/exact/matrix/tests.md`): Admitted-QSL agreement for oversized Identity and equal-byte declared/compound UnitId routes remains Planned under IR-1339 / QSL-1720.
- `FR-275-AC-23:status` (examined, `spec/exact/matrix/tests.md`): | FR-275 | FR-275-AC-23 | TC-201, TC-197; TC-096 (`quire-integration`) | 🚧 TC-096 checks producer-origin exact/non-exact handoff and a compound-to-declared second conversion; the original U25 `1 J` two-step and equal-byte declared/compound controls remain open. QSV FR-124-AC-4/TC-923 retains owner refusal evidence. |
- `TC-201:owner` (examined, `spec/exact/matrix/tests.md`): Reachable admitted-QSL agreement is owned by IR-1338; oversized Identity and equal-byte declared/compound UnitId routes remain Planned under IR-1339 / QSL-1720.
- `TC-201:status` (examined, `spec/exact/matrix/tests.md`): | TC-201 | Convert a quantity exactly between units of one dimension | Integration | P0 | FR-007-AC-20, FR-007-AC-21, FR-007-AC-22, FR-007-AC-23, FR-007-AC-24, FR-007-AC-25, FR-275-AC-23 | 🚧 The moved QSV quantity conversion and handoff controls run in quire-integration TC-096 with admitted identities; the original U25 `1 J` and equal-byte controls remain open. |
- `FR-008-AC-5` (context_only, `spec/exact/functional/FR-008-composite-collection-and-equality.md`): `plan_equality` forms the occurrence-pair plan with no charge, refuses a cross-universe reference pair as `ForeignReference`, and distinguishes unlike collection kinds (`CollectionKindMismatch`) from unlike value kinds (`ValueKindMismatch`); `check_equality` refuses before any charge, including `operator-ineligible` when either operand type bears an IEEE value at any depth; `CheckedEquality::evaluate` charges the selected schedule and its conversions in operand order, and each of the six checked operand failures carries its ix://agent-ix/quire-exact/FR-369 cause, retaining `IllTypedCause` for
- `FR-008-AC-6` (context_only, `spec/exact/functional/FR-008-composite-collection-and-equality.md`): A `Reference<T>` value carries only its supplied `(universe, object-type, identity)` triple, is constructed from no source form, and compares equal only within one universe.
- `FR-275-AC-23` (context_only, `spec/exact/functional/FR-275-single-exact-kernel.md`): After the step-4 value-bearing cutover, QSV's `Conversion::identified_quantity` returns a `quire_exact::Quantity` with the exact magnitude and the target's compound-arm `UnitId`, paired with QSV's `IdentifiedUnit` for that same target; a non-exact conversion returns `HandoffRefusal::NotExact`, and target identity refusal returns `HandoffRefusal::Identity` with neither value nor identified unit. Retaining the identified unit with `UnitScope::form` permits `UnitScope::resolve(&quantity)` to supply the same normalized `QuantityUnit::Compound` to the next conversion. Declared and compound ID arms

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Evidence

Compared all three changed rows with current origin/main: each retains its complete previous text, with one ownership clause appended. The handoff comment d99d0b1e proposed a completed status from an older state; this change correctly preserves current Planned/partial status instead. Read current structured IR-1338, IR-1339 and QSL-1720 records: each is Backlog. IR-1338 owns reachable admitted-QSL equality/reference agreement; IR-1339 and QSL-1720 own the oversized Identity and equal-byte declared/compound UnitId admission or owner-disposition route. Local QSV consumer controls are not credited as admitted-QSL agreement. Read FR-008-AC-5/6, FR-275-AC-23 and TC-026/TC-201 for context; existing requirements and verification methods remain unchanged. No applicable AssuranceProfile was found under spec/. git diff --check passed. No Rust/Cargo tests, global census or coverage measurement was run.

## Review selection

Code review and base spec review apply. Rust review and production gap analysis are inapplicable to this Markdown-only diff. Evidence-strategy analysis is inapplicable because no verification method or obligation changes; its mandatory global advisor would exceed the scoped no-census brief. Base review checks the evidence boundary and owner assertions directly. EARS, integrity, dependency and other lenses are inapplicable: no requirement statement, structural artifact or relationship edge changes.

## Verdict

PASS: three appended owner clauses only; no findings. Existing partial statuses and evidence text are preserved exactly. No new executable evidence or whole-ticket completion is claimed.
