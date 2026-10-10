---
id: TC-031
title: "Fire one injected denial with a limit-independent record"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-runtime/FR-010
    type: verifies
---
# TC-031: Fire one injected denial with a limit-independent record

## Description

Drive injection through RT's remaining exact-feature charge points and check that each request
reaches the shared meter. Evidence: `tests/exact_outcomes.rs` (`--features exact`) for the RT
charge-point census; `ix://agent-ix/quire-exact/TC-906` for the kernel record and precedence.

## Ownership and evidence

`ix://agent-ix/quire-exact/FR-358-AC-1` and `ix://agent-ix/quire-exact/FR-358-AC-2`
test named occurrence, one-shot retry and `equality.plan` denial;
`ix://agent-ix/quire-exact/FR-358-AC-3` specifies the nonzero type by Inspection.
`ix://agent-ix/quire-exact/FR-358-AC-12` and `ix://agent-ix/quire-exact/FR-358-AC-13`
have direct public-meter tests for the exact record and ordinary-limit precedence in
`ix://agent-ix/quire-exact/TC-906`. RT retains the FR-010-AC-1 residue charge-point
driver. Its one-pair reservation assertion is a local observation, not the owner evidence
for all records or for precedence. The RT dependency still needs a separate refresh to the
owner revision containing those tests.

## Test Procedure

1. For each RT residue operation's admitted charge point, inject occurrence 1 under sufficient
   limits. Drive the operation and check the refusal names that point, changes no counter or
   admitted-charge log, and reports the meter's observed denied amount. This is the retained
   FR-010-AC-1 driver in `tests/exact_outcomes.rs`.
2. Inspect the RT exact-feature meter dependency and its use by those drivers; check that the
   returned record and injection-versus-ordinary-limit precedence are the owner behavior in
   `ix://agent-ix/quire-exact/FR-358-AC-12` and
   `ix://agent-ix/quire-exact/FR-358-AC-13`. The owner verifies both through
   `ix://agent-ix/quire-exact/TC-906`; this RT procedure does not duplicate its cases.
3. Use `ix://agent-ix/quire-exact/FR-358-AC-1` through
   `ix://agent-ix/quire-exact/FR-358-AC-3` for named occurrence, one-shot retry and
   nonzero occurrence evidence; retain their RT acceptance-criterion rows in the matrix.

## Expected Results

Each driven RT residue point reaches the shared meter and reports that point without changing
admitted state. The injected record and ordinary-limit precedence have one authoritative
definition and direct evidence in quire-exact FR-358-AC-12/13 and TC-906.
