//! Fixed-population bare-metal consumer used to measure the linked runtime boundary.

#![cfg_attr(not(test), no_std)]
#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![deny(clippy::arithmetic_side_effects, clippy::indexing_slicing)]

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
    let passed = Verdict::passed(context);
    let failed = Verdict::failed_postcondition(context, detail);
    let rejected = Verdict::rejected_precondition(context, detail);
    let mut report = CampaignReport::new(identity);
    let verdict = match operators::checked_add(input, 1) {
        Some(_) => passed,
        None => failed,
    };
    if report.record_verdict(&verdict).is_err() || report.record_verdict(&rejected).is_err() {
        return 0;
    }
    report.record_discard();

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
    let operator_score = option_u32(copied)
        .saturating_add(option_u32(
            operators::index(&values, input as usize).copied(),
        ))
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
