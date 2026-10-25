---
id: TC-206
title: "Inspect the linked exact-profile panic surface"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-runtime/NFR-002
    type: verifies
---
# TC-206: Inspect the linked exact-profile panic surface

## Description

Exercise an exact-enabled, no-std runtime consumer separately from the
default-footprint population. Check the retained runtime-owned objects for
recognized Rust panic relocations, and check the QSL-owned integer division
admission contract at the resolved `quire-exact` dependency. This finite probe
does not certify unretained monomorphizations or third-party object files.

## Test Procedure

1. On the manifest-declared MSRV, build a static-library consumer for
   `thumbv7em-none-eabi` with `exact` enabled and `std` disabled. Its exported
   entry takes opaque inputs and retains calls into each runtime-owned `scalar`
   family: integer-division negotiation, IEEE negotiation, and the lazy Boolean
   connective. While the runtime still owns an exact expression surface,
   retain its checked-package admission and evaluation call as well. Keep the
   arguments and results observable so release optimization cannot remove
   those calls. After FR-275 removes that residue, the population contains the
   `scalar` calls and no obsolete expression call.
2. Inspect the built archive with a tool that reports object ownership and
   relocation targets. Select the retained `quire_contract_runtime` object
   files, record their names and the exact command used, and reject a zero or
   missing population. Search their relocation targets for `rust_begin_unwind`,
   `panic_bounds_check`, `core::panicking` and `slice::index::fail` (including
   mangled spellings recognized by the existing footprint checker). Reject
   any match. Keep the third-party objects outside this RT-owned relocation
   verdict and report them separately if inspected.
3. Inject a reachable `panic!` into a scratch copy of a runtime-owned function
   called by the probe, rebuild the same profile, and require the exact gate
   to reject its relocation. The scratch copy is removed after the control;
   it is not committed. Missing archive, missing inspection tool, unparsable
   output or absent expected RT call is a failed gate, not a pass.
4. Against the resolved `quire-exact` crate, compile a direct caller that
   creates zero and checks `NonZeroInteger::try_from(&zero) == Err(ZeroDivisor)`.
   Compile a nonzero caller that reuses an admitted token for `exact_div`,
   `div_rem_truncating` and `div_mod_floor`. Compile-fail callers pass a plain
   `&Integer` to each helper and must be rejected by the type checker. These
   controls test the public helper boundary, independently of the higher-level
   operation's `Undefined::DivisionByZero` check. Update the runtime's resolved
   dependency before claiming this control if its current lock predates the
   QSL-owned admission API.

## Expected Results

The unmodified exact probe builds with no `std`, retains all applicable
runtime-owned calls and has no recognized panic relocation in those retained
RT objects. The injected panic is detected. Zero cannot enter the three
public kernel division helpers; a nonzero token works. The default
NFR-001-AC-3 size band and FR-275-AC-10 dependency exclusion remain measured
only by their existing default-profile gate.
