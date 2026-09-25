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
   refusal is reported, confirming parameter-order validation. Call the same function with an
   argument count mismatch and a later dangling reference; expect `InputRefusal::Arity`, confirming
   arity is decided before any per-argument check.
7. Evaluate `CheckedPackage::evaluate` on a standalone `CheckedExpression` and check it shares
   `call`'s argument validation and `InputRefusal` set; check that an expression root that reaches
   no `Frame::call` charges no `function.call`, and that an expression root making two `Frame::call`
   invocations charges exactly two, each before its own function's body. Check `evaluate` on an
   expression checked against another package returns `Ok(Evaluation)` with
   `Refused(CheckedInvariant)`.
8. For a shared-corpus function-application vector, check the outcome, refusal and charge sequence
   against the quire-spec-language `ea39f91` authority.
9. Inspect this requirement's corpus for `CheckMode::Kernel`: check that every applied package is
   checked under `CheckMode::Linked` and that no test applies a package checked only under
   `CheckMode::Kernel` (FR-273-AC-6).
10. Recurse through `Frame::call` on a package checked with `CheckingLimits::depth` below
    `MAX_CALL_DEPTH`, and separately recurse by direct, bypassing re-entry into
    `CheckedPackage::call`/`CheckedPackage::evaluate` from within a running body holding its own
    `Rc<CheckedPackage>`; on both paths, check that going one call beyond the checked package's
    configured depth refuses `Refusal::CheckedInvariant` with no charge, and that the two entry paths
    share one depth budget on a given `CheckedPackage` rather than each getting its own
    (FR-273-AC-7).

## Expected Results

Every call against a package `check` admitted is either a well-formed `Evaluation` or a named
`InputRefusal` decided before any charge; arity is decided first and arguments are then validated in
parameter order; the `function.call` charge precedes the function's body on every accepted call;
`evaluate` agrees with `call` on argument validation and refuses identically, and charges
`function.call` once per `Frame::call` the root makes and none for a root that makes none; no applied package is checked only under `CheckMode::Kernel`; and re-entry beyond a checked
package's configured `CheckingLimits::depth`, through `Frame::call` or through a direct, bypassing
re-entry into `CheckedPackage::call`/`evaluate`, refuses `Refusal::CheckedInvariant` before any charge
against one budget shared by all three entry paths.
