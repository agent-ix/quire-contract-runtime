---
id: SR-20261104024413
title: IR-1314 gap-analysis scoped review
type: SpecReview
analysis: gap-analysis
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

Computed quire matrix read from /tmp/titan-ir1314-matrix.json. The changed host qualification has no criterion row and the new test is unbound; NFR-002-AC-7 asserts licenses only, so tagging it would misrepresent coverage. Existing NFR-002 AC-3/AC-7 untagged rows predate the diff and are context, not new findings. Reverse-gap and stub checks scoped to changed gate-tool lines found no stubs; the host lookup obligation exists only in Verification. Optional semantic gap review not selected; ordinary code-review still checks test assertions.

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
- id: IR-1314-AC1
  path: Linear:IR-1314
  role: examined
  excerpt: 'AC1: record on this ticket which dependency path brings in `users` 0.11.0
    (cargo tree -i users) and whether it is a production, build or dev-only dependency.'
- id: IR-1314-AC2
  path: Linear:IR-1314
  role: examined
  excerpt: 'AC2: remove or replace that dependency (a maintained alternative, or drop
    the feature that needs it), with no vendored copy, no compat layer and no new
    pin; or, if the vulnerable code is unreachable, say so on this ticket with the
    evidence.'
- id: IR-1314-AC3
  path: Linear:IR-1314
  role: examined
  excerpt: 'AC3: `make cargo-audit` exits 0 on RT main, or its remaining advisories
    are explicit, reasoned and owned.'
- id: IR-1314-AC4
  path: Linear:IR-1314
  role: examined
  excerpt: 'AC4: check the other first-party repos (IR, CG, integration) for the same
    advisory and list the result here.'
- id: Cargo.lock
  path: Cargo.lock
  role: examined
  excerpt: ' "uzers",'
- id: tools/runtime-gates/Cargo.toml
  path: tools/runtime-gates/Cargo.toml
  role: examined
  excerpt: uzers = "0.12"
- id: tools/runtime-gates/src/gates.rs
  path: tools/runtime-gates/src/gates.rs
  role: examined
  excerpt: use uzers::os::unix::UserExt;
```

## Method selection and limits

Ran code-review with Rust lane, scoped gap-analysis, spec-review base, evidence and scope-boundary. EARS skipped: no Statement text changed; the added Verification paragraph is outside that lens's requirement-statement extraction. Integrity skipped: no artifact/section/ID/relationship structure changed. Dependency lens skipped: no spec relationships added. Object review skipped: no domain objects changed. Failure-domain and risk-complexity skipped: this replacement preserves the existing boundary and behavior; direct Rust review traced HOME refusal and absent passwd NotFound. Criterion-strength/correctness/fuzz skipped: no criteria or Fuzz obligations changed; no Jev client used. Optional semantic gap analysis was not selected. Whole-repository gap assurance, real Kani and aggregate make ci were not run per the brief. Existing unrelated matrix/grammar/type warnings are context, not this ticket's findings. All findings across the four conditional artifacts describe one shared issue.
