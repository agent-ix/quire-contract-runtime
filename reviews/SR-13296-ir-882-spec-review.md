---
id: SR-13296
title: "IR-882 PR 135 spec review: RT migration onto quire_exact::Value"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-runtime@1f744539a5842f92bd378f2fc2fe85c6c3b37567 vs origin/main merge-base; spec/assurance/AD-004-runtime-crate-layout.md, spec/exact/functional/FR-008-composite-collection-and-equality.md, spec/exact/functional/FR-275-single-exact-kernel.md, spec/exact/functional/FR-277-checked-equality-evaluation.md, spec/exact/matrix/TC-024-composite-construction.md, spec/exact/matrix/TC-025-collection-algebra-and-canonical-key.md, spec/exact/matrix/TC-026-equality-matrix.md, spec/exact/matrix/TC-203-checked-equality-evaluation.md, spec/exact/matrix/tests.md, spec/tests.md, PR body plan"
review_set: subset
---

## Summary

Ticket: IR-882. PR agent-ix/quire-contract-runtime#135 (spec and plan only). This review
re-measured the PR's central claims against the code. It checked RT `Value` (`src/exact/composite.rs:262`,
`Rc` nodes, `repr(u64)`), RT `planned_equality`/`plan_pairs` (`src/exact/equality.rs:222,258`), RT
membership (`src/exact/collection.rs:278-296`) and `CheckedEquality::evaluate`
(`src/exact/equality.rs:160`). It compared them with the quire-exact revision RT actually locks
(`Cargo.lock:229`, `6f4cc48`) and with quire-exact `origin/main` (`c2cd5f5`).

Confirmed:

- The kernel `Value` (main) has a variant for every RT variant, with `Arc` nodes.
- The kernel `form_collection`/`coalesce` keeps bag multiplicity and the first-supplied value
  exactly as RT does (FR-008-AC-11).
- The kernel `member_equal` charges `collection.member-walk`/`member-test` with the same sizes as RT.
- `repr(u64)` is unread by any Kani harness.
- On main, `planned_equality_with_plan` returns `EqualityEvaluation { equal, plan }`.

Not confirmed:

- The PR body says "the locked quire-exact checkout exports ... `planned_equality_with_plan`".
  That is false. The locked revision has no such function. It was added in quire-exact `10b628d`
  (#34), after the lock.
- The claim of "no missing kernel capability" holds only if RT owns unit, enum and object-type
  identity resolution. Neither the spec nor the plan schedules that work.

## Examined criteria

- AD-004 IR-882 step-4 paragraph (examined): "IR-882 is the spec-first entry to step 4. At this boundary the value owner is `quire_exact::Value` ... The `CheckedEquality::evaluate` path calls `quire_exact::planned_equality_with_plan` ... No conversion layer, alias, old-`Value` re-export or second pair traversal is a step-4 output."
- FR-275 statement (examined): "The end state is that the runtime holds no distinct `Value` enum, `planned_equality`, `plan_pairs` or collection membership walker; RT's checked evaluator accepts `quire_exact::Value` directly and calls `quire_exact::planned_equality_with_plan` ... There is no conversion layer or old-`Value` re-export."
- FR-275 Kernel shape change (examined): "The locked `quire-exact::Value` has `Arc`-backed option, composite and collection nodes ... IR-882 migrates RT to the kernel value with no conversion layer; the RT `repr(u64)` tag is a layout choice, not a semantic requirement."
- FR-008 statement (examined): "the runtime consumes the shared kernel: `quire_exact` owns `Value`, `ValueType`, collection formation, membership, canonical keys and equality planning."
- FR-008-AC-5 (examined): "`plan_equality` forms the occurrence-pair plan with no charge, refuses a cross-universe reference pair as `ForeignReference`, and distinguishes unlike collection kinds (`CollectionKindMismatch`) from unlike value kinds (`ValueKindMismatch`) ..."
- FR-008-AC-8 (context_only): "`ChargePoint::ALL` has exactly 62 members"
- FR-008-AC-11 (examined): bag multiplicity, first-supplied representation — kernel matches.
- FR-277 Behavior and FR-277-AC-1 (examined): stays PLANNED; no test is tagged (quoin matrix: untagged).
- PR body tests-first plan and downstream inventory (examined).

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | RT locks quire-exact `6f4cc48`, which has no `planned_equality_with_plan`; it was added in `10b628d` (#34). The PR body says the locked checkout exports it, and the plan has no lock-bump slice. Bumping the lock pulls in 45 commits. These add `Value::{String,Uuid,Timestamp,Union}`, which every exhaustive RT match must handle, and a 63rd `ChargePoint` (`ALL: [Self; 63]`), which contradicts FR-008-AC-8's "exactly 62". They also bring IR-705/IR-895 `Meter` changes. Fix: correct the claim; add a slice 0 that bumps the lock and stays green on its own; amend FR-008-AC-8 in that slice. | spec/assurance/AD-004-runtime-crate-layout.md:394-405; Cargo.lock:229; spec/exact/functional/FR-008-composite-collection-and-equality.md:169 |
| FND-002 | medium | The AD-004 step-4 gate is "G; Q-5 answered" (line 390), and Q-5 (consumer `NodeKey` construction) is still open (lines 434, 443). The new paragraph declares IR-882 the entry to step 4, but neither it nor the plan mentions the gate. Fix: state Q-5 as a prerequisite of slice 1, or record which parts of step 4 can proceed without it (RT library constructs no `NodeKey`; CG key paths stay unresolved). | spec/assurance/AD-004-runtime-crate-layout.md:390,394 |
| FND-003 | medium | The FR-275 edit deletes the statement that the kernel lacks cross-unit quantity arithmetic and the equality conversion table. That is still true: kernel `quantity.rs` documents no unit graph and same-unit operations only. Moving onto kernel types changes four RT payloads: `Quantity(Rational, QuantityUnit{Declared(Box<Unit>) or Compound})` becomes `(Rational, UnitId)`, `ValueType::Enum(NodeKey)` becomes `EnumShape`, `EnumValue{declaration, member, ordered}` becomes `EnumMember{VariantId, rank}`, and `ObjectReference(UniverseId, NodeKey, ObjectIdentity)` becomes `(UniverseId, EffectiveId, ObjectId)`. After the move, RT needs a `UnitId` to unit resolver for conversion and arithmetic, and an enum and object-type registry. Neither the spec nor the plan names this work, so "no missing kernel capability" is true only on that unstated condition. Fix: restate the boundary (the caller owns the unit graph, the conversion table and the registries) and list the resolver work in slice 1. | spec/exact/functional/FR-275-single-exact-kernel.md:211-216 |
| FND-004 | medium | The kernel walker differs from RT's in three observable ways, and neither the spec nor the plan records them. (1) Composites of different declarations are one unequal pair under the kernel; RT refuses with `ValueKindMismatch`. (2) Enums compare by `VariantId` only; RT refuses when the declarations differ. (3) Kernel `member_equal` calls `plan_pairs(member, candidate)`, while RT calls `plan_pairs(candidate, member)`, so `ForeignReference` swaps `required` and `supplied` during membership. Fix: state each delta in FR-008 Behavior/AC-5, and name in slice 1 the tests that pin each one. | spec/exact/functional/FR-008-composite-collection-and-equality.md:166; src/exact/equality.rs:258-350; src/exact/collection.rs:278-296 |
| FND-005 | low | The downstream inventory is partly stale. QSL cites `integration/current-head/tests/contract.rs`, which does not exist on QSL `origin/main` (`ed2831ea1`). QSL has no RT dependency at all, so the conclusion holds. CG test consumers are given as directories, not exact paths. The exact files are `tests/composite_equality_support/{agreement,agreement_cases,package}.rs`, `tests/exact_function_support/agreement_cases.rs`, `tests/exact_scalar_support/{agreement,agreement_cases,package}.rs` and `tests/it/{composite_equality_generation,exact_function_agreement,exact_function_generation,exact_scalar_generation,kani_obligations,kani_scalar_native_private,oracle_arithmetic,routed_generation,capability_settlement}.rs`. IR and quire-driver have no RT references, as the body says. Fix: update the PR body. | PR body (Downstream follow-ups) |

## Verdict

**FAIL.** The direction is right: one kernel value, no adapter or re-export, and FR-277-AC-1
kept PLANNED with no kernel-only credit. But the plan's first slice cannot go green as written.
It depends on a kernel function RT's lock does not have (FND-001). The step-4 gate is open
(FND-002). The RT-side resolver work that makes "no missing kernel capability" true is not
specified (FND-003, FND-004).

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-006 | low | The new FR-008 Behavior bullet says membership "passes the sought member before the candidate to the kernel planner", and AC-5 says `ForeignReference` "names the sought member as `required` and candidate as `supplied`". The kernel's `member_equal(candidate, member)` calls `plan_pairs(member, candidate)`, where `member` is the collection's already-retained element and `candidate` is the probe. In a membership test the probe is the thing sought, so "sought member" reads as the opposite operand and can invert the stated direction. Fix: write "the collection's retained member as `required` and the probed candidate as `supplied`" in both places. | spec/exact/functional/FR-008-composite-collection-and-equality.md:91-97,175 |

## New findings (disposition pass 2)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-007 | low | The FND-006 fix leaves a doubled article in FR-008-AC-5: "names the the collection's retained member as `required`". Fix: delete one "the". | spec/exact/functional/FR-008-composite-collection-and-equality.md:175 |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | PR body slice 0 updates `Cargo.lock` to quire-exact main and handles String/Uuid/Timestamp/Union, the IR-705/IR-895 `Meter` changes and the 62-to-63 census in FR-008-AC-8/TC-024, green on its own. The boundary claim is corrected. tests.md marks the AC-8 text change as planned for slice 0. |
| FND-002 | fixed | AD-004:394 reads "its first code slice requires Q-5 to be answered". The Q-5 paragraph at 434-441 states that the standing ruling does not answer it and that QSL decides before slice 1. The plan's slice 1 is "after Q-5". |
| FND-003 | fixed | FR-275 adds a "Caller-owned identity and units" bullet: the kernel has no cross-unit arithmetic or conversion table, and RT owns the `UnitId` resolver, unit graph, conversion table, `EnumShape` and `EffectiveId` registries. Slice 1 implements and tests them. |
| FND-004 | fixed | FR-008 Behavior and AC-5 record all three deltas, and slice 1 names three tests that pin them. The wording of the membership-direction delta is FND-006. |
| FND-005 | fixed | The PR body lists exact CG test files and states that the QSL path is absent from QSL main. |
| FND-006 | fixed | Commit 5510821 changes the FR-008 Behavior bullet to "the collection's retained member before the probed candidate" and AC-5 to the retained member as `required` and the probed candidate as `supplied`, which matches the kernel's `plan_pairs(member, candidate)`. |
| FND-007 | fixed | Commit aa15ee2 removes the doubled article: AC-5 now reads "names the collection's retained member as `required` and the probed candidate as `supplied`". |
