---
id: SR-091
title: "Base specification review of IR-1430 runtime matrix status update"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-runtime@857e7730c6e57a7992b44397540e530c09aacdc0; spec/exact/matrix/tests.md, spec/monitor/matrix/tests.md"
review_set: base
---

## Summary

Ticket: IR-1430. Reviewed the changed matrix entries against their criterion text and the already-tagged named evidence in quire-integration main. The five promoted interface rows and the three incomplete Planned rows are consistent with their stated coverage; FR-010-AC-6 is incorrectly downgraded to Partial.

## Verdict

**CONDITIONAL** — one medium finding: the FR-010-AC-6 edit adds a runtime-path evidence condition absent from the criterion and invalidates an existing whole-criterion test reference.

## Scope

- FR-010-AC-6 — role: examined. Excerpt: “A 0-based `occurrence` cannot be constructed, so an injected denial can never silently match no charge and degrade into ‘no fault injected’.”
- FR-010-AC-6 matrix row — role: examined. Excerpt: “Partial: quire-exact `injected_denial_occurrence_requires_nonzero` ... proves zero cannot be represented by `InjectedDenial::occurrence` ...; RT has no local test exercising injected-denial matching through the runtime charge path.”
- interface-002-AC-1 — role: examined. Excerpt: retained program keeps the complete root key, entry associations, literal bodies and shared declarations after the preparation owners drop.
- interface-004-AC-1 — role: examined. Excerpt: missing, duplicate, sparse and foreign declarations refuse construction, while compact function-qualified bindings retain original source locations.
- interface-006-AC-3 — role: examined. Excerpt: retained projection addresses are scratch-charged; deep projections use the explicit machine; source keys and body paths survive without subtree copies.
- interface-011-AC-3 — role: examined. Excerpt: earlier stopped payloads prevent later execution; native shortages and scratch denial retain typed evidence without partial result or state commit.
- interface-012-AC-1 — role: examined. Excerpt: changed context or invalid required values refuse before execution with state and reading/progress baselines unchanged.
- interface-003-AC-3 — role: examined. Excerpt: native shared-conformance storage/path-capacity failure is not covered; remains Planned.
- interface-007-AC-3 — role: examined. Excerpt: no deep record-projection test establishes its explicit-continuation clause; remains Planned.
- interface-010-AC-3 — role: examined. Excerpt: named tests assert native and scratch stops, but the whole criterion includes stop-before-later-work/state-cleanliness clauses not fully established; remains Planned.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-010-AC-6 is downgraded to Partial because RT has no test of injected-denial matching through the runtime charge path, but the criterion asks that a zero-based occurrence cannot be constructed; the tagged quire-exact test asserts `NonZeroU64::new(0)` is `None` and that `InjectedDenial::occurrence` has type `NonZeroU64`, which establishes the stated construction restriction and its consequence. Restore the prior Implemented status and its named evidence. | spec/exact/matrix/tests.md |

## Dispositions

Disposition pass 1, fix commit `72bf9057908de5d30d8e264091a4acc3a932f121`. The restored row and its named evidence were confirmed at that commit; the `quire-exact` main test body asserts zero is `None` and the occurrence field is `NonZeroU64`.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Commit 72bf905 restores the evidence-backed Implemented status. |

### FND-001 after excerpt

```text
| FR-010 | FR-010-AC-6 | TC-031 | ✅ Implemented: backed by quire-exact test `injected_denial_occurrence_requires_nonzero` (`src/accounting.rs`, tagged FR-358-AC-3), which asserts `NonZeroU64::new(0)` is `None` and `InjectedDenial::occurrence` has type `NonZeroU64`; therefore occurrence zero cannot be constructed |
```
