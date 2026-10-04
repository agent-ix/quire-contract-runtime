---
id: "SR-1397"
title: "IR-345 spec review (dependency): AD-004 migration steps (PR 100)"
type: SpecReview
analysis: dependency
review_set: subset
scope: "agent-ix/quire-contract-runtime@5a09652802d2e0e13a6f233e9a2ad723d9bba16f; spec/assurance/AD-004-runtime-crate-layout.md (Migration, Dependency edges, Open questions); compared with draft PR 95 head and CG origin/main"
relationships:
  - target: ix://agent-ix/quire-contract-runtime/AD-004
    type: references
---

# SR-1397: IR-345 dependency review of PR 100

## Summary

This review checks the order of steps 1 to 6, how each RT step pairs with a CG step, the
dependency-edge table, and how the open questions gate each step. Edges confirmed: RT bans 14 QSL
crates by name. QSL has 18 workspace members, and `qsl-analyze` and `qsl-walk-grow` are missing
from the ban list. `quire-semantic-value` pulls in `quire-canonical` as a workspace git dependency,
plus `serde` and `thiserror`. The negotiators need no `num-*` crate. `quire_exact` is used by no
RT source file today. CG builds `--locked` against RT `branch = "main"`.

The Value-bearing set is confirmed. RT `Value` holds `Quantity`, `EnumValue` and `ObjectReference`.
RT `ValueType::Reference` holds a `NodeKey`, where the shared one holds an `EffectiveId`.

The PR #95 comparison is accurate: 47 files, +579 / -10,056. It has the `pub use quire_exact::{..}`
block of 83 names, `stop.rs` duplicates `quire_semantic_value::stop`, and it carries a
`test-support` dev-dependency.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Step 3 cannot be done without touching the reference set that the AD puts in step 4. Step 3 deletes RT `outcome`. The shared `Refusal::ForeignReference` is `{ required: UniverseId, supplied: UniverseId }`. RT `equality.rs` (a step-4 file) builds the unit variant `Refusal::ForeignReference` from RT `ObjectReference`, and that type holds `UniverseIdentity(Box<[u8]>)`. Step 3 will not compile unless it also moves `ObjectReference`'s universe to `UniverseId`. Draft PR #95 (slice 1 = step 3) did exactly that: it changed `src/exact/reference.rs` to `universe: UniverseId` and deleted `UniverseIdentity`. The AD claims "a step cannot move part of that set without a shim". The PR #95 table also omits this edge. Name the edge, and either move the `UniverseIdentity` to `UniverseId` change into step 3 or merge steps 3 and 4. | AD-004 Migration steps 3 and 4, "Why steps 3 and 4 are separate" |
| FND-002 | medium | The Kani decision Q-3 is placed at step 4, but step 3 already needs it. `verification/kani.rs` imports `crate::exact::{compare_ieee, IeeeValue, Meter, Outcome, ScalarLimits}`, and step 3 deletes every one of these. CG's `kani/generate/scalar.rs` emits `rt::Integer` and `rt::Outcome`, which step 3 repaths. Gate G runs `make kani kani-mutations` at every step, and PR #95 already edits `kani.rs` and `scripts/check_kani_mutations.py`. Move "Kani decision (Q-3)" to step 3's gate. | AD-004 Migration table step 3/4 Gate, Q-3 |
| FND-003 | low | The AD says the RT/CG merge order is "the team leaders'", but the order is forced. CG builds `--locked` against RT `branch = "main"`. A CG PR therefore cannot lock to an RT commit that is not yet on RT main. If RT merges first, CG main keeps its old lock and stays green until the paired CG PR bumps the lock. So the order is RT first, then the CG PR with its lock bump. State this order, and say that RT main is never consumed half-migrated by CG. | AD-004 Migration preamble |

## Owner questions against steps

- **Block IR-349 step 1 (spec only):** Q-1 (decides the residue list, FR-275-AC-16 to AC-18,
  interface-001-AC-7, and whether `expression` exists) and Q-2 (decides that there is no `exact`
  path, and so the wording of FR-275 and interface-001). Step 1 amends the spec to match those two
  answers, so it cannot start until both are answered.
- **Step 2:** nothing blocks it. Q-6/O-1 changes only the git URL, so the step is indifferent to it.
- **Step 3:** Q-3 (FND-002). Q-4 is not blocking, but step 3 is where `Outcome` and the other
  shared enums lose `#[non_exhaustive]`.
- **Step 4:** Q-5 (`from_hex`, and the `decode_admitted` precondition: SR-1398 FND-002). R-1 to R-3
  affect the Value-bearing set.
- **Step 5:** Q-1 again (keep or delete `expression`), R-1 (`Origin`/`Location`), R-2 (depth).

## Verdict

**Dependency: two mediums, one low.** The step order is mostly coherent and each step pairs with a
CG PR. Step 2 is safe. The step 3/4 boundary and the Kani gate need the corrections above.

## Dispositions

Disposition pass 1, reviewed at agent-ix/quire-contract-runtime@c7087f953a24cc49774ed81c5914d5eef0f3fbd0 (fix commits 6a5d672, c7087f9).

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 6a5d672. Step 3 now includes "the object-reference universe changes from `UniverseIdentity` to `UniverseId`, because the shared `Refusal::ForeignReference` carries `UniverseId` values". The "why step 4 is one step" paragraph says step 3 is not clean of that set and keeps only the universe change there. The PR #95 table gains the `reference.rs` row. This matches the PR #95 diff, which I re-read. |
| FND-002 | fixed | 6a5d672. The step 3 gate is now "G; Q-3 answered", and step 3 repoints `verification/kani.rs`, `scripts/check_kani_mutations.py` and the Kani generator's `rt::Integer`/`rt::Outcome`. Q-3 itself says it is needed at step 3. |
| FND-003 | fixed | 6a5d672. The AD now says "the order is forced, not chosen". I re-measured the mechanism. The CG Makefile sets `LOCKED ?= $(if $(wildcard .cargo/config.toml),,--locked)`, and `make use-local` writes that gitignored config. CG's scratch-crate compile tests copy CG's own `Cargo.lock` beside each generated manifest (`tests/it/scratch_crate.rs`) and build `--offline`. So CG main compiles generated crates against its locked RT and stays green after an RT merge, until the paired CG PR bumps the lock. |
