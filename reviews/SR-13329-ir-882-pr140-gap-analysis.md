---
id: SR-13329
title: "IR-882 PR 140 gap analysis: semantic-node slice of AD-004 step 4"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-runtime@2a5119f8dab783682df44a8e235f86cfaceda11c vs origin/main f5c3570; spec/exact/functional/FR-012-carried-compiler-vocabulary.md, spec/exact/functional/FR-275-single-exact-kernel.md, spec/exact/matrix/tests.md, spec/core/functional/interface-001-runtime-api.md, src/exact/**, tests/{exact_vocabulary,exact_meter_state,ir882_qsv_vocabulary}.rs"
review_set: subset
---

## Summary

Ticket: IR-882. PR agent-ix/quire-contract-runtime#140. This is a planless gap analysis; plan completion was not assessed.

The reviewer ran `make spec` on the head and on a fresh `origin/main` f5c3570 worktree. Both report 235/235 docs grammar-clean and "125 unbacked row(s) and 0 contradicted status(es) (--strict)", which matches the author's figures. A line diff of the two full outputs differs only in the evidence-symbol count (157 to 158, all bound and tagged). The unbacked list is identical, so no row moved into or out of the backed set. `make spec` exits 2 on strict coverage on both trees; that is not caused by this PR.

These hold:

- FR-012 Description and Outputs now name the QSV owner for `InvalidSemanticGraph` and `SemanticGraphCause`, which is true at the head.
- FR-012-AC-4's clauses still hold through RT. The 17 retained `#[trace("TC-033", "FR-012-AC-4")]` tests drive every runtime-raised cause through `UnitGraph::admit`, `EnumDeclaration::new` and `member`, now asserting the QSV enum. A grep finds no raise site for `OwnerNotSelected`, `StaleKey`, `ForeignDeclaration` or `UnreducedRational` in `src`.
- FR-275-AC-2: the slice adds no module, alias or feature that keeps the deleted node vocabulary reachable. The AC's Value, walker and registry clauses correctly stay unbacked, and its TC-197 row keeps 🚧.
- interface-001-AC-2 (no re-export of a shared-crate item): no `pub use quire_` exists in `src`.
- interface-001-AC-11: QSV is still reached only through the `exact` feature, and all three touched test files are `#![cfg(feature = "exact")]`.
- AD-004 split rule ("only along an edge where this crate still compiles with each half owned once"): this holds. The crate compiles, and each of the two names has one owner.
- FR-012 matrix row FR-012-AC-3/4/5 is truthful at the head.

## Examined criteria

- FR-012-AC-4 (examined): "Each of the thirteen graph causes is raised by an admissible input to `UnitGraph::admit`, and `UndeclaredCase` by reading a case against its declaration; no graph input raises any of the four compiler-admission causes; `EnumDeclaration::new` raises `NonCanonicalPreimage` and `UnsortedUnorderedMembers` for exactly the malformed member lists named above; and `OwnerNotSelected`, `StaleKey`, `ForeignDeclaration` and `UnreducedRational` have no raise site in the crate."
- FR-012-AC-3 (examined): "`SelectionRefusalCode::ALL` is the eight codes in normative check order, ... and `PackageRefusalCode::as_str` is `invalid_package`."
- FR-012-AC-5 (examined): "`Dimension` and `CompoundUnit` hold no zero exponent, iterate ascending by node key, and compare equal exactly when they denote the same unit, for every construction order of the same terms."
- FR-275-AC-2 (examined): "No module, alias or feature of the runtime keeps a removed kernel definition reachable; after the IR-882 value-bearing cutover, inspection finds no RT-defined second `Value`, ... conversion layer, adapter or old-`Value` re-export."
- interface-001-AC-2 (examined): "The runtime shall not re-export, by `pub use`, alias, wrapper or feature, any item that `quire-exact`, `quire-semantic-value` or the evaluation leaf crate exports."
- interface-001-AC-11 (examined): "The runtime shall depend on `quire-exact` and `quire-semantic-value` only through the `exact` feature."
- TC-197 matrix row (examined): spec/exact/matrix/tests.md:104.
- FR-277-AC-1 (context_only): stays planned.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The TC-197 matrix row says "`ir882_qsv_vocabulary` establishes only deletion of the duplicate semantic-node vocabulary". That test is not bound to TC-197: its tag is `FR-012-AC-4` (see SR-13328 FND-001). It also establishes only that one file path is absent. A reviewer mutant re-adding the RT types in another file passes it. So the row credits evidence that neither binds to the row nor proves what the row says | spec/exact/matrix/tests.md:104; tests/ir882_qsv_vocabulary.rs:8-17 |

## Verdict

**PASS with findings.** The coverage state equals main (125 unbacked, 0 contradicted, unbacked set identical). The FR-012 and FR-275 text is truthful at the head, and no criterion lost backing. FND-001 is closed by the same change that fixes SR-13328 FND-001: reword the TC-197 row to name only what a bound test establishes.

## Dispositions

Round 1 checked fix commit `792755c` at head `792755cdf038c5d36bb7fd1fc7c55baff1b03512`. `make spec` reports 238/238 grammar-clean, and strict coverage shows "110 unbacked row(s) and 0 contradicted" (exit 2). The author reports the same figures but says the earlier 125 "is not reproduced". The reviewer diffed this run against the review-pass run (125) and traced the cause (FND-002).

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | The TC-197 row now reads "`ir882_qsv_vocabulary` backs only FR-275-AC-22 by inspecting runtime source for local semantic-node vocabulary definitions", and the test is now bound to FR-275-AC-22. What the row's text claims matches the test. What Quire computes from the row does not; see FND-002 |
| FND-002 | still-open | New in round 1; see below |
| FND-002 | fixed | Round 2 (head `6ac954f`): the test is tagged `#[trace("FR-275-AC-22")]` only. Fresh `origin/main` f5c3570 gives 125 unbacked; the head gives 124 unbacked and 0 contradicted. FR-275-AC-1 to AC-5 and AC-12 to AC-19 (13 ACs, including AC-2) are reported unbacked again. The one-row difference from main is the TC-197 traces-to row, recorded as FND-003 |
| FND-003 | still-open | New in round 2; see below |

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-002 | high | `#[trace("TC-197", "FR-275-AC-22")]` binds the shared `TC-197` id. Every FR-275 AC verified by `Inspection (TC-197)` therefore now counts as backed by this one name-check test: AC-1, AC-2, AC-3, AC-4, AC-5 and AC-12 to AC-19, plus the TC-197 traces-to row and an FR-275 functional-coverage row. That is 15 rows, and it accounts exactly for the drop from 125 to 110 unbacked. The PR, the TC-197 row and AD-004 all say FR-275-AC-2 stays unbacked, yet `quire coverage --strict` no longer lists it. This is coverage inflation. The reviewer probed the fix: tagging only `#[trace("FR-275-AC-22")]` gives 124 unbacked, and FR-275-AC-2 and the other TC-197 ACs are reported unbacked again. Alternatively, give AC-22 its own TC id | tests/ir882_qsv_vocabulary.rs:137; spec/exact/matrix/tests.md:104; spec/exact/functional/FR-275-single-exact-kernel.md:187 |

## New findings (disposition pass 2)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-003 | low | The TC-197 matrix row now counts as backed in strict coverage: its traces-to line drops out of the unbacked list, which accounts for 125 to 124. The reason is that the row lists FR-275-AC-22 among its targets, and the AC-22 tag satisfies it. The row's own text says the test "does not bind TC-197 or back the other criteria in this row", and the row's other 13 ACs are unbacked, yet Quire reads the planned TC-197 row as backed by a semantic-node name check. A fix is to give FR-275-AC-22 its own TC row so TC-197 stays unbacked. Alternatively, accept it as Quire's any-target row semantics, with that reason recorded | spec/exact/matrix/tests.md:104; spec/exact/functional/FR-275-single-exact-kernel.md:187 |
