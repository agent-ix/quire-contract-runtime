---
id: interface-001
title: "Generated-oracle runtime API"
type: interface
---
# [interface-001] Generated-oracle runtime API

## Contract

```yaml
name: GeneratedOracleRuntime
version: quire-contract-runtime-v1
ownership: quire-contract-runtime
features:
  default: []
  alloc: []
  std: [alloc]
  proptest: [std]
  snapshot-json: [alloc]
  exact: [alloc]
associated_types:
  - ContractIdentity
  - ExecutionPoint
  - Observation
  - FailureDetail
  - Verdict
  - CampaignReport
  - CampaignSnapshot
  - DecodedCampaignSnapshot (snapshot-json only)
operations:
  - name: construct_verdict
    inputs: [requirement identity, revision identity, execution point, clause observations]
    output: Passed | FailedPostcondition | RejectedPrecondition
    semantics: exactly one terminal category; no Boolean conversion is provided
  - name: inspect_verdict
    inputs: [Verdict]
    output: terminal kind, common provenance, optional structured terminal detail
    semantics: rejection and failure remain distinguishable from success
  - name: evaluate_short_circuit_boolean
    inputs: [left value, lazy right value]
    output: Boolean
    semantics: skip the right operand exactly when the named Boolean operator permits it
  - name: evaluate_total_boolean
    inputs: [lazy left value, lazy right value]
    output: Boolean
    semantics: evaluate left then right exactly once regardless of the left value
  - name: evaluate_checked_value
    inputs: [option, slice/index, or supported integer operands]
    output: defined value | None
    semantics: return None for absence, invalid index, overflow, zero division, or signed division overflow
  - name: record_campaign_verdict
    inputs: [per-requirement report, Verdict]
    output: updated complete counters or typed identity mismatch
    semantics: refuse identity mismatch; saturate accepted, rejected, failed, and discarded counts
  - name: adapt_to_proptest
    feature: proptest
    inputs: [Verdict]
    output: proptest TestCaseResult
    semantics: map pass to success, failure to Fail, and rejection to Reject
  - name: snapshot_campaign
    inputs: [CampaignReport]
    output: CampaignSnapshot
    semantics: copy complete counts and borrow exact identity; no allocation, authentication or mutable import
  - name: encode_campaign_snapshot
    feature: snapshot-json
    inputs: [CampaignSnapshot]
    output: bounded Vec<u8> | SnapshotError
    semantics: deterministic complete runtime.campaign-snapshot/v1 JSON; no partial output
  - name: decode_campaign_snapshot
    feature: snapshot-json
    inputs: [bounded byte slice]
    output: DecodedCampaignSnapshot | SnapshotError
    semantics: closed structural validation, owned bounded identities, immutable inspection only; not execution authentication
  - name: adapt_to_proptest_and_record
    feature: proptest
    inputs: [per-requirement report, Verdict]
    output: recorded proptest TestCaseResult
    semantics: record the verdict before mapping so the campaign census retains rejection; map identity mismatch to a proptest failure retaining expected and observed identity
invariants:
  - every evidence-bearing value retains exact borrowed source identity
  - rejected preconditions are never successful evidence
  - the default surface requires no allocator, standard library, or normal dependency
  - public runtime evaluation and accounting operations have no intentional panic path
  - a CampaignReport always contains accepted, rejected, failed, and discarded counters
  - imported snapshots cannot be converted to mutable CampaignReport values
  - snapshot-json uses alloc without std; allocator exhaustion is not universally recoverable
compatibility:
  enums: non-exhaustive; consumers must preserve future unknown states
  const-evaluation: the checked index helper is runtime-only because safe slice lookup is not const-stable at Rust 1.75
  msrv: Rust 1.75
  licensing: AGPL-3.0-or-later
  publication: disabled through the v0.1 human release decision
```

## Exact kernel surface

Generated oracles and `quire-contract-codegen` reach exact semantic values through
`quire_contract_runtime::exact` (generated source imports it as `rt`). That path is the path the
runtime guarantees stays available; it is not the only permitted path to `quire-exact`, which
`quire-contract-codegen` also depends on directly (ADR-011 X-1). The module path is the IR
build-plan seam S4 (runtime operators): the runtime operators that generated code and the code
generator call. That name is unrelated to ADR-011's stage-DAG entry S4 (Linked checked package) or
its migration path SEAM-4. This section fixes which items the seam carries, who owns each one, and
which way the crate dependencies point, so that replacing the runtime's own kernel implementation
with the upstream `quire-exact` crate changes where kernel items are defined and leaves the seam
itself unchanged.

Ownership follows QSL ADR-013 O-13 and O-16 and ADR-011 X-1. `quire-exact` is the one owner of the
exact value kernel: values and value types, the kernel evaluation `Outcome` with its `Undefined`,
`Refusal` and `Incomplete` reasons, charge-before-work `Meter` accounting, `Origin`/`Location`
provenance, the checked `NodeKey`, and the scalar and collection operations over them. The runtime
keeps what the kernel does not define: the function-application boundary (`Frame`, `Body`,
`CheckedPackage`, `Evaluation`, `plan_call`), the static checking environments generated oracles
build before they evaluate, and the backend negotiation predicates. The runtime's verdict,
observation and campaign types lie outside `exact` and are never kernel types: a kernel `Outcome`
reports what one evaluation produced, and a `Verdict` reports what an execution point established
about a contract.

A generated oracle proving a kernel operation, or a runtime operation that calls one, may not call
that same operation to compute its own expectation — only an operation that is not the one under
proof may be called this way. The expectation for a kernel operation instead comes from the QSpec
operation vectors or a checked-in specification model that calls no `quire-exact` operation
(ADR-013 O-13; ADR-011 §2.3 rule 8).

Classification is by rule, not by a frozen list, because the upstream kernel is still converging
(QSL-131 redesigns `Value`, `ValueType` and the enum and quantity shapes). An item is a kernel item
exactly when the pinned `quire-exact` revision exports an item of that name; every other `exact` item
is runtime-owned. When an item moves from the runtime to the kernel it keeps its `exact` path and
takes the kernel's shape. Consumers adapt to that shape; the path and the owner do not move again.
The `consumed` lists below record what generated code and the code generator call today, classified
against `quire-exact` revision `0dc0834`; they illustrate the rule and do not replace it.

```yaml
exact:
  module: quire_contract_runtime::exact
  feature: exact
  consumers: [generated oracle source, quire-contract-codegen host code]
  kernel:
    owner: quire-exact
    rule: every exact item whose name the pinned quire-exact revision exports
    runtime_role: re-export unchanged at the same exact path; no runtime definition
    named_kernel_types: [Value, ValueType, Outcome, Refusal, Undefined, BoundViolation, Meter, ChargePoint, Incomplete, ScalarLimits, NodeKey, Origin, Location]
    oracle_restriction: a generated oracle proving a kernel operation, or a runtime operation that calls one, may not call that operation for its own expectation (ADR-013 O-13; ADR-011 §2.3 rule 8)
    consumed:
      values: [Value, ValueType, FieldDeclaration, Presence, CardinalityBound, CollectionKind, CollectionType]
      outcomes: [Outcome, Refusal, IllTyped, IllTypedCause]
      accounting: [Meter, ScalarLimits]
      provenance: [Origin, Location]  # expected future use; not consumed by generated oracle or codegen source today
      identity: [NodeKey]
      integer_rational: [Integer, IntegerDomain, IntegerInterval, IntegerArithmetic, evaluate_integer_arithmetic, Rational, RationalDomain, RationalArithmetic, evaluate_rational_arithmetic, OrderingOperator, OrderedOperands, order_numbers, ComparisonOperator]
      division: [DivisionProfile, QuotientRemainder, divide, modulo]
      decimal: [Decimal, DecimalType, DecimalOperation, DecimalResult, RoundingMode, evaluate_decimal]
      ieee: [IeeeValue, IeeeWidth, IeeeOperation, IeeeResult, IeeeComparison, evaluate_ieee, compare_ieee, convert_ieee_width]
      text: [Text, TextPayload, TextProfile, TextType, admit_text]
      quantity: [Quantity]
  runtime_owned:
    owner: quire-contract-runtime
    rule: every exact item the pinned quire-exact revision does not export, plus the items below regardless of upstream names
    always_runtime: [Frame, Body, CheckedPackage, Evaluation, plan_call, negotiate_integer_division, negotiate_ieee]
    consumed:
      function_application: [PackageDeclarations, FunctionDeclaration, CheckedPackage, CheckMode, CheckingLimits, CheckRefusal, InputRefusal, Frame]
      checking: [TypeEnvironment, CompositeDeclaration, CompositeShape, ObjectTypeDeclaration, RecursionEdges, DeclarationCause, InvalidDeclaration, ObjectEnvironment]
      equality: [CheckedEquality, EqualityOperand, EqualityOperator, EqualitySchedule]
      enumeration: [EnumValue]
      quantity: [QuantityOperation, QuantityTarget, QuantityUnit, Conversion, evaluate_quantity, convert_quantity]
    negotiation:
      rule: the negotiate_* predicates AD-016 WP7 selects
      operations: [negotiate_integer_division, negotiate_ieee]  # today
      types: [IntegerDivisionConsumer, IntegerDivisionBounds, IntegerDivisionDisposition, IeeeItemRequirement, IeeeBackendCapabilities, IeeeDisposition, IeeeUnsupportedCause]
      consumed: none; the code generator's own negotiate_* functions are codegen-owned and outside this seam
  outside_exact: [Verdict, VerdictKind, Observation, ClauseOutcome, FailureDetail, CampaignReport]
  dependencies:
    - quire-contract-runtime -> quire-exact (normal, optional, enabled only by the exact feature)
    - quire-exact -> no quire-contract-runtime, quire-contract-codegen, quire-contract-ir or quire-spec-language crate (leaf)
    - quire-contract-codegen and generated oracles -> quire_contract_runtime::exact
    - quire-contract-codegen -> quire-exact (normal; ADR-011 X-1)
```

The table below records, for every runtime-owned `exact` operation that evaluates a
`quire-exact`-exported counterpart today, which `quire-exact` operation it calls
(interface-001-AC-9):

| Runtime-owned `exact` operation | `quire-exact` operation it calls |
| --- | --- |
| `evaluate_quantity` | `evaluate_quantity_arithmetic` |

**Open issue.** `quire-exact` revision `0dc0834` declares Rust 1.98 and is not `no_std`, which
conflicts with `compatibility.msrv` (Rust 1.75) and the `exact` feature's no_std build that
interface-001-AC-13 requires. Resolving the conflict is a pending owner decision; IR-18 tracks it.

### Acceptance criteria

| ID | Criterion | Verification |
| --- | --- | --- |
| interface-001-AC-1 | The `exact` module shall expose every item that generated oracle source or `quire-contract-codegen` — at the `quire-contract-codegen` revision that pins this runtime revision — imports from it at the path `quire_contract_runtime::exact::<Name>`. | Inspection |
| interface-001-AC-2 | Where the pinned `quire-exact` revision exports an item whose name the `exact` module exposes, the `exact` module shall expose that `quire-exact` item unchanged. | Inspection |
| interface-001-AC-3 | Where the pinned `quire-exact` revision exports an item whose name the `exact` module exposes, the `exact` module shall contain no public item definition with that name. | Inspection |
| interface-001-AC-4 | When an item's owner changes from the runtime to `quire-exact`, the `exact` module shall keep exposing that item at the path it exposed before the change. | Inspection |
| interface-001-AC-5 | Where the pinned `quire-exact` revision exports `Value`, `ValueType`, `Outcome`, `Refusal`, `Undefined`, `BoundViolation`, `Meter`, `ChargePoint`, `Incomplete`, `ScalarLimits`, `NodeKey`, `Origin` or `Location`, the `exact` module shall expose the `quire-exact` definition of that type — an explicit per-type instance of interface-001-AC-2 for the `named_kernel_types` list. | Inspection |
| interface-001-AC-6 | Where the `exact` module exposes the `quire-exact` definition of `NodeKey`, the runtime's library source shall not call a `NodeKey` constructor. | Inspection |
| interface-001-AC-7 | The runtime shall define `Frame`, `Body`, `CheckedPackage`, `Evaluation` and `plan_call` in its own source. | Inspection |
| interface-001-AC-8 | The runtime shall define the `negotiate_*` predicates AD-016 WP7 selects (today: `negotiate_integer_division`, `negotiate_ieee`) and the requirement, capability and disposition types they take and return in its own source. | Inspection |
| interface-001-AC-9 | A runtime-owned `exact` operation the correspondence table above maps to a `quire-exact` operation shall call the `quire-exact` operation the table maps it to. | Inspection |
| interface-001-AC-10 | The runtime shall define every item the `outside_exact` list names outside the `exact` module. | Inspection |
| interface-001-AC-11 | The runtime shall depend on `quire-exact` only through the `exact` feature. | Inspection |
| interface-001-AC-12 | `quire-contract-codegen` shall resolve the same `quire-exact` revision that the runtime's `exact` module re-exports. | Inspection (this check belongs in `quire-contract-codegen`'s own lockfile, not the runtime's) |
| interface-001-AC-13 | Pending the owner decision on `quire-exact`'s MSRV and `no_std` status (see Open issue above; `quire-exact` revision `0dc0834` declares Rust 1.98 and is not `no_std`), while the `exact` feature is enabled and the `std` feature is disabled, the runtime shall build for `thumbv7em-none-eabi` at the Rust version that `compatibility.msrv` declares. | Test (feature matrix row `build-exact-no-std-msrv`, TC-005) |
| interface-001-AC-14 | The runtime's normal dependencies shall include no crate published from agent-ix/quire-spec-language other than `quire-exact`. | Inspection |
