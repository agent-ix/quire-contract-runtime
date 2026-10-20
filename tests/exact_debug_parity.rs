//! Debug-rendering regression pin for the compound unit the runtime still defines whose
//! `Debug` impl is hand-written because its fields sit behind one `Box`.
//! FR-007-AC-6's own oracle -- equal Debug renderings against the quire-spec-language
//! authority -- is removed from this repository; recreating it in
//! agent-ix/quire-integration is planned under Linear IR-430. This is
//! not that oracle (FR-007-AC-13, TC-035): it only pins each type's own
//! rendering against a fixed string, so a field added to a `*Fields` struct
//! but not to its `Debug` impl is caught here even while that oracle
//! is red. The enum pin left with the enum owner cutover (IR-882).
#![cfg(feature = "exact")]

use ix_trace_rs::trace;
use quire_contract_runtime::exact::CompoundUnit;

/// Only the runtime-held `CompoundUnit` and `Dimension` rendering is pinned here; the kernel
/// scalars' own renderings are `quire-exact`'s to pin.
#[trace("TC-035", "FR-007-AC-13")]
#[test]
fn tc_035_debug_parity_compound_unit() {
    let compound = CompoundUnit::dimensionless();
    assert_eq!(
        format!("{compound:?}"),
        "CompoundUnit { terms: {}, dimension: Dimension({}) }"
    );
}
