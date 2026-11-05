---
id: SR-20261104024426
title: IR-499 PR205 spec-review-base review
type: SpecReview
analysis: base
scope: agent-ix/quire-contract-runtime; PR205; src/bin/check_deny_source.rs; spec/exact/matrix/TC-198-qsl-crate-edge-guard.md
review_set: subset
---

## Summary

Ticket IR-499; PR205 frozen head 591978d1d1cab6b220958eac3999ba443c661d62, base 0970d98. Only src/bin/check_deny_source.rs and TC-198 step 5 changed. Split comments 5081c0a2 and 5ff51184 were read. No applicable AssuranceProfile exists under spec/. Consumer files, shared matrices, requirement methods, dependencies and CI gates are unchanged.

## Verdict

PASS within changed scope; no defects found. This is not whole-repository assurance or ticket closure.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Coverage

Base checklist applied to the added TC-198 step 5. The command selects the actual binary tests; the exact-token/nonzero/diagnostic-association claims match their assertions. The procedure explicitly denies execution credit for dependency injections, preserving steps 2 through 4 and the whole test-case expected result. No IDs, relationships, requirement statements, domain objects, authored verification methods, pins, tracking ledgers or file snapshots change. EARS, integrity, dependency, object and method-advisor lenses add no applicable changed material; no new method strategy is being chosen. Structural and link checks are confined to TC-198 and its existing FR-275 relationship.

## Verification limits

Read-only review of source and existing author receipts at /tmp/ir499-gate-next-focused; no Cargo, tests, native mutations, make gates, prover, global matrix or census rerun. Existing green.log records 3 passing tests, none ignored; red.log records both new tests failing against old production while the existing control passes. native-mutations.log records unmodified cargo-deny success plus qsl-eval normal/dev/build dependency rejection only; it does not record all four named crate families. Parser tests are partial discriminator evidence, not whole TC-198, FR-275-AC-8 completion or qualified Inspection evidence. IR-499 remains open. The lead retains the final scoped gate.

## Examined units

```yaml
scope:
- id: TC-198-step-5
  path: spec/exact/matrix/TC-198-qsl-crate-edge-guard.md
  role: examined
  excerpt: "5. Run`cargo test --locked --bin check_deny_source`. The diagnostic controls\n\
    \   require a nonzero exit and the exact injected source token within its own\n\
    \   `error[source-not-allowed]` diagnostic. A similarly prefixed source or a QSL\n\
    \   source appearing only in a later licence error or warning must not qualify.\n\
    \   These are partial discriminator controls, not execution of the dependency\n\
    \   injections in steps 2 through 4."
- id: rejects_qsl_source
  path: src/bin/check_deny_source.rs
  role: examined
  excerpt: Nonzero exit; error[source-not-allowed] diagnostic association; exact whitespace-delimited
    injected source token.
- id: cargo_deny
  path: src/bin/check_deny_source.rs
  role: examined
  excerpt: cargo deny --workspace --color never check licenses bans sources
- id: TC-198-new-controls
  path: src/bin/check_deny_source.rs
  role: examined
  excerpt: source_rejection_does_not_borrow_a_later_diagnostics_source; source_rejection_requires_the_exact_injected_source
- id: TC-198-existing-control
  path: src/bin/check_deny_source.rs
  role: examined
  excerpt: tc_198_source_mutation_rejects_other_failures_and_accepts_only_qsl_refusal
- id: FR-275-AC-8
  path: spec/exact/functional/FR-275-single-exact-kernel.md
  role: context_only
  excerpt: '`make deny` reports cargo-deny''s `source-not-allowed` error for the QSL
    Git repository, and exits non-zero, when a normal, build or dev dependency resolves
    from it; a licence failure alone does not satisfy this.'
```
