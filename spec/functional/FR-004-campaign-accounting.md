---
id: FR-004
title: "Account for every campaign outcome"
type: FR
relationships:
  - target: ix://agent-ix/quire-contract-runtime/FR-001
    type: depends_on
  - target: ix://agent-ix/quire-contract-runtime/interface-001
    type: implements
---
# FR-004: Account for every campaign outcome

## Description

When a harness records a case, the runtime shall update a per-requirement campaign report containing
accepted, rejected, failed, and discarded counters.

## Inputs

- A verdict or an explicit external-framework discard event.

## Outputs

- A complete, saturating `CampaignReport` for one requirement and revision.

## Behavior

- Passed and failed cases shall increment accepted.
- Failed cases shall also increment failed.
- Rejected preconditions shall increment rejected.
- Framework discards shall increment discarded.
- Reports shall always serialize or format all four counters as one indivisible value.

### Immutable snapshot transport

`CampaignReport::snapshot()` shall copy the complete counts and borrow exact identity,
without allocation. Later recording shall not change the captured snapshot. A snapshot
shall expose read-only identity/count accessors and conservative `at_limit`: true when
any counter or the saturating total equals `u64::MAX`. It shall not claim proven overflow,
exact cardinality at that boundary, authenticated execution, completion, or a fresh run.
There shall be no public unchecked counter constructor or import into a mutable report.

The optional `snapshot-json` feature shall imply `alloc` but neither `std` nor `proptest`.
Its encoder and decoder shall own `runtime.campaign-snapshot/v1`, a narrow domain format
containing exactly `schemaVersion`, `requirement`, `revision`, `counterSemantics` and
`counts`. Counter semantics shall be `saturating-u64-v1`; counts shall contain exactly
`accepted`, `rejected`, `failed`, `discarded` as required unsigned u64 JSON integers.
Failed shall not exceed accepted. Total shall be derived, never an independent wire field.
Identity strings shall preserve decoded UTF-8 bytes, including empty strings, without
normalization or revision coercion. Decoder output shall own strings and expose an
immutable borrowed snapshot, not a reconstructed report.

Encoding shall emit the listed member order, including the listed counter order. Decode
shall reject missing/duplicate/unknown members (including escaped-equivalent duplicate
keys), unsupported schema/counter semantics, incorrect types, negative integers including
negative zero, floating/exponent spelling, overflow, invalid UTF-8 and trailing values.
No ignored arbitrary nested metadata or intermediate Value tree is permitted.

Input and output are each limited to 65536 bytes; each decoded identity is limited to
4096 UTF-8 bytes. Check input bytes before parsing; check identity length before copying
it into owned storage. Encoder shall preflight a conservative escaped-size bound and use
a bounded sink. Resource-limit errors shall be distinct from malformed/semantic errors;
no partial snapshot or serialized output is returned. Parser scratch is bounded by the
input cap, and its actual allocation behavior must be inspected at the exact dependency
pin. Allocator exhaustion can still abort: these limits do not promise recoverable OOM.
A native isolated memory-ceiling control must distinguish process failure from a returned
semantic refusal. No source/report mutation or filesystem access occurs in the codec.

This schema does not authenticate imported counters. Package/run/candidate identity,
generated terminal outcomes, policy and independent producer verification remain outside
the runtime. No verdict, attestation, receipt, retained store or generic envelope is added.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-004-AC-1 | Mixed outcome sequences produce the specified four counters. | Test (TC-006) |
| FR-004-AC-2 | Counter overflow saturates and never panics or wraps. | Test (TC-006) |
| FR-004-AC-3 | No public report constructor can omit a metric. | Inspection |
| FR-004-AC-4 | Allocation-free immutable snapshots retain exact identity and all counters after subsequent report recording; imported snapshots cannot resume a mutable report. | Test (TC-015) |
| FR-004-AC-5 | JSON transport round-trips real mixed reports and refuses independently authored malformed, incomplete, duplicate, unsupported or inconsistent data. | Test (TC-015) |
| FR-004-AC-6 | Conservative at-limit inspection distinguishes representable values from potentially saturated counts/totals without inventing overflow history. | Test (TC-015) |
| FR-004-AC-7 | Byte/identity/depth limits fail boundedly without partial output; allocator/process failure is not mislabeled as semantic refusal. | Test (TC-015) |

## Dependencies

- **Upstream**: [FR-001](./FR-001-verdict-observation.md).
