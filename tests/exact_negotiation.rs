//! FR-009 I13 backend negotiation: one disposition per item, in input order,
//! from the item's declared requirement and the backend's declared
//! capabilities alone, through the public `exact` surface.
#![cfg(feature = "exact")]

use std::collections::BTreeSet;
use std::fs;
use std::path::Path;

use quire_contract_runtime::exact::{
    negotiate_ieee, negotiate_integer_division, IeeeBackendCapabilities, IeeeDisposition,
    IeeeItemRequirement, IeeeOperationKind, IeeeUnsupportedCause, IeeeWidth, Integer,
    IntegerDivisionBounds, IntegerDivisionConsumer, IntegerDivisionDisposition, IntegerInterval,
    RoundingMode,
};

fn int(value: i128) -> Integer {
    Integer::from(value)
}

fn bound() -> IntegerInterval {
    IntegerInterval::new(int(0), int(1)).unwrap()
}

/// A backend that implements everything a baseline item asks for: `Binary64`,
/// `Add`, `TowardZero` rounding, the exceptional policy and a finite proof.
fn capable_backend() -> IeeeBackendCapabilities {
    IeeeBackendCapabilities {
        widths: BTreeSet::from([IeeeWidth::Binary64]),
        operations: BTreeSet::from([IeeeOperationKind::Add]),
        roundings: BTreeSet::from([RoundingMode::TowardZero]),
        exceptional_policy: true,
        finite_proof: true,
    }
}

/// An item the `capable_backend` fully supports.
fn baseline_item() -> IeeeItemRequirement {
    IeeeItemRequirement {
        width: IeeeWidth::Binary64,
        operation: IeeeOperationKind::Add,
        rounding: RoundingMode::TowardZero,
        requires_finite_proof: false,
    }
}

// ---- AC-1: positional, order-preserving, empty-safe --------------------------

/// Trace: TC-030, FR-009-AC-1
#[test]
fn tc_030_integer_division_empty_slice_gives_empty_result() {
    assert_eq!(negotiate_integer_division(&[]), Vec::new());
}

/// Trace: TC-030, FR-009-AC-1
#[test]
fn tc_030_ieee_empty_slice_gives_empty_result() {
    assert_eq!(negotiate_ieee(&[], &capable_backend()), Vec::new());
}

/// Trace: TC-030, FR-009-AC-1
#[test]
fn tc_030_integer_division_reversed_input_gives_reversed_dispositions() {
    let items = [
        IntegerDivisionConsumer::Mathematical,
        IntegerDivisionConsumer::Finite(IntegerDivisionBounds {
            operand: Some(bound()),
            intermediate: Some(bound()),
            result: Some(bound()),
        }),
        IntegerDivisionConsumer::Finite(IntegerDivisionBounds::default()),
    ];
    let forward = negotiate_integer_division(&items);
    assert_eq!(
        forward,
        [
            IntegerDivisionDisposition::Supported,
            IntegerDivisionDisposition::Supported,
            IntegerDivisionDisposition::RequiresBound,
        ]
    );

    let mut reversed_items = items;
    reversed_items.reverse();
    let reversed = negotiate_integer_division(&reversed_items);
    let mut expected = forward;
    expected.reverse();
    assert_eq!(reversed, expected);
}

/// Trace: TC-030, FR-009-AC-1
#[test]
fn tc_030_ieee_reversed_input_gives_reversed_dispositions() {
    let mut backend = capable_backend();
    // Otherwise the `requires_finite_proof` item below would also be
    // `Supported`, collapsing it with the first item.
    backend.finite_proof = false;
    let items = [
        baseline_item(),
        IeeeItemRequirement {
            width: IeeeWidth::Binary32,
            ..baseline_item()
        },
        IeeeItemRequirement {
            requires_finite_proof: true,
            ..baseline_item()
        },
    ];
    let forward = negotiate_ieee(&items, &backend);
    assert_eq!(
        forward,
        [
            IeeeDisposition::Supported,
            IeeeDisposition::Unsupported(IeeeUnsupportedCause::Width(IeeeWidth::Binary32)),
            IeeeDisposition::RequiresBound,
        ]
    );

    let mut reversed_items = items;
    reversed_items.reverse();
    let reversed = negotiate_ieee(&reversed_items, &backend);
    let mut expected = forward;
    expected.reverse();
    assert_eq!(reversed, expected);
}

// ---- AC-2: integer division bound-subset table --------------------------------

/// Trace: TC-030, FR-009-AC-2
#[test]
fn tc_030_mathematical_consumer_is_supported() {
    assert_eq!(
        negotiate_integer_division(&[IntegerDivisionConsumer::Mathematical]),
        [IntegerDivisionDisposition::Supported]
    );
}

/// Trace: TC-030, FR-009-AC-2
#[test]
fn tc_030_finite_consumer_is_supported_only_for_the_full_bound_set() {
    for has_operand in [false, true] {
        for has_intermediate in [false, true] {
            for has_result in [false, true] {
                let bounds = IntegerDivisionBounds {
                    operand: has_operand.then(bound),
                    intermediate: has_intermediate.then(bound),
                    result: has_result.then(bound),
                };
                let complete = has_operand && has_intermediate && has_result;
                let expected = if complete {
                    IntegerDivisionDisposition::Supported
                } else {
                    IntegerDivisionDisposition::RequiresBound
                };
                let got = negotiate_integer_division(&[IntegerDivisionConsumer::Finite(bounds)]);
                assert_eq!(
                    got,
                    [expected],
                    "operand={has_operand} intermediate={has_intermediate} result={has_result}"
                );
            }
        }
    }
}

// ---- AC-3: IEEE cause order, first failure wins --------------------------------

/// Trace: TC-030, FR-009-AC-3
#[test]
fn tc_030_ieee_each_cause_fires_alone_in_order() {
    let backend = capable_backend();

    let width_fails = IeeeItemRequirement {
        width: IeeeWidth::Binary32,
        ..baseline_item()
    };
    assert_eq!(
        negotiate_ieee(&[width_fails], &backend),
        [IeeeDisposition::Unsupported(IeeeUnsupportedCause::Width(
            IeeeWidth::Binary32
        ))]
    );

    let operation_fails = IeeeItemRequirement {
        operation: IeeeOperationKind::Multiply,
        ..baseline_item()
    };
    assert_eq!(
        negotiate_ieee(&[operation_fails], &backend),
        [IeeeDisposition::Unsupported(
            IeeeUnsupportedCause::Operation(IeeeOperationKind::Multiply)
        )]
    );

    let rounding_fails = IeeeItemRequirement {
        rounding: RoundingMode::NearestEven,
        ..baseline_item()
    };
    assert_eq!(
        negotiate_ieee(&[rounding_fails], &backend),
        [IeeeDisposition::Unsupported(
            IeeeUnsupportedCause::Rounding(RoundingMode::NearestEven)
        )]
    );

    let mut policy_fails_backend = backend.clone();
    policy_fails_backend.exceptional_policy = false;
    assert_eq!(
        negotiate_ieee(&[baseline_item()], &policy_fails_backend),
        [IeeeDisposition::Unsupported(
            IeeeUnsupportedCause::ExceptionalPolicy
        )]
    );
}

/// Trace: TC-030, FR-009-AC-3
#[test]
fn tc_030_ieee_two_failing_causes_report_the_earlier_one() {
    let backend = capable_backend();

    let width_and_operation = IeeeItemRequirement {
        width: IeeeWidth::Binary32,
        operation: IeeeOperationKind::Multiply,
        ..baseline_item()
    };
    assert_eq!(
        negotiate_ieee(&[width_and_operation], &backend),
        [IeeeDisposition::Unsupported(IeeeUnsupportedCause::Width(
            IeeeWidth::Binary32
        ))]
    );

    let width_and_rounding = IeeeItemRequirement {
        width: IeeeWidth::Binary32,
        rounding: RoundingMode::NearestEven,
        ..baseline_item()
    };
    assert_eq!(
        negotiate_ieee(&[width_and_rounding], &backend),
        [IeeeDisposition::Unsupported(IeeeUnsupportedCause::Width(
            IeeeWidth::Binary32
        ))]
    );

    let mut policy_fails_backend = backend.clone();
    policy_fails_backend.exceptional_policy = false;

    let width_and_policy = IeeeItemRequirement {
        width: IeeeWidth::Binary32,
        ..baseline_item()
    };
    assert_eq!(
        negotiate_ieee(&[width_and_policy], &policy_fails_backend),
        [IeeeDisposition::Unsupported(IeeeUnsupportedCause::Width(
            IeeeWidth::Binary32
        ))]
    );

    let operation_and_rounding = IeeeItemRequirement {
        operation: IeeeOperationKind::Multiply,
        rounding: RoundingMode::NearestEven,
        ..baseline_item()
    };
    assert_eq!(
        negotiate_ieee(&[operation_and_rounding], &backend),
        [IeeeDisposition::Unsupported(
            IeeeUnsupportedCause::Operation(IeeeOperationKind::Multiply)
        )]
    );

    let operation_and_policy = IeeeItemRequirement {
        operation: IeeeOperationKind::Multiply,
        ..baseline_item()
    };
    assert_eq!(
        negotiate_ieee(&[operation_and_policy], &policy_fails_backend),
        [IeeeDisposition::Unsupported(
            IeeeUnsupportedCause::Operation(IeeeOperationKind::Multiply)
        )]
    );

    let rounding_and_policy = IeeeItemRequirement {
        rounding: RoundingMode::NearestEven,
        ..baseline_item()
    };
    assert_eq!(
        negotiate_ieee(&[rounding_and_policy], &policy_fails_backend),
        [IeeeDisposition::Unsupported(
            IeeeUnsupportedCause::Rounding(RoundingMode::NearestEven)
        )]
    );
}

/// Trace: TC-030, FR-009-AC-3
#[test]
fn tc_030_non_rounding_operation_ignores_an_unsupported_rounding_direction() {
    assert!(!IeeeOperationKind::NumericEqual.rounds());
    let backend = IeeeBackendCapabilities {
        widths: BTreeSet::from([IeeeWidth::Binary64]),
        operations: BTreeSet::from([IeeeOperationKind::NumericEqual]),
        // The backend never offers `NearestEven`; a rounding operation would
        // fail here, but `NumericEqual` does not round.
        roundings: BTreeSet::from([RoundingMode::TowardZero]),
        exceptional_policy: true,
        finite_proof: true,
    };
    let item = IeeeItemRequirement {
        width: IeeeWidth::Binary64,
        operation: IeeeOperationKind::NumericEqual,
        rounding: RoundingMode::NearestEven,
        requires_finite_proof: false,
    };
    let got = negotiate_ieee(&[item], &backend);
    assert_eq!(got, [IeeeDisposition::Supported]);
    assert_ne!(
        got,
        [IeeeDisposition::Unsupported(
            IeeeUnsupportedCause::Rounding(RoundingMode::NearestEven)
        )]
    );
}

// ---- AC-4: the finite-proof seam -----------------------------------------------

/// Trace: TC-030, FR-009-AC-4
#[test]
fn tc_030_undischarged_finite_proof_requires_bound_only_absent_an_unsupported_cause() {
    let mut backend = capable_backend();
    backend.finite_proof = false;

    let proof_only = IeeeItemRequirement {
        requires_finite_proof: true,
        ..baseline_item()
    };
    assert_eq!(
        negotiate_ieee(&[proof_only], &backend),
        [IeeeDisposition::RequiresBound]
    );

    // The same requirement against a backend that can discharge the proof is
    // fully `Supported`.
    let discharging_backend = capable_backend();
    assert_eq!(
        negotiate_ieee(&[proof_only], &discharging_backend),
        [IeeeDisposition::Supported]
    );
}

/// Trace: TC-030, FR-009-AC-4
#[test]
fn tc_030_missing_capability_and_undischarged_proof_reports_the_capability_cause() {
    let mut backend = capable_backend();
    backend.finite_proof = false;

    let width_and_proof = IeeeItemRequirement {
        width: IeeeWidth::Binary32,
        requires_finite_proof: true,
        ..baseline_item()
    };
    assert_eq!(
        negotiate_ieee(&[width_and_proof], &backend),
        [IeeeDisposition::Unsupported(IeeeUnsupportedCause::Width(
            IeeeWidth::Binary32
        ))]
    );
}

// ---- AC-5: no Meter, no Outcome -------------------------------------------------

/// The exact source text of a top-level `fn name(...) { ... }` item, found by
/// brace matching from its first opening brace. Panics if `name` is absent.
fn function_source<'a>(source: &'a str, name: &str) -> &'a str {
    let needle = format!("fn {name}(");
    let start = source
        .find(&needle)
        .unwrap_or_else(|| panic!("{name} not found in source"));
    let from_start = &source[start..];
    let open = from_start.find('{').unwrap();
    let mut depth = 0_usize;
    for (i, c) in from_start[open..].char_indices() {
        match c {
            '{' => depth += 1,
            '}' => {
                depth -= 1;
                if depth == 0 {
                    return &from_start[..open + i + 1];
                }
            }
            _ => {}
        }
    }
    panic!("unbalanced braces reading {name}");
}

/// Trace: TC-030, FR-009-AC-5
#[test]
fn tc_030_negotiators_take_no_meter() {
    let src_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/exact");

    let division_source = fs::read_to_string(src_dir.join("division.rs")).unwrap();
    let negotiate_integer_division_source =
        function_source(&division_source, "negotiate_integer_division");
    assert!(
        !negotiate_integer_division_source.contains("Meter"),
        "negotiate_integer_division mentions Meter:\n{negotiate_integer_division_source}"
    );

    let ieee_source = fs::read_to_string(src_dir.join("ieee.rs")).unwrap();
    let negotiate_ieee_source = function_source(&ieee_source, "negotiate_ieee");
    assert!(
        !negotiate_ieee_source.contains("Meter"),
        "negotiate_ieee mentions Meter:\n{negotiate_ieee_source}"
    );
}

/// Trace: TC-030, FR-009-AC-5
#[test]
fn tc_030_no_disposition_converts_into_an_outcome_variant() {
    // If either disposition ever grew a conversion into `Outcome` — most
    // plausibly `impl From<IeeeDisposition> for Outcome<T>` or the integer
    // equivalent — it would read `From<IeeeDisposition>` or
    // `From<IntegerDivisionDisposition>` somewhere in `src/`. Scanning every
    // source file, rather than only the two that declare the dispositions,
    // catches a conversion placed anywhere in the crate.
    let src_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut checked_any = false;
    for entry in fs::read_dir(src_dir.join("exact")).unwrap() {
        let path = entry.unwrap().path();
        if path.extension().and_then(|e| e.to_str()) != Some("rs") {
            continue;
        }
        let source = fs::read_to_string(&path).unwrap();
        for forbidden in ["From<IeeeDisposition>", "From<IntegerDivisionDisposition>"] {
            assert!(
                !source.contains(forbidden),
                "{} converts a disposition into another type ({forbidden}); \
                 dispositions are provider dispositions, never Outcome variants",
                path.display()
            );
        }
        checked_any = true;
    }
    assert!(checked_any, "no source files were scanned");

    // Both negotiators also stay checked against the closed set of dispositions
    // this test knows about. `IntegerDivisionDisposition` and `IeeeDisposition`
    // are `#[non_exhaustive]` (NFR-002-AC-3: downstream generated oracles must
    // not exhaustively match them), so this crate's own `tests/` — a separate,
    // downstream-compiled crate — can no longer enforce coverage with a plain
    // exhaustive `match` and a compile error. `matches!` plus an explicit
    // assertion keeps the same guard at test-run time instead: an `Outcome`
    // variant folded onto either enum, or an undeclared variant, fails this
    // assertion rather than silently passing.
    let integer_items = [
        IntegerDivisionConsumer::Mathematical,
        IntegerDivisionConsumer::Finite(IntegerDivisionBounds::default()),
    ];
    for disposition in negotiate_integer_division(&integer_items) {
        assert!(
            matches!(
                disposition,
                IntegerDivisionDisposition::Supported | IntegerDivisionDisposition::RequiresBound
            ),
            "unexpected integer division disposition: {disposition:?}"
        );
    }

    let ieee_items = [baseline_item()];
    for disposition in negotiate_ieee(&ieee_items, &capable_backend()) {
        assert!(
            matches!(
                disposition,
                IeeeDisposition::Supported
                    | IeeeDisposition::Unsupported(_)
                    | IeeeDisposition::RequiresBound
            ),
            "unexpected IEEE disposition: {disposition:?}"
        );
    }
}
