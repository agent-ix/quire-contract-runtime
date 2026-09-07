---
id: REV-008
title: "Proposed runtime-owned campaign snapshot transport"
type: SpecReview
analysis: gap-analysis
scope: "Runtime FR-004 and codegen native campaign-result prerequisite"
review_set: subset
---

# Runtime-owned campaign snapshot transport

## Summary

Specification-first proposal; independent review required before implementation.
Base is runtime `60749bcaa9725c33362de399e4430ca105b4a50b`, after the deliberate
legacy-evidence deletion. Codegen PR #27 currently pins older runtime `e360dad`;
a later exact-pin compatibility change must be independently qualified. This
proposal does not revive deleted envelopes, a local evidence store, collectors,
generic attestations or Make-integrity policy rejected by runtime issue #10.

Codegen REV-017 identifies a genuine native-process boundary: `CampaignReport`
has private counters, borrowed requirement/revision identity, and no validated
transport. Its Display string is diagnostic text. Reconstructing a report by
replaying invented verdicts is neither native measurement nor a sound decoder.
The runtime should own complete report transport; generated campaign terminal
outcomes and policy remain codegen-owned. Quoin retains and verifies producer
records without executing producers. A valid snapshot is not authenticated.

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-8001 | high | A complete report cannot cross a native subprocess boundary through its current public API without a separately owned transport. | FR-004; codegen REV-017 |
| FND-8002 | high | Saturating counters cannot prove exact cardinality at their limit; importing them as exact counts would invent precision. | FR-004-AC-2 |
| FND-8003 | high | A validated counter tuple proves structural consistency, not that any event occurred or a campaign completed. | FR-004; runtime FR-005 |
| FND-8004 | medium | Requirement/revision identity is not package/run/candidate identity; the enclosing generated producer must supply those joins separately. | FR-001; codegen REV-017 |

## Proposed API and ownership

Keep the allocation-free default runtime unchanged in its dependency policy.
Add an immutable borrowed `CampaignSnapshot<'a>` exported from an actual
`CampaignReport` with exact identity and the complete existing `CampaignCounts`.
The snapshot has private fields and read-only accessors; it is not a new mutable
report constructor and offers no import-to-resumed-report operation. Export is
an allocation-free view, not a claim that the report is final or freshly zeroed.

An optional `snapshot-json` feature may add an allocating, runtime-owned bounded
JSON encoder/decoder. It implies `alloc`, not `std` or `proptest`. The exact
Serde/JSON dependency versions, licenses and Rust 1.75 compatibility require
review before adding dependencies; this packet installs nothing. Decoder output
owns the identity strings and exposes a borrowed immutable snapshot view. No
deserialization into `CampaignReport`, text-verdict parsing or public unchecked
counter constructor is introduced. Consumers need not reconstruct mutable state
merely to read imported counts.

The intended API shape is `CampaignReport::snapshot()`,
`encode_campaign_snapshot(&CampaignSnapshot) -> Result<Vec<u8>, SnapshotError>`,
and `decode_campaign_snapshot(&[u8]) -> Result<DecodedCampaignSnapshot, SnapshotError>`.
The feature-gated functions are not available without `snapshot-json`; all
snapshot data inspection remains allocation-free. Exact naming/internal types
remain reviewable before the interface and owning requirements are amended.

## Complete domain wire contract

One narrow versioned native data schema, `runtime.campaign-snapshot/v1`, contains
exactly `schemaVersion`, `requirement`, `revision`, `counterSemantics` and
`counts`. `counterSemantics` is exactly `saturating-u64-v1`. Counts contains
exactly four required unsigned 64-bit JSON integers: `accepted`, `rejected`,
`failed`, and `discarded`. Neither absent metrics nor nulls default to zero.

Enforce `failed <= accepted`. Derive total as the existing saturating sum of
accepted, rejected and discarded; failed is already included in accepted and
must not be summed twice. Total is not an independent wire field that can
contradict its constituents. Refuse unknown versions/members, duplicate members
(including escaped-equivalent keys), wrong types, floats, negative/out-of-range
integers, malformed UTF-8 and trailing non-whitespace bytes before exposing a
snapshot. Encoding emits one fixed member order and no optional count omission.
The schema is new domain data, not one of the deleted generic evidence schemas.

Bound input and output to 65536 bytes and each decoded UTF-8 identity to 4096
bytes. Check input size before JSON parsing. The fixed shallow shape does not
permit arbitrary nested metadata; retain the parser's depth limit and refuse
unknown nested fields rather than recursively building a Value tree. Oversize
identity and output are structured resource-limit failures, not truncated identities.
Encoder preflights the conservative escaped-size bound, writes into a bounded
output sink, and never first builds an unbounded intermediate output. Fixed-shape
deserialization admits only the two bounded identity strings and fixed numeric
fields; implementation must audit parser scratch allocation against the capped
input, rather than claim that a wire cap bounds every allocator internally.
Qualification must include a native isolated memory-ceiling failure control.
Ordinary allocating Serde/JSON operations can still abort on allocator exhaustion:
the Result API promises structured limit/malformed-input refusal, not universal
recoverable OOM. Preserve that residual explicitly unless every allocating path
is independently qualified as fallible; do not turn a killed parser into a
semantic rejection or successful snapshot. The allocation-free export is separate.

Runtime identity remains exact opaque UTF-8: no trim, normalization, numeric
revision coercion or new global nonempty rule. The runtime's current identity
constructors admit arbitrary borrowed strings; transport should preserve that
contract within its explicit resource bounds. Package-qualified codegen binding
separately checks valid source identities and canonical u64 revision spelling.

## Saturation and trust semantics

Existing recording behavior is unchanged: individual counters and total
saturate rather than wrap or panic. A received maximum may mean exactly the
maximum or a saturation; current reports retain no overflow history. Do not
invent an `overflowed=false` field or call every maximum a proven overflow.

Expose a derived conservative `at_limit` observation whenever any count or
the saturated total is `u64::MAX`. It means exact cardinality is not established
by this transport alone. Below that boundary, structural values are representable
without saturation, but imported values are still untrusted measurements.
The decoder does not reject a legitimate saturated snapshot as malformed.
Any codegen policy requiring exact invocation counts must separately refuse or
mark unavailable the at-limit case; that policy is not a runtime verdict.

An attacker can supply a valid four-counter tuple. Passing this decoder cannot
create `passed`, `verified`, `run_qualified`, attested or release-ready evidence.
No digest, signature, package, candidate, run ID, timestamp, proof result, tool
provenance or human decision is added to this runtime schema. The independently
expected execution context and authorized producer are checked by the consuming
shared chain. Reusing prior report accounting must remain explicit; a snapshot
does not claim its report began at zero for the current campaign.

## Codegen terminal-outcome boundary remains separate

The generated runner owns actual `Result<Summary, OutcomeError>`, attempted
counts, accepted/rejected floors, discard ceiling, retry/shrink semantics and
exhaustion. Runtime report transport alone cannot replace those outcomes.
`Exhausted` is not failure-or-success inferred from process exit, and its reason
string is diagnostic detail, not a verdict parser input. Generated runtime
snapshots must be obtained from the actual report, then joined to the actual
terminal outcome and fixed campaign configuration in a separate producer schema.
Nested exhaustion policy data must be bounded to the real generated variants,
not generalized into arbitrary recursive outcome records.

Codegen must compare all four counters and saturating attempted total between the
actual terminal summary and snapshot for success and every terminal-error variant,
including any nested exhaustion-policy summary. Such nested policy is only None
or exactly one BelowAcceptedFloor, BelowRejectedFloor or AboveDiscardCeiling;
recursive Exhausted, Failed, inconsistent summaries or other variants refuse.
Do not duplicate one requirement report per clause. Both requirement and revision
must match an independently supplied enclosing full package-qualified reference;
an imported record cannot declare its own expected identity. Native
build/run/profile/config/source and replay checks from REV-017
remain mandatory. Completing this transport does not complete codegen issue #5.

## Required controls and implementation gate

Before implementation, independently review this proposal and amend FR-004,
interface-001, the feature/size contracts, test matrix and execution plan.
Bank real report export plus independently authored expected raw JSON controls:

1. Actual mixed pass/fail/rejection/discard report round-trips every counter and
   exact Unicode/escaped identity; empty report remains explicitly all-zero.
2. Independently mutate each omitted/duplicate/unknown/wrong-type field, escaped
   duplicate, version, negative/float/overflow integer and failed>accepted.
   Assert structured refusal, with the healthy native snapshot counterfactual.
3. Exact maximum and one-event saturation preserve the original runtime counters;
   sum saturation with no individual maximum still sets conservative at_limit.
   Below-boundary report does not claim authenticated execution either.
4. Byte/identity/depth boundaries, invalid UTF-8 and long escaped strings fail
   boundedly. Valid Unicode identity remains byte-exact; no normalization.
5. Compile-fail controls protect private fields, lack of unchecked constructor,
   no resumed-report import and unavailable JSON API on the default feature set.
6. Stable and Rust 1.75 tests, feature/dependency matrix, denied-warning docs,
   panic/unsafe audits, license gate and governed default footprint remain intact.
   Extend declared feature-producer population honestly; do not relabel historical
   Kani/footprint/feature evidence as verification of this new parser. Include an
   isolated snapshot-json feature row and a real no_std-target compile: existing
   all-features includes std and cannot establish this feature's independence.

Additional independently reviewed controls cover negative-zero and exponent
integer spellings, exact decoded-byte limits with escaped Unicode, immutable
snapshot stability after later report recording, independently mismatched
revision, and inconsistent nested policy summaries at the later codegen boundary.

## Independent design review disposition

The independent reviewer checked current runtime accounting/identity and actual
generated terminal outcomes against this proposal. No architectural blocker was
found. The allocation-exhaustion limits, exhaustive identity/outcome join, and
isolated no_std feature qualification above incorporate the three review items.
This is approval of the bounded implementation direction, not implemented
transport, parser qualification, proof, release or human evidence sufficiency.

No implementation, new dependency, native parser qualification or source release
is claimed by this proposal. Current shared-assurance pin compatibility remains
a separate campaign integration gate, with the owner decision boundary unchanged.
