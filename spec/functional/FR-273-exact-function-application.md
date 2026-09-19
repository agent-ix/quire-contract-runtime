---
id: FR-273
title: "Apply checked total pure functions under exact semantics"
type: FR
relationships:
  - target: ix://agent-ix/quire-contract-runtime/FR-006
    type: depends_on
  - target: ix://agent-ix/quire-contract-runtime/FR-008
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-146
    type: implements
---
# FR-273: Apply checked total pure functions under exact semantics

## Description

When the `exact` feature is enabled, the runtime shall apply a checked package's named function, and
evaluate a checked standalone expression, by calling `quire_spec_language::value::CheckedPackage::call`
and `CheckedPackage::evaluate` against a package a prior `PackageDeclarations::check` (`CheckMode::Linked`)
has proved pure, total and definedness-safe on every reachable path, with outcomes, refusals and
charges equal to the pinned quire-specification authority (`7d7943a`, FR-146) on every shared-corpus
vector, and equal to the quire-spec-language authority (`68bdacb`) that carries it. As with FR-006
through FR-008, the runtime is an implementation of that definition, not a second semantic authority:
`quire_spec_language::value::CheckedPackage` decides every call, and this requirement only states
which type the generated oracle calls and what it carries across the boundary.

## Inputs

- A `CheckedPackage` produced by a prior `PackageDeclarations::check` under `CheckMode::Linked`, and
  either a function name with a `Vec<Value>` of arguments (`call`) or a `CheckedExpression` with a
  `Vec<Value>` for its parameters (`evaluate`).
- An `ObjectEnvironment` giving every `Value::Reference` argument a universe to be validated against.
- A `&mut Meter` charging the call.

## Outputs

- `Result<Evaluation, InputRefusal>`. `Evaluation` carries the call's `Outcome<Value>`, an optional
  source `Location` and any `LocatedLoss` values the call's Boolean or arithmetic connectives lost.
- `InputRefusal` is the closed, pre-charge set `Arity`, `WrongValueKind`, `DanglingReference` and
  `UnknownFunction`; it is never returned beside a charge and never carries a `Meter` side effect.

## Behavior

- Purity, termination (by the function's `decreases` measure) and every definedness obligation on
  every reachable path are proved once, statically, by `PackageDeclarations::check`, before any
  package is callable and before any charge. `call` and `evaluate` never re-derive these proofs and
  never accept a package `check` rejected: there is no approximate or unchecked application path.
  `CheckMode::Kernel` (typing only, no definedness proof) stays out of scope, as it does for every
  earlier exact requirement.
- Application is total, never short-circuiting: every argument is evaluated left to right and every
  `Value::Reference` argument validated against the supplied `ObjectEnvironment` before the
  `function.call` charge, and the `function.call` charge precedes the function's body. This mirrors
  AD-001's separation of total from short-circuit evaluation and contrasts with FR-007's short-circuit
  Boolean connectives: a function call never skips an argument's evaluation on account of another
  argument's value.
- `function.call` is the accounting vocabulary point FR-008 ported ahead of this requirement
  (`ChargePoint::FunctionCall`, `"function.call"`); this requirement is the first to charge it during
  a live call rather than carrying it unexercised.
- An application whose function no registered backend can discharge settles `unsupported` with a
  warning naming the required capability, identified by the function's declared identity: an
  undischargeable capability reached from inside a called function's body propagates out of the call
  as the same `unsupported` provider disposition FR-009 defines for its own operators, never as a
  refusal or a partial `Outcome`.
- `InputRefusal` is decided before any charge and before the function's body runs: an unknown function
  name, an argument count or kind mismatch against the checked signature, or an argument
  `Value::Reference` the supplied `ObjectEnvironment` cannot resolve, each refuse call input with no
  `Meter` participation, exactly as `plan_equality`'s pre-charge refusals do for equality.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-273-AC-1 | `CheckedPackage::call` and `CheckedPackage::evaluate` are called only against a package `PackageDeclarations::check` admitted under `CheckMode::Linked`; a package `check` rejects is never applied. | Test (TC-194) |
| FR-273-AC-2 | Every argument is evaluated left to right and every `Value::Reference` argument validated before the `function.call` charge, and the `function.call` charge precedes the function's body on every call. | Test (TC-194) |
| FR-273-AC-3 | Arity, value-kind and dangling-reference input mismatches, and an unknown function name, each refuse as the matching `InputRefusal` variant before any charge; no `Meter` observes a refused call. | Test (TC-194) |
| FR-273-AC-4 | An undischargeable capability reached from a called function's body settles the application `unsupported`, naming the capability, never as a refusal or a partial `Outcome`. | Test (TC-195) |
| FR-273-AC-5 | Outcome, refusal and charge sequence agree with the quire-spec-language `68bdacb` authority on every shared-corpus function-application vector. | Test (TC-194) |
| FR-273-AC-6 | `CheckMode::Kernel` application is out of scope: no test in this requirement's corpus applies a package checked only under `CheckMode::Kernel`. | Inspection |

## Dependencies

- **Upstream**: [FR-006](./FR-006-exact-outcomes-and-accounting.md); [FR-008](./FR-008-composite-collection-and-equality.md);
  `ix://agent-ix/quire-specification` at `7d7943a` (FR-146, `expressions/FR-146-check-total-pure-functions.md`);
  quire-spec-language at `68bdacb`.
