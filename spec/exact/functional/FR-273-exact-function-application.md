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
  - target: ix://agent-ix/quire-contract-codegen/FR-021
    type: references
---
# FR-273: Apply checked total pure functions under exact semantics

## Description

When the `exact` feature is enabled, the runtime shall apply a checked package's named function, and
evaluate a checked standalone expression, through `quire_contract_runtime::exact::CheckedPackage::call`
and `CheckedPackage::evaluate` against a package this crate's `PackageDeclarations::check` (`CheckMode::Linked`) admitted, a
package whose functions the authority's `PackageDeclarations::check` proved pure, total and
definedness-safe on every reachable path, with outcomes, refusals and
charges equal to the quire-specification authority (FR-146) on every shared-corpus
vector, and equal to the quire-spec-language authority that carries it. As with FR-006
through FR-008, the runtime is an implementation of that definition, not a second semantic authority:
`quire_spec_language::value` decides every application, refusal and charge question. This
requirement ports the call surface only, into `quire_contract_runtime::exact` under the authority's
own names and order: admission, argument validation, `function.call` accounting and the
`Evaluation` envelope. Admission keeps the authority's `PackageDeclarations::check` name and role
but not its proofs: the runtime's `PackageDeclarations::check` admits only the facts a generated
package carries — its declared types, unique names and a termination measure the producer states it
discharged upstream (`measure_discharged`) — and refuses `CheckMode::Kernel`. The authority's
typer, fact derivation, termination prover, task machine and expression IR are not ported: function
bodies, and the root of a `CheckedExpression`, are resumable states the generated oracle supplies,
already lowered to Rust by the code generator, so `evaluate` drives a generated body and is call surface,
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
  `function.call` once for each application the root makes. The code generator must lower every
  source-level application to the runtime-managed call-frame mechanism (a code-generator obligation,
  like the proof precondition above).
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
- The current synchronous `Body` closure and `Frame::call` contract cannot suspend a caller. The
  runtime SHALL replace that contract with a resumable body interface. For each invocation the
  generated producer creates owned state containing its next instruction, bound values and the
  destination for a child's result. The runtime invokes one transition at a time with either the
  initial arguments or a completed child value; a transition returns either a request to
  call a named checked function with completed arguments, or a final `Outcome<Value>`. The suspended
  parent state remains in an explicit runtime-owned call-frame stack. The runtime validates and
  charges the child call, pushes its state, and resumes the parent with the child's value after
  successful completion. A stopped child outcome propagates without evaluating later arguments or body
  work. The scheduler SHALL return to its loop between transitions; neither a generated transition
  nor the scheduler recursively invokes another body transition. `CheckedPackage::call` and
  `CheckedPackage::evaluate` seed that scheduler, while generated nested applications use only its
  call request. There is no synchronous nested-call path in the generated-body contract.
- The generated-oracle producer owns the lowering of every function and standalone-expression root
  into that resumable state, including left-to-right argument evaluation and result destinations.
  It SHALL emit finite transitions free of uncharged loops and direct recursive applications.
  Every nonterminal transition requests a runtime-charged operation;
  therefore a finite `work_units` budget bounds the number of runtime-managed transitions. This
  guarantee depends on the producer's upstream purity, termination and definedness proofs. An
  arbitrary host implementation of the body interface can loop, recurse, re-enter a top-level call
  or perform unmetered work; such unchecked host code is outside the generated-program guarantee,
  and interface conformance alone does not prove its totality.
- The runtime-managed call stack SHALL grow independently of the native stack. No application
  call-depth ceiling, counter or `CallDepthExceeded` refusal SHALL decide an outcome. The shared
  `Meter`'s `work_units` limit is execution fuel for the generated program, including each admitted
  `function.call`. Exhaustion at a denied charge SHALL return `Incomplete` with
  `limit_kind: work_units` and the denied charge point; the denied charge SHALL leave counters and
  charge log unchanged and SHALL NOT change the package's static totality verdict. The ordinary
  charge order remains: evaluate arguments left to right, validate the completed inputs, charge
  `function.call`, then bind parameters and run the body. Nested calls use the same meter in
  execution order.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-273-AC-1 | `CheckedPackage::call` and `CheckedPackage::evaluate` are called only against a package `PackageDeclarations::check` admitted under `CheckMode::Linked`; a package this crate's `PackageDeclarations::check` rejects is never applied. | Test (TC-194) |
| FR-273-AC-2 | Declared arity is decided before any per-argument check, each argument's value kind and carried references are validated in parameter order, and all of that precedes the `function.call` charge, which itself precedes the function's body on every `call`. | Test (TC-194) |
| FR-273-AC-3 | Arity, value-kind and dangling-reference input mismatches, and an unknown function name, each refuse as the matching `InputRefusal` variant before any charge; no `Meter` observes a refused call. | Test (TC-194) |
| FR-273-AC-4 | A function whose declared operator requirements no registered backend can discharge negotiates `unsupported`, naming the required capability, before any application; the disposition is never an `Outcome` variant, never an `InputRefusal`, and takes no `Meter`. | Test (TC-195) |
| FR-273-AC-5 | Outcome, refusal and charge sequence agree with the quire-spec-language authority on every shared-corpus function-application vector. | Test (TC-194) |
| FR-273-AC-6 | `CheckMode::Kernel` application is out of scope: no test in this requirement's corpus applies a package checked only under `CheckMode::Kernel`. | Inspection (TC-194) |
| FR-273-AC-7 | A checked, decreasing function completes 4,096 runtime-managed calls on a 64 KiB native stack when `work_units` suffices; across depths 1, 128 and 4,096, no two generated-body transitions are simultaneously active, and inspection shows the scheduler returns to one loop before invoking the next transition. No depth-specific refusal or incomplete outcome exists. | Test (TC-194) |
| FR-273-AC-8 | For an admitted chain of `N` calls whose bodies only request the next call or return a literal, the expected charge vector is `function.call` repeated `N` times and the expected `work_units` spend is `N`. A limit of `N` completes; a limit of `N - 1` returns `Incomplete` at the Nth `function.call`, naming `work_units`, with exactly `N - 1` calls in both the counter and log, and without changing the checked package's static totality verdict. | Test (TC-194) |
| FR-273-AC-9 | In a nested call with two argument applications and an enclosing application, the charge vector is exactly three `function.call` entries in left argument, right argument, enclosing call order; the enclosing charge precedes parameter binding and body work. With `work_units: 2`, the enclosing charge is denied, the log and counter retain exactly the first two calls, and no enclosing body work occurs. | Test (TC-194) |
| FR-273-AC-10 | A resumable body transition returns a child-call request while its owned parent state retains live locals and the child-result destination; after the child completes, the runtime resumes that parent with the child's value and does not invoke a second body transition from within the first. A stopped child propagates without resuming the parent. | Test (TC-194) |

## Kernel ownership

The function-application surface (`Frame`, `Body`, `CheckedPackage`, `Evaluation`, `plan_call`) is
not in `quire-exact` today. Under [FR-275](./FR-275-single-exact-kernel.md) the runtime's copy of it
is part of the residue: ported QSL code that is not authorized, with no exception, no expiry and no
approval. It runs over the `quire-exact` `Value`, `Meter` and `Outcome`. It is to be deleted; QSL
is to own it (QSL-358, as relayed) and the runtime is to consume it. How the runtime handles the copy until then is an open
owner decision. The "port of the authority" wording above describes that source.

## Dependencies

- **Upstream**: [FR-006](./FR-006-exact-outcomes-and-accounting.md); [FR-008](./FR-008-composite-collection-and-equality.md);
  [FR-009](./FR-009-i13-backend-negotiation.md);
  `ix://agent-ix/quire-specification` (FR-146, `expressions/FR-146-check-total-pure-functions.md`);
  quire-spec-language. The quire-exact owner must amend FR-369-AC-3 to remove its RT
  `CallDepthExceeded` producer obligation before this requirement can merge; its
  `UnknownCheckedFunction`, `ForeignCheckedExpression` and `MeterBorrowConflict` obligations remain.
- **Producer**: `ix://agent-ix/quire-contract-codegen/FR-021` owns emitted resumable body states,
  argument order and continuation slots. Its current synchronous-closure contract requires an
  amendment to use the runtime's new stepping interface before a generated oracle can satisfy this
  requirement.
