---
id: SR-20261102072206
title: "IR-722 spec-review/scope-boundary construction review"
type: SpecReview
analysis: scope-boundary
scope: "agent-ix/quire-contract-runtime; IR-722; src/monitor/, src/lib.rs, tests/monitor_program.rs, spec/monitor/functional/interface-002-monitor-program.md, interface-001"
review_set: subset
---

## Summary

IR owns source selection/admission; RT owns retained arenas and executable-integrity checks; QSV owns nominal membership and the unit table. Generic call/result conformance remains expressly deferred to value admission at execution.

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
