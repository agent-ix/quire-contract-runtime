---
id: SR-11560
title: "IR-61 residue slice code and Rust review"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-runtime@170c5c15e1b94fff2e1334120b77919b10f0b174 (base 0fcccfc70f58407e1a25c041a0f6b8639f94911f); tests/integration.rs; tests/exact_vocabulary.rs; tests/exact_meter_state.rs; src/exact/reference.rs"
review_set: subset
---

## Summary

Ticket: IR-61. Branch code/ir-61-residue-slice, no PR yet. This review covers the Rust half of
`git diff origin/main...170c5c1`: the FR-004-AC-8 test rewrite, the three F12 inequality
controls, and the F15 doc-comment requalification in `src/exact/reference.rs`. The rust-review
lane is included in this artifact. The spec link edits are reviewed in SR-11562, and
criterion-to-test strength in SR-11561.

## Verdict

**PASS with two low findings.** The rewritten `tc_006_mismatched_identity_leaves_every_counter_and_snapshot_untouched`
now records one passed, one failed, one rejected verdict and one discard before each mismatch.
It asserts the exact prior counts (2/1/1/1) and asserts that both the wrong-requirement and the
wrong-revision control leave the counts and the snapshot unchanged. Each control uses a fresh
report, so neither control can hide the other. The test's assertions are not inside guards that
could skip them. The `Trace: TC-006, FR-004-AC-8` tag names the right criterion, and the test name matches what it checks.

Mutation evidence, from a throwaway worktree:

- Zeroing all counters on mismatch is killed at `tests/integration.rs:188`. The same mutant
  survives the main version of this test.
- Zeroing `discarded` only on a revision mismatch (the second control) is killed at `:188`.
- Counting the refused verdict before returning `Err` is killed at `:188`.
- A `snapshot()` that returns zero counts survives this test. It is killed elsewhere in the
  suite. See FND-001.

The rust-review lane found no new `unsafe`, `#[allow]`, panic, or library-code change. The
`.unwrap()` calls are in test setup only, matching the file's convention.
`cargo clippy -p quire-contract-runtime --all-targets --all-features -- -D warnings`,
`cargo fmt --check` and `RUSTDOCFLAGS=-D warnings cargo doc --all-features` are clean.

The `reference.rs` claim is true at QSpec origin/main 3e51cf9. QSpec FR-009 compares universe,
type and object identities. QSpec FR-204 types that triple as the key. The doc comment changes no
behaviour.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The FR-004-AC-8 test's snapshot leg compares `report.snapshot()` with `snapshot_before`, and both come from the same `snapshot()` call path. The snapshot is never anchored to `prior_counts`. A mutant `snapshot()` that returns `CampaignCounts::new()` passes every test in `tests/integration.rs`, and only `tc_015` in `src/accounting_tests.rs` kills it. Add `assert_eq!(snapshot_before.counts(), prior_counts)` so this test's own oracle covers the "prior snapshot" clause. | tests/integration.rs:180-189 |
| FND-002 | low | The F15 fix spells the upstream ids `ix://agent-ix/quire-specification/FR-009` and `.../FR-204`. Line 3 of the same doc comment uses `quire-specification/FR-143`, as do the other 34 upstream references under `src/exact/`. One file now uses two spellings for the same upstream namespace, which is the inconsistency F16 describes. Use `quire-specification/FR-009` and `quire-specification/FR-204` to match `src/`, or convert `src/` as a whole. | src/exact/reference.rs:3-6 |

## Dispositions

Round 1 checks fix head 129c9d6ee790321368052e8d4f003c64bfb5a9fa against
`git diff 170c5c1..129c9d6`.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | `tests/integration.rs:181` now reads `assert_eq!(snapshot_before.counts(), prior_counts);`. The mutant `snapshot()` that returns zero counts is now killed at `:181`, inside the FR-004-AC-8 test itself. |
| FND-002 | fixed | `src/exact/reference.rs:6` now reads `//! quire-specification/FR-009 and quire-specification/FR-204`. This matches line 3 and the other upstream references under `src/exact/`. The warning-denied `cargo doc` is clean, and behaviour is unchanged. |

## Coverage

- Examined FR-004-AC-8 (TC-006), FR-012-AC-5 (TC-033) and FR-011-AC-7 (TC-032) as context for
  the F12 controls. Read QSpec FR-009 and FR-204 at origin/main 3e51cf9.
- `cargo test --locked --test integration`: 5 passed. With `--features exact`, the
  `exact_vocabulary` tests (25) and the `exact_meter_state` tests (9) passed.
- Full `make ci` was not run in this review. The leader owns it.
