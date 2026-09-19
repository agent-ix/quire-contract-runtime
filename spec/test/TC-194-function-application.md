---
id: TC-194
title: "Apply checked functions totally, before any charge"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-runtime/FR-273
    type: verifies
---
# TC-194: Apply checked functions totally, before any charge

## Description

Check `CheckedPackage::call` and `CheckedPackage::evaluate` over a package `PackageDeclarations::check`
admitted under `CheckMode::Linked`, and check every `InputRefusal` variant against a matching malformed
call. Evidence: `tests/exact_function_application.rs` (`--features exact`), planned for
agent-ix/quire-contract-runtime#34.

## Test Procedure

1. Check a package `check` admits under `CheckMode::Linked`; call each of its functions with
   well-typed arguments and check `call` returns `Ok(Evaluation { .. })`, never an `InputRefusal`.
2. Call a function with an argument count that disagrees with its checked signature; expect
   `InputRefusal::Arity` with no `Meter` counter changed.
3. Call a function with an argument of the wrong value kind for its checked parameter; expect
   `InputRefusal::WrongValueKind` with no `Meter` counter changed.
4. Call a function with a `Value::Reference` argument the supplied `ObjectEnvironment` cannot
   resolve; expect `InputRefusal::DanglingReference` with no `Meter` counter changed.
5. Call an unknown function name against a checked package; expect `InputRefusal::UnknownFunction`
   with no `Meter` counter changed.
6. Call a function taking two or more arguments where a later argument's `Value::Reference` is
   dangling and an earlier argument alone would already refuse; check the earlier argument's
   refusal is reported, confirming left-to-right validation order.
7. Evaluate `CheckedPackage::evaluate` on a standalone `CheckedExpression` and check the same
   argument-validation and `function.call` charge-ordering behavior as `call`.
8. For a shared-corpus function-application vector, check the outcome, refusal and charge sequence
   against the quire-spec-language `68bdacb` authority.

## Expected Results

Every call against a package `check` admitted is either a well-formed `Evaluation` or a named
`InputRefusal` decided before any charge; arguments are validated left to right; the `function.call`
charge precedes the function's body on every accepted call; and `evaluate` agrees with `call` on
argument validation and charge order.
