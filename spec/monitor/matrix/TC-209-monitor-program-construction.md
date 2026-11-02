---
id: TC-209
title: "Retained monitor program construction"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-runtime/interface_002
    type: verifies
---
# TC-209: Retained monitor program construction

## Description

Exercise the public construction boundary of interface-002. These are construction
controls, not the event, watermark, gap or real-producer qualification of TC-200.
The latter remains the complete FR-276 acceptance procedure.

## Test Procedure

1. Move owned function/expression arenas and the shared declaration/unit environment
   into Program, drop preparation storage and one Rc owner, then inspect the surviving
   owner. Check the complete nested root key, selection bytes, every entry association,
   function signature/body location, literal State/Results contents and declarations.
   Invalid lookups return absent.
2. Replace the fourth entry with an absent function or wrong signature. Independently
   change counter width, gap, uncertainty, and Time unit identity. Retain activation
   capacities zero and above u64::MAX; refuse a negative activation capacity.
3. Inject duplicate function keys and duplicate expression source locations; inject
   invalid function/body/Call/If addresses, a direct expression cycle and a foreign
   function argument. Keep valid shared DAG nodes and distinct inline body paths.
   Validate a 20,000-level shared inline arena on a 512 KiB native stack.
4. Bind two signature-compatible entry roles to one retained function.

## Expected Results

- The survivor retains every supplied entry and actual literal content, without
  executing init or allocating monitor state.
- Invalid construction returns the precise typed code before execution; exact premises
  are not silently truncated to host integers or replaced by defaults.
- Each malformed closure is refused; finite sharing, deep flat arenas and distinct
  inline locations remain accepted without recursive graph descent.
- Compatible role aliases remain valid. No distinct-function premise is invented.
- No construction result is reported as FR-276 build/step acceptance. TC-200 and
  FR-276-AC-1 through AC-10 remain required independently.
