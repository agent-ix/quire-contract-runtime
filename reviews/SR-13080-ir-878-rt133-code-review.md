---
id: SR-13080
title: "IR-878 quire-contract-runtime PR 133 code review: FR-008 AC-10/AC-11 collection tests"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-runtime PR #133 vs main; tests/exact_collection.rs, spec/exact/matrix/tests.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-runtime/FR-008
    type: reviews
---

## Summary

Ticket: IR-878. This is a diff-scoped code review of PR #133, with the rust-review lane folded in. The PR changes only Rust test code and one matrix row, so no production source changed. `tc_025_p3` now covers four kinds: sequence, ordered set, set and bag. All four use one shared occurrence vector, `[2, 1, 2, 3]`. The test asserts the retained elements, the `collection.member-walk` and `collection.member-test` counts per kind, and that `collection.bound` and `collection.result-retain` are the last two charges. The test carries `#[trace("TC-025", "FR-008-AC-3", "FR-008-AC-10", "FR-008-AC-11")]`, with no `/// Trace:` lines.

Gates run in a throwaway worktree with its own target dir:
- `cargo fmt --all -- --check` is clean.
- `make lint` is clean: clippy `--all-targets --all-features -D warnings`, plus the footprint thumbv7em clippy.
- `cargo test --all-features --test exact_collection` passes 9/9.

## Reviewed units

- tests/exact_collection.rs `tc_025_p3_collection_membership_charges_retention_order_and_result_retain` (examined)
- spec/exact/matrix/tests.md FR-008 AC-10/11/12 row (examined)
- src/exact/collection.rs `form`, `coalesce`, `member_equal`, `bound_and_retain` (context_only; unchanged by the PR)

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Verdict

The test code is clean. The table-driven case list keeps each kind's expected elements and comparison count side by side. The per-kind `{kind:?}` assertion messages make a failure name its kind. The comment's arithmetic, 0 + 1 + 1 + 2 = 4, matches `coalesce`, which compares in retention order and stops at the first equal member. Using `to_u64` on the integer extraction is safe because every fixture value is non-negative. No unsafe code, panics in production, tautologies or stubs were introduced. The trace form follows the repository convention. Mutation results and whether each clause is asserted are recorded in SR-13081. Two medium AC-11 backing gaps are recorded there, not here.

## Round 1 follow-up

The author removed the FR-008-AC-11 trace tag from `tc_025_p3` while retaining its ordering assertions. A new untagged decimal test asserts that an ordered set formed from `[1.0 (10,1), 2, 1.00 (100,2)]` retains scales `[1, 0]`. Both tests name IR-885 in comments for the outstanding bag clause. No production code changed. The focused `exact_collection` suite passes 10/10; M12 fails the new assertion with `[2, 0]` and was reverted. Formatting and `make lint` passed after the import layout was formatted. Strict coverage is 89/197 versus 88/197 on main, exactly +1 (AC-10). AC-11 and AC-12 remain unbacked. The original clean code-review verdict stands for the reviewed diff.
