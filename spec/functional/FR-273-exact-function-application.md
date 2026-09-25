---
id: FR-273
title: "Apply checked total pure functions under exact semantics"
type: FR
relationships:
  - target: ix://agent-ix/quire-contract-runtime/FR-006
    type: depends_on
  - target: ix://agent-ix/quire-contract-runtime/FR-008
    type: depends_on
  - target: ix://agent-ix/quire-contract-runtime/FR-009
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-146
    type: implements
---
# FR-273: Apply checked total pure functions under exact semantics

## Description

When the `exact` feature is enabled, the runtime shall apply a checked package's named function, and
evaluate a checked standalone expression, through `quire_contract_runtime::exact::CheckedPackage::call`
and `CheckedPackage::evaluate` against a package this crate's `PackageDeclarations::check` (`CheckMode::Linked`) admitted, a
package whose functions the authority's `PackageDeclarations::check` proved pure, total and
definedness-safe on every reachable path, with outcomes, refusals and
charges equal to the pinned quire-specification authority (`7d7943a`, FR-146) on every shared-corpus
vector, and equal to the quire-spec-language authority (`ea39f91`) that carries it. As with FR-006
through FR-008, the runtime is an implementation of that definition, not a second semantic authority:
`quire_spec_language::value` decides every application, refusal and charge question. This
requirement ports the call surface only, into `quire_contract_runtime::exact` under the authority's
own names and order: admission, argument validation, `function.call` accounting and the
`Evaluation` envelope. Admission keeps the authority's `PackageDeclarations::check` name and role
but not its proofs: the runtime's `PackageDeclarations::check` admits only the facts a generated
package carries — its declared types, unique names and a termination measure the producer states it
discharged upstream (`measure_discharged`) — and refuses `CheckMode::Kernel`. The authority's
typer, fact derivation, termination prover, task machine and expression IR are not ported: function
bodies, and the root of a `CheckedExpression`, are host callables the generated oracle supplies,
already lowered to Rust by the code generator, so `evaluate` runs a host body and is call surface,
not an expression interpreter. The generated oracle, which links this `#![no_std]` crate alone,
calls the ported surface.

## Inputs

- A `CheckedPackage` produced by a prior `PackageDeclarations::check` under `CheckMode::Linked`, and
  either a function name with a `Vec<Value>` of arguments (`call`) or a `CheckedExpression` with a
  `Vec<Value>` for its parameters (`evaluate`).
- An `ObjectEnvironment` giving every `Value::Reference` argument a universe to be validated against.
- A `&mut Meter` charging the call.

## Outputs

- `Result<Evaluation, InputRefusal>` from both `call` and `evaluate`. `Evaluation` carries the call's `Outcome<Value>`, an optional
  source `Location` and any `LocatedLoss` values the call's Boolean or arithmetic connectives lost.
- `InputRefusal` is the closed, pre-charge set `Arity`, `WrongValueKind`, `DanglingReference` and
  `UnknownFunction`; it is never returned beside a charge and never carries a `Meter` side effect.

## Behavior

- Purity, termination (by the function's `decreases` measure) and every definedness obligation on
  every reachable path are proved once, statically, by the authority's own
  `PackageDeclarations::check` in quire-spec-language. That proof lives outside this crate and
  outside its own Kani proof surface (AD-002, Risks): this crate depends on it rather than
  reproducing it. It is a precondition on the producer and code generator: the runtime trusts its
  producer to admit only bodies the authority proved. This crate's own `PackageDeclarations::check`
  enforces only: every declared parameter and result type is well typed in the package's type
  environment, function names are unique, each function's `measure_discharged` flag is set, and
  `CheckMode::Kernel` is refused. Nothing links a runtime `CheckedPackage` to an authority proof,
  and a body is arbitrary host Rust. `call` and `evaluate` never re-derive these proofs and never
  accept a package this crate's `PackageDeclarations::check` refused.
  `CheckMode::Kernel` (typing only, no definedness proof) stays out of scope, as it does for every
  earlier exact requirement.
- Application is total, never short-circuiting: every supplied argument is validated — its value kind
  against the checked parameter type, and every `Value::Reference` it carries at any depth against the
  supplied `ObjectEnvironment` — in parameter order, and the declared arity is decided before any
  per-argument check, all of it before the `function.call` charge; the `function.call` charge then
  precedes the function's body. This mirrors AD-001's separation of total from short-circuit
  evaluation and contrasts with FR-007's short-circuit Boolean connectives: a function reads every one
  of its arguments as a completed value, so no argument's value can cause another to go unread.
- `function.call` is the accounting vocabulary point FR-008 ported ahead of this requirement
  (`ChargePoint::FunctionCall`, `"function.call"`); this requirement is the first to charge it during
  a live call rather than carrying it unexercised. `call` charges one `function.call` for the called
  function's own body; `evaluate` runs a standalone `CheckedExpression` root, so it charges
  `function.call` once for each `Frame::call` the root makes; the runtime counts only `Frame::call`
  invocations, so the code generator must lower every source-level application in an expression
  to a `Frame::call` (a code-generator obligation, like the proof precondition above).
- `evaluate` on an expression checked against a different package returns
  `Ok(Evaluation { outcome: Refused(CheckedInvariant), .. })` before any charge, and
  `check_expression` checks parameter and result types only.
- A function whose declared operator requirements no registered backend can discharge settles
  `unsupported` with a warning naming the required capability. FR-009's negotiators decide that
  disposition over the function's declared requirements, before any application and with no `Meter`
  participation; it is a provider disposition, so it is never an `Outcome` variant, never an
  `InputRefusal`, and never a value `call` or `evaluate` returns.
- `InputRefusal` is decided before any charge and before the function's body runs: an unknown function
  name, an argument count or kind mismatch against the checked signature, or an argument
  `Value::Reference` the supplied `ObjectEnvironment` cannot resolve, each refuse call input with no
  `Meter` participation, exactly as `plan_equality`'s pre-charge refusals do for equality.
- Unbounded host recursion through a checked package is silent stack corruption on the governed
  `thumbv7em-none-eabi` target: re-entry into a checked package through `CheckedPackage::call`,
  `CheckedPackage::evaluate` or `Frame::call` is bounded by the runtime's own
  `CheckingLimits::depth`, not an authority checker limit (at most
  `MAX_CALL_DEPTH`), by one budget shared across all three entry paths, and exceeding it refuses as
  `Refusal::CheckedInvariant` before any charge. The bound is per-`CheckedPackage`, not universal: a
  host body that builds a *fresh* `CheckedPackage` at each hop gets a fresh budget and can still
  overflow the host stack — but so does a body that recurses without touching this crate's runtime at
  all, since under AD-002 a body is arbitrary host Rust and its own stack usage is the host's concern,
  not this crate's.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-273-AC-1 | `CheckedPackage::call` and `CheckedPackage::evaluate` are called only against a package `PackageDeclarations::check` admitted under `CheckMode::Linked`; a package this crate's `PackageDeclarations::check` rejects is never applied. | Test (TC-194) |
| FR-273-AC-2 | Declared arity is decided before any per-argument check, each argument's value kind and carried references are validated in parameter order, and all of that precedes the `function.call` charge, which itself precedes the function's body on every `call`. | Test (TC-194) |
| FR-273-AC-3 | Arity, value-kind and dangling-reference input mismatches, and an unknown function name, each refuse as the matching `InputRefusal` variant before any charge; no `Meter` observes a refused call. | Test (TC-194) |
| FR-273-AC-4 | A function whose declared operator requirements no registered backend can discharge negotiates `unsupported`, naming the required capability, before any application; the disposition is never an `Outcome` variant, never an `InputRefusal`, and takes no `Meter`. | Test (TC-195) |
| FR-273-AC-5 | Outcome, refusal and charge sequence agree with the quire-spec-language `ea39f91` authority on every shared-corpus function-application vector. | Test (TC-194) |
| FR-273-AC-6 | `CheckMode::Kernel` application is out of scope: no test in this requirement's corpus applies a package checked only under `CheckMode::Kernel`. | Inspection (TC-194) |
| FR-273-AC-7 | Re-entry into a checked package through `CheckedPackage::call`, `CheckedPackage::evaluate` or `Frame::call` is bounded by the runtime's own `CheckingLimits::depth`, not an authority checker limit (at most `MAX_CALL_DEPTH`) by one budget shared across all three entry paths, and exceeding it refuses as `Refusal::CheckedInvariant` before any charge. The bound is per-`CheckedPackage`, not universal. | Test (TC-194) |

## Dependencies

- **Upstream**: [FR-006](./FR-006-exact-outcomes-and-accounting.md); [FR-008](./FR-008-composite-collection-and-equality.md);
  [FR-009](./FR-009-i13-backend-negotiation.md);
  `ix://agent-ix/quire-specification` at `7d7943a` (FR-146, `expressions/FR-146-check-total-pure-functions.md`);
  quire-spec-language at `ea39f91`.
