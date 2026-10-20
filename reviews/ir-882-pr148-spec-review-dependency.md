---
id: SR-13440
title: "Dependency review of RT R-3 definition cutover"
type: SpecReview
analysis: dependency
scope: "agent-ix/quire-contract-runtime; spec/exact/functional/FR-012-carried-compiler-vocabulary.md, spec/assurance/AD-004-runtime-crate-layout.md, ix://agent-ix/quire-specification/FR-272"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-runtime/FR-012
    type: reviews
---

## Summary

Ticket: IR-882. Checked the new FR-012 reference to QSpec FR-272 and the enablement order for the later RT code cutover. QSpec FR-272 assigns byte and domain digest mismatches to stale-dependency families; QSV currently exposes the three retained definition causes, while a QSV owner FR and tests remain the stated prerequisite for deleting RT's local vocabulary.

## Verdict

PASS. The reference target exists, and the proposed order is QSV owner contract before RT deletion, with no implementation claim in this PR.

## Examined

FR-012-AC-7 (examined): QSV definition types become RT's direct owner after R-3. AD-004 R-3 prerequisite (examined): QSV owner FR and tests precede RT code cutover. QSpec FR-272 cause-family table (context only): digest mismatches sit under stale-dependency/source-digest-mismatch, not invalid-package.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |
