---
id: SR-5243
title: "IR-715 spec-review-evidence review of proptest feature evidence"
type: SpecReview
analysis: evidence
scope: "agent-ix/quire-contract-runtime@8a55e08cfc12160e688ace6f5d7c85d080653fcc; spec/proptest_adapter/functional/FR-003-proptest-adapter.md, spec/proptest_adapter/matrix/TC-004-proptest-map.md, spec/proptest_adapter/matrix/tests.md, src/lib.rs, tests/proptest_adapter.rs"
review_set: subset
---

## Summary

Reviewed PR #115 for IR-715 at the frozen head. The matrix marks FR-003-AC-2 complete through TC-004 and names the feature-off doctest, yet Quire strict inspection binds AC2 only to the feature-on integration test. Thus the authored evidence location and computed trace disagree. Bind the feature-off compile-fail example to AC2, then recompute the matrix before retaining the Complete claim.

## Verdict

**CONDITIONAL** — TC-004 completion claims feature-off evidence through the wrong bound test.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | TC-004 completion claims feature-off evidence through the wrong bound test | spec/proptest_adapter/matrix/tests.md:14 |

## Coverage

Examined FR-003-AC-1 (tri-state mapping), FR-003-AC-2 (feature-off absence and feature-on presence), FR-003-AC-3 (recording semantics), TC-004's two steps, the local matrix row, and the changed source/test trace placement. Plan completion: not assessed.

## Evidence

`cargo test --doc --no-default-features` passed 11 doctests, including the feature-off `src/lib.rs` compile-fail example. The feature-on integration assertions are behind `#![cfg(feature = "proptest")]`. `quire matrix --scope . --strict --format json` binds FR-003-AC-2 solely to `tests/proptest_adapter.rs:11` and reports the PR-wide inherited 49 untagged criteria outside this review slice. Author pre-PR logs: `/tmp/ir715-prepr-ci.log`, `/tmp/ir715-prepr-rest.log`, `/tmp/ir715-prepr-deny-mutations.log`, and `/tmp/ir715-prepr-final.log`.

## Dispositions

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed 1d6ef4849e97e196abc0d9c4bbd62e9a4aa8dda4 | The matrix now names the executable feature-off probe, matching Quire’s sole AC2 verifying claim. |

## Disposition Verdict

**PASS at 1d6ef4849e97e196abc0d9c4bbd62e9a4aa8dda4** — FND-001 is fixed; no new findings in the fix diff.
