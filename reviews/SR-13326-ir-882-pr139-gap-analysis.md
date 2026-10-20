---
id: SR-13326
title: "IR-882 PR 139 gap analysis: key-owner slice of AD-004 step 4"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-runtime@22e885d76b7c1bca523ed79f0129a102da6557cc vs origin/main 9567ff0; spec/exact/functional/FR-012-carried-compiler-vocabulary.md, spec/exact/matrix/tests.md, spec/exact/matrix/TC-033-carried-compiler-vocabulary.md, spec/core/matrix/tests.md, src/exact/**, tests/exact_vocabulary.rs"
review_set: subset
---

## Summary

Ticket: IR-882. PR agent-ix/quire-contract-runtime#139. This is a planless gap analysis; plan completion was not assessed.

The reviewer measured strict coverage with `quire coverage --scope . --strict --json` on both trees. Fresh `origin/main` 9567ff0 has 126 unbacked rows and 0 contradicted statuses; the head has 125 and 0. That matches the author's figures. The reviewer diffed the unbacked sets: FR-012-AC-6 is no longer unbacked, and the FR-012 functional-coverage row only moved, from line 67 to line 66. No row became newly unbacked. `make spec` validation reports 232/232 docs grammar-clean. `make spec` still exits 2 on strict coverage; that is the same state as main, and is not caused by this PR.

These hold:

- FR-012-AC-1 is retired correctly. Its original criterion text and ID are kept, and both tagged tests were deleted in the same commit (`22e885d`) as the constructors.
- FR-012-AC-6 is tagged by `tc_033_runtime_has_no_local_node_key_constructor` through `#[trace("TC-033", "FR-012-AC-6")]`.
- FR-012-AC-6's clauses hold at this head by reviewer inspection. RT has no local `NodeKey` type and no RT `from_bytes` or `from_hex`, and TC-033 has no parser or round-trip test left. No library `NodeKey` construction or hex decoder exists anywhere in `src/`, `verification/` or `examples/`.
- FR-275-AC-2 is not violated by this slice: no alias, conversion layer or re-export keeps the removed key reachable.
- The AD-004 split rule holds: "IR-349 may split step 4 only along an edge where this crate still compiles with each half owned once". The crate compiles, and `NodeKey` has one owner.
- FR-275-AC-2's Value, walker and registry clauses and FR-277-AC-1 correctly stay planned and untagged.
- QSL ADR-012 §13.4 item 2 asks what produces production `NodeKey`s in RT. It stays open and owned by IR-882. This slice does not need to answer it, because RT library code takes keys only as caller-supplied parameters. QSL PR #1070, which AD-004 Q-5 requires to land with or before this work, is MERGED. It corrected ADR-012 lines 875 and 1104 to "RT may hold shared `NodeKey`s inside `quire_exact::Value` and constructs none in non-test code".

## Examined criteria

- FR-012-AC-6 (examined): "At the IR-882 step-4 cutover, inspection finds no RT-local `NodeKey`, no RT `from_bytes` or `from_hex` constructor, and no TC-033 parser or round-trip test of those local constructors. RT library code has no `NodeKey` construction or hex decoder."
- FR-012-AC-1 (examined): "`NodeKey::from_hex` accepts exactly the 64-lowercase-hex-digit inputs ... Retired at IR-882 step 4; this ID remains reserved with its original criterion text."
- FR-012-AC-2 (examined): "`NODE_KEY_DOMAIN` and `COMPOUND_UNIT_DOMAIN` equal their normative strings, and no code path constructs either from parts."
- FR-275-AC-2 (examined): "No module, alias or feature of the runtime keeps a removed kernel definition reachable; after the IR-882 value-bearing cutover, inspection finds no RT-defined second `Value` ... conversion layer, adapter or old-`Value` re-export."
- interface-001-AC-6 (examined): "The runtime's library source shall not call a `NodeKey` constructor."
- interface-001-AC-5 (context_only): "The runtime shall define none of `Value`, `ValueType`, ... `NodeKey`, `Origin` or `Location`".
- FR-277-AC-1 (context_only): stays planned; `CheckedEquality::evaluate` still uses the RT walker.
- TC-033 (examined): spec/exact/matrix/TC-033-carried-compiler-vocabulary.md.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | TC-033's procedure was not updated with the cutover, yet the matrix now cites TC-033 as ✅ evidence for FR-012-AC-6. The TC-033 document still says "Check the pre-cutover RT-local node-key parsing and round-tripping", keeps steps 1-2 ("Before the step-4 cutover only, check the local parser...") as live steps, and its Expected Results still describe "the local node-key tests". It has no step for the AC-6 inspection. The TC-033 matrix row lists FR-012-AC-6, with a ✅ status, against a procedure that never checks it | spec/exact/matrix/TC-033-carried-compiler-vocabulary.md:11-30; spec/exact/matrix/tests.md:98 |
| FND-002 | low | interface-001-AC-6 ("library source shall not call a `NodeKey` constructor") is the same claim the new test inspects, but its core-matrix row was not touched. It still reads "🚧 partly adopted ... require separate end-state inspection under IR-349" with no mention of the AC-6 evidence. interface-001-AC-5's `NodeKey` clause also now holds for RT, and is likewise unrecorded | spec/core/matrix/tests.md:24 |
| FND-003 | low | The FR-012-AC-2 row still reads "✅ implemented for the current RT vocabulary; evidence leaving with the residue will be reclassified in its cutover". This PR is that cutover for `NODE_KEY_DOMAIN`. `tc_033_digest_domains_equal_their_normative_strings` now asserts the kernel's `quire_exact::NODE_KEY_DOMAIN`, which is owner evidence, not RT vocabulary. The row was not reclassified | spec/exact/matrix/tests.md:68; tests/exact_vocabulary.rs:405-409 |

## Verdict

**PASS with findings.** The coverage delta is real: 126 to 125 unbacked rows, 0 contradicted on both trees, with nothing newly unbacked. AC-1 was retired cleanly, and AC-6 holds at this head. FND-001 should be fixed in this PR, together with SR-13325 FND-001, which covers how much of AC-6 the cited test actually automates. Fixing both makes the ✅ on FR-012-AC-6 rest on a procedure and a test that check AC-6's full text. FND-002 and FND-003 are bookkeeping.

## Dispositions

Round 1 checked the fix commit `abe9f4a` at head `abe9f4ac6c95b5d7d483682975ea7c582473a59f`. The reviewer re-ran the gates on a fresh worktree: `make fmt-check lint test-features doc msrv size deny test` exited 0. `make spec` reports 235/235 grammar-clean, and strict coverage has 125 unbacked rows and 0 contradicted, unchanged from the first pass.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | TC-033 now opens with "Check the absence of RT-local node-key ownership after the IR-882 key-owner cutover". Step 1 runs `tc_033_runtime_has_no_local_node_key_constructor` as the AC-6 inspection. The retired steps moved to "Retired Procedure (FR-012-AC-1)", and Expected Results now state the AC-6 outcome |
| FND-002 | fixed | Both interface-001 rows in `spec/core/matrix/tests.md` now record the evidence: the AC-5 row says its `NodeKey` clause holds, and the AC-6 row names the recursive inspection test |
| FND-003 | fixed | FR-012-AC-2 now has its own row: "checks kernel-owned `quire_exact::NODE_KEY_DOMAIN` and RT-owned `COMPOUND_UNIT_DOMAIN`; the NodeKey domain assertion is owner evidence, not RT vocabulary evidence" |
