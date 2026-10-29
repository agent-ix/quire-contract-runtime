---
id: SR-2026102919
title: "Base specification review of RT PR 180 identity replacement plan"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-runtime; spec/core/functional/FR-001-verdict-observation.md, spec/accounting/functional/FR-004-campaign-accounting.md, spec/exact/functional/FR-012-carried-compiler-vocabulary.md"
review_set: base
relationships:
  - target: ix://agent-ix/quire-contract-runtime/FR-001
    type: reviews
  - target: ix://agent-ix/quire-contract-runtime/FR-004
    type: reviews
  - target: ix://agent-ix/quire-contract-runtime/FR-012
    type: reviews
---

## Summary

Ticket: IR-1105. Reviewed RT PR #180 at 8566e1a5018ded988518eeed5d0969e23f74f9b8. The three added Planned sections describe a future replacement of carried revision and node identities. FR-001-AC-2, FR-004-AC-8, FR-012-AC-2, and FR-012-AC-6 remain intact, as do the tests tagged to them. The proposed replacement remains contingent on QSpec identity and QSL producer contracts. No applicable AssuranceProfile was found under spec/.

## Verdict

PASS for the spec-only diff. It creates no executable claim or frozen expected digest. The revision mismatch still refuses without changing campaign counts or an earlier snapshot, and the planned shared-kernel replacement preserves foreign-declaration, foreign-universe, and cross-domain distinctions without an RT constructor. `quire validate --scope . 'spec/**/*.md'` exited 0. Strict coverage reports the existing 85 unbacked rows and 0 contradicted statuses, with no new ACs in this PR.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Coverage

Examined the new Planned sections in FR-001, FR-004, and FR-012 and the active FR-001-AC-2, FR-004-AC-8, FR-012-AC-2 and FR-012-AC-6 rows. FR-004-AC-4/5/9 and FR-012-AC-7 were context for the current snapshot and shared-kernel contracts. The four active ACs have tagged tests in `tests/integration.rs` and `tests/exact_vocabulary.rs`. No build or code tests ran for this spec-only review.
