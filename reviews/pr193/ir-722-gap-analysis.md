---
id: SR-20261102072202
title: "IR-722 gap-analysis construction review"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-runtime; IR-722; src/monitor/, src/lib.rs, tests/monitor_program.rs, spec/monitor/functional/interface-002-monitor-program.md, interface-001"
review_set: subset
---

## Summary

Reviewed the frozen construction diff, public API, error paths and criterion/test bindings.

Frozen head: 4da072fad208ae36627cdc435408aa3c7bd2bfe2; base: 66eb9d8362e63d12e81f158abd2fae8e51dd561e.

## Verdict

**FAIL** — Findings require disposition.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The computed matrix leaves FR-276-AC-1 through AC-10 untagged; this construction change provides no monitor execution qualification. This is the acknowledged continuing IR-722 delivery gap, not a construction regression. | spec/monitor/functional/FR-276-consume-timed-monitor-plan.md:133-142 |

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

## Dispositions

Round 1 inspected faf1d5c238704140d5e02f4659a63500c1b3eb38 against the original frozen head.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | deferred | Already-open IR-722 retains FR-276-AC-1 through AC-10 for the subsequent executor and genuine producer/reference qualification slices. The authorized RT-first construction PR does not close or narrow them; no execution acceptance or Done state is granted. |

The original full-delivery FAIL remains an accurate view of unimplemented FR-276 execution acceptance. Its finding is deferred to the continuing IR-722 execution slices, which remain open on the same ticket. Construction-only merge scope is eligible after code dispositions, the mandatory full make ci gate and joint API freeze; this disposition grants neither runtime execution nor producer qualification.

Supplied focused evidence: 10 passed, zero failed/ignored; all-target/all-feature Clippy passed. Reviewer ran no Cargo or Kani. Full make ci remains owed; joint API freeze remains pending.

```yaml
dispositions:
- fnd: FND-001
  outcome: deferred
  reason: Already-open IR-722 retains FR-276-AC-1 through AC-10 for the subsequent
    executor and genuine producer/reference qualification slices. The authorized RT-first
    construction PR does not close or narrow them; no execution acceptance or Done
    state is granted.
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
bindings:
- test_id: retained_program_outlives_preparation_and_keeps_four_entry_keys
  ac_id: interface-002-AC-1
  trace: correct
- test_id: missing_and_mistyped_fourth_entry_refuse_before_any_execution
  ac_id: interface-002-AC-2
  trace: correct
- test_id: duplicate_source_keys_invalid_edges_and_cycles_refuse
  ac_id: interface-002-AC-3
  trace: correct
- test_id: argument_ownership_is_checked_through_the_expression_closure
  ac_id: interface-002-AC-3
  trace: correct
- test_id: counter_premises_keep_width_64_and_reject_out_of_width_gap
  ac_id: interface-002-AC-2
  trace: correct
- test_id: compatible_entry_roles_may_share_one_function
  ac_id: interface-002-AC-4
  trace: correct
- test_id: shared_expression_nodes_and_distinct_inline_paths_are_not_cycles_or_duplicate_keys
  ac_id: interface-002-AC-3
  trace: correct
- test_id: deeply_shared_inline_arena_validates_on_a_small_native_stack
  ac_id: interface-002-AC-3
  trace: correct
- test_id: exact_activation_capacity_includes_zero_and_values_above_host_width
  ac_id: interface-002-AC-2
  trace: correct
- test_id: duplicate_expression_locations_and_each_call_or_if_child_refuse
  ac_id: interface-002-AC-3
  trace: correct
dispositions: []
```

## Final RT carry (round 5)

Final RT carry inspected 019de067d34569f4fc6bb6cf8deebbbcd4893c0d. The local tests/monitor_program.rs identity fixture is deleted. TC-209 routes authentic construction controls to quire-integration IT-018/TC-098 and explicitly identifies diagnostic record/layout-key semantics. Production validation selects an existing UnitTable id through declared_key equality, retains exact Time dimension identity, and adds no affine or positive-scale restriction. Integration f9a59bda was already reviewed with authentic declarations, nested key and emitted negative/affine units; all integration findings are fixed. RT FND-003 and FND-004 are now fixed. Earlier FND-001/FND-002 remain fixed; no new source/contract defect found in this final carry.

Supplied final TC-033 evidence has seven passing tests including the semantic-identity inspection; all-target/all-feature Clippy passes. Integration ordinary aggregate evidence is reported as 81 passing, zero failed and six existing ignored. No ignored heavy proof is credited as executed. Reviewer ran no Cargo/Kani and edited no source.

Coverage is explicitly not green: current strict coverage is reported as 118 unbacked/zero contradicted, comprising unchanged baseline 114 plus four interface criteria now tested externally without local RT binders. The prior user exception covered identical 114 only and does not authorize 118; a new ruling is pending. The existing heavy IEEE proof was stopped and remains uncompleted. No all-gates-green or merge-ready claim is made. Full FR-276 execution acceptance remains deferred on open IR-722; this is only the RT-first construction slice.

Current method verdict: **FAIL** for continuing full-delivery coverage gap; its explicit deferral remains in force.

```yaml
dispositions: []
```
