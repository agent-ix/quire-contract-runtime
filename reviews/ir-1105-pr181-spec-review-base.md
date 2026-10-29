---
id: SR-2026102920
title: "RT PR 181 identity boundary specification review"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-runtime; spec/accounting/functional/FR-004-campaign-accounting.md, spec/core/functional/FR-001-verdict-observation.md, spec/exact/functional/FR-012-carried-compiler-vocabulary.md, spec/exact/matrix/TC-033-carried-compiler-vocabulary.md, spec/exact/matrix/tests.md"
review_set: base
relationships:
  - target: ix://agent-ix/quire-contract-runtime/FR-012
    type: reviews
---

## Summary

Ticket: IR-1105. Examined FR-001-AC-2, FR-004-AC-8, FR-012-AC-2, TC-033 step 2, and the changed matrix row at PR #181 head 758c519. AD-004's test fixture guidance was read as context. No applicable AssuranceProfile was found in this repository's spec tree.

## Verdict

FAIL. The changed AC conflicts with the existing architecture guidance, and the changed test procedure mandates a source scan barred by the specification review checklist. `quire validate --scope . 'spec/**/*.md'` passes and `quoin validate --repo . --json` reports zero structural findings.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | FR-012-AC-2 forbids minting identity fixtures, but AD-004 still explicitly allows `NodeKey::from_digest` fixture minting; update that owner guidance in the same spec change. | spec/exact/functional/FR-012-carried-compiler-vocabulary.md:112; spec/assurance/AD-004-runtime-crate-layout.md:286,427,548-552,571 |
| FND-002 | high | TC-033 step 2 requires inspecting source and fixtures for digest construction, violating the spec-review checklist's ban on test cases scanning source or fixtures; use behavior and a separate inspection claim instead. | spec/exact/matrix/TC-033-carried-compiler-vocabulary.md:28-31 |

## Coverage

Examined FR-001-AC-2 and FR-004-AC-8: retained string identity and mismatch behavior remain explicit. Examined FR-012-AC-2 and TC-033 step 2: findings above. Examined the FR-012-AC-2 TestMatrix row: it correctly remains pending. AD-004 fixture guidance and unchanged TC-033 steps 1, 3-6 were context only.
