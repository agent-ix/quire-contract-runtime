---
id: SR-20261102072203
title: "IR-722 spec-review/base construction review"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-runtime; IR-722; src/monitor/, src/lib.rs, tests/monitor_program.rs, spec/monitor/functional/interface-002-monitor-program.md, interface-001"
review_set: subset
---

## Summary

Reviewed the interface operations, success multiplicities, typed refusal prose, immutable inspection and delivery boundary. Shared NodeKeys retain content identity and package-lock selection context; no new digest mechanism is introduced.

Frozen head: 4da072fad208ae36627cdc435408aa3c7bd2bfe2; base: 66eb9d8362e63d12e81f158abd2fae8e51dd561e.

## Verdict

**PASS** — No defects found within this method and construction scope.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Coverage

Plan completion: not assessed

No applicable AssuranceProfile found. Source, tests, public contract and upstream QSpec FR-487 at the supplied revision were inspected. Supplied evidence: nine focused controls pass; all-target/all-feature Clippy and fmt pass; changed spec validation passes with native type-resolution warnings. No Cargo or Kani commands run by this reviewer. Full make ci remains owed after review. No target-memory proof is claimed. Optional gap semantic review was not separately invoked. EARS is inapplicable: no new FR/NFR/StR requirement statement. Risk/complexity, security and criterion-strength lenses were not selected for this bounded interface addition.

## Examined scope

```yaml
scope:
- id: interface-002-AC-1
  path: spec/monitor/functional/interface-002-monitor-program.md
  role: examined
  excerpt: After construction consumes the preparation parts and one Rc owner drops,
    another owner retains the complete root key with nested operand keys, selection
    bytes, all four entry associations, literal bodies and shared nominal declarations.
    Invalid arena lookups return absent. No monitor execution credit is granted by
    this construction control.
- id: interface-002-AC-2
  path: spec/monitor/functional/interface-002-monitor-program.md
  role: examined
  excerpt: Missing or mistyped on_gap, invalid counter width, a gap outside the declared
    width, negative uncertainty and a tick unit under another dimension yield the
    typed construction refusal before any entry executes; width64 with maximum gap
    u64::MAX remains representable. Activation capacity zero and a positive value
    above u64::MAX remain retained exactly, while a negative value refuses without
    a host-width truncation.
- id: interface-002-AC-3
  path: spec/monitor/functional/interface-002-monitor-program.md
  role: examined
  excerpt: Duplicate source locations, invalid expression edges, direct expression
    cycles including unreachable arena nodes, and a reachable argument of another
    function refuse with their distinct typed codes. A finite shared subexpression
    and distinct inline body paths under the same source node remain admissible; a
    20,000-level shared inline arena validates on a 512 KiB native stack without recursive
    graph descent.
- id: interface-002-AC-4
  path: spec/monitor/functional/interface-002-monitor-program.md
  role: examined
  excerpt: Two compatible entry roles may reference the same retained function; construction
    does not add a distinct-function premise to FR-487.
- id: src/lib.rs
  path: src/lib.rs
  role: examined
  excerpt: //!
- id: src/monitor/mod.rs
  path: src/monitor/mod.rs
  role: examined
  excerpt: //! Owned executable monitor programs (FR-276).
- id: src/monitor/program.rs
  path: src/monitor/program.rs
  role: examined
  excerpt: //! The single retained executable representation, independent of compiler
    storage.
- id: src/monitor/validation.rs
  path: src/monitor/validation.rs
  role: examined
  excerpt: //! Executable integrity checks; source-language admission remains upstream.
- id: tests/monitor_program.rs
  path: tests/monitor_program.rs
  role: examined
  excerpt: //! Construction controls; these do not qualify monitor execution or the
    producer handoff.
- id: spec/core/functional/interface-001-runtime-api.md
  path: spec/core/functional/interface-001-runtime-api.md
  role: examined
  excerpt: 'id: interface_001'
- id: FR-276
  path: spec/monitor/functional/FR-276-consume-timed-monitor-plan.md
  role: context_only
  excerpt: The runtime uses the capacity supplied by QSL rather than deriving a second
    formula.
```

## Round 2 carry review

Round 2 examined dd42a20f32935425c0da1347db2c1606d0567198. Borrowed function keys and expression (key, body_path) pairs are reserved once, sorted with canonical Ord, and adjacent equal locations refuse. Only scratch vectors reorder; owned arenas and FunctionId/ExpressionId references retain their original positions. No custom comparator, public type/field/API, gate or dependency changed. The changed ordering of duplicate checks does not violate a promised first-refusal order. Supplied exact-panic-probe evidence reports panic_references=0 and both negative controls pass. Supplied round2 focused controls report 10 passing and all-target/all-feature Clippy passes. No new current defect found.

TC-209 now exclusively owns construction diagnostics: all ten source test case tags and all four interface Verification cells use it. Its verifies edge targets interface_002; its procedure matches actual construction assertions and expressly excludes TC-200/FR-276 execution acceptance. The recomputed matrix keeps all four interface criteria tagged and all ten FR-276 criteria untagged. Coverage comparison confirms the exact baseline and candidate 114 unbacked rows, zero contradicted statuses, with equal rows including multiplicity. Earlier construction tags therefore grant no remaining TC-200 execution credit. No hand matrix or FR-276 acceptance text changed.

Code FND-001/FND-002 remain fixed; gap FND-001 remains deferred to the still-open IR-722 execution slices. Full-delivery gap verdict remains FAIL. Construction carry review is PASS; mandatory full local make ci is still owed and known baseline strict coverage failure remains explicit. Joint API freeze acknowledgement is reported by the dispatching owner; this delta changes no public API. No Cargo/Kani command or source edit was performed by this reviewer.

```yaml
scope:
- id: src/monitor/validation.rs
  path: src/monitor/validation.rs
  role: examined
  excerpt: //! Executable integrity checks; source-language admission remains upstream.
- id: tests/monitor_program.rs
  path: tests/monitor_program.rs
  role: examined
  excerpt: //! Construction controls; these do not qualify monitor execution or the
    producer handoff.
- id: TC-209
  path: spec/monitor/matrix/TC-209-monitor-program-construction.md
  role: examined
  excerpt: 'id: TC-209'
- id: spec/monitor/functional/interface-002-monitor-program.md
  path: spec/monitor/functional/interface-002-monitor-program.md
  role: examined
  excerpt: 'id: interface_002'
bindings: []
dispositions: []
```

## Final RT carry (round 5)

Final RT carry inspected 019de067d34569f4fc6bb6cf8deebbbcd4893c0d. The local tests/monitor_program.rs identity fixture is deleted. TC-209 routes authentic construction controls to quire-integration IT-018/TC-098 and explicitly identifies diagnostic record/layout-key semantics. Production validation selects an existing UnitTable id through declared_key equality, retains exact Time dimension identity, and adds no affine or positive-scale restriction. Integration f9a59bda was already reviewed with authentic declarations, nested key and emitted negative/affine units; all integration findings are fixed. RT FND-003 and FND-004 are now fixed. Earlier FND-001/FND-002 remain fixed; no new source/contract defect found in this final carry.

Supplied final TC-033 evidence has seven passing tests including the semantic-identity inspection; all-target/all-feature Clippy passes. Integration ordinary aggregate evidence is reported as 81 passing, zero failed and six existing ignored. No ignored heavy proof is credited as executed. Reviewer ran no Cargo/Kani and edited no source.

Coverage is explicitly not green: current strict coverage is reported as 118 unbacked/zero contradicted, comprising unchanged baseline 114 plus four interface criteria now tested externally without local RT binders. The prior user exception covered identical 114 only and does not authorize 118; a new ruling is pending. The existing heavy IEEE proof was stopped and remains uncompleted. No all-gates-green or merge-ready claim is made. Full FR-276 execution acceptance remains deferred on open IR-722; this is only the RT-first construction slice.

Current method verdict: **PASS** for construction review; unresolved aggregate gates still prevent a merge-ready conclusion.

```yaml
dispositions: []
```

## QSpec 595 amendment carry

QSpec #595 amendment carry inspected RT 8e0ff8fb57d5d24f7fe88d3619149e011de8e95d and integration 7ee2876a342446bf4a9b963c9cfa764a10e6b908 against the prior reviewed construction slice. Canonical source ruling now appends event_position to State; seventeen root bindings and public Layout/API remain unchanged. The outer State validator borrows the shared record, requires exact ordered required fields, Integer now/history_start/event_position, canonical buffer_0..n Sequence fields matching capacity count, and Sequence activations. It returns TypeMismatch at the State key; no compatibility default or host counter mirror appears. Source admission still owns activation/result internals.

Reviewed actual QSL State variants and matching shared declarations, valid zero/two-buffer cohorts, typed missing-position closure, malformed outer fields, State-specific refusal assertions, and retained literal zero. Supplied evidence is 14 passing controls, all-target Clippy pass, and intended red/green for the complete missing-position closure. These establish construction only. FR-276-AC-11 event increments and watermark/poll/gap preservation remain open execution obligations, together with AC-1..10. No genuine temporal producer or execution acceptance is granted. Previous code findings remain fixed; full-delivery gap remains deferred/open. Pending consumer acknowledgement, strict coverage authorization and incomplete heavy IEEE proof remain explicit; no merge-ready or all-gates-green claim is made. Reviewer ran no Cargo/Kani and changed no source.

No new defect in this method. A separate EARS method records the newly compound SHALL sentence; its disposition is still required.

## Final event-position review carry

Final event-position carry inspected 23a4cf5523644e3c4ca31065d7cffbe216fefe4d. The initializer and admitted-event increment are separate obligation statements with unchanged semantics. EARS FND-001 is fixed. The only engine grammar warning in supplied validation remains the pre-existing Reporting sentence at line 85, not this amendment. No RT production change after reviewed 8e0ff8fb; the closed State validator and shared-field ownership remain unchanged. All prior RT code findings remain fixed; no new defect found.

Integration final ordinary aggregate evidence at 33020725707bd6e559c00090fc33138f6a0ec5a3 reports exit 0, 84 passing, zero failed and six existing ignored; fourteen construction controls pass. Ignored proofs are not credited. State initialization literal retention does not prove event increments: FR-276-AC-1 through AC-11 remain open execution obligations on IR-722. Full-delivery gap stays deferred/open. RT strict-coverage authorization and stopped heavy IEEE proof remain unresolved; user merge hold persists. This is a construction review result, not all-gates-green or merge authorization. Reviewer ran no Cargo/Kani and edited no source.

Current method verdict: **PASS** for construction amendment review, subject to unresolved aggregate gates and merge hold.

```yaml
dispositions: []
```
