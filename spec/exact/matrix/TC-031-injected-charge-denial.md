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

[quire-exact FR-358-AC-1](ix://agent-ix/quire-exact/FR-358) and [quire-exact FR-358-AC-2](ix://agent-ix/quire-exact/FR-358)
test named occurrence, one-shot retry and `equality.plan` denial;
[quire-exact FR-358-AC-3](ix://agent-ix/quire-exact/FR-358) specifies the nonzero type by Inspection.
[quire-exact FR-358-AC-12](ix://agent-ix/quire-exact/FR-358) and [quire-exact FR-358-AC-13](ix://agent-ix/quire-exact/FR-358)
have direct public-meter tests for the exact record and ordinary-limit precedence in
`ix://agent-ix/quire-exact/TC-906`. RT retains the FR-010-AC-1 residue charge-point
driver. Its one-pair reservation assertion is a local observation, not the owner evidence
for all records or for precedence. The RT dependency still needs a separate refresh to the
owner revision containing those tests.

## Test Procedure

1. For each RT residue operation's admitted charge point, inject occurrence 1 under sufficient
   limits. Drive the operation and check `WorkUnits`, the named point, `limit = consumed`, the
   point's expected `next_charge`, post-refusal work consumption equal to the record's
   `consumed`, zero result consumption, and absence of the denied point from the admitted-charge
   log. This is what the retained FR-010-AC-1 driver in `tests/exact_outcomes.rs` asserts; it
   does not snapshot every counter, admission count or the whole log.
2. Inspect the RT exact-feature meter dependency and its use by those drivers; check that the
   returned record and injection-versus-ordinary-limit precedence are the owner behavior in
   [quire-exact FR-358-AC-12](ix://agent-ix/quire-exact/FR-358) and
   [quire-exact FR-358-AC-13](ix://agent-ix/quire-exact/FR-358). The owner verifies both through
   `ix://agent-ix/quire-exact/TC-906`; this RT procedure does not duplicate its cases.
3. Use [quire-exact FR-358-AC-1](ix://agent-ix/quire-exact/FR-358) through
   [quire-exact FR-358-AC-3](ix://agent-ix/quire-exact/FR-358) for named occurrence, one-shot retry and
   nonzero occurrence evidence; retain their RT acceptance-criterion rows in the matrix.

## Expected Results

Each driven RT residue point reaches the shared meter and satisfies the fields and post-refusal
observations in step 1. Full meter-state atomicity is owner evidence under
[quire-exact FR-358-AC-1](ix://agent-ix/quire-exact/FR-358); the injected record and ordinary-limit precedence
have one authoritative definition and direct evidence in
[quire-exact FR-358-AC-12](ix://agent-ix/quire-exact/FR-358), [quire-exact FR-358-AC-13](ix://agent-ix/quire-exact/FR-358)
and `ix://agent-ix/quire-exact/TC-906`.
