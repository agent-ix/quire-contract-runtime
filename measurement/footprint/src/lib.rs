//! Fixed-population bare-metal consumer used to measure the linked runtime boundary.

#![cfg_attr(not(test), no_std)]
#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![deny(clippy::arithmetic_side_effects, clippy::indexing_slicing)]

use core::hint::black_box;
#[cfg(not(test))]
use core::panic::PanicInfo;

use quire_contract_runtime::{
    operators, CampaignCounts, CampaignReport, ClauseId, ClauseKind, ClauseOutcome,
    ContractIdentity, ExecutionPoint, FailureDetail, FailureKind, Observation, RequirementId,
    RevisionId, Verdict, VerdictContext,
};

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo<'_>) -> ! {
    loop {}
}

/// Retains the representative entry point in the static-library archive.
#[used]
pub static FOOTPRINT_ENTRY: extern "C" fn(u32) -> u64 = quire_runtime_footprint;

/// Exercises every public constructor, both accounting mutations, and every operator family.
// Implements: NFR-001
pub extern "C" fn quire_runtime_footprint(input: u32) -> u64 {
    let identity = ContractIdentity::new(
        RequirementId::new("NFR-001"),
        RevisionId::new("footprint-v1"),
    );
    let detail = FailureDetail::new(
        ClauseId::new("checked-add"),
        FailureKind::Undefined,
        input,
        None,
    );
    let observations = [
        Observation::new(
            ClauseId::new("linked-boundary"),
            ClauseKind::Postcondition,
            ClauseOutcome::Passed,
            None,
        ),
        Observation::new(
            ClauseId::new("definedness"),
            ClauseKind::Guard,
            ClauseOutcome::Undefined,
            Some(detail),
        ),
    ];
    let context = VerdictContext::new(
        identity,
        ExecutionPoint::new("thumbv7em-none-eabi"),
        &observations,
    );
    // `black_box` keeps each built value opaque to the optimiser: the entry point's result
    // depends only on counters, so a toolchain that folds the unused verdict data away would
    // otherwise link almost none of the runtime and the fixed population would measure nothing.
    let passed = black_box(Verdict::passed(black_box(context)));
    let failed = black_box(Verdict::failed_postcondition(black_box(context), detail));
    let rejected = black_box(Verdict::rejected_precondition(black_box(context), detail));
    let mut report = CampaignReport::new(identity);
    let verdict = match operators::checked_add(input, 1) {
        Some(_) => passed,
        None => failed,
    };
    if report.record_verdict(&verdict).is_err() || report.record_verdict(&rejected).is_err() {
        return 0;
    }
    report.record_discard();
    let report = black_box(report);

    let values = [input, 1];
    let optional = Some(input);
    let copied = operators::option_copied(operators::option_ref(&optional));
    let flag = input & 1 == 0;
    let boolean_score = u64::from(operators::and_short_circuit(flag, || !flag))
        .saturating_add(u64::from(operators::or_short_circuit(flag, || !flag)))
        .saturating_add(u64::from(operators::implies_short_circuit(flag, || !flag)))
        .saturating_add(u64::from(operators::and_total(|| flag, || !flag)))
        .saturating_add(u64::from(operators::or_total(|| flag, || !flag)))
        .saturating_add(u64::from(operators::implies_total(|| flag, || !flag)));
    let indexed = match usize::try_from(input) {
        Ok(at) => operators::index(&values, at).copied(),
        Err(_) => None,
    };
    let operator_score = option_u32(copied)
        .saturating_add(option_u32(indexed))
        .saturating_add(option_u32(operators::checked_add(input, 1)))
        .saturating_add(option_u32(operators::checked_sub(input, 1)))
        .saturating_add(option_u32(operators::checked_mul(input, 2)))
        .saturating_add(option_u32(operators::checked_div(input, input)))
        .saturating_add(option_u32(operators::checked_rem(input, input)));

    CampaignCounts::new()
        .total()
        .saturating_add(report.counts().total())
        .saturating_add(boolean_score)
        .saturating_add(operator_score)
}

fn option_u32(value: Option<u32>) -> u64 {
    match value {
        Some(value) => u64::from(value),
        None => 0,
    }
}

#[cfg(test)]
mod population_tests;
