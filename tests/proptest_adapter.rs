#![cfg(feature = "proptest")]

use proptest::test_runner::TestCaseError;
use quire_contract_runtime::{
    proptest_adapter, ClauseId, ContractIdentity, ExecutionPoint, FailureDetail, FailureKind,
    RequirementId, RevisionId, Verdict, VerdictContext,
};

/// Trace: TC-004, FR-003-AC-1
#[test]
fn tc_004_adapter_preserves_all_three_outcomes() {
    let context = VerdictContext::new(
        ContractIdentity::new(RequirementId::new("FR-003"), RevisionId::new("rev-1")),
        ExecutionPoint::new("property"),
        &[],
    );
    let detail = FailureDetail::new(ClauseId::new("clause"), FailureKind::Contract, 1, None);

    assert!(proptest_adapter::adapt(&Verdict::passed(context)).is_ok());
    assert!(matches!(
        proptest_adapter::adapt(&Verdict::failed_postcondition(context, detail)),
        Err(TestCaseError::Fail(_))
    ));
    assert!(matches!(
        proptest_adapter::adapt(&Verdict::rejected_precondition(context, detail)),
        Err(TestCaseError::Reject(_))
    ));
}
