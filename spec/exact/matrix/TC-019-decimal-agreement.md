---
id: TC-019
title: "Agree with the authority on exact decimal vectors"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-runtime/FR-007
    type: verifies
---
# TC-019: Agree with the authority on exact decimal vectors

## Description

Execute every QSpec TC-185 vector on the runtime and on quire-spec-language.
Evidence: The QSL agreement oracle is removed from this repository; recreating it in agent-ix/quire-integration is planned with **UNRESOLVED OPEN OWNER — planner ticket ID pending** (IR-430 removal is Done). Step 5 keeps evidence in the
upscale allocation bound in `tests/exact_allocation.rs` (`--features exact`).

## Ownership and evidence

Steps 1–3 remain a QSL agreement gap with **UNRESOLVED OPEN OWNER — planner ticket ID pending**. Step 5 currently has RT allocation
coverage, which leaves in IR-349. `ix://agent-ix/quire-exact/FR-361-AC-6` owns that denied
retain-upscale allocation bound; FR-361-AC-4/AC-5 cover scale-expansion/arithmetic denial.
`ix://agent-ix/quire-exact/FR-363` owns retained-representation decimal ordering only.
Neither contract establishes TC-185 agreement or all rounding/arithmetic cases in step 4.
No local allocation check constitutes an agreement pass.

## Test Procedure

1. Check the evaluated list equals D01–D23.
2. For every vector with an authority run, compare value and outcome kind on both sides; for
   vectors whose charges the authority meters, also compare charges, counters and every injected
   denial.
3. For D09, D13 and D20–D23, check the runtime's admitted point sequence, charge amounts
   and consumed counters against the QSpec TC-185 expectations at the exact limit tuple;
   repeat one under the first short counter and require `Incomplete` with that counter,
   exact denied amount and charge point, with no later charge or retained result.
   D20–D21 must short `integer_bits` at `ordering.arithmetic`; D22–D23 must charge
   result-retain upscale before materialization. For every authority-metered vector, also
   require equal value, outcome kind, charge sequence and consumed counters on both sides;
   any difference fails. D23 currently has no authority run: its local QSpec charge check
   remains mandatory, while its two-sided agreement is unavailable and cannot count as a pass.
   Name any other unavailable or unmetered authority case; it remains an agreement gap,
   not permission to omit the local QSpec assertions.
4. Check each operand-derived decimal amount exact and one under: add alignment, subtract
   cancellation, multiply, divide, negate with rounding and retain upscale.
5. Deny the digits charge of a `2^20` scale upscale and check no allocation reaches 4096 bytes.

## Expected Results

The planned agreement run shall account for all 23 vectors and name every admission-only,
unmetered or unavailable authority case, including D23. Value/outcome and charge agreement
shall be claimed only for vectors actually executed on both sides. Step 3 fails on any
local TC-185 charge/counter mismatch or any mismatch in an executed authority comparison. Local allocation evidence
alone does not meet this expected result (OPEN OWNER UNRESOLVED — planner ticket ID pending).
