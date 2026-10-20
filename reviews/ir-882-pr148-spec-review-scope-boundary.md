---
id: SR-13442
title: "Scope boundary review of RT R-3 cause ownership"
type: SpecReview
analysis: scope-boundary
scope: "agent-ix/quire-contract-runtime; spec/assurance/AD-004-runtime-crate-layout.md, spec/exact/functional/FR-012-carried-compiler-vocabulary.md, spec/exact/functional/FR-275-single-exact-kernel.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-runtime/FR-012
    type: reviews
---

## Summary

Ticket: IR-882. Reviewed responsibility allocation between QSpec's diagnostic families, QSV's definition refusal vocabulary and RT's temporary local residue. The new criterion requires direct QSV consumption after the owner contract, with no RT alias or adapter.

## Verdict

PASS. The spec distinguishes QSV definition-admission causes from stale-dependency digest causes and leaves RT's present local vocabulary explicitly planned for deletion.

## Examined

FR-012-AC-7 (examined): RT relinquishes the three-cause definition surface and consumes QSV directly. FR-012-AC-3 (context only): QSV retains eight selection spellings and `invalid_package`. FR-275 residue row (examined): local definition vocabulary is temporary residue. AD-004 R-3 decision (examined): three extra causes retire; QSV owns the valid code/cause pairing.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |
