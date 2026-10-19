---
id: SR-13312
title: "IR-882 PR 136 gap analysis: slice 0 charge-point census and FR-008 agreement"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-runtime@ef89b4f6a69a5a48d6466e75c1708fa11399bbc9 vs origin/main merge-base b4ea971082aa7c7c152af724c8a26755a855e34a; spec/exact/functional/FR-008-composite-collection-and-equality.md, spec/exact/matrix/tests.md, spec/exact/matrix/TC-024-composite-construction.md, tests/exact_composite.rs"
review_set: subset
---

## Summary

Ticket: IR-882 (slice 0). PR agent-ix/quire-contract-runtime#136. This is a planless gap analysis. Plan completion: not assessed.

Census, measured against quire-exact source at `1c6cbcb6` (its `main` at review time and the revision in the PR's `Cargo.lock`): `enum ChargePoint` in `src/accounting.rs` has 63 variants, `ChargePoint::ALL` is `[Self; 63]` with 63 entries, and the last variant and last `ALL` entry are `TemporalVisit`, whose `as_str` is `"temporal.visit"`. The owner test `charge_point_vocabulary_is_unique_and_round_trips` (`#[trace("TC-916", "FR-368-AC-1")]`) asserts `seen.len() == 63`. The PR's 63-point claim and the `temporal.visit` spelling and position are therefore true.

`quire trace --id FR-008-AC-8` at the head resolves four `rust-trace-attribute` claims, including the renamed `tc_024_p1_charge_point_vocabulary_pins_kernel_census_and_spellings` (`#[trace("TC-024", "FR-008-AC-8")]`). The test asserts `ChargePoint::ALL.len() == 63`, that `temporal.visit` is last, and that `as_str` and `from_code` round-trip it. It passes. `quire trace --id FR-277-AC-1` resolves no claim, and the FR-277 matrix row and TC-203 still read PLANNED, so FR-277-AC-1 stays PLANNED and untagged.

FR-008-AC-8's text ("`ChargePoint::ALL` has exactly 63 members including `temporal.visit`"), FR-008's charge-point paragraph and Kernel ownership section, and the FR-008-AC-7/AC-8 row of `spec/exact/matrix/tests.md` agree with each other and with the test.

Strict coverage: `make spec` at the head and at `origin/main` (b4ea9710) both report "226/226 docs grammar-clean", "Coverage: 90/198 rows backed", and "124 unbacked row(s) and 0 contradicted status(es) (--strict)", both exit 2, and their sorted outputs are byte-identical. The PR body's 124-findings claim holds, and the PR neither adds nor removes an unbacked row.

## Examined criteria

- FR-008-AC-8 (examined): "Every one of the twelve added charge points round-trips its QSpec spelling, `ChargePoint::ALL` has exactly 63 members including `temporal.visit`, and an injected denial at each of the twelve yields `Incomplete` on `work_units` naming that point with every counter left unchanged."
- FR-008 Kernel ownership (examined): "The consumed kernel exposes 63 charge points, including its `temporal.visit` point. RT pins the count and selected spellings in TC-024; its retained TC-024/025/026 tests cover residue charge denials, not the full census."
- spec/exact/matrix/tests.md FR-008-AC-7/AC-8 row (examined): "✅ 63-point kernel census including `temporal.visit` pinned by RT; retained RT tests check the twelve added denial points and residue behavior"
- spec/exact/matrix/TC-024-composite-construction.md Description and Test Procedure step 1 (examined).
- FR-277-AC-1 (examined): "On the bounded depth-four `Sequence` input with equal `quire_exact::Value` operands, the actual `CheckedEquality::evaluate` path completes `true` and admits exactly 17 `EqualityPair` charges ..."; untagged, PLANNED.
- spec/assurance/AD-004-runtime-crate-layout.md:126 (context_only): a dated RT-versus-QSL comparison that cites "62 charge points against 52". It is a historical measurement, not a current claim.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The TC-024 test-case document was not updated and contradicts FR-008, the matrix row and the renamed test. It still says "The kernel-wide 62-point census and spelling round-trip obligation has no local RT census after IR-349; quire-exact FR-368-AC-1/2 tests all 62 spellings and round trips", and Test Procedure step 1 says "Check owner FR-368-AC-1/2 for the full 62-member `ChargePoint` census ... Locally check `function.call` and `collection.visit` spellings". The PR adds a local 63-point RT census to TC-024 and checks `temporal.visit`. A reader of TC-024 gets the wrong count and is told that no local census exists. Fix: state the 63-point local census and the `temporal.visit` check in the Description and in step 1. The TC-024 row of `spec/exact/matrix/tests.md` (line 90) could also mention the local pin. | spec/exact/matrix/TC-024-composite-construction.md:16-24 |

## Verdict

**FAIL until FND-001 is fixed, then PASS.** Every measured claim holds: the 63-point census and the `temporal.visit` spelling and position, FR-008-AC-8's text and matrix agreement, tags through `#[trace(...)]` only, FR-277-AC-1 untagged, and the unchanged 124 strict-coverage count. The single defect is the stale TC-024 procedure document, which the slice's own census change made wrong. It is a spec-text fix that belongs in this PR.

## Dispositions

Review pass only; no disposition round has run yet.
