---
id: TC-205
title: "Inspect QSV semantic-node vocabulary ownership"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-runtime/FR-275
    type: verifies
---
# TC-205: Inspect QSV semantic-node vocabulary ownership

## Description

Check FR-275-AC-22 independently of the broader, planned TC-197 kernel inspection.

## Test Procedure

1. Parse every Rust source file under `src/`, including macro bodies. Reject a local definition or
   alias of QSV's `SemanticGraphCause`, `InvalidSemanticGraph` or semantic-node term checker, even
   when the local item has another name.
2. Run the inspection against representative renamed aliases, a copied cause enum and a renamed
   term checker; require each to be rejected.

## Expected Results

The runtime defines or aliases none of QSV's semantic-node vocabulary. The inspection rejects each
representative local copy or alias.
