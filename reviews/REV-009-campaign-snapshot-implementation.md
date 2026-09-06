---
id: REV-009
title: "Bounded campaign snapshot implementation handoff"
type: SpecReview
analysis: spec-correctness
scope: "FR-004 immutable snapshot and optional JSON transport; bounded independent review complete"
review_set: subset
---

# Bounded campaign snapshot implementation handoff

## Summary

This is implementer qualification, not independent acceptance or a release decision.
Approved proposal is REV-008 at `5a1a221af7b4bdaf0cb65af15095de1f872ef9fc`.
Owning requirements/interface/test matrix and independently authored native controls were
committed first in `e7d6742`; the original library failed the snapshot test compilation
with E0599 (missing CampaignReport::snapshot), before production implementation.

Core snapshots copy complete counters and borrow identity without allocation. Import is
immutable and unqualified; no mutable report constructor, verdict reconstruction, source
mutation, process runner, retained evidence framework or downstream source pin is added.

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-9001 | medium | Source metadata initially used an unsupported empty-square planned marker. The test matrix now uses the existing in-progress spelling; independent acceptance remains pending. No status vocabulary was widened. | TC-015 |
| FND-9002 | high | Allocator failure remains process failure, not a guaranteed Result error. A real capped child reaches the actual decoder and aborts; the same capped executable/input succeeds without memory pressure. | FR-004-AC-7 |
| FND-9003 | medium | JSON Schema alone cannot enforce byte lengths, lexical integer spellings, raw duplicate keys or cross-field counts. The narrow checked-in schema documents these codec-only constraints explicitly. | FR-004-AC-5; FR-004-AC-7 |
| FND-9004 | medium | Default footprint, old Kani evidence and unqualified imported bytes cannot qualify the new parser or generated native campaign outcomes. Full shared release gates and downstream promotion remain open. | NFR-001; REV-008 |

## Implementation boundary

Exact optional serde 1.0.229 and serde_json 1.0.151 dependencies use alloc with defaults
disabled. Only the runtime's serde dependency edge changes in Cargo.lock; no registry
version/checksum was refreshed. Their license/MSRV declarations and SliceRead allocation
paths were inspected before addition as recorded in PLAN-015; native gates verify the
declarations instead of treating them as proof. No third-party parser source was copied.

Closed Serde visitors deserialize only the known two-object shape, refuse unknown values
without recursive traversal and detect escaped-equivalent duplicate keys before accepting
data. Identity size is checked before its owned copy. A typed refusal cell carries resource
or version errors across the visitor boundary; error strings are never parsed for outcomes.
The encoder checks bounded identity lengths and a conservative escaped-size budget before
allocating its capped sink. Valid bounded identities cannot require more than the output cap;
the sink also enforces the cap defensively. Resource refusal returns no partial public bytes.

The exact source-policy guard now enumerates only the approved new symbols and read-only
methods, and extends its alias/trait checks to snapshot types. Existing bypass probes remain.
Compile-fail documentation checks private fields, absent unchecked constructors, no mutable
import, and absent JSON APIs without the feature. The four original counter fields and
recording semantics are unchanged. At-limit reports are accepted without invented precision.

## Native qualification

Commands ran in this isolated worktree. External qualification logs are under `/tmp`; they
are not retained attestations and do not claim authenticated producer execution.

- `cargo test --locked --offline --features snapshot-json --test snapshot`: eight tests,
  including independent raw-JSON member mutations, real mixed recording, saturation,
  exact/over byte bounds, Unicode property round trips and the memory-ceiling control.
- Exact Rust 1.75.0 runs the six domain test targets with isolated snapshot-json and with
  all features. The isolated profile deliberately has zero proptest-adapter tests; the
  all-feature profile executes those adapter tests. See `/tmp/runtime-snapshot-msrv-all.log`.
- `cargo +1.75.0 test --locked --offline --no-default-features --features snapshot-json --doc`
  runs ten compile-fail cases. See `/tmp/runtime-snapshot-msrv-doc.log`.
- `python3 scripts/run_feature_matrix.py --json` executes twelve declared rows: core,
  alloc, std, isolated snapshot-json and all-feature tests/docs, default footprint test,
  and a separately labeled Rust 1.75.0 thumbv7em-none-eabi library build. See
  `/tmp/runtime-snapshot-feature-matrix.json`. The new build-only row is not a target test
  or allocating-codec footprint measurement.
- A separate clean-target `cargo +1.75.0 build --locked --offline --lib --no-default-features
  --features snapshot-json --target thumbv7em-none-eabi` passes. Default `cargo tree --locked
  --offline --no-default-features -e normal` contains only this crate: zero normal dependencies.
- Stable feature-unification tests with serde_json arbitrary_precision and preserve_order
  execute the same eight snapshot controls successfully; maps and lexical integers stay strict.
- All-target/all-feature denied-warning Clippy and rustdoc pass, as do existing panic and
  unsafe audits. `make deny` passes licenses (existing unused allowance warnings remain).
- `make size` measures the unchanged governed population at 907 bytes with zero panic
  relocations, within 500..4096. This is not the optional codec's footprint.
- Independent jsonschema 4.26.0 Draft202012 validation checks the structural schema, one
  healthy document and 39 independently mutated invalid documents. Two positive structural
  counterexamples demonstrate that failed>accepted and oversized UTF-8 byte length still
  require the stricter codec; their schema acceptance is not a codec success claim.
- `make spec` validates all 59 documents before this review was added and backs all seven
  FR-004 criteria. Ambient module discovery reports duplicate declarations and the old process
  status-column mismatch. This is not exact-stack status qualification; those integration
  advisories are preserved, not suppressed.

### Memory-ceiling interpretation

The Linux control uses real prlimit with a 128 MiB address-space ceiling and disabled core
dumps. It preallocates valid input and bounded filler metadata, uses fallible filler
reservations to exhaust headroom, then marks entry into the real decoder. Acceptance requires
observed SIGABRT after that marker, with a healthy same-limit child counterfactual. A returned
decoder error or successful decode under pressure fails this control; an early setup failure
also fails it. The marker establishes reachability only, not an evidence verdict. No source
file is modified, and no universal platform/OOM recovery guarantee follows from this test.

## Remaining gates

Exact shared-stack integration and full release gates remain open. Kani is installed but
no new parser proof is claimed or borrowed from old proof
transcripts. Runtime JSON does not transport generated terminal outcomes: their bounded
policy shape, full package/run/candidate identity joins, execution authentication and codegen
dependency promotion remain separately owned followup work. Transport completion alone is
not codegen issue #5 completion.

## Independent immutable-source review checkpoint

The coordinator independently reviewed implementation
`2c9385ad46894f5f7bac4f4280b562be836d5651`: complete codec, accounting, schema,
requirements, test and gate changes. The reviewer separately executed all eight native
snapshot controls and both library tests, including the real prlimit healthy/SIGABRT pair
and public record_verdict near-limit control. No blocking finding remained in the bounded
immutable structural-transport scope. TC-015 and FR-004-AC-4 through AC-7 are now marked
implemented in that scope only; no parser proof, authentication, full release qualification,
optional-codec footprint or human sufficiency follows from this acceptance.

The exact implementation-head feature-producer rerun passed all twelve rows. Its raw
`/tmp/runtime-snapshot-feature-matrix.json` SHA-256 is
`c446bcdeee6a39eac61ddaa4c1dabc44d9efc4f9d485e296d12a613d276750ba`.
This checkpoint is documentation-only and does not modify the reviewed production source.
