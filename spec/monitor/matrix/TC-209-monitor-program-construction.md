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

## Evidence Status

**Planned: incomplete whole-case evidence.** The retained complete `monitor_program`
run in [quire-integration PR #41](https://github.com/agent-ix/quire-integration/pull/41)
reports 86 passed, zero failed and zero ignored against the coherent
producer/reader/runtime cohort (quire-contract-runtime 0.1.0). The named controls
back the original procedures 2 through 4 and interface-002-AC-2 through AC-4. The
cross-function source-location controls in procedure 3 and interface-002-AC-5/AC-6
remain Planned until the RT representation change and a direct RT construction
test land. Procedure 1 and
interface-002-AC-1 are backed in quire-integration `tests/monitor_program.rs` by
`retained_program_outlives_preparation_and_keeps_four_entry_keys`: after the
preparation parts are consumed and one `Rc` owner drops, it asserts the root key,
package lock, occurrence, the original clock, admitted-order and progress
selection bytes against literals written in the test (not values read back from the
returned Layout), the four entry associations, literal bodies, shared nominal
declarations and absent invalid arena lookups. The complete TC-209 case stays
Planned because interface-002-AC-5/AC-6 remain Planned. No native rerun was needed
for this evidence update.

PR #41 is now published on integration main with the evaluated source unchanged.
The unchanged main-only guard accepted the clean owner main checkout, and the
completed strict reference scan reports 161 unbacked rows and zero contradicted
statuses (exit 1). This is a completed scan, not a passing global coverage gate.
It does not fill the missing retention assertions or accept every IT-018 intention:
the source-body provenance finding in IT-018-SC-15 also remains open. TC-200 and
all FR-276 temporal criteria remain Planned independently. No source tag,
diagnostic literal arena or landing sequence supplies that missing evidence.

## Test Procedure

Run direct `Program::try_new` construction controls in RT for the source-location
rule and the complete `monitor_program` target in `quire-integration` (IT-018 / TC-098)
for the existing producer/reader boundary. The latter's live QSL producer and strict
IR reader provide semantic identities and their matching declaration/unit meanings.
The direct RT control uses original `NodeKey` values as inputs, not a copied QSL
fixture or reminted key. The derived layout-key probe and ordinary diagnostic record
shapes test construction storage; they do not claim an emitted temporal plan or
FR-487 activation/result schema.

1. Move owned function/expression arenas and the shared declaration/unit environment
   into Program, drop preparation storage and one Rc owner, then inspect the surviving
   owner. Check the complete nested root key, selection bytes, every entry association,
   function signature/body location, literal State/Results contents and declarations.
   Invalid lookups return absent.
2. Refuse a missing, extra, optional, mistyped or misordered outer State field,
   including event_position missing, non-Integer or before activations. Replace the fourth entry with an absent function or wrong signature. Independently
   change counter width, gap, uncertainty, and Time unit identity. Retain activation
   capacities zero and above u64::MAX; refuse a negative activation capacity. Keep
   the actually emitted negative-scale and affine Time units without adding an
   upstream admission restriction.
3. Inject duplicate function keys and duplicate expression source locations within
   one derived function context, between two neutral copies, and between a neutral
   and a bound copy. Inject invalid function/body/Call/If addresses, a direct
   expression cycle, mixed binding ownership, a foreign function argument and a
   foreign local. Keep valid shared neutral DAG nodes and distinct inline body
   paths. Construct two functions whose State and delta parameter expressions and
   parameter-dependent IntegerAdd expressions share their exact admitted source
   keys and body paths, but whose transitive owners are distinct FunctionIds.
   Inspect both retained expressions and functions to confirm the original source
   locations and separate operand associations. Validate a 20,000-level shared
   inline arena on a 512 KiB native stack.
4. Bind two signature-compatible entry roles to one retained function.

## Expected Results

- The survivor retains every supplied entry and actual literal content, without
  executing init or allocating monitor state.
- Invalid construction returns the precise typed code before execution; exact premises
  are not silently truncated to host integers or replaced by defaults.
- Each malformed closure is refused by its typed duplicate-location or binding
  code. The two function-scoped copies are admitted with unchanged original source
  locations; finite neutral sharing, deep flat arenas and distinct inline locations
  remain accepted without recursive graph descent.
- Compatible role aliases remain valid. No distinct-function premise is invented.
- No construction result is reported as FR-276 build/step acceptance. TC-200 and
  FR-276-AC-1 through AC-11 remain required independently.
