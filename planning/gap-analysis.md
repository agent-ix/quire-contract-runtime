# Runtime v0.1 gap analysis

Date: 2026-08-30

Candidate source revision: `790fb7780e893206240ba4fa5c5b376d60ec127e`

Evidence: `evidence/v0.1-candidate/sha256sums.txt`

## Requirement audit

| Requirement | Authoritative evidence | Result |
|---|---|---|
| #2 baseline, dual license, publication lock | `Cargo.toml`, both license files, `deny.stdout`, protected `main` API response | pass |
| #2 stakeholder/functional/non-functional/interface/test requirements | 20 documents under `spec/`; `quire-validate` output | pass |
| #2 composite review and assurance artifacts | `planning/foundation-review.md`; AP/AD/CAC/MP/AA under `spec/assurance/` | pass |
| #2 implementation plan and dependency DAG | `planning/implementation-plan.md` | pass |
| #1 three distinct terminal verdicts | `src/verdict.rs`; TC-001 output | pass |
| #1 identity, observations, structured details | `src/identity.rs`, `src/observation.rs`; TC-001 output | pass |
| #1 default no_std/allocation-free surface | `#![no_std]`, empty default feature set, default dependency tree containing only this crate | pass |
| #1 size, panic, feature, compatibility contracts | crate/README docs, panic audit, feature matrix, layout and rlib measurements | pass |
| #1 permissive generated-code surface | default dependency tree empty; crate `MIT OR Apache-2.0` | pass |
| #3 short-circuit and total operators | `src/operators.rs`; exhaustive TC-002 truth/evaluation tests | pass |
| #3 safe option/index/arithmetic/division helpers | checked sealed trait; boundary/property TC-003 tests | pass |
| #3 optional proptest mapping | pinned proptest feature; TC-004 maps pass/fail/reject distinctly | pass |
| #3 complete per-requirement accounting | `CampaignReport`; TC-006 mixed, mismatch, and saturation tests | pass |
| #3 Kani harness coverage | five checked-in proofs; pinned Kani 0.67.0 CI result | pass |
| Epic local gates and measurements | local `make ci`, exact PGM-01 envelope validation, and retained MP-001 outputs/digests | pass |
| Protected remote gates | successful checks for pre-reconciliation revision retained under `evidence/historical/`; rebased candidate run | pending deliberate dispatch |

## Gap disposition

No unresolved implementation or specification gap was found. The source candidate is within the
256 KiB rlib ceiling at 238,870 bytes, has no default normal dependency, contains no unsafe or
intentional panic surface, and passes every locally available gate.

The following are release/workflow gates, not silently accepted gaps:

1. PGM-01 (`agent-ix/quire-contract-ir#3`) is in review at PR #12. This candidate reconciles exact
   revision `0b8669b80f98b6c11954f922b32d9edae8a11983` and its envelope schema digest, but must reconcile
   again after the policy merges.
2. Local Kani was unavailable and remains truthfully recorded as `skipped-unavailable`. Pinned Kani
   0.67.0 executed all five proofs successfully for a historical pre-reconciliation revision; the
   rebased candidate requires a fresh manual CI dispatch.
3. Manual-CI PR #6 must merge, protected checks must pass on the rebased candidate, and protected
   code-owner review must complete on runtime PR #5.
4. The human release owner must record the v0.1 decision in `planning/release-decision.md` after merge
   evidence is collected. No agent or automated gate may substitute for that decision.

Implementation gap-analysis result: **pass, with PGM-01 merge/reconciliation, current protected
checks, code-owner review, and the human release decision still open**.
