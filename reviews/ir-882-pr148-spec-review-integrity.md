---
id: SR-13439
title: "Integrity review of RT R-3 definition cause retirement spec"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-contract-runtime; spec/assurance/AD-004-runtime-crate-layout.md, spec/exact/functional/FR-012-carried-compiler-vocabulary.md, spec/exact/functional/FR-275-single-exact-kernel.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-runtime/FR-012
    type: reviews
---

## Summary

Ticket: IR-882. Compared the R-3 decision narrative, new criterion, and residue table for present-tense consistency. The new AC is atomic enough to inspect at the code cutover; the audit sentence in AD-004 no longer describes the final PR's own spec text.

## Verdict

CONDITIONAL. Correct AD-004's use-audit sentence so it distinguishes pre-decision mentions from the new FR-012 and FR-275 text.

## Examined

AD-004 R-3 audit sentence and decision (examined): names the three retired RT causes. FR-012-AC-7 (examined): inspect no local types or adapter after code cutover. FR-012-AC-3 (context only): preserved spelling contract. FR-275 residue table (examined): records planned definition retirement.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | AD-004 says the retired names occur only in `definition.rs` and this AD, but this PR also adds them to FR-012 and FR-275 | spec/assurance/AD-004-runtime-crate-layout.md:488 |

## Detail

The sentence introduces itself as an RT source/test/spec use audit and then says the three names "occur only in `src/exact/definition.rs` and this AD." The same reviewed head mentions all three in the added FR-012 behavior and FR-275 residue table. Make the claim explicitly historical, or limit it to RT source/tests and separately identify the authored spec references.

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | At PR #148 head `87ce12b234902a40ce4aa7a0f70166fd56efeaf7`, AD-004 now limits the absence claim to RT source producers, consumers and tests, says the `definition.rs`/AD occurrence was before the amendment, and explicitly names the new FR-012 and FR-275 retirement references. The same fix names landed QSV FR-125 and TC-924. |
