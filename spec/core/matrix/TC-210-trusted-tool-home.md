---
id: TC-210
title: "Preserve trusted host tool home"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-runtime/NFR-002
    type: verifies
---
# TC-210: Preserve trusted host tool home

## Description

Check NFR-002-AC-11 through the real Unix account lookup in the host gate tool.
The current test UID must have an OS account entry. The OS account database is
an external authority; this test does not verify its contents.

## Test Procedure

Run `cargo test --locked -p quire-contract-runtime-gates trusted_home_ignores_caller_home`.
The parent obtains the account home and starts the same test in a subprocess with
`HOME` pointing to a distinct temporary directory. The subprocess calls
`trusted_home` and checks the selected cargo path. Environment changes are confined
to the child process.

## Expected Results

The subprocess selects the OS account home and its `.cargo/bin/cargo`, despite the
misleading `HOME`. Both parent and subprocess pass. A replacement that trusts
`HOME` fails the subprocess home equality assertion.
