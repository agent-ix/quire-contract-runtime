---
id: SR-008
title: "Scope-boundary and EARS-conformance review of the exact runtime oracle extension"
type: SpecReview
analysis: scope-boundary
scope: "agent-ix/quire-contract-runtime#16 at 13a832a; FR-006 through FR-012, FR-273 and their TC-016–TC-023, TC-024–TC-026, TC-030–TC-034, TC-194–TC-195 corpus, plus the matching spec/test-matrix.md rows"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-runtime/FR-006
    type: reviews
  - target: ix://agent-ix/quire-contract-runtime/FR-007
    type: reviews
  - target: ix://agent-ix/quire-contract-runtime/FR-008
    type: reviews
  - target: ix://agent-ix/quire-contract-runtime/FR-009
    type: reviews
  - target: ix://agent-ix/quire-contract-runtime/FR-010
    type: reviews
  - target: ix://agent-ix/quire-contract-runtime/FR-011
    type: reviews
  - target: ix://agent-ix/quire-contract-runtime/FR-012
    type: reviews
  - target: ix://agent-ix/quire-contract-runtime/FR-273
    type: reviews
  - target: ix://agent-ix/quire-contract-runtime/TM-001
    type: references
---
# SR-008: Scope-boundary and EARS-conformance review of the exact runtime oracle extension

## Summary

Issue #16 asks for "one repo-level `/specify` + self `/spec-review` for the full runtime
extension before code," with a claimed scope of "mathematical/bounded integers, rationals,
decimals, IEEE floats, text/enums/units, records, all collection kinds, object/graph/state,
temporal, and protocol encodings admitted by the backend profile."

Measuring the actual corpus against that claim (not the issue text's own framing of it) finds
the claim overstates what remains to be authored. Nine of the ten named families are already
fully specified, cross-referenced to the quire-spec-language authority, coverage-matrix-backed,
and green under `make spec`:

| Issue-claimed family | Owning FR(s) | Status |
| --- | --- | --- |
| Mathematical / bounded integers | FR-002 (`CheckedInteger`), FR-007-AC-1, FR-007-AC-7 | Specified |
| Rationals | FR-007-AC-7, FR-007-AC-10 | Specified |
| Decimals | FR-007-AC-2, FR-007-AC-11 | Specified |
| IEEE floats | FR-007-AC-3, FR-007-AC-9 | Specified |
| Text / enums / units | FR-007-AC-4, FR-007-AC-5, FR-007-AC-12, FR-012 (unit-graph vocabulary) | Specified |
| Records | FR-008-AC-1 (record/tuple/object-type declarations) | Specified |
| All collection kinds | FR-008-AC-3, FR-008-AC-4 (set/bag/ordered-set canonical key) | Specified |
| Object / graph | FR-008-AC-2, FR-008-AC-6 (containment graph, terminal `Reference`) | Specified |
| State (mutation / frame obligations) | none in this repo | Not RT's arrow — see Finding FND-701 |
| Temporal | none | Out of scope, upstream-blocked — see Finding FND-702 |
| Protocol encodings admitted by backend profile | none | Out of scope, upstream-blocked — see Finding FND-702 |

Backend negotiation (FR-009), the injected-denial qualification seam (FR-010), determinate
meter state at a stop (FR-011), compiler-vocabulary carry-through (FR-012) and total pure
function application (FR-273) are also already specified and are the mechanisms the
issue's exit criteria ("undefined/overflow/NaN/bound/unsupported outcomes remain typed";
"publish=false, no ambient effects") are tested through.

`make spec` (`quire validate` + `quire coverage --strict`) is green at HEAD (13a832a):
123/131 matrix rows backed, 0 `unbacked_rows`, 0 `status_lies` under
`quire coverage --format json`; the residual 8-row gap is `spec/evidence/suites.md`
(a registry with no matrix rows of its own — same explanation SR-002 already gave for the
same shape of gap).

No new FR was authored in this pass. Authoring one for "state," "temporal" or "protocol
encodings" now would mean the runtime inventing semantics ahead of the language authority
that owns them, which every existing FR in this family (FR-006 through FR-273) already
states is out of bounds: "the runtime is an implementation of that definition, not a second
semantic authority."

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-701 | low | "Object/graph/state" is only two-thirds specified by design, not by omission. Object and containment-graph construction are FR-008's job; mutable "state" (the FR-340 `modifies`/`creates`/`deletes` frame-obligation sense the issue's own family list implies) is negotiated at CG's `negotiate_*` point (AD-016 arrow 4), never reaches RT's arrow-3 `quire-exact` ops, and RT exposes no `negotiate_frame` predicate alongside `negotiate_ieee`/`negotiate_integer_division`. AD-016 itself records this construct as `unsupported` (warned) at arrows 3–5 until IR lowers the FR-340 node (IR#109, WP10). There is no RT-owned requirement to write until that IR work lands; writing one now would assign this crate a negotiation responsibility AD-016 places elsewhere. | AD-016 (quire-specification) "Frames and unbounded constructs" table; IR#109 |
| FND-702 | low | Temporal semantics and protocol-encoding negotiation are not merely unwritten, they are pinned out of scope by this repo's own FR-008 ("Temporal and protocol encodings and replay are out of scope (agent-ix/quire-spec-language#121)"). QSL#121 ("[V1-A05] Implement complete native reference execution and finite simulation") is OPEN, owned by Agent-A's lane, and is the authority that would define these semantics in the first place — `runtime::execute` does not yet extend to every admitted state/value/model clause. quire-contract-ir's TC-045 independently corroborates the grouping, listing "graph/temporal/protocol encodings" together as still-unsupported negotiation mutations. Authoring RT operators for either family now would make this crate a second semantic authority pending an upstream ruling that has not shipped, which FR-006 through FR-273 each explicitly disclaim being. | FR-008 Behavior section; agent-ix/quire-spec-language#121; quire-contract-ir TC-045-exact-backend-negotiation.md |
| FND-703 | low | `spec/index.md`'s "Requirements Architecture" paragraph and "In Scope" list predate FR-009 through FR-012, FR-273 and the `exact` feature entirely — it names "FR-001 through FR-008" and "TC-001 through TC-026" only, and the In Scope bullets never mention exact-oracle operators at all. Fixed in this pass (see Files touched). | spec/index.md |
| FND-704 | low | Verification only, no defect: no EARS, atomicity or invented-unsupported-alternative defect was found across FR-006 through FR-012 and FR-273. Every FR opens its Description with an EARS `When <trigger>, the runtime shall <response>` sentence; every out-of-scope citation (model domains QSL#120, replay/temporal/protocol QSL#121, `CheckMode::Kernel`) names a specific upstream ticket or a stated non-goal rather than an invented alternative; `quire coverage --strict` reports 0 `unbacked_rows` and 0 `status_lies` for this family. This corpus has already been through prior remediation rounds (branches `agent-e/spec-remediation`, `agent-e/35-ac5-charge-plan-sweep`) that split previously-bundled ACs and fixed stale matrix rows; this review found nothing left for a further round to fix. | quire coverage --format json at 13a832a |

## Verdict

**PASS.** No blocking findings. Two low-severity findings (FND-701, FND-702) record that
three of the issue's ten claimed families are legitimately out of scope for this repo right
now, each citing the specific upstream item that blocks it, so the next reader does not
re-open the same question. One low-severity documentation drift (FND-703) is fixed in this
pass. No corrective FR content is owed by this pass.

## Recommendation

Issue #16's exit criteria are met for every family this crate can specify without
out-running its own semantic authority. The issue should either be narrowed to record that
scope explicitly (dropping "temporal" and "protocol encodings" until QSL#121 lands, and
"state" until IR#109 lands), or left open with a comment carrying this review's coverage
table so the next agent who reads it does not re-derive the same measurement from scratch.
