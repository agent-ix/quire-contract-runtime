//! IR-882 ownership checks for the semantic node and definition vocabularies.
#![cfg(feature = "exact")]

use ix_trace_rs::trace;
use quire_semantic_value::{
    definition::SelectionRefusalCode,
    semantic_node::{check_terms, SemanticGraphCause},
};
use quire_exact::{Integer, NodeKey};

#[trace("TC-197")]
#[test]
fn qsv_owns_definition_and_semantic_node_vocabulary() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    for duplicate in ["src/exact/definition.rs", "src/exact/node.rs"] {
        assert!(!root.join(duplicate).exists(), "duplicate QSV owner: {duplicate}");
    }
    assert_eq!(
        SelectionRefusalCode::SelectionUnknownTrigger.as_str(),
        "selection_unknown_trigger"
    );
    assert_eq!(
        check_terms(&[(NodeKey::from_digest([3; 32]), Integer::zero())]),
        Err(SemanticGraphCause::ZeroExponent)
    );
}
