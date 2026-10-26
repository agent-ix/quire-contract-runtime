//! Consumer controls for deeply nested kernel type metadata.
#![cfg(feature = "exact")]

use ix_trace_rs::trace;
use quire_exact::{CardinalityBound, CollectionKind, CollectionType, ValueType};

const DEPTH: usize = 16_384;
const CHILD_TEST: &str = "default_stack_child_entry_for_deep_value_type_traits";
const CHILD_COMPLETED: &str = "deep-value-type-child-completed";

fn nested_type(
    leaf: ValueType,
    kind: CollectionKind,
    bound: Option<CardinalityBound>,
) -> ValueType {
    let mut value_type = ValueType::collection(CollectionType::new(kind, leaf, bound));
    for level in 0..DEPTH {
        value_type = if level % 2 == 0 {
            ValueType::option(value_type)
        } else {
            ValueType::collection(CollectionType::new(
                CollectionKind::Sequence,
                value_type,
                None,
            ))
        };
    }
    value_type
}

fn exercise_deep_type() {
    let original = nested_type(ValueType::Boolean, CollectionKind::Set, None);
    let copy = original.clone();
    assert!(original == copy, "a deep clone must retain every type link");

    let rendered = format!("{original:?}");
    assert!(rendered.starts_with("Collection(CollectionType { kind: Sequence, element: Option("));
    assert!(rendered.contains("element: Boolean, bound: None"));
    assert!(rendered.ends_with("bound: None })"));

    let different_leaf = nested_type(ValueType::Integer, CollectionKind::Set, None);
    let different_kind = nested_type(ValueType::Boolean, CollectionKind::Bag, None);
    let different_bound = nested_type(
        ValueType::Boolean,
        CollectionKind::Set,
        Some(CardinalityBound::at_most(3)),
    );
    assert!(
        original != different_leaf,
        "the deep leaf must affect equality"
    );
    assert!(
        original != different_kind,
        "the deep collection kind must affect equality"
    );
    assert!(
        original != different_bound,
        "the deep collection bound must affect equality"
    );

    let enclosing = CollectionType::new(CollectionKind::Sequence, original.clone(), None);
    let enclosing_copy = enclosing.clone();
    assert!(
        enclosing == enclosing_copy,
        "an enclosing collection must clone its deep member"
    );
    assert!(format!("{enclosing:?}").contains("element: Collection("));

    drop(enclosing_copy);
    drop(enclosing);
    drop(different_bound);
    drop(different_kind);
    drop(different_leaf);
    drop(copy);
    drop(original);
}

#[test]
#[trace("TC-024", "FR-008-AC-13")]
fn deep_value_type_traits_complete_on_small_and_default_stacks() {
    std::thread::Builder::new()
        .stack_size(64 * 1024)
        .spawn(exercise_deep_type)
        .expect("spawn the small-stack consumer")
        .join()
        .expect("deep type traits complete on a small stack");

    let executable = std::env::current_exe().expect("test executable");
    let child = std::process::Command::new(executable)
        .args(["--ignored", "--exact", CHILD_TEST, "--nocapture"])
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
    assert!(
        stdout.contains("running 1 test"),
        "child ran no test: {stdout}"
    );
    assert!(
        stdout.contains(CHILD_COMPLETED),
        "child did not finish the deep type exercise: {stdout}"
    );
    assert!(
        stdout.contains(&format!("test {CHILD_TEST} ... ok")),
        "child did not pass the selected test: {stdout}"
    );
}

#[test]
#[ignore = "run by deep_value_type_traits_complete_on_small_and_default_stacks in a child process"]
#[trace("TC-024", "FR-008-AC-13")]
fn default_stack_child_entry_for_deep_value_type_traits() {
    exercise_deep_type();
    println!("{CHILD_COMPLETED}");
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
