---
id: SR-35025
title: "Code review of IR-470 exact panic gate"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-runtime; Cargo.toml, Cargo.lock, Makefile, deny.toml, measurement/exact-panic/, scripts/check_exact_panic*.py, src/scalar/division.rs, tools/kernel-api-check/"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-runtime/NFR-002
    type: reviews
---

## Summary

Reviewed PR #165 at cee7730f8cb43231d17a0beff19f818c77def9a5 against NFR-002-AC-8, AC-9, AC-10 and TC-206. The exact probe and its injected control passed, but two existing aggregate gates fail on this head.

## Verdict

**FAIL** — workspace Clippy and license checks reject the change.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | Removing the large-enum expectation makes workspace Clippy fail with `-D warnings` | src/scalar/division.rs:31 |
| FND-002 | high | Both new AGPL workspace crates lack scoped license exceptions, so `make deny` fails | deny.toml:21 |

## Coverage

- NFR-002-AC-8 examined: `make exact-panic` built a no-std exact archive, reported fourteen RT objects and required calls, and found zero recognized RT panic relocations.
- NFR-002-AC-9 examined: the scratch injected panic was detected; the unmodified probe passed on a subsequent build.
- NFR-002-AC-10 examined: one positive kernel unit test and three compile-fail doctests passed.
- `cargo fmt --check` passed; `cargo clippy --workspace --all-targets --all-features -- -D warnings` failed at `IntegerDivisionConsumer`; `make deny` failed on the two new crates.
- No CI workflow, `src/exact/expression.rs`, or `clippy.toml` changed. PR #161 touches `src/exact/expression.rs`; #162 touches Cargo.toml, Cargo.lock and clippy.toml, so the latter may need rebase reconciliation.

## Dispositions

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | fixed | A reasoned local `clippy::large_enum_variant` allowance restores workspace Clippy on the public by-value enum; checked at 39cf1a1aa17acb0de88dc299adb279128a3462f7. |
| FND-002 | fixed | `deny.toml` now lists only the two new first-party crates as AGPL exceptions; `cargo deny --workspace check licenses bans sources` passes. The installed Quire v0.36.2 lacks the separate `check-lock` subcommand, so the composite `make deny` remains externally blocked. |

## Follow-up verification

At d6d27b8e5f357a664a37b70bfbdcd2c9c98a5c76, `scripts/check_exact_panic.py` adds `CheckedPackage>::check_expression` to the required probe relocations. The consumer already calls `check_expression` before `evaluate`. A focused `make exact-panic-probe` passed and reported that relocation among the required calls, zero recognized RT panic relocations, and detection of the scratch injected panic. No new code finding arose from this delta.

At 075929af662d7767857080283fee39a388195b42, the parser requires relocation column headers and rejects malformed lines in RT-owned relocation tables. Focused `make exact-panic-probe` parsed the legitimate `objdump -rC` output and passed. Its same-archive control injected a malformed RT row into the positive report and observed the scanner's specific malformed-relocation rejection; the reachable panic control still failed as intended. No new code finding arose from this delta.
