---
id: SR-12903
title: IR-878 RT130 gap-analysis
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-runtime; PR130 head 4917ef5337a35ac0b8bb0195621e0def49689372 vs main a0bbc149; FR-275-AC-6, TC-202, tests/deny_one_copy.rs, Makefile deny target, scripts/check_one_copy.awk"
review_set: subset
---

## Summary

Ticket: IR-878. PR: quire-contract-runtime#130. Planless gap analysis scoped to the PR diff. Plan completion: not assessed. The PR adds TC-202 and the test `deny_rejects_a_second_first_party_git_lock_entry`, which binds FR-275-AC-6 and TC-202 through `#[trace]`, and moves AC-6 out of TC-197.

## Verdict

PASS. FR-275-AC-6 moves from unbacked to backed by a real behavioral test of the existing `make deny` one-copy stage; no stub, tautology or coverage inflation was found. The only evidence-soundness caveat (the test checks committed HEAD rather than the working tree) is recorded once, in SR-12902 FND-001.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Coverage

Re-measured with quire 0.36.2 `quire coverage --scope . --strict`: head 88/186 rows backed, 110 unbacked, 0 contradicted; main a0bbc149 86/185 backed, 111 unbacked, 0 contradicted. The PR body's numbers match. `quire trace --id FR-275-AC-6` and `--id TC-202` each resolve one structural claim at tests/deny_one_copy.rs:62. Main reported `FR-275-AC-6 ... has no backing symbol`; head does not. The strict run still exits 1 on the pre-existing 110 unbacked rows.

The production mechanism under test (Makefile:180-182 and scripts/check_one_copy.awk) is unchanged by this PR and has an owning requirement (FR-275 One copy, AC-5/AC-6). Code with no owning requirement: none added. Mutation evidence: three committed gate mutations (awk always exits 0, awk line removed from the recipe, predicate narrowed to `quire-*`) each made the test fail, so the binding is not vacuous.

## Examined units

```yaml
scope:
- id: FR-275-AC-6
  path: spec/exact/functional/FR-275-single-exact-kernel.md
  role: examined
  excerpt: "`make deny` exits non-zero when `Cargo.lock` holds a second entry for any first-party Git crate."
- id: FR-275-AC-5
  path: spec/exact/functional/FR-275-single-exact-kernel.md
  role: context_only
  excerpt: "`Cargo.lock` holds exactly one entry each for `quire-exact`, `quire-semantic-value`, `quire-canonical` and `quire-canonical-derive`."
- id: TC-202
  path: spec/exact/matrix/TC-202-one-copy-deny-mutation.md
  role: examined
  excerpt: "Reject duplicate first-party Git lock entries through make deny"
```
