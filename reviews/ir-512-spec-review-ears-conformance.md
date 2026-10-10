---
id: SR-5085
title: spec-review/ears-conformance review of IR-512 function application
type: SpecReview
analysis: ears-conformance
scope: agent-ix/quire-contract-runtime@19ce9e7f3e7c206fe8ca1bf555fb742576ddcaa2; spec/exact/functional/FR-273-exact-function-application.md,
  spec/exact/matrix/TC-194-function-application.md, spec/exact/matrix/tests.md
review_set: subset
---

## Summary

Review of the frozen IR-512 spec diff at PR #111. Ticket: IR-512.

## Verdict

**PASS: Quire reports all three changed documents grammar-clean and the reviewed requirement responses are concrete.**

## Examined units

- `FR-273-AC-1` (examined): `CheckedPackage::call` and `CheckedPackage::evaluate` are called only against a package `PackageDeclarations::check` admitted under `CheckMode::Linked`; a package this crate's `PackageDeclarations::check` rejects is never applied.
- `FR-273-AC-2` (examined): Declared arity is decided before any per-argument check, each argument's value kind and carried references are validated in parameter order, and all of that precedes the `function.call` charge, which itself precedes the function's body on every `call`.
- `FR-273-AC-3` (examined): Arity, value-kind and dangling-reference input mismatches, and an unknown function name, each refuse as the matching `InputRefusal` variant before any charge; no `Meter` observes a refused call.
- `FR-273-AC-4` (examined): A function whose declared operator requirements no registered backend can discharge negotiates `unsupported`, naming the required capability, before any application; the disposition is never an `Outcome` variant, never an `InputRefusal`, and takes no `Meter`.
- `FR-273-AC-5` (examined): Outcome, refusal and charge sequence agree with the quire-spec-language authority on every shared-corpus function-application vector.
- `FR-273-AC-6` (examined): `CheckMode::Kernel` application is out of scope: no test in this requirement's corpus applies a package checked only under `CheckMode::Kernel`.
- `FR-273-AC-7` (examined): A checked, decreasing function completes 4,096 runtime-managed calls on a 64 KiB native stack when `work_units` suffices; inspection shows explicit call frames and no call-depth outcome branch, so increasing call depth alone does not produce a refusal or incomplete outcome.
- `FR-273-AC-8` (examined): For an admitted call chain requiring `w` work units, a `work_units` limit of `w` admits it and a limit of `w - 1` returns `Incomplete` at the first denied charge, naming `work_units` and its charge point, without recording the denied charge or changing the package's totality verdict.
- `FR-273-AC-9` (examined): In a nested call with multiple argument applications, argument charges occur left to right before the enclosing `function.call`; that charge precedes parameter binding and body work. All calls share one meter and preserve the exact charge sequence on completion and on fuel exhaustion.
- `TC-194` (examined): Run admitted and refused function applications, explicit-frame deep calls, exact and one-under work fuel, and nested charge ordering.
- `FR-273 matrix row` (examined): FR-273-AC-7, FR-273-AC-8, FR-273-AC-9 are planned; old depth-limit tests must be replaced.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |
