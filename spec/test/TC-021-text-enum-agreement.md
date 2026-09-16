---
id: TC-021
title: "Agree with the authority on text and enum vectors"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-runtime/FR-007
    type: verifies
---
# TC-021: Agree with the authority on text and enum vectors

## Description

Execute every QSpec 5d88578 TC-186 vector on the runtime and on quire-spec-language d9d5273.
Evidence: `conformance/qsl-agreement/tests/tc_186_text_enum.rs` (`make conformance`).

## Test Procedure

1. Check the evaluated and admission-only lists together equal the TC-186 census.
2. Compare admission lengths, invalid UTF-8 provenance, profile and declaration mismatches,
   ordered and unordered enum comparison, charges and counters on both sides.
3. Deny every named charge; sweep six profiles × twelve sequences² × six operators, and every enum
   declaration × member × operator.

## Expected Results

17 vectors agree exactly; T05b and the T09 stale-key half are admission-only and named.
