//! Shared deep type exercise for the integration test and its child process.

use quire_exact::{CardinalityBound, CollectionKind, CollectionType, ValueType};

pub(crate) const CHILD_COMPLETED: &str = "deep-value-type-child-completed";
const DEPTH: usize = 100_000;

fn nested_option() -> ValueType {
    let mut value_type = ValueType::Boolean;
    for _ in 0..DEPTH {
        value_type = ValueType::option(value_type);
    }
    value_type
}

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

pub(crate) fn exercise_deep_type() {
    let options = nested_option();
    let options_copy = options.clone();
    assert_eq!(
        options, options_copy,
        "every Option layer must survive cloning"
    );
    let rendered_options = format!("{options:?}");
    assert_eq!(
        rendered_options.matches("Option(").count(),
        DEPTH,
        "debug output must traverse every Option layer"
    );
    assert!(rendered_options.contains("Boolean"));
    drop(options_copy);
    drop(options);

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
