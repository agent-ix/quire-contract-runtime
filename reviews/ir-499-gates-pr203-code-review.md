---
id: SR-20261104024421
title: IR-499 PR203 code-review review
type: SpecReview
analysis: code-review
scope: agent-ix/quire-contract-runtime; PR203; src/bin/check_linked_footprint.rs;
  spec/exact/matrix/TC-199-exact-no-std-and-footprint.md
review_set: subset
---

## Summary

Frozen head 4a19c442738d3d19d2b1541278a88264e00493ce; ticket IR-499, PR203. No applicable AssuranceProfile exists under spec/. Only the binary parser and TC-199 procedure step 3 changed. Consumer/snapshot sources, shared matrix, FR-275, dependencies, pins, CI and proof configurations are untouched.

## Verdict

PASS within the changed scope. No defects found; this is not whole-repository assurance or ticket closure.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Coverage

Rust lane: checked typed error propagation, exit 2 for missing inspection population, exact crate-name component recognition, existing overflow checks and panic matching, direct production-path assertions, absence of unsafe or concurrency changes, and no duplication or gate narrowing. Actual objdump -r / size -A output from the existing MSRV archive was inspected without building: the LTO-prefixed runtime object contains a dot-delimited quire_contract_runtime component and remains included; the footprint harness remains included; its .core- object and compiler_builtins objects remain excluded. Measured selected sections in the existing size log total 973 bytes. The UNKNOWN relocation type produced by this host objdump does not hide symbol names, so the panic-symbol check still sees its operands. Existing red-all.log shows precisely the three new tests fail against old production code; green.log records all five tests passing, no ignored tests. clippy.log and fmt-check.log record focused successful checks. real-artifact-controls.json records runtime-named panic exit 1 with the intended diagnostic and identical foreign-named panic exit 0 at 973 bytes. No legitimate observed runtime/header shape is lost by the changed matcher.

## Verification limits

No Cargo invocation, test rerun, make gate, global matrix/coverage scan, prover or new census was performed by this reviewer. Existing author logs were inspected; the lead owns one final scoped gate at the final reviewed head. Runtime-named versus foreign-named host panic objects appended to a real ARM archive are mixed-architecture CLI/parser controls, not a genuine ARM panic build. These controls establish neither all TC-199 nor whole FR-275-AC-9/10/11; Inspection obligations are not relabeled or credited. IR-499 remains open.

## Examined units

```yaml
scope:
- id: TC-199-step-3
  path: spec/exact/matrix/TC-199-exact-no-std-and-footprint.md
  role: examined
  excerpt: "3. Run `make size`; expect `.text` plus `.rodata` inside NFR-001-AC-3's\
    \ 500 byte to 4 KiB band and no\n   panic relocation (FR-275-AC-11). Run `cargo\
    \ test --locked --bin\n   check_linked_footprint` for the report-parser controls:\
    \ empty, malformed,\n   foreign-only and core-only relocation populations fail\
    \ as unavailable even\n   with an in-band size report; recognized runtime/harness\
    \ headers with no panic\n   relocation pass. A similarly named foreign crate contributes\
    \ neither linked\n   bytes nor panic references. The retained-panic and both size-bound\
    \ controls\n   must also pass. These parser contr"
- id: TC-199-parser-controls
  path: src/bin/check_linked_footprint.rs
  role: examined
  excerpt: missing_relocation_population_is_unavailable; foreign_panic_relocations_do_not_contaminate_runtime_objects;
    foreign_crate_name_does_not_contribute_linked_bytes
- id: footprint-parser
  path: src/bin/check_linked_footprint.rs
  role: examined
  excerpt: included_object; linked_section_bytes; panic_references; evaluate_reports;
    GateError::exit_code
- id: FR-275-AC-9
  path: spec/exact/functional/FR-275-single-exact-kernel.md
  role: context_only
  excerpt: With `exact` enabled and `std` disabled, the runtime builds for `thumbv7em-none-eabi`.
    The row `build-exact-no-std-msrv` builds it on the root `Cargo.toml` `rust-version`
    floor.
- id: FR-275-AC-10
  path: spec/exact/functional/FR-275-single-exact-kernel.md
  role: context_only
  excerpt: The dependency graph of `quire-contract-runtime-footprint` for `thumbv7em-none-eabi`
    contains no `quire-exact`, `quire-semantic-value`, `quire-canonical` or `quire-canonical-derive`,
    and after AD-004 step 5 contains no evaluation leaf.
- id: FR-275-AC-11
  path: spec/exact/functional/FR-275-single-exact-kernel.md
  role: context_only
  excerpt: '`make size` measures linked `.text` plus `.rodata` inside NFR-001-AC-3''s
    500 byte to 4 KiB band.'
```
