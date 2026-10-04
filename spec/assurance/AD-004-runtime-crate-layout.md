---
id: AD-004
title: "Runtime crate layout: modules of this crate and the shared exact kernel it consumes"
type: ArchitectureDescription
status: proposed
owner: runtime-maintainers
system: quire-contract-runtime v0.1 source layout under src/ (the core, accounting and snapshot, the proptest adapter, and the exact feature's 23 files), the dependency edges it ends with, what it keeps against what quire-exact and quire-semantic-value own, and the migration steps IR-349 executes
relationships:
  - target: ix://agent-ix/quire-contract-runtime/AD-001
    type: references
  - target: ix://agent-ix/quire-contract-runtime/AD-002
    type: references
  - target: ix://agent-ix/quire-contract-runtime/AD-003
    type: references
  - target: ix://agent-ix/quire-contract-runtime/FR-001
    type: references
  - target: ix://agent-ix/quire-contract-runtime/FR-004
    type: references
  - target: ix://agent-ix/quire-contract-runtime/FR-006
    type: references
  - target: ix://agent-ix/quire-contract-runtime/FR-007
    type: references
  - target: ix://agent-ix/quire-contract-runtime/FR-008
    type: references
  - target: ix://agent-ix/quire-contract-runtime/FR-009
    type: references
  - target: ix://agent-ix/quire-contract-runtime/FR-010
    type: references
  - target: ix://agent-ix/quire-contract-runtime/FR-011
    type: references
  - target: ix://agent-ix/quire-contract-runtime/FR-012
    type: references
  - target: ix://agent-ix/quire-contract-runtime/FR-273
    type: references
  - target: ix://agent-ix/quire-contract-runtime/FR-275
    type: references
  - target: ix://agent-ix/quire-contract-runtime/NFR-001
    type: references
---
# Runtime crate layout

AD-001 describes the core, AD-002 the function-application boundary and AD-003 the seam to codegen.
None says where the code lives or what this crate keeps once the shared kernel is consumed. This AD
does (IR-345, the layout half of AD-016 owner decision 2). Measured on 2026-10-04 at the `origin/main`
of this repository, of `quire-spec-language` (QSL, crates `quire-exact` and `quire-semantic-value`)
and of `quire-contract-codegen` (CG), read-only; no commit is asserted anywhere. Counts are
re-measured, not copied from `quire-contract-ir` AD-007, and agree with it. QSL document ids (ADR-011,
AD-016) are QSL's and are cited by id only.

## System Boundary

In scope: the files under `src/`, `verification/`, `measurement/` and `tests/`; the features; the
dependency edges and `deny.toml` guards; the owner of every `exact` item; the paths in CG that name
this crate's `exact` module. Out of scope: kernel behaviour (QSL's, specified by FR-006 to FR-012 as
behaviour of QSL's crates), CG's own layout, IR, and where QSL's two crates live (O-1 of AD-007 in
`quire-contract-ir`: open, the owner's, not decided here; this crate is indifferent, see Decisions).

## Views

The layout is described as it is, as the shared names classify, as it ends, and as the dependency
edges and the consumer's paths follow from that.

### Current layout

Measured: 34 files and 13,791 lines under `src/`; `exact/` is 23 files and 12,080 lines. Outside
`src/`: `verification/kani.rs` 295, `measurement/footprint` 124, `tests/` 16 files and 9,591 lines
(12 `exact_*` files, 8,686 lines).

Core, accounting, snapshot, adapter (no `exact` dependency; unchanged by this AD):

| File | Lines | Feature | Owns |
| --- | --- | --- | --- |
| `lib.rs` | 81 | none | feature gates, core re-exports, `exact` module declaration |
| `identity.rs`, `observation.rs`, `verdict.rs` | 74, 176, 170 | none | FR-001 identity, observation, failure and verdict types |
| `operators.rs` | 217 | none | FR-002 panic-free checked and short-circuit helpers over `bool` and integers |
| `accounting.rs` (+ `accounting_tests.rs` 53) | 297 | none | FR-004 `CampaignReport`, `CampaignCounts`, `CampaignSnapshot`, `IdentityMismatch` |
| `snapshot_json.rs` | 351 | `snapshot-json` | `runtime.campaign-snapshot/v1` codec |
| `proptest_adapter.rs` | 37 | `proptest` | FR-003 adapter |

`exact/` (feature `exact`; target module per the split asked for in IR-345; owner after is who defines the
item once this AD is realized):

| Target | File (lines) | Owns today | Owner after |
| --- | --- | --- | --- |
| scalar | `integer` 700, `rational` 327, `decimal` 1,103, `numeric` 468, `division` 233, `ieee` 1,792, `comparison` 143 | exact integer, rational, decimal, IEEE values and operation tables, comparison and ill-typed vocabulary, I13 negotiators, lazy Boolean connective | `quire-exact`; this crate keeps the negotiators (`division` 180-233, `ieee` 760-857) and the connective (`numeric` 408-468), about 210 lines |
| scalar | `quantity` 588, `unit` 491 | quantity arithmetic, unit graph, compound units | `quire-semantic-value` (`quantity`, `unit`) and `quire-exact` (`quantity`) |
| text | `text` 539 | text admission, normalization, comparison | `quire-exact` (`text`) |
| text | `enumeration` 189 | enumeration declarations and values | `quire-semantic-value` (`enumeration`) |
| composite | `composite` 1,585, `collection` 387, `equality` 573, `key` 125 | `Value`, `ValueType`, declarations, `TypeEnvironment`, collections, canonical key, equality plan, `CheckedEquality` | `quire-exact` (`value`, `collection`, `equality`, `key`) and `quire-semantic-value` (`declaration`) |
| composite | `containment` 226, `reference` 253, `node` 170, `definition` 144 | value graph, object references and environment, `NodeKey`, graph and package refusal vocabulary | `quire-semantic-value` (`containment`, `object_closure`, `semantic_node`, `definition`) and `quire-exact` (`reference`, `identity`, `node`) |
| expression | `expression` 978 | function application (AD-002, FR-273) | vocabulary (about 215 lines) to `quire-semantic-value` (`checking`, `call`, `loss`, `location`); the call mechanism (about 760 lines) has no home outside this crate: Q-1 |
| outcome | `outcome` 222, `accounting` 685 | `Outcome`, `Undefined`, `Refusal`, `Meter`, charge points, limits | `quire-exact` (`outcome`, `accounting`) |
| (root) | `mod.rs` 159 | the `pub use` list of every item above | deleted |

The three line sums by target are 5,845 (scalar), 728 (text), 3,463 (composite), 978 (expression)
and 907 (outcome), plus `mod.rs`: 12,080. `src/exact_accounting_tests.rs` (232) and
`src/exact_integer_tests.rs` (23) test the deleted `Meter` and `Integer` and are not counted in it.

### Shared names: classification

Method. Every `pub struct` and `pub enum` name in `src/exact` (151) was compared with the same
names in `quire-exact` (102) and `quire-semantic-value` (72). 124 names exist in at least one (the
figure AD-007 gives; 82 in `quire-exact`, 49 in `quire-semantic-value`, 7 in both). For each, the
declaration was compared field by field and variant by variant after stripping comments, attributes
and module paths (a script over the sources, run once), then the six named below were read in
full (impl surfaces, not only declarations). Behaviour was not executed and not compared beyond
those six, so "same declaration" is a candidate for "same semantics", settled by the step's tests.

| Class | Count | Rule | Names |
| --- | --- | --- | --- |
| Same declaration | 49 | delete, depend on the shared crate | e.g. `BoundViolation`, `BoundedInteger`, `ScalarLimits`, `Incomplete`, `IntegerInterval`, `NodeKey`, `ValueGraph`, `GraphRefusal`, `EnumDeclaration`, `PackageRefusal`, `DecimalLoss`, `IeeeValue` |
| Same declaration except RT adds `#[non_exhaustive]` | 45 | delete; consumers lose `#[non_exhaustive]` (see Risks) | e.g. `Outcome`, `LimitKind`, `RoundingMode`, `IeeeWidth`, `IllTypedCause`, `ComparisonOperator`, `ValueLoss`, `CheckMode`, `Component` |
| Same fields, boxed in RT | 8 | delete; representation only | `Integer`, `Decimal`, `DecimalType`, `Rational`, `RationalDomain`, `Text`, `EnumValue`, `CompoundUnit` |
| Same name, different declaration | 22 | delete after the adapter in the step | `Meter`, `ChargePoint`, `InjectedDenial`, `Refusal`, `Undefined`, `Value`, `ValueType`, `Quantity`, `QuantityOperation`, `ObjectReference`, `ObjectTypeDeclaration`, `FieldDeclaration`, `TypeEnvironment`, `CheckedEquality`, `DeclarationCause`, `CollectionType`, `UnitGraph`, `CheckingLimits`, `InputRefusal`, `PackageCause`, `Origin`, `Location` |
| RT-only | 27 types, 7 free items | keep, or delete per Q-1 and the successors below | see What this crate keeps |

The six read in full:

| Name | Finding |
| --- | --- |
| `Integer` | same public methods (`zero`, `one`, `is_zero`, `is_negative`, `magnitude_bits`, `decimal_digits`, `to_u64`, `is_even`), `From<i64>`, `From<i128>`, `From<u64>`, `FromStr`, `Display`, and the same `Debug` text. RT holds an inline `i64` that promotes to a boxed `BigInt` (laid out for CBMC, `verification/kani.rs`); QSL wraps a `BigInt`. QSL adds `From<usize>` and makes the arithmetic methods public. Same semantics, different layout (class 3) |
| `Decimal` | same constructors and accessors, same `evaluate_decimal` signature; RT boxes its two fields. QSL adds the typestate `Placement`, `Placed`, `Admitted` and drops `#[non_exhaustive]` on `RoundingMode` and `DecimalOperation`. Same semantics, different layout |
| `Outcome` | the enum is the same four variants; RT adds `#[non_exhaustive]`, `#[repr(u64)]` and `From<T>`. The payloads differ: `Undefined` gains `SumOutOfDomain` and loses `Copy`; `Refusal` is a unit-variant `Copy` enum in RT and carries its targets in QSL (`IntegerOutOfDomain { target }`, `ForeignReference { required, supplied }`, and so on) with different `code()` and `cause()` results. Different |
| `Meter` | RT: ten counters, injected denial, an always-on admitted-charge log capped at `CHARGE_LOG_CAPACITY`, `charge_log_truncated`; QSL: counters, a `Cancel` handle, the log only under `test-support`, `Charge` and `charge` public, 62 charge points against 52 (ten more: `DeclarationCheck`, `DispatchSelect`, `GraphEdge`, `GraphExpand`, `GraphResultRetain`, `LookupKey`, `LookupResultRetain`, `ModelDeref`, `ModelNavigate`, `PopulationVisit`), `InjectedDenial::occurrence` is `u64` (RT `NonZeroU64`). Read from `check_injected`: the denial is matched on the count of admissions and never cleared, and a denied charge does not advance that count, so every later charge at the denied point is denied again (FR-010-AC-5 says single-shot; not executed). Different |
| `NodeKey` | same 32 bytes, `Display` and `Debug`. Constructors differ: RT `from_bytes` (`const`) and `from_hex`; QSL `from_digest` and `decode_admitted`. Same semantics, different constructors; `from_hex` has no QSL counterpart |
| `ObjectClosure` | no RT type has the name. RT's `ObjectEnvironment` is the same algorithm (admit objects, fill slots, check every reference is a member, same four cause variants). QSL's `new` takes a `tolerated_dangling` list, adds `find`, and `attribute` takes a `FieldRef` where RT takes a `&str`; the refusal boxes its object. Successor under another name, different signatures |

### Target layout

Modules of this crate after the steps below (one feature `exact`; no `exact` module):

| Module | From | Public surface | Deleted from it (owner) |
| --- | --- | --- | --- |
| `verdict`, `identity`, `observation`, `operators` | same files | unchanged (FR-001, FR-002) | none |
| `accounting`, `snapshot_json` | same files | unchanged (FR-004) | none |
| `proptest_adapter` | same file | unchanged (FR-003) | none |
| `scalar` (`exact`) | `division`, `ieee`, `numeric` fragments | `negotiate_integer_division`, `IntegerDivisionBounds`, `IntegerDivisionConsumer`, `IntegerDivisionDisposition`, `negotiate_ieee`, `IeeeBackendCapabilities`, `IeeeItemRequirement`, `IeeeUnsupportedCause`, `IeeeDisposition`, `evaluate_boolean_short_circuit`, `ShortCircuitConnective` (11 items; FR-009, FR-006) | all of `integer`, `rational`, `decimal`, `comparison`, `quantity`, `unit` and the rest of `division`, `ieee`, `numeric` (`quire-exact`, `quire-semantic-value`): about 5,630 lines |
| `text` | none | module not created | `text`, `enumeration`: 728 lines |
| `composite` | none | module not created | `composite`, `collection`, `equality`, `key`, `containment`, `reference`, `node`, `definition`: 3,463 lines |
| `outcome` | none | module not created | `outcome`, `accounting`: 907 lines |
| `expression` (`exact`) | `expression` | `Body`, `Frame`, `FunctionDeclaration`, `PackageDeclarations`, `CheckedPackage`, `CheckedExpression`, `CheckRefusal`, `CheckCause`, `CallPlan`, `plan_call`, `plan_evaluation`, `Evaluation`, `EvaluationRefusal`, `DepthAboveMaximum`, `MAX_CALL_DEPTH` (15 items; FR-273), over the shared crates' values | the vocabulary (`CheckMode`, `CheckingLimits`, `Origin`, `Location`, `InputRefusal`, `ValueLoss`, `LocatedLoss`) to `quire-semantic-value`. The module exists only if Q-1 is answered as recommended |

### What this crate keeps, and what it deletes

| Kept | Why it is not a copy |
| --- | --- |
| core, accounting, snapshot, adapter, `verification/kani.rs`, footprint crate | not QSL code |
| the 11 `scalar` items | absent from both shared crates (measured: no `negotiate_*` and no lazy-right connective in either); QSL deleted its negotiators on purpose (FR-275, as relayed by QSL) |
| the 15 `expression` items (pending Q-1) | absent from both shared crates; the QSL types of the same names live in `qsl-semantics`, `qsl-package` and `qsl-eval`, which this crate may not depend on (FR-275 bans) |

Deleted outright: everything else in `src/exact`, `mod.rs` included (`CHARGE_LOG_CAPACITY` and
`charge_log_truncated` have no counterpart and go with `Meter`; R-4), with its tests (`src/exact_*_tests.rs` and the
kernel halves of `tests/exact_*.rs`). No test comparing this crate's output with a second
implementation is kept (FR-275). RT-only names with a successor in the shared crates under another
name: `UniverseIdentity` to `UniverseId`, `ObjectIdentity` to `ObjectId`, `ObjectEnvironment`,
`ObjectEnvironmentCause`, `ObjectEnvironmentRefusal` to `ObjectClosure`, `ObjectClosureCause`,
`ObjectClosureRefusal`. `InvalidObjectIdentity`, `InvalidTextLiteral` and `UnitDeclaration` have no same-named
item in either crate and were not traced further: the step that meets them finds their successor or
records the gap.

### The end state of the shared kernel (AD-016 owner decision 2)

- This crate depends on `quire-exact` and `quire-semantic-value`, once each, optional, enabled only
  by `exact`. It contains no second implementation of any item they define.
- No re-export shim: there is no `quire_contract_runtime::exact` module, no `pub use` of either crate,
  no alias, wrapper, feature or legacy reader. Consumers import `quire_exact::` and
  `quire_semantic_value::` paths. A kept item that must name a shared type names it in its own
  signature, which is a dependency, not a re-export.
- FR-275's Description ("Its `exact` module is a re-export of those QSL-owned crates at the
  `quire_contract_runtime::exact` path") and interface-001-AC-1, AC-2, AC-4 and AC-5 say the opposite.
  This AD follows the owner decision as stated in the IR-345 assignment; step 1 amends them (Q-2).
- Direct dependencies `num-bigint`, `num-integer`, `num-traits` and `unicode-normalization` are
  named today only by `integer.rs`, `rational.rs`, `ieee.rs` and `text.rs`, none of which keeps code that
  names them (the negotiators do not); they are removed in the step that deletes those files.

### CG paths that change in the same step

Measured at CG `origin/main`. CG's emitted manifest carries no pinned revision; it names this
crate by git URL and branch (`src/core/profile.rs`).

| CG path | Names `quire_contract_runtime::exact` as | Count |
| --- | --- | --- |
| `src/oracle/scalar/mod.rs` | generator import (`:58`) and emitted `use quire_contract_runtime::exact as rt;` (`:2396`) | 105 `rt::` refs, 47 distinct |
| `src/oracle/equality/mod.rs` | generator import (`:79`) and emitted alias (`:1417`); serializable mirrors of `IllTypedCause`, `RecursionEdges`, `DeclarationCause` (`:194-271`) | 112 refs, 31 distinct |
| `src/oracle/function/mod.rs` | generator alias (`:145`) and emitted alias (`:1452`); mirrors of `Origin`, `Location` (`:413-429`) | 90 refs, 20 distinct |
| `src/kani/generate/scalar.rs` | `rt::Integer` in generated Kani text (`:23`, `:82`) | 9 refs, 4 distinct |
| `src/core/profile.rs` | emitted `Cargo.toml` (`oracle_crate_manifest`): one `quire-contract-runtime` dependency, `features = ["exact"]` and the Kani metadata | the manifest must also name `quire-exact` and `quire-semantic-value` |
| `Cargo.toml` (`:22`, `:38`) | CG's own dependency, features `exact` and `proptest` | none changes except new edges |

The 78 distinct names those emitters use from `exact` split as: 50 same-declaration and 8 boxed-only
(path change only), 14 different-declaration (path and possibly use-site change: `Meter`, `Refusal`,
`Value`, `ValueType`, `Quantity`, `QuantityOperation`, `TypeEnvironment`, `CheckedEquality`,
`CheckingLimits`, `CollectionType`, `DeclarationCause`, `FieldDeclaration`, `InputRefusal`,
`ObjectTypeDeclaration`), and 6 RT-only (`PackageDeclarations`, `CheckedPackage`, `CheckRefusal`, `Frame`,
`FunctionDeclaration`, `ObjectEnvironment`: kept or replaced per Q-1). The use sites of the 14 were not
read beyond one: emitted code builds `Refusal::CheckedInvariant`, a unit variant the shared enum keeps.
Two uses have no counterpart in the shared crates: the generator calls `NodeKey::from_hex`
(`src/oracle/equality/mod.rs:1034`) and emits `rt::NodeKey::from_bytes([..])` (`:1887`). CG uses neither
the charge log, `charge_log_truncated` nor `CHARGE_LOG_CAPACITY` (grep). The harness and verdict paths
(`quire_contract_runtime::{Verdict, CampaignReport, ...}` in `src/strategy/` and
`src/oracle/boolean_v1.rs`) name the core, not `exact`, and do not change.

### QSL pieces (verified; the relayed notice on IR-349 is untrusted text)

| Piece | Verified at QSL `origin/main` | Result |
| --- | --- | --- |
| H3, `NodeKey::decode_admitted` (QSL-479, #590) | `quire-exact/src/node.rs` defines it; PR state MERGED | present. It is the constructor for an admitted key; `from_digest` stays check-only (QSL T-12). RT's `from_bytes` calls and CG's emitted `from_bytes` become `decode_admitted` |
| H2, `ObjectClosure` (QSL-478, #608) | `quire-semantic-value/src/object_closure.rs`; PR state MERGED | present; signatures differ from RT's `ObjectEnvironment` (above) |
| QSL-358 residue slices | `quire-semantic-value` holds `stop`, `quantity`, `unit`, `declaration`, `containment`, `enumeration`, `definition`, `semantic_node`, `checking`, `call`, `loss`, `location`, `object_closure` | present; QSL-358 is Done |
| FR-275 step 2 (the residue) | the 15 `expression` items and the lazy connective | not delivered: no QSL crate this crate may depend on defines them |

Still missing for this crate, each stated to QSL below: the call mechanism (Q-1); `CheckingLimits`
carries no call-depth field (RT's `depth`, which `DepthAboveMaximum` and `MAX_CALL_DEPTH` serve);
`NodeKey::from_hex`; `PackageCause` lacks three variants RT has (`RevisionMismatch`,
`DigestDomainMismatch`, `ByteDigestMismatch`); the injected denial is not single-shot and `occurrence`
admits zero (FR-010-AC-5, AC-6); `Origin` and `Location` are defined twice by QSL, in `quire-exact` as a
provenance pair (node key and occurrence) and in `quire-semantic-value` as the call-path pair this
crate's function application uses (the two crates' `Origin` and `Location` differ from each other;
`FieldDeclaration` is likewise defined in both and differs).

### Dependency edges this crate ends with

Cited from `quire-contract-ir` AD-007 (Repo-level edges, O-1) and re-measured here.

| Edge | Kind | Today | End |
| --- | --- | --- | --- |
| RT to `quire-exact` | normal, optional (`exact`), git `branch = "main"` | declared, named by no `src/` file | used; one lock entry |
| RT to `quire-semantic-value` | same | absent | added; one lock entry |
| RT to `quire-canonical` | transitive, through `quire-semantic-value` (git, own repository, adds `sha2`, `ryu-js`, `serde`, `thiserror`) | absent | `deny.toml` needs `allow-git` for that repository and licence exceptions for `quire-semantic-value`, `quire-canonical` and `quire-canonical-derive`; the new third-party crates must pass `make deny` (not measured) |
| RT to other QSL crates | none | 14 banned by name | banned by name, with `qsl-analyze` and `qsl-walk-grow` added (QSL has 18 workspace members; AD-007 names the same two) |
| footprint crate to either | none | none | none (FR-275-AC-10, extended to all three crates) |
| RT to IR, CG | none | none | none |

Indifference to O-1: the git URL in each of the two dependencies is the only line that changes if
QSL's two crates move to another repository. The 14-name ban list and the `allow-git` entry for the QSL
repository go with it; nothing else here depends on where the crates live.

## Decisions

- A. Every `exact` item has one owner (AD-007 decision A). This crate owns the items in What this crate
  keeps and none other.
- B. No re-export shim and no compatibility layer, at any step: a step that deletes an item changes its CG
  users in the same step.
- C. The module for the kept scalar items is `scalar` and for function application `expression` (if Q-1);
  the feature stays `exact`, because the feature is what pulls in the two crates and generated manifests
  already request it. The module names `text`, `composite` and `outcome` are not created: nothing is left to
  put in them.
- D. Dependency spelling is FR-275's, extended to `quire-semantic-value`: git, `branch = "main"`, optional,
  no `rev`, `tag` or `path`, no committed `[patch]`, one lock entry.
- E. Tests follow the code: a test of a deleted item is deleted with it; a test of a kept item stays; a
  matrix row whose evidence leaves keeps a planned status with the reason (FR-275-AC-15).

### Invariants a test can check

Local labels; the repository assigns requirement ids when one is authored.

- L-1. No file under `src/exact`, no `pub use` of `quire_exact` or `quire_semantic_value`, and no
  `pub` item named in the 124 shared names (inspection; a name list is a gate input, not a record).
- L-2. The only QSL crates in `Cargo.lock` are `quire-exact` and `quire-semantic-value`, once each (extends
  FR-275-AC-5 and the one-copy check); `make deny-mutations` bans each other QSL workspace crate.
- L-3. The footprint graph holds none of the three crates (extends the `make size` check).
- L-4. Every public item defined under the `exact` feature is in the keep table.
- L-5. CG's compile tests build every generator's output against this crate's head (AD-003 T-2).

## Risks

What is measured today, the migration that removes it, what is open and with whom, and what is routed.

### Current state and gaps

- The kernel is two implementations today, and `quire-exact` is declared but unused. The draft PR #95
  (slice 1, open) is the first move; see Migration.
- `#[non_exhaustive]` leaves with the kernel: this crate has 66 sites in `src/exact`, the two QSL crates 1.
  AD-003 states 72 sites for the whole crate. A kernel enum gaining a variant then breaks CG's
  `match` arms (compile error, not a silent default), and CG's `unreachable!` arms over `non_exhaustive`
  enums (IR-352) become dead patterns for the shared enums. Whether QSL intends the shared enums to
  stay exhaustive is Q-4.
- `Refusal` is no longer `Copy` and carries payloads, and `Outcome<bool>` is not `From<bool>`: the 14
  different-declaration names CG uses may need use-site edits in an emitter, not only path edits (not
  measured beyond the `CheckedInvariant` site).
- Kani. RT's `Integer`, `Value` and `ValueType` were laid out for CBMC (`verification/kani.rs`, and CG's
  generated Kani crates carry the field-sensitivity flag for the same reason). The shared `Integer` wraps
  `BigInt` and `Value` holds `Arc`. Whether the existing proof harness and CG's generated Kani harnesses
  still discharge is not measured; `make kani` and `make kani-mutations` decide it per step. PR #95 removes two
  mutation rows because their target file left.
- This crate's `make spec` baseline at `origin/main` on 2026-10-04: `quire validate` exits 0 (93 of 93
  documents grammar-clean, 0 findings); `quire coverage --strict` exits non-zero with 56 unbacked rows and 5
  contradicted statuses (IR-499). The same two counts are this change's pre-existing baseline.

### Migration

Each step is one coherent PR with its gate. Gate G for a step: `make fmt-check lint test-features doc
msrv size deny deny-mutations test kani kani-mutations` (`make ci` stops at `spec`, so run them as the
foundation slice did), plus `make spec` with no more than 56 unbacked and 5 contradicted. CG gate: CG's tests
build every generator's output against the RT head (T-2). A step that deletes an item has a paired CG
PR for the same item; with no shim the two merge back to back (their order is the team leaders'), the CG one
developed against the RT branch with `make use-local`.

| Step | RT change | CG, same step | Gate |
| --- | --- | --- | --- |
| 1 | Spec only: amend FR-275 (Description, step 2, AC-16 to AC-18), interface-001-AC-1, AC-2, AC-4, AC-5, AC-7 and AD-002 to this AD (no re-export; the residue list; the Q-1 answer) | none | `make spec` not above baseline |
| 2 | Add `quire-semantic-value`: `Cargo.toml`, `deny.toml` (`allow-git`, licence exceptions, ban list gains `qsl-analyze` and `qsl-walk-grow`), lock; no deletion | none | G; `make deny-mutations` bans each listed crate |
| 3 | Scalar, outcome, accounting, text: delete `integer`, `rational`, `decimal`, `comparison`, `numeric`, `text`, `accounting`, `outcome`; reduce `division`, `ieee` to the negotiators; connective to `scalar`; drop the four direct dependencies; adapt `Value` and the rest to the shared scalars | `rt::` scalar, outcome, meter, text paths to `quire_exact::`; manifest names the two crates | G |
| 4 | The `Value`-bearing set together: `composite`, `collection`, `equality`, `key`, `containment`, `reference`, `node`, `definition`, `quantity`, `unit`, `enumeration` | the different-declaration use sites, `NodeKey` constructor, the three serializable mirrors | G; Kani decision (Q-3) |
| 5 | Function application: vocabulary to `quire-semantic-value`; `ObjectEnvironment` to `ObjectClosure`; `expression` over the shared values (or deleted per Q-1) | `PackageDeclarations`, `CheckedPackage` emitters, `Origin` and `Location` mirrors | G |
| 6 | Close: `exact` module removed, `verification/kani.rs` imports, docs and `README`, matrix dispositions | none | G; L-1 to L-4 |

Why steps 3 and 4 are separate and 4 is one: the measured blockers are that `Value` holds `Quantity`,
`EnumValue`, `ObjectReference` and `NodeKey`, whose shared forms use `UnitId`, `EnumMember`, `UniverseId`
and `EffectiveId`, and that `ValueType::Reference` names a `NodeKey` where the shared one names an
`EffectiveId` (PR #95's own finding, re-read in both sources). A step cannot move part of that set without
a shim. IR-349 may split step 4 only along an edge where this crate still compiles with each half owned once.

Draft PR #95 (slice 1) against this layout, re-measured from its diff (47 files, +579, -10,056):

| In #95 | Matches this layout | Still needed |
| --- | --- | --- |
| deletes `integer`, `rational`, `comparison`, `numeric`, `decimal`, `text`, `accounting`, `outcome` whole and reduces `division` and `ieee` to the negotiators | yes: this is step 3 | rebase onto step 2; remove four direct dependencies |
| `mod.rs` ends with `pub use quire_exact::{..}` of 83 names at the `exact` path | no: this is the shim decision B refuses | delete the block; CG imports the owner paths in the same step |
| `src/exact/stop.rs` (56 lines): a private `Stop` and conversion trait | no: it is a copy of `quire_semantic_value::stop` (`Stop`, `outcome_from_stop`, `outcome_into_stop`, same three variants) | consume `quire-semantic-value` (step 2) |
| `src/exact/boolean.rs` keeps the lazy connective | yes (kept, in `scalar`) | move to `scalar` |
| dev-dependency on `quire-exact` with `test-support` | yes, for tests of kept code only | keep only if a kept test needs the charge log |
| no CG change | no | paired CG PR |
| spec edits (FR-275, interface-001, matrix) | partly | re-state against step 1 |
| `Origin` and `Location`: left open | answered here: QSL defines two pairs (Routed gaps) | wait for QSL |

### Open questions

| Question | Owner | Recommendation | Cost of the alternative |
| --- | --- | --- | --- |
| Q-1. Who owns the function-application call mechanism (`Body`, `Frame`, `CheckedPackage::call`, `plan_call`, 15 items)? FR-275-AC-16 to AC-18 and interface-001-AC-7 say this crate defines none; QSL's classification (relayed, untrusted) says it is runtime-owned; measured: neither shared crate has it | owner | this crate keeps it (`expression`), amending those criteria in step 1: bodies are host closures a generated oracle supplies (FR-273), which no QSL crate may own below layer 5 | deleting it leaves CG's function oracles with no runtime call surface until QSL builds one |
| Q-2. Confirm no `exact` re-export path (AD-016 owner decision 2 as stated in the IR-345 assignment) against FR-275's current text | owner | confirm; amend FR-275 and interface-001 in step 1 | a re-export is the shim this AD excludes |
| Q-3. Kani: do kernel proofs live in QSL, and are RT's two removed mutation rows retired? | owner | RT proves and mutates only code it owns; kernel proofs are QSL's | RT keeps harnesses over code it no longer holds |
| Q-4. Do the shared enums stay without `#[non_exhaustive]`? | QSL | state the policy; CG matches exhaustively either way | silent breakage on a new variant |
| Q-5. Where does `NodeKey::from_hex` belong? | QSL | QSL adds it to `quire-exact`, or CG decodes hex and calls `decode_admitted` | a decoder copied into CG |
| Q-6. Where do `quire-exact` and `quire-semantic-value` live (O-1 of AD-007)? | owner | not this crate's: no change to this layout | none here |

### Routed gaps

To QSL (stated needs, not requests to copy): R-1 one `Origin`/`Location`/`FieldDeclaration` per name across
`quire-exact` and `quire-semantic-value`; R-2 a call-depth field in `CheckingLimits` or the statement that
the depth guard is the host's; R-3 the `PackageCause` variants, or their retirement; R-4 the injected denial
single-shot and non-zero (FR-010-AC-5, AC-6) and a bounded charge log, or the statement that the kernel's
behaviour is the specification; R-5 Q-4 and Q-5.

To CG: the paired PR for each of steps 3 to 5 (the table above); mirrors of `DeclarationCause`, `Origin` and
`Location` follow the shared definitions (AD-003 R3-C8).
