---
id: SR-5240
title: "IR-715 code-review review of proptest feature evidence"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-runtime@8a55e08cfc12160e688ace6f5d7c85d080653fcc; spec/proptest_adapter/functional/FR-003-proptest-adapter.md, spec/proptest_adapter/matrix/TC-004-proptest-map.md, spec/proptest_adapter/matrix/tests.md, src/lib.rs, tests/proptest_adapter.rs"
review_set: subset
---

## Summary

Reviewed PR #115 for IR-715 at the frozen head. The newly added FR-003-AC-2 tag is on a test behind #![cfg(feature = "proptest")]. It can prove the public module exists with the feature, but cannot prove that the module is absent without it. Quire strict matrix reports this test as the only AC2 binder; the feature-off compile_fail doctest is not bound. Move the AC2 trace to executable feature-off evidence, while retaining the feature-on control under the appropriate criterion.

## Verdict

**CONDITIONAL** — Feature-on test is falsely traced to the feature-off criterion.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Feature-on test is falsely traced to the feature-off criterion | tests/proptest_adapter.rs:9 |

## Coverage

Examined FR-003-AC-1 (tri-state mapping), FR-003-AC-2 (feature-off absence and feature-on presence), FR-003-AC-3 (recording semantics), TC-004's two steps, the local matrix row, and the changed source/test trace placement. Plan completion: not assessed.

## Evidence

`cargo test --doc --no-default-features` passed 11 doctests, including the feature-off `src/lib.rs` compile-fail example. The feature-on integration assertions are behind `#![cfg(feature = "proptest")]`. `quire matrix --scope . --strict --format json` binds FR-003-AC-2 solely to `tests/proptest_adapter.rs:11` and reports the PR-wide inherited 49 untagged criteria outside this review slice. Author pre-PR logs: `/tmp/ir715-prepr-ci.log`, `/tmp/ir715-prepr-rest.log`, `/tmp/ir715-prepr-deny-mutations.log`, and `/tmp/ir715-prepr-final.log`.
