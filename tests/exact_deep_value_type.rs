//! Consumer controls for deeply nested kernel type metadata.
#![cfg(feature = "exact")]

use ix_trace_rs::trace;
use quire_exact::{CardinalityBound, CollectionKind, CollectionType, ValueType};

#[path = "support/deep_value_type.rs"]
mod deep_value_type;

#[test]
#[trace("TC-024", "FR-008-AC-13")]
fn deep_value_type_traits_complete_on_small_and_default_stacks() {
    std::thread::Builder::new()
        .stack_size(64 * 1024)
        .spawn(deep_value_type::exercise_deep_type)
        .expect("spawn the small-stack consumer")
        .join()
        .expect("deep type traits complete on a small stack");

    let child = std::process::Command::new(env!("CARGO_BIN_EXE_exact_deep_value_type_child"))
        .env_remove("RUST_MIN_STACK")
        .output()
        .expect("spawn the default-stack child process");
    let stdout = String::from_utf8_lossy(&child.stdout);
    assert!(
        child.status.success(),
        "deep type traits complete in child: status={:?}, stdout={}, stderr={}",
        child.status,
        stdout,
        String::from_utf8_lossy(&child.stderr)
    );
    assert_eq!(
        stdout.trim(),
        deep_value_type::CHILD_COMPLETED,
        "child did not finish the deep type exercise: {stdout}"
    );
}

#[test]
#[trace("TC-024", "FR-008-AC-13")]
fn shallow_value_type_debug_matches_derived_representation() {
    let value_type = ValueType::option(ValueType::collection(CollectionType::new(
        CollectionKind::Set,
        ValueType::Boolean,
        Some(CardinalityBound::at_most(3)),
    )));
    assert_eq!(
        format!("{value_type:?}"),
        "Option(Collection(CollectionType { kind: Set, element: Boolean, bound: Some(CardinalityBound { minimum: 0, maximum: 3 }) }))"
    );
    assert_eq!(
        format!("{value_type:#?}"),
        "Option(\n    Collection(\n        CollectionType {\n            kind: Set,\n            element: Boolean,\n            bound: Some(\n                CardinalityBound {\n                    minimum: 0,\n                    maximum: 3,\n                },\n            ),\n        },\n    ),\n)"
    );
}
