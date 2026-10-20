//! IR-882 ownership check for the semantic-node refusal vocabulary.
#![cfg(feature = "exact")]

use ix_trace_rs::trace;
use quire_exact::{Integer, NodeKey};
use quire_semantic_value::semantic_node::{check_terms, SemanticGraphCause};

#[trace("FR-012-AC-4")]
#[test]
fn qsv_owns_semantic_node_vocabulary() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    assert!(!root.join("src/exact/node.rs").exists());
    assert_eq!(
        check_terms(&[(NodeKey::from_digest([3; 32]), Integer::zero())]),
        Err(SemanticGraphCause::ZeroExponent)
    );
}
