---
id: FR-006
title: "Return typed exact outcomes under metered scalar accounting"
type: FR
relationships:
  - target: ix://agent-ix/quire-contract-runtime/FR-002
    type: depends_on
---
# FR-006: Return typed exact outcomes under metered scalar accounting

## Description

When a generated oracle evaluates a complete-V1 scalar operator through the optional `exact`
feature, the runtime shall consume the authoritative `quire-exact` outcome and metering
contracts for every charge named by
`quire.value.accounting/v1` in agent-ix/quire-specification
(`proposals/quire-v1/definitions/value-accounting.md`). The runtime consumes the kernel
implementation of that definition: values and outcome kinds agree with the
quire-spec-language authority (FR-007), and charge schedules are taken from the QSpec definition.

## Inputs

- Operands of one exact scalar family, a declared result domain where the family has one, and a
  `ScalarLimits` tuple with the ten counters in `ScalarLimitsV1` field order.
- Optionally, one injected denial naming a charge point and a 1-based occurrence.

## Outputs

- `Outcome<T>`: `Completed(T)`, `Undefined(Undefined)`, `Refused(Refusal)` or
  `Incomplete(Incomplete)`, plus the admitted charge sequence and consumed counters on the `Meter`.
- Ill-typed operand combinations are reported before evaluation, beside the outcome, as `IllTyped`.

## Behavior

- A completed `false` is a value, never a refusal. `Undefined`, `Refused` and `Incomplete` carry
  closed, typed reasons; no variant carries a message string.
- When the `exact` feature consumes `quire-exact` under FR-275, the runtime SHALL consume its
  typed `Outcome::Refused(Refusal)` directly and SHALL define no separate refusal record or
  code/cause mapping. RT imports the typed kernel outcome directly in its consumers;
  FR-275 defines that boundary. The kernel owns
  `Refusal::code()` and `Refusal::cause()` under [quire-exact FR-096-AC-8](ix://agent-ix/quire-exact/FR-096).
- Each charge is decided before the work it pays for, and each size amount is derived before the
  value it measures is materialized. A denied charge consumes nothing and exposes no partial value.
- The kernel owns meter storage and accounting. Its bounded diagnostic charge log is exposed
  under `quire-exact`'s `test-support` feature. Until IR-349 removes the local meter, its
  admitted-charge sequence and `CHARGE_LOG_CAPACITY` remain governed by FR-011-AC-5. After
  that removal, RT shall consume the kernel's test-support diagnostic log rather than require
  a production log or define a local capacity constant. Consumed counters remain exact.
- Size counters are high-water marks; `work_units` and `result_units` are cumulative. The
  `Incomplete` record names the first unavailable counter in field order, its limit, the consumed
  amount before the charge, the exact denied amount (a mathematical integer) and the charge point.
- Result-domain membership is decided without a charge, after the arithmetic charges and before
  retention; an out-of-domain result is refused with no result unit.
- An injected denial at `(point, occurrence)` yields `Incomplete` on `work_units` at that point with
  counters unchanged by the denied charge. Its precedence over a real short counter, the contents
  of its record and how occurrences are counted are FR-010.
- What the meter holds at an `Undefined` or `Refused` stop, the atomicity of one charge, the
  field-order scan over a charge's own size vector, the value and truncation behavior of
  `CHARGE_LOG_CAPACITY`, and exact derived charge requests are FR-011.
- The `exact` surface uses no host floating point, no `std`, no `unsafe` and no intentional panic
  path, and is re-exported from private modules only.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-006-AC-1 | The four outcome dispositions are distinct; completed `false` is a value. `Refusal` and `Undefined` are closed enums; `Incomplete` is a typed record whose reason uses the closed `LimitKind` and `ChargePoint` enums. The runtime consumes these kernel types directly under FR-275. | Test (TC-016) |
| FR-006-AC-2 | Ill-typed operand combinations are reported before evaluation with zero charges, and provenance-bearing refusals (invalid UTF-8 offset, stale identity) are typed. | Test (TC-020, TC-021, TC-022) |
| FR-006-AC-3 | Every charge point and limit kind round-trips its QSpec spelling; charges precede work; size counters are high-water, work/result cumulative; the first short counter in field order is reported with the exact denied amount. | Test (TC-016, TC-017) |
| FR-006-AC-4 | An injected denial at any admitted charge point yields `Incomplete` on `work_units` naming that point, with no result units and no partial value. | Test (TC-017) |
| FR-006-AC-6 | Inspection identifies [quire-exact FR-096-AC-8](ix://agent-ix/quire-exact/FR-096) as the owner of `Refusal::code()` and `Refusal::cause()` for the twelve kernel refusal causes and `CheckedInvariant`. Under FR-275, RT consumes the typed kernel `Outcome::Refused(Refusal)` directly, with no RT record or code/cause mapping. | Inspection |
| FR-006-AC-7 | Inspection of `src/exact/` finds no host floating-point types, fields, casts or operations in the exact surface, including `f32` and `f64` uses. | Inspection |
| FR-006-AC-8 | With `exact` enabled and `std` disabled, the runtime builds for `thumbv7em-none-eabi` on the supported MSRV through the `build-exact-no-std-msrv` row of `make test-features`. | compile-time-check (TC-199, `make test-features`) |
| FR-006-AC-9 | Every public item exposed through `quire_contract_runtime::exact` is re-exported by `src/exact/mod.rs` from a private child module; child module paths are inaccessible to consumers. | compile-time-check |
| FR-006-AC-10 | Non-test code in `src/exact/` has no panic site. | Inspection |

## Verification

For FR-006-AC-7, search and read `src/exact/` for host floating-point types, fields,
casts and operations. For FR-006-AC-10, search and read non-test `src/exact/` code for
`unwrap`, `expect`, `panic!`, `unreachable!`, `todo!`, `unimplemented!`, `assert*!`, direct
slice or array indexing, and unchecked arithmetic that can overflow. A reachable
occurrence of any of these is a panic site; code under `#[cfg(test)]` is excluded.
NFR-001-AC-2 owns the crate-wide `#![forbid(unsafe_code)]` check, and NFR-002-AC-1
owns the public API no-intentional-panic behavior. For FR-006-AC-9, add one `compile_fail`
doctest importing each private child path under `quire_contract_runtime::exact`:
`boolean`, `collection`, `composite`, `containment`, `definition`, `division`,
`enumeration`, `equality`, `expression`, `ieee`, `key`, `node`, `quantity`,
`reference`, `stop`, and `unit`. Compare this list against every child declaration in
`src/exact/mod.rs` so a newly added child cannot be omitted. Each doctest must fail
because its path is private, not because the import is malformed or the module is absent.
Inspect `mod.rs` for direct public definitions, including `pub fn` and `pub struct`;
any such definition fails the re-export-only clause.

**Note:** FR-006-AC-7 through FR-006-AC-10 bind `src/exact/` while that module exists.
Under FR-275 they retire together with that module and are replaced by the kernel's
equivalent criteria; they must never pass vacuously against an absent directory.

## Kernel ownership

The runtime shall consume `quire-exact` directly for kernel outcomes, charges and scalar
operations; it shall define no replacement kernel or re-export path ([FR-275](./FR-275-single-exact-kernel.md)).
The criteria above remain obligations on that consumed behavior. They do not require RT-local
kernel tests. [The exact matrix](../matrix/tests.md) records current local evidence separately
from its disposition after the implementation removes the copy.

FR-006-AC-1 remains open in RT. [quire-exact FR-362-AC-11](ix://agent-ix/quire-exact/FR-362) and
`quire-exact/tests/ir673_scalar.rs::scalar_outcomes_keep_only_admitted_prefixes`
establish the four scalar stop dispositions, including completed `false`, but do
not census every closed reason type. The owner declarations are
`quire_exact::Outcome`, `Undefined`, `Refusal`, and `Incomplete` in
`quire-exact/src/outcome.rs`, with `LimitKind` and `ChargePoint` in
`quire-exact/src/accounting.rs`. `Incomplete` is a struct, not an enum; its
typed reason fields use those closed enums. TC-016 retains the RT consumer
check of the whole criterion, which has no local binder yet.

Owner evidence covers only matching subsets: [quire-exact FR-096-AC-8](ix://agent-ix/quire-exact/FR-096) owns
kernel refusal codes and causes; `FR-362-AC-11` covers public scalar outcome stops;
`FR-368-AC-1` and `FR-368-AC-2` cover the charge and limit vocabularies;
`FR-359-AC-6` and `FR-359-AC-7` cover counter order and consumption;
`FR-358-AC-1`, `FR-358-AC-2` and `FR-358-AC-8` through `FR-358-AC-11` cover named denial
and the bounded test-support log. `FR-361-AC-7` through `FR-361-AC-9` cover the specified
large-work allocation cases. The kernel code/cause mapping belongs to that owner AC; the
diagnostic record belongs to `ix://agent-ix/quire-spec-language/FR-096`. These one-crate tests
do not establish ill-typed caller behavior or RT/QSL agreement; those remain explicit in the matrix.

This amendment deletes no implementation or test. The remaining local kernel copy is IR-349
work; remaining RT evaluation-residue deletion is open IR-349 step 2 and IR-583 (backlog) work. QSL-358 is
Done and covered QSL-side extraction, not the remaining RT deletion. Neither copy is RT-owned
by remaining present. IR-430 is Done and covered removal of RT agreement tests; the remaining
quire-integration agreement work has open quire-integration ticket IR-669.

## Dependencies

- **Upstream**: [FR-002](../../core/functional/FR-002-safe-operators.md);
  `ix://agent-ix/quire-specification` (`value-accounting.md`).
