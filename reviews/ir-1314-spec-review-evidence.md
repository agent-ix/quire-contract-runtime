---
id: SR-20261104024415
title: IR-1314 spec-review/evidence scoped review
type: SpecReview
analysis: evidence
scope: agent-ix/quire-contract-runtime; IR-1314; spec/core/non-functional/NFR-002-panic-compatibility-license.md;
  tools/runtime-gates; Cargo.lock
review_set: subset
---

## Summary

Frozen head aa78bcd82f947741acbf5991d9c3e9a9759df72a; ticket IR-1314; PR #201. Subset review of this dependency replacement and the added host qualification only, not whole-repository assurance. No applicable AssuranceProfile exists. No plan supplied.

Plan completion: not assessed

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The added trusted-home obligation has no acceptance criterion, and trusted_home_ignores_caller_home has only Provenance, so the regression has no criterion binding in the computed matrix. | spec/core/non-functional/NFR-002-panic-compatibility-license.md:67; tools/runtime-gates/src/gates.rs:639 |

## Verdict

CONDITIONAL (one shared traceability defect; production behavior passed)

## Coverage

quoin advise --json ran successfully; output /tmp/titan-ir1314-advise.json. No obligation entry can advise the added host-home text because it has no AC/metric. Reviewer judgement: subprocess unit/integration behavior and advisory scanning are suitable inexpensive evidence; solver escalation is unnecessary. Existing license AC does not cover trusted home. The behavioral test is substantive but lacks a criterion to attach its evidence to.

Independent cargo test --locked -p quire-contract-runtime-gates: 8 passed in an isolated reviewer target under the cargo lock, including the subprocess HOME regression; fake runner tests do not invoke real Kani. Independent make cargo-audit: exit 0, 82 packages, 1403 advisories loaded. Coder before/after tree evidence shows users 0.11.0 -> host gate-tool normal dependency, replaced by uzers 0.12.2; root runtime and footprint manifests do not acquire it. Before audit names RUSTSEC-2025-0040, RUSTSEC-2023-0040, RUSTSEC-2023-0059; after audit has none. Remote main lock snapshots for IR 6bdf9d76da06a2fc947069a8808eb6bb9d8034b0, CG a6df4a232b4eb0f09928464c4ba7e54f7367168e, integration 33f1f4fb5d62c6e20c82b4c8d1e23190fba1fa3b contain no users package. Official RustSec recommends maintained uzers: https://rustsec.org/advisories/RUSTSEC-2025-0040.html. Maintained upstream dependency, no vendor/compat/new pin, audit ignores, gate weakening or hosted workflow changes. trusted_home still queries current UID passwd entry and returns the identical io::ErrorKind::NotFound on absence; no HOME trust introduced. Coder HOME mutation log shows wrong HOME implementation fails and restored tests pass. Independent focused Clippy (--all-targets, -D warnings), cargo fmt --all -- --check, and make deny with distributed quire check-lock also passed; deny emitted existing wildcard/unencountered-license warnings. Final aggregate non-Kani gate remains lead-owned.

## Examined scope

```yaml
scope:
- id: NFR-002-verification-host
  path: spec/core/non-functional/NFR-002-panic-compatibility-license.md
  role: examined
  excerpt: 'The workspace supply-chain qualification also includes the host-only

    `quire-contract-runtime-gates` executable. Its Unix account lookup dependency

    must preserve OS account home lookup rather than trust the caller''s `HOME` when

    selecting tool binaries. This tool dependency is outside the runtime library

    and linked-footprint dependency populations. Use `cargo audit` for advisory

    qualification in addition to the license check.'
```

## Method selection and limits

Ran code-review with Rust lane, scoped gap-analysis, spec-review base, evidence and scope-boundary. EARS skipped: no Statement text changed; the added Verification paragraph is outside that lens's requirement-statement extraction. Integrity skipped: no artifact/section/ID/relationship structure changed. Dependency lens skipped: no spec relationships added. Object review skipped: no domain objects changed. Failure-domain and risk-complexity skipped: this replacement preserves the existing boundary and behavior; direct Rust review traced HOME refusal and absent passwd NotFound. Criterion-strength/correctness/fuzz skipped: no criteria or Fuzz obligations changed; no Jev client used. Optional semantic gap analysis was not selected. Whole-repository gap assurance, real Kani and aggregate make ci were not run per the brief. Existing unrelated matrix/grammar/type warnings are context, not this ticket's findings. All findings across the four conditional artifacts describe one shared issue.

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | NFR-002-AC-11 now states the host account-home obligation, TC-210 specifies the subprocess control and OS account-entry precondition, and the real regression binds that criterion. Computed Quire matrix reports AC-11 tagged by this test. |

## Disposition pass 1 verification

Inspected complete fix range aa78bcd..8d3184e18a697a191e30ab06fbc30f2a57d3db18, including the final acceptance-table whitespace correction. NFR-002 Scope now explicitly limits host tool selection to AC-11; AC-11 has a falsifiable distinct-HOME condition with the existing OS account-entry precondition. TC-210 links the NFR and describes the actual isolated subprocess procedure. The trace attribute is recognized by the computed matrix: NFR-002-AC-11 is tagged by tools/runtime-gates/src/gates.rs:642, tests::trusted_home_ignores_caller_home. The test retains actual passwd lookup, distinct temporary HOME, equality and cargo path assertions, child exit-status checking, and no global environment mutation. Independent isolated-target gate-tool tests passed 8/8 on this head. Distributed Quire spec validation passed with existing unrelated warnings. Existing findings remain unchanged below their original review sections; no new findings. Round-one latest verdict: PASS for this ticket subset; no open reviewer findings. Aggregate non-Kani gates remain lead-owned.

```yaml
scope:
- id: NFR-002-AC-11
  path: spec/core/non-functional/NFR-002-panic-compatibility-license.md
  role: examined
  excerpt: For a current UID with an OS account entry, the host gate selects its tool
    home from that account even when caller `HOME` names a different directory.
- id: TC-210
  path: spec/core/matrix/TC-210-trusted-tool-home.md
  role: examined
  excerpt: Check NFR-002-AC-11 through the real Unix account lookup in the host gate
    tool.
bindings:
- test_id: tests::trusted_home_ignores_caller_home
  ac_id: NFR-002-AC-11
  trace: correct
```
