---
id: SR-13120
title: "IR-885 quire-contract-runtime PR 134 code review: bag retention keeps every occurrence"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-runtime PR #134 head ee0985f vs main 1a7e12b; src/exact/collection.rs, tests/exact_collection.rs"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-runtime/FR-008
    type: reviews
---

## Summary

Ticket: IR-885 (parent IR-878). This is a diff-scoped code review of PR #134 with the rust-review lane folded in. The PR changes `coalesce` in `src/exact/collection.rs`: a distinct member is now stored with a list of its later equal occurrences instead of a multiplicity, and the output lists the first occurrence then each later one, rather than `repeat_n(first, multiplicity)`. A bag of decimals `[1.0, 2, 1.00]` therefore retains scales `[1, 2, 0]` instead of `[1, 1, 0]`. A new test asserts this on two decimal inputs, and FR-008-AC-11 is tagged on three tests.

Gates, run in a throwaway worktree with its own target dir under the heavy lock:
- `cargo fmt --all -- --check` is clean.
- `make lint` is clean (clippy `--all-targets --all-features -D warnings`, plus the footprint thumbv7em clippy).
- `cargo test --all-features --test exact_collection` passes 11/11; `exact_equality` passes 7/7; `exact_composite` passes 10/10.
- `make size` passes at 973 bytes. The footprint graph holds no exact dependencies, so this change cannot move it.

Mutation checks against the PR's tests (all reverted):
- Multiplicity coalescing restored (`repeat_n(member, repeats.len() + 1)`): killed by the new bag test.
- Later occurrences placed before the first (wrong tie order): killed by the new bag test.
- One later occurrence dropped (`repeats.skip(1)`): killed by the bag test and `tc_025_p3`.
- Elements reversed before the key sort (reverse authored tie order): killed by the bag test.
- Bag skips the key sort: killed by the bag test and `tc_025_p3`.
- M12, an ordered set keeping a later equal value: killed by `ordered_set_retains_first_equal_decimal_representation`.
- `sort_by` replaced with `sort_unstable_by`: survives (FND-002).

## Reviewed units

- src/exact/collection.rs `coalesce` (examined; changed)
- src/exact/collection.rs `bound_and_retain`, `sort_by_key`, `member_equal`, `form` (context_only; unchanged)
- src/exact/composite.rs `Value::occ` (context_only)
- tests/exact_collection.rs `bag_retains_each_decimal_occurrence_in_stable_key_order` (examined; new)
- tests/exact_collection.rs `tc_025_p3_collection_membership_charges_retention_order_and_result_retain` and `ordered_set_retains_first_equal_decimal_representation` (examined; tag added)
- spec/exact/functional/FR-008-composite-collection-and-equality.md Description, Behavior lines 67-71 and FR-008-AC-11 (context_only)
- quire-specification FR-144 and quire-exact `src/collection.rs` `coalesce` and FR-379 at origin/main (context_only; the authority and the kernel)

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The new bag retention contradicts the semantic authority and the consumed kernel. FR-008 binds RT to the QSL authority and FR-144. FR-144 defines a bag as a member-to-multiplicity map in which a later equal occurrence "adds one occurrence to the equal bag member", and FR-144-AC-21 makes `1.0` and `1.00` one member of multiplicity 2. quire-exact `coalesce` keeps `(member, multiplicity)`. The PR makes RT's local copy of `coalesce` diverge from the kernel on every bag holding equal values with different representations | src/exact/collection.rs:253-275 |
| FND-002 | low | The "stable key order" tie order rests on `slice::sort_by` stability, but no test can detect an unstable sort: `sort_unstable_by` survives because both bag inputs have four or fewer elements | tests/exact_collection.rs:244-292 |

## Verdict

CHANGES REQUESTED; not mergeable as-is because of FND-001.

As Rust the change is sound. It adds no panic, no indexing, no unsafe code and no new integer conversion. `Vec::new()` for a member with no repeats does not allocate. The comparison loop still compares each candidate only with distinct members, in retention order, stopping at the first equal member. So `collection.member-walk` and `collection.member-test` counts are unchanged, and `tc_025_p3`'s counts still hold. Equal values of one element type have equal `occ`, so `occ(result)`, the `collection.result-retain` charge and the bound count are also unchanged. No refusal path changes. Tie order among equal keys is authored order: the group order is first occurrence then later occurrences in source order, and the key sort is stable. It is deterministic and uses no hash. Sets, ordered sets and sequences are unaffected: a set or ordered set never pushes to `repeats`, and a sequence never reaches `coalesce`.

The defect is in what the change implements, not how. FR-008's Description says RT "is an implementation of that definition, not a second semantic authority", and that `quire_spec_language::value` decides every construction and canonical-order question. That authority, quire-specification FR-144 (lines 52, 223-224 and 263-264, and AC-21), models a bag as distinct members with multiplicities. The kernel RT depends on, quire-exact `src/collection.rs` `coalesce` at origin/main 582d586, keeps the first representation repeated, and its FR-379 says a bag "merge[s] equal members". FR-008-AC-11's "every occurrence in ascending canonical-key order" reads consistently with that model, since FR-144's key table lists "a bag occurrence ... once per multiplicity". It does not clearly require a distinct representation for each occurrence. The spec text the PR cites is therefore at best ambiguous, and the fix is not justified by it. The consistent resolution is IR-885 option (b): restore multiplicity retention, clarify FR-008 lines 70-71 and AC-11 to say a bag lists each distinct member's first occurrence once per multiplicity, and tag AC-11 with a test asserting `[1, 1, 0]`. If the owner wants per-occurrence representations, that is an FR-144 and quire-exact change first, not an RT-only fork.

Downstream: quire-contract-codegen depends on `quire_contract_runtime::exact` (`src/oracle/equality/mod.rs`) and generates code that calls it, so a bag's stored element representations would differ between RT and the kernel at runtime. Equality results, charges and work-unit totals do not change. RT has no digest or serialization over bag elements. No CG, quire-exact or QSL test was found that pins bag element representations.
