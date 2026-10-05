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
`quire-contract-ir`: the owner answered, both are extracted into their own repositories, IR-346 and
IR-582; this crate follows, see the end state).

## Views

The layout is described as it is, as the shared names classify, as it ends, and as the dependency
edges and the consumer's paths follow from that.

### Current layout

Measured: 34 files and 13,791 lines under `src/`; `exact/` is 23 files and 12,080 lines. Outside
`src/`: `verification/kani.rs` 295, `measurement/footprint` 124 (`wc -l` over its `.rs` files; 137 with the manifest), `tests/` 16 files and 9,591 lines
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
| expression | `expression` 978 | function application (AD-002, FR-273) | vocabulary (about 215 lines) to `quire-semantic-value` (`checking`, `call`, `loss`, `location`); the call mechanism (about 760 lines) is a port by its own header and has no home in a crate this one may depend on: Q-1 |
| outcome | `outcome` 222, `accounting` 685 | `Outcome`, `Undefined`, `Refusal`, `Meter`, charge points, limits | `quire-exact` (`outcome`, `accounting`) |
| (root) | `mod.rs` 159 | the `pub use` list of every item above | deleted |

The five line sums by target are 5,845 (scalar), 728 (text), 3,463 (composite), 978 (expression)
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
| Same fields, boxed in RT | 7 | delete; representation only | `Decimal`, `DecimalType`, `Rational`, `RationalDomain`, `Text`, `EnumValue`, `CompoundUnit` |
| Same value, different representation | 1 | delete; layout only | `Integer` (RT `{small: i64, big: Option<Box<BigInt>>}`, QSL `(BigInt)`) |
| Same name, different declaration | 22 | delete after the adapter in the step | `Meter`, `ChargePoint`, `InjectedDenial`, `Refusal`, `Undefined`, `Value`, `ValueType`, `Quantity`, `QuantityOperation`, `ObjectReference`, `ObjectTypeDeclaration`, `FieldDeclaration`, `TypeEnvironment`, `CheckedEquality`, `DeclarationCause`, `CollectionType`, `UnitGraph`, `CheckingLimits`, `InputRefusal`, `PackageCause`, `Origin`, `Location` |
| RT-only | 27 types, 7 free items and the alias `Body` | keep (the 11 `scalar` items), delete when the evaluation leaf lands (the `expression` items, Q-1), or delete with a successor (below) | see What this crate keeps |

The six read in full:

| Name | Finding |
| --- | --- |
| `Integer` | same public methods (`zero`, `one`, `is_zero`, `is_negative`, `magnitude_bits`, `decimal_digits`, `to_u64`, `is_even`), `From<i64>`, `From<i128>`, `From<u64>`, `FromStr`, `Display`, and the same `Debug` text. RT holds an inline `i64` that promotes to a boxed `BigInt` (laid out for CBMC, `verification/kani.rs`); QSL wraps a `BigInt`. QSL adds `From<usize>` and makes the arithmetic methods public. Same semantics, different layout |
| `Decimal` | same constructors and accessors, same `evaluate_decimal` signature; RT boxes its two fields. QSL adds the typestate `Placement`, `Placed`, `Admitted` and drops `#[non_exhaustive]` on `RoundingMode` and `DecimalOperation`. Same semantics, different layout |
| `Outcome` | the enum is the same four variants; RT adds `#[non_exhaustive]`, `#[repr(u64)]` and `From<T>`. The payloads differ: `Undefined` gains `SumOutOfDomain` and loses `Copy`; `Refusal` is a `Copy` enum in RT whose only fields are `would_be`, the two admitted flags and the cardinality record, and carries its targets in QSL (`IntegerOutOfDomain { target }`, `ForeignReference { required, supplied }`, and so on) with different `code()` and `cause()` results. Different |
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
| `expression` (`exact`) | `expression` | none: the module does not exist in the end state. The 15 current items (`Body`, `Frame`, `FunctionDeclaration`, `PackageDeclarations`, `CheckedPackage`, `CheckedExpression`, `CheckRefusal`, `CheckCause`, `CallPlan`, `plan_call`, `plan_evaluation`, `Evaluation`, `EvaluationRefusal`, `DepthAboveMaximum`, `MAX_CALL_DEPTH`; FR-273) are a port and are replaced by the evaluation leaf crate QSL provides (Q-1, IR-583) | the whole file once IR-583 lands (step 5); the vocabulary (`CheckMode`, `CheckingLimits`, `Origin`, `Location`, `InputRefusal`, `ValueLoss`, `LocatedLoss`) is already in `quire-semantic-value` |

### What this crate keeps, and what it deletes

| Kept | Why it is not a copy |
| --- | --- |
| core, accounting, snapshot, adapter, `verification/kani.rs`, footprint crate | not QSL code |
| the 11 `scalar` items | absent from both shared crates (measured: no `negotiate_*` and no lazy-right connective in either); QSL deleted its negotiators on purpose (FR-275, as relayed by QSL) |

Not kept by this AD: the 15 `expression` items. Absence from both shared crates does not make them
runtime-owned. `src/exact/mod.rs` calls the module "a `no_std + alloc` port ... every type, field and variant
keeps the authority's name and order" and `expression.rs` says it ports the authority's call surface;
FR-275-AC-13 defines a port as code that keeps the QSL authority's item names, FR-275's residue list names
function application as vendored code with no exception, no expiry and no approval, and AD-003
decision F says the same. QSL defines the same names in `qsl-package`, `qsl-semantics` and `qsl-eval`,
which this crate may not depend on. Their end is the evaluation leaf crate QSL provides in its own
repository (Q-1, IR-583): this crate depends on it, deletes the items and keeps no second version.

Interim. What this crate does with these items before IR-583 lands is not ruled by the owner: FR-275's
Open questions record it as open ("how the runtime handles its residue copy until QSL-358 lands (open,
owner decision) ... no interim policy is stated"), and the owner's Q-1 answer (option (a): QSL provides
evaluation, this crate depends on it and deletes its copy) says nothing about interim handling or early
deletion. The rest of this paragraph is this AD's inference from that answer, not a ruling. Inference:
the items stay in the tree until IR-583 lands, only as the defect FR-275 already records (AC-13 unmet,
residue list non-empty); no step adds behaviour to them; steps 3 and 4 only repoint what they must to keep
the crate compiling; no consumer is given a new use of them; they are deleted in step 5, in the PR that
adds the dependency on the leaf. This AD grants no exception, sets no expiry, and asks for none.
Alternative, if the owner rules otherwise: delete them early (in step 3 or 4), at the cost that CG's
function oracles lose their runtime call surface until IR-583 lands.

Deleted outright: everything else in `src/exact`, `mod.rs` included (`CHARGE_LOG_CAPACITY` and
`charge_log_truncated` have no counterpart and go with `Meter`; R-4), with its tests (`src/exact_*_tests.rs` and the
kernel halves of `tests/exact_*.rs`). No test comparing this crate's output with a second
implementation is kept (FR-275). RT-only names with a successor in the shared crates under another
name, each with a changed representation (`UniverseIdentity` is a `Box<[u8]>`, `UniverseId` a 32-byte
digest; `ObjectIdentity` is bytes, `ObjectId` a string): `UniverseIdentity` to `UniverseId`, `ObjectIdentity` to `ObjectId`, `ObjectEnvironment`,
`ObjectEnvironmentCause`, `ObjectEnvironmentRefusal` to `ObjectClosure`, `ObjectClosureCause`,
`ObjectClosureRefusal`. `InvalidObjectIdentity`, `InvalidTextLiteral` and `UnitDeclaration` have no same-named
item in either crate and were not traced further: the step that meets them finds their successor or
records the gap.

### The end state of the shared kernel (AD-016 owner decision 2)

- This crate depends on `quire-exact` and `quire-semantic-value`, as the extracted leaf repositories
  (Q-6), and on the evaluation leaf crate QSL provides in its own repository (Q-1, IR-583) for the
  former `expression` items, once each, optional, enabled only by `exact`. It contains no second
  implementation of any item they define, and its `expression` bucket is deleted once IR-583 lands:
  no keep, no exception, no copy without an expiry.
- No re-export shim: there is no `quire_contract_runtime::exact` module, no `pub use` of any of the
  three shared crates (`quire-exact`, `quire-semantic-value`, the evaluation leaf), no alias, wrapper,
  feature or legacy reader. Consumers import the owner crates' own paths (`quire_exact::`,
  `quire_semantic_value::`, the leaf's). A kept item that must name a shared type names it in its own
  signature, which is a dependency, not a re-export.
- Merged statements this AD contradicts, in three groups. Each statement is amended, relied on, or
  amended conditionally, as stated; none is left to a step that does not name it.
- Group 1, the no-shim and kernel statements (Q-2, decided; hold today; amended in step 1):
  FR-275 Description ("Its `exact` module is a re-export of those QSL-owned crates at the
  `quire_contract_runtime::exact` path"), Outputs ("a runtime `exact` module that defines only the
  runtime-owned negotiators and otherwise only re-exports QSL-owned crates"), Behavior "The residue"
  ("keeps the `exact` path") and "The end state" (`exact` "re-exports `quire-exact` and
  `quire-semantic-value`"), AC-1 ("the runtime's `exact` module defines no public item that
  `quire-exact` exports": vacuous without the module, and the criterion that stops a kept module
  redefining kernel items, so it is restated over the kept modules), AC-16 and AC-17 ("The runtime's
  `exact` module defines no item other than the backend negotiators", which move to `scalar`);
  interface-001 prose of the exact kernel
  surface ("That path is the path the runtime guarantees stays available", "it keeps its `exact`
  path"), its contract yaml (`module: quire_contract_runtime::exact`, `runtime_role: re-export
  unchanged at the same exact path`, the `runtime_owned` and `outside_exact` blocks, and the
  `dependencies` line "generated oracles -> quire_contract_runtime::exact"), and interface-001-AC-1,
  AC-2, AC-3, AC-4, AC-5 and AC-6, which name the `exact` module and become vacuous without it; AC-10
  (names the `exact` module and the `outside_exact` block).
- Group 2, the statements naming the shared crates' repositories and the bans (Q-6, decided; they
  hold today and break only when the dependency moves, so they are amended in step 2, in the PR that
  needs IR-582 and changes the dependency): FR-275 Dependency spelling, "One copy" (`deny.toml` admits
  the QSL repository as the one git source), Guarded edges (the 14 name bans), FR-275-AC-4, AC-7 and AC-8, the
  Open question "Upkeep of the ban list" (moot once the bans go); interface-001 AC-11 and AC-14 and the
  `dependencies` line "runtime -> quire-exact" alone (which gain `quire-semantic-value`); and the
  checks that enforce the bans: `scripts/check_deny_bans.sh` (`make deny-mutations`), TC-198, and the
  `CLAUDE.md` description of `deny.toml`.
- Group 3, the evaluation-item statements (Q-1, decided as option (a); amended conditionally): FR-273
  and AD-002 (they describe the ported surface), the function-application clause of FR-275's step 2 (it
  says QSL-358 places function application in `quire-semantic-value`; under Q-1 it goes to the IR-583
  leaf; the rest of that step, already delivered, is not contradicted), the residue list and the interface-001 `residue`
  block and AC-9 (their deletion tracker names QSL-358, and the list stays non-empty until step 5).
  What they are amended to depends on the surface IR-583 states. IR-583 has not yet stated it, and this AD
  does not decide its size or shape; these amendments wait on that statement and land with or before step 5.
- Merged statements this AD relies on and must not weaken: FR-275-AC-13 (no port of QSL code, a port
  being code that keeps the authority's item names), the owner ruling the residue list quotes, AC-18
  (the residue list is empty), AD-003 decision F, and interface-001-AC-7 (the runtime defines none
  of the call-surface items), which option (a) already satisfies and which needs no change unless the
  leaf's surface turns out to reuse one of its names.
- Direct dependencies `num-bigint`, `num-integer`, `num-traits` and `unicode-normalization` are
  named today only by `integer.rs`, `rational.rs`, `ieee.rs` and `text.rs`, none of which keeps code that
  names them (the negotiators do not); they are removed in the step that deletes those files.

### CG paths that change in the same step

Measured at CG `origin/main`. CG's emitted manifest carries no pinned revision; it names this
crate by git URL and branch (`src/core/profile.rs`).

| CG path | Names `quire_contract_runtime::exact` as | Count |
| --- | --- | --- |
| `src/oracle/scalar/mod.rs` | the generator's `use quire_contract_runtime::exact::{..}` and the emitted `use quire_contract_runtime::exact as rt;` | 105 `rt::` refs, 47 distinct |
| `src/oracle/equality/mod.rs` | the same import and emitted alias; serializable mirrors of `IllTypedCause`, `RecursionEdges`, `DeclarationCause` | 112 refs, 31 distinct |
| `src/oracle/function/mod.rs` | the same two aliases; mirrors of `Origin`, `Location` | 90 refs, 20 distinct |
| `src/kani/generate/scalar.rs` | `rt::Integer` in the generated Kani harness text (its doc comments also name the path) | 9 refs, 4 distinct |
| `src/core/profile.rs` | `oracle_crate_manifest`, the emitted `Cargo.toml`: one `quire-contract-runtime` dependency, `features = ["exact"]` and the Kani metadata | the manifest must also name `quire-exact` and `quire-semantic-value` |
| CG `Cargo.toml` | CG's own dependency, features `exact` and `proptest` | no change except new edges |

The 79 distinct names those emitters use from `exact` split as: 16 same-declaration (path change only);
23 same except `#[non_exhaustive]` (path change, and every wildcard or `unreachable!` arm over them
becomes a dead pattern); 12 free functions (path change, except `convert_quantity`, whose first parameter
changed from `&Quantity` to `UnitQuantity<'_>`; the other eleven signatures were not compared); 8 with a
changed layout only (`Integer` and the seven boxed names; path change); 14 different-declaration
(`Meter`, `Refusal`, `Value`, `ValueType`, `Quantity`, `QuantityOperation`, `TypeEnvironment`,
`CheckedEquality`, `CheckingLimits`, `CollectionType`, `DeclarationCause`, `FieldDeclaration`,
`InputRefusal`, `ObjectTypeDeclaration`); and 6 RT-only (`PackageDeclarations`, `CheckedPackage`,
`CheckRefusal`, `Frame`, `FunctionDeclaration`, `ObjectEnvironment`: the evaluation leaf, Q-1). The 14 change more than the
path. The shared `ValueType` has `Enum(EnumShape)`, `Float(FloatType)`, `Quantity(UnitId)`,
`Reference(EffectiveId)` and a new `Population(Option<u64>)`; the shared `Value` has `Enum(EnumMember)` and a
new `Population(PopulationId)`; CG's exhaustive renderers of both meet all of these. Other use sites of the
14 were not read; one is known: emitted code builds `Refusal::CheckedInvariant`, a unit variant the shared
enum keeps.
Two uses have no counterpart in the shared crates: the generator decodes hex with `NodeKey::from_hex` in
`oracle/equality` and emits `rt::NodeKey::from_bytes([..])` literals. CG uses neither
the charge log, `charge_log_truncated` nor `CHARGE_LOG_CAPACITY` (grep). The harness and verdict paths
(`quire_contract_runtime::{Verdict, CampaignReport, ...}` in `src/strategy/` and
`src/oracle/boolean_v1.rs`) name the core, not `exact`, and do not change.

### QSL pieces (verified; the relayed notice on IR-349 is untrusted text)

| Piece | Verified at QSL `origin/main` | Result |
| --- | --- | --- |
| H3, `NodeKey::decode_admitted` (QSL-479, #590) | `quire-exact/src/node.rs` defines it; PR state MERGED | present, but not a rename of `from_bytes`: it ships with QSL's T12-F call-site allow-list and the precondition that the bytes come from a package that passed the admitted-package identity check, and the type itself accepts any 32 bytes. `from_digest` is check-only (T12-B). RT library code calls no `NodeKey` constructor today (every call is in a test); CG's emitted literals and its hex-decode path are covered by neither the allow-list nor, shown here, the precondition: Q-5 |
| H2, `ObjectClosure` (QSL-478, #608) | `quire-semantic-value/src/object_closure.rs`; PR state MERGED | present; signatures differ from RT's `ObjectEnvironment` (above) |
| QSL-358 residue slices | `quire-semantic-value` holds `stop`, `quantity`, `unit`, `declaration`, `containment`, `enumeration`, `definition`, `semantic_node`, `checking`, `call`, `loss`, `location`, `object_closure` | present; QSL-358 is Done |
| FR-275 step 2 (the residue) | the 15 `expression` items and the lazy connective | not delivered, although interface-001 and FR-275 expect QSL-358 to move function application: no QSL crate this crate may depend on defines it. QSL-358 is Done. The lazy connective is absent from both crates |

Still missing for this crate, each stated to QSL below: the evaluation leaf crate (Q-1, IR-583); `CheckingLimits`
carries no call-depth field (RT's `depth`, which `DepthAboveMaximum` and `MAX_CALL_DEPTH` serve);
`NodeKey::from_hex`; `PackageCause` lacks three variants RT has (`RevisionMismatch`,
`DigestDomainMismatch`, `ByteDigestMismatch`); the injected denial is not single-shot and `occurrence`
admits zero (FR-010-AC-5, AC-6); `Origin` and `Location` are defined twice by QSL, in `quire-exact` as a
provenance pair (node key and occurrence) and in `quire-semantic-value` as the call-path pair this
crate's function application uses (the two crates' `Origin` and `Location` differ from each other;
`FieldDeclaration` is likewise defined in both and differs).

### Dependency edges this crate ends with

Cited from `quire-contract-ir` AD-007 (Repo-level edges, O-1) and re-measured here. The owner answered
O-1 (Q-6): `quire-exact` and `quire-semantic-value` are extracted into their own repositories (IR-582,
QSL lane), so the end state names those repositories, not the QSL repository.

| Edge | Kind | Today | End |
| --- | --- | --- | --- |
| RT to `quire-exact` | normal, optional (`exact`), git `branch = "main"` on its own repository | declared on the `agent-ix/quire-exact` repository, named by no `src/` file | used; one lock entry |
| RT to `quire-semantic-value` | same, own repository | absent | added; one lock entry |
| RT to the evaluation leaf crate | same, own repository (Q-1, IR-583) | absent | added in step 5 with the deletion of the `expression` items; one lock entry |
| RT to `quire-canonical` | transitive, through `quire-semantic-value` (git, own repository, adds `sha2`, `ryu-js`, `serde`, `thiserror`) | absent | `deny.toml` needs `allow-git` for that repository and licence exceptions for `quire-semantic-value`, `quire-canonical`, `quire-canonical-derive` and the evaluation leaf; the new third-party crates must pass `make deny` (not measured) |
| RT to the QSL repository | none | `quire-exact` only, 14 crates banned by name | none: the `allow-git` entry for it and the 14 name bans are removed in step 2, and `unknown-git = "deny"` is then the guard |
| footprint crate to any of them | none | none | none (FR-275-AC-10, extended to every shared crate) |
| RT to IR, CG | none | none | none |

## Decisions

- A. Every `exact` item has one owner (AD-007 decision A). This crate owns the items in What this crate
  keeps and none other.
- B. No re-export shim and no compatibility layer, at any step: a step that deletes an item changes its CG
  users in the same step.
- C. The module for the kept scalar items is `scalar`; there is no `expression` module (Q-1);
  the feature stays `exact`, because the feature is what pulls in the shared crates and generated manifests
  already request it. The module names `text`, `composite` and `outcome` are not created: nothing is left to
  put in them.
- D. Dependency spelling is FR-275's, applied to each of the three shared crates and naming that crate's own
  repository (FR-275's text names the QSL repository; step 2 amends it): git, `branch = "main"`, optional,
  no `rev`, `tag` or `path`, no committed `[patch]`, one lock entry.
- E. Tests follow the code: a test of a deleted item is deleted with it; a test of a kept item stays; a
  matrix row whose evidence leaves keeps a planned status with the reason (FR-275-AC-15).

### Invariants a test can check

Local labels; the repository assigns requirement ids when one is authored.

- L-1. No file under `src/exact`, no `pub use` of `quire_exact`, `quire_semantic_value` or the evaluation leaf, and no
  `pub` item named in the 124 shared names (inspection; a name list is a gate input, not a record).
- L-2. `quire-exact`, `quire-semantic-value` and the evaluation leaf each appear once in `Cargo.lock`
  (extends FR-275-AC-5), and no crate of the QSL repository appears (`unknown-git = "deny"` with no
  `allow-git` for it).
- L-3. The footprint graph holds none of the three crates (extends the `make size` check).
- L-4. Every public item defined under the `exact` feature is in the keep table.
- L-5. CG's compile tests build every generator's output against this crate's head (AD-003 T-2).

## Risks

What is measured today, the migration that removes it, what is open and with whom, and what is routed.

### Current state and gaps

- The kernel is two implementations today, and `quire-exact` is declared but unused. The draft PR #95
  (slice 1, open) is the first move; see Migration.
- `#[non_exhaustive]` leaves with the kernel: this crate has 66 `#[non_exhaustive]` sites in `src/exact`; the two QSL crates have no such
  attribute (their one textual hit is a `finish_non_exhaustive()` call).
  AD-003 states 72 sites for the whole crate. A kernel enum gaining a variant then breaks CG's
  `match` arms (compile error, not a silent default), and CG's `unreachable!` arms over `non_exhaustive`
  enums (IR-352) become dead patterns for the shared enums. The owner answered Q-4: follow the shared
  crate, so this crate drops its attributes and its matches are exhaustive (step 3).
- `Refusal` is no longer `Copy` and carries payloads, and `Outcome<bool>` is not `From<bool>`: the 14
  different-declaration names CG uses may need use-site edits in an emitter, not only path edits (not
  measured beyond the `CheckedInvariant` site).
- Kani. RT's `Integer`, `Value` and `ValueType` were laid out for CBMC (`verification/kani.rs`, and CG's
  generated Kani crates carry the field-sensitivity flag for the same reason). The shared `Integer` wraps
  `BigInt` and `Value` holds `Arc`. Whether the existing proof harness and CG's generated Kani harnesses
  still discharge is not measured; `make kani` and `make kani-mutations` decide it per step. The owner
  answered Q-3: the proofs run against the shared `Integer`, with no RT-local shadow type.
  PR #95 removes two mutation rows because their target file left.
- This crate's `make spec` baseline at `origin/main` on 2026-10-04: `quire validate` exits 0 (93 of 93
  documents grammar-clean, 0 findings); `quire coverage --strict` exits non-zero with 56 unbacked rows and 5
  contradicted statuses (IR-499). The same two counts are this change's pre-existing baseline.

### Migration

Each step is one coherent PR with its gate. Gate G for a step: `make fmt-check lint test-features doc
msrv size deny deny-mutations test kani kani-mutations` (`make ci` stops at `spec`, so run them as the
foundation slice did), plus `make spec` with no more than 56 unbacked and 5 contradicted. CG gate: CG's tests
build every generator's output against the RT head (T-2). A step that deletes an item has a paired CG
PR for the same item, and the order is forced, not chosen. CG builds `--locked` against this crate's
`branch = "main"` unless a local patch is active (its Makefile drops `--locked` when `.cargo/config.toml`
exists), so a CG PR cannot lock to an RT commit that is not yet on RT main. The RT PR therefore merges
first; CG main stays on its old lock and stays green; the paired CG PR is developed and gated against the
RT branch with `make use-local` (patch active, no `--locked`), then lands after the RT merge with its lock
bumped. No shim is involved: CG never consumes a half-migrated RT main, because its lock only moves in the
CG PR that adopts the new paths.

| Step | RT change | CG, same step | Gate |
| --- | --- | --- | --- |
| 1 | Spec only; needs no ticket. Amend Group 1 only, the statements that hold today: FR-275 (Description, Outputs, Behavior "The residue" and "The end state", AC-1, AC-16, AC-17) and interface-001 (prose, contract yaml `module`, `runtime_role`, `runtime_owned`, `outside_exact` and the `exact` dependency line, AC-1 to AC-6, AC-10) to no re-export (Q-2). The interim-copy Open question stays open (Interim). AC-18 and AC-13 stay. Groups 2 and 3 are not touched here | none | `make spec` not above baseline |
| 2 | Needs IR-582 (the repositories must exist). In one PR: add `quire-semantic-value` and re-point `quire-exact` at the extracted repositories (`Cargo.toml`, lock); `deny.toml` (`allow-git` for the extracted repositories and `quire-canonical`, licence exceptions; the QSL-repository `allow-git` and the 14 name bans removed); amend Group 2 (FR-275 Dependency spelling, "One copy", Guarded edges, AC-4, AC-7, AC-8, the ban-list Open question; interface-001 AC-11, AC-14 and the `quire-exact` dependency line); rewrite `scripts/check_deny_bans.sh` so `make deny-mutations` asserts cargo-deny's sources rejection for a crate added from the QSL repository, where it now asserts `error[banned]`; change TC-198 and the `CLAUDE.md` `deny.toml` description to match; no deletion | none | G, with `deny-mutations` green on the rewritten script; the script proves a git source outside the allow-list is rejected |
| 3 | Scalar, outcome, accounting, text: delete `integer`, `rational`, `decimal`, `comparison`, `numeric`, `text`, `accounting`, `outcome`; reduce `division`, `ieee` to the negotiators; connective to `scalar`; drop the four direct dependencies; adapt `Value` and the rest to the shared scalars; the object-reference universe changes from `UniverseIdentity` to `UniverseId`, because the shared `Refusal::ForeignReference` carries `UniverseId` values and the equality code builds it from an `ObjectReference`; `verification/kani.rs` and `scripts/check_kani_mutations.py` repoint, and the proofs run against the shared `Integer` (Q-3), with no RT-local shadow type; every `#[non_exhaustive]` on the kept `scalar` enums is dropped and matches are exhaustive (Q-4), and interface-001's `compatibility.enums: non-exhaustive` statement is amended in this step to match | `rt::` scalar, outcome, meter, text paths to `quire_exact::`; manifest names the two crates; the Kani generator's `rt::Integer` and `rt::Outcome`; CG's wildcard and `unreachable!` arms over the shared enums removed | G; needs step 2, so IR-582; does not need IR-583 |
| 4 | The `Value`-bearing set together: `composite`, `collection`, `equality`, `key`, `containment`, `reference`, `node`, `definition`, `quantity`, `unit`, `enumeration` | the different-declaration use sites, `NodeKey` construction (Q-5), the three serializable mirrors | G; Q-5 answered |
| 5 | Needs IR-583 stating the leaf's surface, and IR-582 through step 2. Function application: add the dependency on the evaluation leaf (`Cargo.toml`, lock, and a `deny.toml` `allow-git` entry and licence exception for the leaf's repository), delete the 15 ported `expression` items and the `expression` file (vocabulary is already in `quire-semantic-value`); `ObjectEnvironment` to `ObjectClosure`; amend Group 3, spec first, as the stated surface requires | the `PackageDeclarations`, `CheckedPackage` emitters repointed to the leaf, `Origin` and `Location` mirrors, and the emitted manifest names the leaf beside `quire-exact` and `quire-semantic-value` | G; needs IR-583 and IR-582 |
| 6 | Close: `exact` module removed, `verification/kani.rs` imports, docs and `README`, matrix dispositions | none | G; L-1 to L-4 |

Why step 4 is one step: `Value` holds `Quantity`, `EnumValue`, `ObjectReference` and `NodeKey`, whose
shared forms use `UnitId`, `EnumMember`, `UniverseId` and `EffectiveId`, and `ValueType::Reference` names a
`NodeKey` where the shared one names an `EffectiveId` (PR #95's own finding, re-read in both sources).
Step 3 is not clean of that set: the shared `Refusal::ForeignReference` is `{ required, supplied:
UniverseId }`, so the one reference-type change above (the universe, and no more) belongs to step 3, as
PR #95 did. What stays in step 4 is the rest: `Quantity`, `EnumValue`, `NodeKey`, object identity and the
`EffectiveId` registry change. IR-349 may split step 4 only along an edge where this crate still compiles
with each half owned once.

Draft PR #95 (slice 1) against this layout, re-measured from its diff (47 files, +579, -10,056):

| In #95 | Matches this layout | Still needed |
| --- | --- | --- |
| deletes `integer`, `rational`, `comparison`, `numeric`, `decimal`, `text`, `accounting`, `outcome` whole and reduces `division` and `ieee` to the negotiators | yes: this is step 3 | rebase onto step 2; remove four direct dependencies |
| `mod.rs` ends with `pub use quire_exact::{..}` of 83 names at the `exact` path | no: this is the shim decision B refuses | delete the block; CG imports the owner paths in the same step |
| `src/exact/stop.rs` (56 lines): a private `Stop` and conversion trait | no: it is a copy of `quire_semantic_value::stop` (`Stop`, `outcome_from_stop`, `outcome_into_stop`, same three variants) | consume `quire-semantic-value` (step 2) |
| `src/exact/reference.rs`: `ObjectReference` universe changed to `UniverseId`, `UniverseIdentity` deleted | yes: this is the reference-type change step 3 needs | keep it in step 3 and nothing more of the reference set |
| `src/exact/boolean.rs` keeps the lazy connective | yes (kept, in `scalar`) | move to `scalar` |
| dev-dependency on `quire-exact` with `test-support` | yes, for tests of kept code only | keep only if a kept test needs the charge log |
| no CG change | no | paired CG PR |
| spec edits (FR-275, interface-001, matrix) | partly | re-state against step 1 |
| `Origin` and `Location`: left open | answered here: QSL defines two pairs (Routed gaps) | wait for QSL |

### Owner answers and open questions

Q-1 to Q-4 and Q-6 are answered. The answers are the owner's, given in the IR planner's session on
2026-10-04 and recorded verbatim by the planner on IR-349 (Q-1 to Q-4), IR-346 and IR-582 (Q-6), with the
work for Q-1 on IR-583; this AD records them as recorded there and measures nothing in them. Q-5 stays open,
routed to QSL by this lane, with no owner answer recorded.

| Question | Owner | Answer (source) | Consequence in this AD |
| --- | --- | --- | --- |
| Q-1. What replaces the ported function-application call mechanism (the 15 `expression` items)? | owner | decided, option (a): QSL provides evaluation in a `no_std` plus `alloc` leaf crate in its own repository (IR-349 answer; work IR-583); this crate depends on it and deletes its ported copy, with no second version. IR-349 is blocked by IR-583 for these items | the `expression` bucket is deleted when IR-583 lands (step 5); no keep, no exception, no copy without an expiry; Interim above; step 5 and the Group 3 amendments need IR-583, step 1 does not |
| Q-2. No `exact` re-export path | owner | decided: confirmed (IR-349 answer): step 1 amends FR-275 and interface-001, and CG changes in the same step | Decision B stands; the Group 1 amendments under the end state are decided and land in step 1 |
| Q-3. Kani against the shared `Integer` | owner | decided: prove against the shared type (IR-349 answer) | the proofs run against the shared `Integer`, with no RT-local shadow type; stated in step 3 |
| Q-4. `#[non_exhaustive]` on shared enums | owner | decided: follow the shared crate, none (IR-349 answer): this crate drops its 66 attributes in `src/exact` and matches are exhaustive | stated in step 3; the five kept `scalar` enums that carry it drop it; the 6 sites outside `exact` are not part of this answer |
| Q-5. May a consumer crate construct a `NodeKey` at all, and under what precondition? `decode_admitted` is for bytes of an admitted package under QSL's allow-list; CG emits literal keys and decodes hex (`from_hex`, absent from QSL), which neither the allow-list nor, shown here, the precondition covers. RT library code constructs none | QSL | open, routed to QSL by this lane; no owner answer is recorded (the IR-349 answer covers Q-1 to Q-4 only): QSL states which constructor a generator or an emitted oracle may use and where hex decoding lives | the CG paths that build keys stay unresolved in step 4 until QSL answers |
| Q-6. Where do `quire-exact` and `quire-semantic-value` live (O-1 of AD-007)? | owner | decided: extract both into their own repositories (IR-346 and IR-582; work IR-582, QSL lane); IR-349 is blocked by IR-582 | the end state depends on the extracted repositories; step 2, with the Group 2 amendments, needs IR-582 |

### Routed gaps

To QSL (stated needs, not requests to copy): R-1 one `Origin`/`Location`/`FieldDeclaration` per name across
`quire-exact` and `quire-semantic-value`; R-2 a call-depth field in `CheckingLimits` or the statement that
the depth guard is the host's; R-3 the `PackageCause` variants, or their retirement; R-4 the injected denial
single-shot and non-zero (FR-010-AC-5, AC-6) and a bounded charge log, or the statement that the kernel's
behaviour is the specification; R-5 Q-4 and Q-5; R-6 whether QSL-358's function-application move (expected by FR-275 and interface-001) is still planned (answered by Q-1: the evaluation leaf, IR-583).

To CG: the paired PR for each of steps 3 to 5 (the table above); mirrors of `DeclarationCause`, `Origin` and
`Location` follow the shared definitions (AD-003 R3-C8).
