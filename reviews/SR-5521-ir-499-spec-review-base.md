---
id: SR-5521
title: "Spec review — IR-499 gate status correction"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-runtime@117096c945086f0dae0c6bf0766c306c6c1f743d; spec/exact/matrix/tests.md:58,59,85,86"
review_set: subset
---

## Summary

Ticket: IR-499. Reviewed PR #116 at 117096c945086f0dae0c6bf0766c306c6c1f743d, limited to four status cells in `spec/exact/matrix/tests.md`. Both revisions have 159 minted rows, 73 backed and 94 unbacked; contradicted statuses fall from four to zero.

## Verdict

**PASS** — All four edited matrix cells accurately distinguish existing gates from unbound strict-coverage evidence. The two affected Test Cases and seven FR-275 criteria remain linked and keep their verification methods.

## Examined units

- FR-275-AC-7/8 functional row (spec/exact/matrix/tests.md:58): | FR-275 | FR-275-AC-7, FR-275-AC-8 | TC-198 | 🚧 `deny.toml` and `make deny-mutations` (`scripts/check_deny_bans.sh`) contain the QSL Git source guard and normal, build and dev dependency mutation checks; AC-7 is an inspection, while AC-8 and TC-198 have no Quire test-symbol binder. IR-499 remains open for strict-coverage evidence |
- FR-275-AC-9/10/11/20/21 functional row (spec/exact/matrix/tests.md:59): | FR-275 | FR-275-AC-9, FR-275-AC-10, FR-275-AC-11, FR-275-AC-20, FR-275-AC-21 | TC-199 | 🚧 `make test-features`, `make size` and `make msrv` contain the exact no-std, footprint graph, size and toolchain checks; AC-20 is an inspection, while AC-9 through AC-11, AC-21 and TC-199 have no Quire test-symbol binder. IR-499 remains open for strict-coverage evidence |
- TC-198 summary row (spec/exact/matrix/tests.md:85): | TC-198 | Fail the build on a dependency from the QSL Git source | Integration | P0 | FR-275-AC-7, FR-275-AC-8 | 🚧 source rejection gate exists in `deny.toml` and `make deny-mutations`; Quire has no test-symbol binder for TC-198 |
- TC-199 summary row (spec/exact/matrix/tests.md:86): | TC-199 | Build the exact profile no_std and keep the default footprint | Integration | P0 | FR-275-AC-9, FR-275-AC-10, FR-275-AC-11, FR-275-AC-20, FR-275-AC-21 | 🚧 exact profile and footprint gates exist in `make test-features`, `make size` and `make msrv`; Quire has no test-symbol binder for TC-199 |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |
