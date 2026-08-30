---
id: interface-001
title: "Generated-oracle runtime API"
type: interface
---
# [interface-001] Generated-oracle runtime API

## Contract

```yaml
name: GeneratedOracleRuntime
version: quire-contract-runtime-v1
ownership: quire-contract-runtime
features:
  default: []
  alloc: []
  std: [alloc]
  proptest: [std]
associated_types:
  - ContractIdentity
  - ExecutionPoint
  - Observation
  - FailureDetail
  - Verdict
  - CampaignReport
operations:
  - name: construct_verdict
    inputs: [requirement identity, revision identity, execution point, clause observations]
    output: Passed | FailedPostcondition | RejectedPrecondition
    semantics: exactly one terminal category; no Boolean conversion is provided
  - name: inspect_verdict
    inputs: [Verdict]
    output: terminal kind, common provenance, optional structured terminal detail
    semantics: rejection and failure remain distinguishable from success
  - name: evaluate_short_circuit_boolean
    inputs: [left value, lazy right value]
    output: Boolean
    semantics: skip the right operand exactly when the named Boolean operator permits it
  - name: evaluate_total_boolean
    inputs: [lazy left value, lazy right value]
    output: Boolean
    semantics: evaluate left then right exactly once regardless of the left value
  - name: evaluate_checked_value
    inputs: [option, slice/index, or supported integer operands]
    output: defined value | None
    semantics: return None for absence, invalid index, overflow, zero division, or signed division overflow
  - name: record_campaign_verdict
    inputs: [per-requirement report, Verdict]
    output: updated complete counters or typed identity mismatch
    semantics: refuse identity mismatch; saturate accepted, rejected, failed, and discarded counts
  - name: adapt_to_proptest
    feature: proptest
    inputs: [Verdict]
    output: proptest TestCaseResult
    semantics: map pass to success, failure to Fail, and rejection to Reject
  - name: adapt_to_proptest_and_record
    feature: proptest
    inputs: [per-requirement report, Verdict]
    output: recorded proptest TestCaseResult
    semantics: record the verdict before mapping so the campaign census retains rejection; map identity mismatch to a proptest failure retaining expected and observed identity
invariants:
  - every evidence-bearing value retains exact borrowed source identity
  - rejected preconditions are never successful evidence
  - the default surface requires no allocator, standard library, or normal dependency
  - public runtime evaluation and accounting operations have no intentional panic path
  - a CampaignReport always contains accepted, rejected, failed, and discarded counters
compatibility:
  enums: non-exhaustive; consumers must preserve future unknown states
  msrv: Rust 1.75
  licensing: MIT OR Apache-2.0
  publication: disabled through the v0.1 human release decision
```
