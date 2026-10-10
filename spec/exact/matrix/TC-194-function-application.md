---
id: TC-194
title: "Apply checked functions with explicit frames and work fuel"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-runtime/FR-273
    type: verifies
---
# TC-194: Apply checked functions with explicit frames and work fuel

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
   `call`'s argument validation and `InputRefusal` set; check that an expression root making no
   function application charges no `function.call`, and that a root making two applications charges
   exactly two, each before its own function's body. Check `evaluate` on an
   expression checked against another package returns `Ok(Evaluation)` with
   `Refused(CheckedInvariant { cause: ForeignCheckedExpression })`.
8. For a shared-corpus function-application vector, check the outcome, refusal and charge sequence
   against the quire-spec-language authority.
9. Inspect this requirement's corpus for `CheckMode::Kernel`: check that every applied package is
   checked under `CheckMode::Linked` and that no test applies a package checked only under
   `CheckMode::Kernel` (FR-273-AC-6).
10. Run a checked decreasing function through depths 1, 128 and 4,096 with sufficient `work_units`
    on a 64 KiB native stack. Count simultaneously active generated-body transitions independently
    of the runtime: increment on entry and decrement before each transition returns its step. Assert
    a maximum of one at every depth and the declared result at 4,096. Inspect every scheduler path
    to confirm that it returns to one loop before invoking another transition and has no depth-stop
    branch. This combination rejects linear native-stack growth even if a 4,096-call sample alone
    happens to fit (FR-273-AC-7).
11. Use an admitted fixture with `N = 64` calls: each body only requests the next call or returns a
    literal, so the test-authored expected vector is `function.call` repeated exactly 64 times and
    the expected `work_units` spend is 64, independent of any measured run. At limit 64 assert that
    vector, the result, and the counter. At limit 63 assert `Incomplete` at the 64th
    `function.call`, a counter of 63, and exactly 63 logged calls with no denied entry. A fresh
    limit-64 run of the same checked package must still complete (FR-273-AC-9).
12. From a standalone expression, apply a left argument function, then a right argument function,
    then an enclosing function; each body only returns a literal. Assert the test-authored vector
    `[function.call(left), function.call(right), function.call(enclosing)]` and observe that the
    enclosing body's parameters are bound only after its charge. At `work_units: 2`, assert
    `Incomplete` at the enclosing charge, two consumed work units, precisely the first two log
    entries, and no enclosing body entry (FR-273-AC-10).
13. Exercise a resumable body fixture whose owned state holds one live local across a yielded child-call
    request. Assert the runtime resumes the same parent state with the child's completed value, restores
    the live local, and has no simultaneously active parent and child body transitions. Inspect the
    runtime interface so every nonterminal transition requests a metered call. The generator's own
    FR-021 tests must separately show that it emits this state and never places an uncharged loop
    or direct recursive application inside a transition (FR-273-AC-11).
14. Trigger a missing function from within an admitted checked body and both re-entrant meter
    borrow paths; assert `UnknownCheckedFunction` and `MeterBorrowConflict` respectively. Check a
    foreign expression carries `ForeignCheckedExpression` (FR-273-AC-8).

## Expected Results

Every call against a package `check` admitted is either a well-formed `Evaluation` or a named
`InputRefusal` decided before any charge; arity is decided first and arguments are then validated in
parameter order; the `function.call` charge precedes the function's body on every accepted call;
`evaluate` agrees with `call` on argument validation and refuses identically, and charges
`function.call` once per application the root makes and none for a root that makes none; no applied
package is checked only under `CheckMode::Kernel`; deep runtime-managed calls complete through
explicit frames when fuel suffices, with at most one active generated-body transition at each tested
depth; the first denied charge returns `Incomplete` on `work_units` without recording that charge;
and nested argument, call and body charges remain in evaluation order. A yielded parent resumes with
its owned locals and its child's completed value; a stopped child propagates without a parent resume.
The foreign-expression and checked-body faults retain their distinct typed invariant causes.
