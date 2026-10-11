---
id: SR-7520
title: "IR-752 code and Rust review of exact quantity tests"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-runtime@fe731da5c15aeae1de53bc370da3af0012870b4f; tests/exact_semantics.rs; src/exact/quantity.rs; src/exact/unit.rs; spec/exact/functional/FR-007-exact-scalar-families.md; spec/exact/matrix/TC-034-local-exact-semantics.md; quire-specification FR-142-AC-6"
review_set: subset
---

## Summary

Ticket: IR-752. Reviewed the one-file test diff against RT main 6a55c681e488e4b577f090e1baab146f6480440f, the quantity implementation and the stated RT and QSpec criteria. The disposition pass checked the two-line fix at fe731da5c15aeae1de53bc370da3af0012870b4f; no production or CI file changed. No applicable AssuranceProfile was found.

## Verdict

**PASS** — The exact-value assertions use literal rational oracles and expected compound units built from explicit canonical root keys and exponents. An incorrect arithmetic result, wrong root, wrong exponent, failure to cancel, incorrect `0^0`, or incorrectly completed negative power fails an assertion. The two-line fix adds the right-affine operations and introduces no code or Rust review finding. The gap-analysis artifact records the finding and its fixed disposition.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Coverage

- Examined FR-007-AC-12, TC-034, and QSpec FR-142-AC-6; read FR-142-AC-5 as context for affine refusal.
- The `Trace: TC-034, FR-007-AC-12` tags on both new test functions bind the RT criterion; QSpec FR-142-AC-6 is an external behavioral authority and has no RT-local trace id.
- `cargo test --features exact --test exact_semantics`: 3 passed, 0 failed. `git diff --check`: clean.
- Full native gates, strict coverage, and Kani are owned by the lead and were not rerun in this scoped review.
