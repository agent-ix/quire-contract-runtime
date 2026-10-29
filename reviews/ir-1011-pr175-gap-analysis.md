---
id: SR-568104
title: "IR-1011 PR 175 FR-006 AC3 evidence gap review"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-runtime#175@b5a8de084cf41346fed425f156db7ae63ba9df30; FR-006-AC-3, TC-016, TC-017, spec/exact/matrix/tests.md and the three changed tests"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-runtime/FR-006
    type: references
---

## Summary

Planless manual AC-to-test review of the PR diff. The new trace tags mark FR-006-AC-3, but the added tests do not establish every part of that criterion or the literal TC-017 procedure. A repository-wide Quoin gap audit and run evidence were deferred by the current static-only order.

## Verdict

**FAIL** — the FR-006-AC-3 trace now implies complete local evidence while the LimitKind reverse path and TC-017's concrete scalar denial remain unverified.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The tagged LimitKind census checks hard-coded spellings and a search over `ALL`, but never round-trips a QSpec spelling through a kernel input API; the negative loop only checks the local set. Thus the TC-016 and FR-006-AC-3 vocabulary obligation is not fully evidenced by this trace. | tests/exact_outcome_consumer.rs:182 |
| FND-002 | medium | The new first-short test builds a synthetic `Charge`; it does not execute TC-017 step 1's `2^64 × 2^64` scalar operation, verify its `integer-arithmetic.arithmetic` point, or its exact denied amount 130 after consumed 65. Existing owner evidence may cover a kernel subset, but this RT test does not establish the stated TC-017 procedure. | tests/exact_accounting_consumer.rs:57 |

## Coverage

Plan completion: not assessed. Examined FR-006-AC-3 and the full TC-016/TC-017 procedures against the changed trace-tagged tests. ChargePoint uses the QSpec vector and checks `from_code`; high-water, cumulative and first-short meter tests have discriminating assertions; the function-call denial checks no body side effect. No `src/` code changed, so the PR adds no reverse code-to-spec surface. The RT matrix still labels AC3 and TC-017 open, which is truthful; the trace tag is premature. No tests or builds ran under the one-lane order. QSL-1106's downstream boundary is unresolved and is not claimed as evidence here.

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | deferred | The unsupported LimitKind census and every new FR-006-AC-3 trace tag were removed at 3ee6928, so this PR no longer claims full TC-016/AC3 coverage. The underlying QSpec vocabulary and kernel reverse-intake evidence remain on open IR-1011 for a later slice. |
| FND-002 | deferred | The synthetic meter assertion remains as partial evidence without an FR-006-AC-3 trace tag. TC-017's concrete 2^64 multiplication and exact 130 denial remain on open IR-1011, with QSL-1106 owning the downstream case boundary. |

## Disposition verdict

**CONDITIONAL for this partial evidence slice** — no premature AC3 binding remains; the complete criterion and TC-017 procedure remain open. Static review only; final gates have not run.

## Delta review at 8a9d044

QSpec revision 938452fb now exports a LimitKind vector with all ten ordered ScalarLimitsV1 spellings and eight undeclared spellings. RT's new test consumes that source directly, compares ordered equality, and intends to test declared and undeclared `from_code` behavior. The locked quire-exact revision 6ec67288 lacks `LimitKind::from_code`, so the test cannot compile or supply recognized evidence yet. FR-006-AC-3 remains untagged and TC-017's concrete multiplication case remains open. The prior FND-001 and FND-002 deferrals remain applicable; no new gap finding is added. No build or test ran.

## Delta review at ecb66be

RT Cargo.lock now selects quire-exact 03a9a7f, which exports the intended `LimitKind::from_code`; the QSpec vector test now has a real reverse path for declared and undeclared spellings. This improves the TC-016 vocabulary subset, subject to execution in the sole build lane. FR-006-AC-3 remains untagged because its complete meter-before-work and TC-017 procedure evidence is still open; the matrix remains truthful. No new gap finding or AC narrowing was found in this delta. No build or test ran in this review.
