use core::mem::size_of;

use quire_contract_runtime::{
    CampaignCounts, CampaignReport, ContractIdentity, FailureDetail, Observation, Verdict,
    VerdictContext,
};

fn main() {
    println!("CampaignCounts={}", size_of::<CampaignCounts>());
    println!("ContractIdentity={}", size_of::<ContractIdentity<'_>>());
    println!("FailureDetail={}", size_of::<FailureDetail<'_>>());
    println!("Observation={}", size_of::<Observation<'_>>());
    println!("VerdictContext={}", size_of::<VerdictContext<'_>>());
    println!("Verdict={}", size_of::<Verdict<'_>>());
    println!("CampaignReport={}", size_of::<CampaignReport<'_>>());
}
