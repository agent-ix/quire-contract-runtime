---
id: SR-35014
title: "Base spec review of IR-349 RT PR 159 (AD-004 delivery split to IR-1009)"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-runtime; PR #159 at 56fd812cad256616ed5ca92a76f6fa66ef501320 against origin/main 754e2566003295c54a4c96b8bcc3928de3e6e8b2; spec/assurance/AD-004-runtime-crate-layout.md; spec/core/matrix/tests.md; spec/exact/functional/FR-275-single-exact-kernel.md; spec/exact/matrix/TC-197-single-kernel-ownership.md; spec/exact/matrix/tests.md"
review_set: subset
---

## Summary

Ticket: IR-349. Draft PR agent-ix/quire-contract-runtime#159 at frozen head
56fd812, diff against origin/main 754e256 only. Fresh review: an earlier
reviewer's partial scratchpad files were not reused (only their SR ids, after a
census showed no collision in RT main, the one open head, or the scratchpad).

The PR moves AD-004 step 5 (RT `expression` port deletion, evaluation-leaf
dependency, paired CG emitters, Group 3 amendments) and the empty-`exact`
part of step 6 from IR-349 to IR-1009. IR-349 keeps steps 1 to 4. The PR also
says the IR-497 depth redesign can be done on `std`.

Verified against the repository:

- No acceptance-criterion row in any file is edited. The changed matrix rows
  keep identical AC and TC lists, and every changed status stays 🚧
  planned/pending. `quire coverage --strict` gives 93 unbacked and
  0 contradicted on both head and main.
- The rewording of the FR-275 and interface-001 status rows is accurate for
  the current code. `src/exact/` now holds only `boolean.rs`, `division.rs`,
  `expression.rs`, `ieee.rs` and `mod.rs`, so dropping the quantity, unit,
  enumeration, definition and "shared value shapes" residue wording is
  correct. TC-197 no longer lists IR-583, which is Done in Linear.
- IR-1009 exists (Backlog, project Unified Evaluation Core), and IR-590 blocks
  it. IR-582 and IR-583 are Done.
- The std-feasibility claim holds in the code. RT
  `src/exact/expression.rs:89` defines `MAX_CALL_DEPTH = 128` and documents
  that `Frame::call`, `CheckedPackage::call` and `CheckedPackage::evaluate`
  recurse on the host stack. Nothing in that code needs `no_std` to be
  redesigned. quire-exact at locked 1c6cbcb already hand-writes
  `Clone`/`PartialEq`/`Hash`/`Drop`/`Debug` for the recursive `ValueType`
  (`src/value/value_type.rs:152-229`), with `Option(Arc<ValueType>)` and a
  `Collection` arm, so that walks run on a fixed stack. RT
  `src/snapshot_json.rs` uses a `serde_json::Deserializer` with the default
  128 recursion limit. Its input is the flat campaign snapshot, which carries
  no `ValueType`, so that limit does not constrain IR-497.
- The PR adds no compatibility layer, fallback or dual path. AD-004 states
  "There is no interim copy approval, fallback, alias or compatibility layer".
- FR-278-AC-1 is not edited. FR-278 still makes the removal of the
  no-`CallDepthExceeded` carrier depend on the IR-497 cutover (owned by
  quire-exact FR-369-AC-1).

## Scope examined

- AD-004 "Delivery scope after the no_std evaluator deferral" (both new paragraphs), migration table rows 5 and 6, and `system:` frontmatter: examined.
- AD-004 step 3 row, decision C, the What-this-crate-keeps table (11 `scalar` items), and Q-1: context only.
- FR-275 Description, step-2 tracking sentence, Residue intro, and Downstream dependency: examined. FR-275-AC-1/13/16/18/29/30: context only, confirmed unchanged.
- FR-273-AC-7, AC-12, AC-13: context only, unchanged. FR-278-AC-1: context only, unchanged.
- spec/exact/matrix/tests.md rows FR-273 AC-12/13, FR-275 AC-1..19, FR-275 AC-29/30, TC-194, TC-197: examined.
- spec/core/matrix/tests.md interface-001 rows: examined.
- TC-197 Description: examined.
- Code: RT src/exact/*, src/lib.rs, src/snapshot_json.rs; quire-exact 1c6cbcb value.rs, value/value_type.rs, collection.rs: context only.
- Linear: IR-349, IR-1009, IR-497, IR-512, IR-582, IR-583, IR-590, IR-882 state and relations: context only.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Nothing names a live owner for creating the `scalar` module. AD-004 says IR-349 owns steps 1-4 "including the 11 runtime-owned `scalar` items". Step 3 only says "connective to `scalar`", and the new step 6 gives removal of the `exact` module to IR-1009. On head, all 11 items still sit in `src/exact/{boolean,division,ieee}.rs` and there is no `scalar` module (interface-001:168: "a scalar module that does not exist yet"). Linear moved IR-349 to Done at 2026-10-25T04:28Z, so the work this PR keeps on IR-349 has no open ticket. Fix: say which step and ticket create `scalar` and move all 11 items, and make that ticket's state match. | spec/assurance/AD-004-runtime-crate-layout.md:378-390,419,422 |
| FND-002 | medium | Linear contradicts the spec's statement that the IR-590 leaf "is not a precondition" for steps 1-4, and its claim that IR-497 is feasible on std: IR-590 still `blocks` both IR-349 and IR-497. The IR-497 paragraph also does not name the criteria that wait on it, FR-278-AC-1 and quire-exact FR-369-AC-1 (the `CallDepthExceeded` removal). So their std path is only implied, and the tracker keeps them blocked on the deferred no_std work. Fix: name FR-278-AC-1/FR-369-AC-1 as IR-497 std scope, and have the planner remove the IR-590 blocks-IR-497 and IR-590 blocks-IR-349 edges. | spec/assurance/AD-004-runtime-crate-layout.md:391-401 |
| FND-003 | medium | The receiving ticket does not list what it receives. The IR-1009 description says only that it holds "deleting the RT expression port". It does not cover the step-6 end-state closure (removing the `exact` module, L-1 to L-4) or the criteria that now wait on it (FR-273-AC-12/13, FR-275-AC-1/13/16/18/29/30, TC-194/197/199). Those moves are recorded only in the spec prose. Fix: the planner adds that scope and AC list to IR-1009. | spec/assurance/AD-004-runtime-crate-layout.md:378-390 |
| FND-004 | low | The paragraph says the std redesign "can be exercised by 100,000-deep calls and type values on a small native thread stack". No std AC states that figure: FR-273-AC-7 is 4,096 calls on a 64 KiB stack, and 100,000 belongs to the no_std AC-13. A reader could take the number as std acceptance. Cite AC-7's figures, or mark the 100,000 as illustrative. | spec/assurance/AD-004-runtime-crate-layout.md:391-394 |

## Verdict

Not mergeable as is. Changes requested, all small. Every AC is preserved
verbatim and only its owner changes. Nothing is narrowed or deleted, Planned
stays Planned, the std claim holds in the code, and there is no compat
wording. FND-001 is the one issue that leaves work without an owner: the
`scalar` relocation has no open ticket now that IR-349 is Done. FND-002 and
FND-003 are mostly tracker fixes for the planner, plus one cross-reference
sentence in AD-004. FND-004 is wording.

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-005 | low | The rewritten IR-1009 description calls the expression-port deletion "step 6 of RT AD-004". At 4a2f998, AD-004 assigns that deletion to step 5 (row "5 (IR-1009)"); step 6 is the end-state closure that follows it. This is tracker text, not spec, so no PR change is needed: the planner should correct the step number in IR-1009. | spec/assurance/AD-004-runtime-crate-layout.md:425-426 |

## Dispositions

Round 1, checked at head 4a2f998bab9f45e3c0fd5e76ff114f70372710f1 (fix commit 4a2f998 over 56fd812) and against Linear state at review time.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | AD-004 now states "Its step 3 creates the `scalar` module, moves all 11 runtime-owned items out of `src/exact`", and the step-3 row is labelled "3 (IR-349)". IR-349 is back to In Progress in Linear. |
| FND-002 | still-open | The spec half is fixed: AD-004:382 says "IR-590 is not a precondition for IR-349 or IR-497", and :398-400 give IR-497 the std `CallDepthExceeded` removal for FR-278-AC-1 and quire-exact FR-369-AC-1. The tracker half is not: Linear still records IR-590 blocks IR-349 and IR-590 blocks IR-497, which contradicts that new sentence. Only a planner edit in Linear is missing (remove both edges); no further PR change is needed. |
| FND-003 | fixed | The IR-1009 description now lists "the end-state closure that follows, plus the criteria waiting on it: FR-273-AC-12/13, FR-275-AC-1/13/16/18/29/30, TC-194/197/199". (Its step number is off; see FND-005.) |
| FND-004 | fixed | AD-004 now cites "FR-273-AC-7's required std call test is 4,096 calls on a 64 KiB native stack" and puts the 100,000-call, 512 KiB test under FR-273-AC-13. |
