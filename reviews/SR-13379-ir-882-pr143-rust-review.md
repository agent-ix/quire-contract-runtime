---
id: SR-13379
title: "Rust review of IR-882 checked-function owner-value cutover"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-runtime; Rust lane for src/exact/expression.rs, tests/exact_function_application.rs, tests/exact_outcomes.rs, tests/ir882_direct_kernel_value.rs; FR-273-AC-1, FR-273-AC-2, FR-273-AC-3"
review_set: subset
---

## Summary

Ticket: IR-882. This separate Rust-lane artifact records the requested idiom, panic, boundary, trace, and gate checks at PR #143 head aba5eb8. The catalog permits `analysis: code-review` for this lane; its title and review marker identify the method as rust-review.

## Verdict

**PASS** — The changed Rust code follows the repository's stated conventions and adds no panic, unchecked cast, unsafe block, test bypass, or duplicated owner type. See SR-13380 for the new union branch's missing test.

## Examined criteria

- FR-273-AC-1, examined: `CheckedPackage::call` and `CheckedPackage::evaluate` are called only against a package `PackageDeclarations::check` admitted under `CheckMode::Linked`; a package this crate's `PackageDeclarations::check` rejects is never applied.
- FR-273-AC-2, examined: Declared arity is decided before any per-argument check, each argument's value kind and carried references are validated in parameter order, and all of that precedes the `function.call` charge, which itself precedes the function's body on every `call`.
- FR-273-AC-3, examined: Arity, value-kind and dangling-reference input mismatches, and an unknown function name, each refuse as the matching `InputRefusal` variant before any charge; no `Meter` observes a refused call.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Checks

- Read repository `AGENTS.md`, `CLAUDE.md`, `CONTRIBUTING.md`, `clippy.toml`, `deny.toml`, and `rustfmt.toml`; repository conventions take priority.
- Existing public types are used directly at signatures; no alias, adapter, re-export, or compatibility path was added.
- The new test has a valid `#[trace("TC-194", "FR-273-AC-2")]` binding; the computed matrix resolves both IDs.
- `cargo fmt --check`, `make lint`, `cargo deny check`, and the three touched integration targets passed. The generic `cargo clippy --workspace --all-targets --all-features` failed on the footprint crate's intentional no-std panic handler because that command compiles it for the host; `make lint` uses its required thumb target and passed.
