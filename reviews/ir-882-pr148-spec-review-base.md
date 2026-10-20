---
id: SR-13438
title: "Base review of RT R-3 definition cause retirement spec"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-runtime; spec/assurance/AD-004-runtime-crate-layout.md, spec/exact/functional/FR-012-carried-compiler-vocabulary.md, spec/exact/functional/FR-275-single-exact-kernel.md, spec/exact/matrix/TC-033-carried-compiler-vocabulary.md, spec/exact/matrix/tests.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-runtime/FR-012
    type: reviews
---

## Summary

Ticket: IR-882. Reviewed the spec-only PR against RT source, QSV's current definition vocabulary, and QSpec FR-272's cause families. The existing six FR-012 criteria and TC-033 artifact remain; new FR-012-AC-7 is an inspectable, planned cutover obligation.

## Verdict

PASS for the base checklist. The local definition code remains in RT, and this PR makes no present-tense claim that AC-7 is implemented. A stale use-audit sentence in AD-004 is recorded in SR-13439.

## Examined

FR-012-AC-3 (examined): selection codes and `invalid_package` spelling remain byte-exact. FR-012-AC-7 (examined): no RT-local definition causes, aliases, re-exports or adapters after the later cutover. FR-275-AC-14/15 (examined): preserve requirements and mark departed evidence planned. TC-033 (examined): adds an inspection step after R-3; its matrix row is planned. AD-004 R-3 decision (examined): retire three RT-only causes and wait for QSV's owner contract.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |
