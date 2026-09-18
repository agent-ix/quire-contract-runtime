//! FR-009 I13 backend negotiation: one disposition per item, in input order,
//! from the item's declared requirement and the backend's declared
//! capabilities alone, through the public `exact` surface.
#![cfg(feature = "exact")]

use std::collections::BTreeSet;

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

/// Trace: TC-030, FR-009-AC-5
#[test]
fn tc_030_negotiators_take_no_meter_and_dispositions_are_closed_enums() {
    // Neither negotiator's signature accepts a `Meter`; this test never
    // constructs one, and none is in scope to pass.
    let integer_items = [
        IntegerDivisionConsumer::Mathematical,
        IntegerDivisionConsumer::Finite(IntegerDivisionBounds::default()),
    ];
    for disposition in negotiate_integer_division(&integer_items) {
        // Exhaustive with no wildcard arm: an `Outcome` variant folded onto
        // this enum, or an undeclared variant, would fail to compile here.
        match disposition {
            IntegerDivisionDisposition::Supported | IntegerDivisionDisposition::RequiresBound => {}
        }
    }

    let ieee_items = [baseline_item()];
    for disposition in negotiate_ieee(&ieee_items, &capable_backend()) {
        match disposition {
            IeeeDisposition::Supported
            | IeeeDisposition::Unsupported(_)
            | IeeeDisposition::RequiresBound => {}
        }
    }
}
