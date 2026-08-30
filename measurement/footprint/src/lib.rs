#![no_std]

use core::panic::PanicInfo;

use quire_contract_runtime::{
    operators, CampaignReport, ClauseId, ClauseKind, ClauseOutcome, ContractIdentity,
    ExecutionPoint, FailureDetail, FailureKind, Observation, RequirementId, RevisionId, Verdict,
    VerdictContext,
};

#[panic_handler]
fn panic(_info: &PanicInfo<'_>) -> ! {
    loop {}
}

/// Representative linked boundary used only for the release-footprint measurement.
// Implements: NFR-001
#[no_mangle]
pub extern "C" fn quire_runtime_footprint(input: u32) -> u64 {
    let identity = ContractIdentity::new(
        RequirementId::new("NFR-001"),
        RevisionId::new("footprint-v1"),
    );
    let observation = Observation::new(
        ClauseId::new("linked-boundary"),
        ClauseKind::Postcondition,
        ClauseOutcome::Passed,
        None,
    );
    let observations = [observation];
    let context = VerdictContext::new(
        identity,
        ExecutionPoint::new("thumbv7em-none-eabi"),
        &observations,
    );
    let detail = FailureDetail::new(
        ClauseId::new("checked-add"),
        FailureKind::Undefined,
        input,
        None,
    );
    let verdict = match operators::checked_add(input, 1) {
        Some(_) => Verdict::passed(context),
        None => Verdict::failed_postcondition(context, detail),
    };
    let mut report = CampaignReport::new(identity);
    if report.record_verdict(&verdict).is_err() {
        return 0;
    }

    report.counts().total()
}
