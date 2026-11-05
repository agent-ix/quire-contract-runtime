---
id: SR-20261104024416
title: IR-1314 spec-review/scope-boundary scoped review
type: SpecReview
analysis: scope-boundary
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
| FND-001 | low | No findings (placeholder) | - |

## Verdict

PASS (subset)

## Coverage

The owning component is tools/runtime-gates (cross-cutting host qualification); the runtime library and linked footprint remain outside this dependency population. OS passwd identity is an external assumed account authority; the uzers adapter and cargo path selection are exercised by the real subprocess regression. The OS database itself is not verified. RustSec advisory scanning covers the workspace tool without claiming changes in no_std/runtime dependencies. No missing responsibility allocation in the changed paragraph.

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
