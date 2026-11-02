---
id: interface_012
title: "Admitted input association and atomic monitor entry dispatch"
type: interface
relationships:
  - target: ix://agent-ix/quire-contract-runtime/FR-276
    type: references
  - target: ix://agent-ix/quire-contract-runtime/interface_003
    type: references
---
# [interface-012] Admitted input association and atomic monitor entry dispatch

## Contract

```yaml
name: MonitorEntryDispatch
ownership: quire-contract-runtime
feature: exact
associated_types: [Layout, Monitor, InputContext, Reading, Event, Watermark, MonitorStep, MonitorFailure]
operations:
  - name: event
    inputs: [monitor, event]
    output: MonitorStep
    semantics: compare the admitted association and atomically execute the selected event entry
  - name: watermark
    inputs: [monitor, watermark]
    output: MonitorStep
    semantics: apply an admitted inclusive progress assertion without moving the reading baseline
  - name: poll
    inputs: [monitor, reading]
    output: unit
    semantics: validate a bare counter reading without consuming an event or changing any monitor state
invariants:
  - source admission and independent selection remain the driver authority
  - shape or nonempty evidence alone grants no execution authority
  - canonical clock-binding components are values, never a new binding identity
  - only a completed entry commits state, results and an admitted reading baseline
```

## Operations

### event

Layout additionally retains the original canonical value encodings
`clock_binding`, `admitted_order` and `progress_context`. These are the driver's
independently admitted associations, not hashes, display strings, fresh IDs or
another plan/State DTO. Clock-binding encoding contains exactly the components
admitted by the selected QSpec FR-252 profile; absence remains explicit, and no
nonsemantic annotation participates. The supplying authority owns its canonical
value encoding. Byte equality therefore compares the complete original values;
RT introduces neither an authority-document parser nor another normalization.

InputContext borrows the attached package lock, clause, subject, occurrence,
clock-binding value, counter, tick unit, counter width, profile and admitted
order value. Event supplies that context, the exact raw reading and canonical
Inputs. The driver admits the complete event and its independent association
before calling RT. RT compares every attached component before execution,
rejects an out-of-width reading without truncation and checks required Inputs
with native bounded membership. Invalid input leaves state and baseline unchanged
and produces no obligation result. Order is never inferred from arrival or tick
comparison.

A fresh monitor has no reading baseline. Its first admitted event passes delta 0
to on_event with no gap check. Later differences are computed modulo the declared
counter width, including width 64 without an overflowing shift. Successful
on_event commits only its returned canonical State and Results and the exact
admitted reading as the new baseline. The State's unwrapped event_position is
updated by the source entry, not a host counter or Layout field.

A gap routes to the fourth entry only after the same complete input admission.
Its faulting reading becomes the baseline only when on_gap completes. It is not
an admitted event; no on_event call or host-generated settlement is made. Gap
implementation and controls are the final dispatch slice.

### watermark

Watermark supplies the same InputContext, the original admitted progress context,
a nonempty original authority assertion, its admitted `inclusive_boundary:
Integer` and canonical Scope. The driver validates QSpec FR-094/FR-160 authority,
required-source/subject/scope/execution coverage and any FR-289 half-open mapping
before transfer. A bare watermark number, EOF, shutdown or a half-open endpoint
is not this API's admitted inclusive progress. The complete context must match
Layout; a regressing boundary refuses without execution or state change.
Same-revision authority conflicts remain refused by the supplying admission.

The entry receives State, the exact inclusive boundary and Scope. A successful
step returns exactly its source Results and replaces State and the remembered
progress boundary, without moving the counter baseline. Repeated pending results
and settlements remain source-program semantics. RT neither derives temporal
meaning nor invents an instance identity.

### poll

Poll compares context and reading range only. It executes no source entry,
returns no results and changes neither canonical State, reading baseline nor
progress boundary. An eventual first event still uses delta 0.

## Failures and ownership

MonitorFailure is the common typed execution envelope; the earlier draft's
build-specific name is replaced without an alias. Input refusal, operating stop,
native admission failure and executor invariant failure remain distinguishable.
A source-body invariant terminates the monitor; later inputs refuse. Input
refusal and resource stop do not terminate it. Operating limits remain supplied
at build, apply to required native input checking and selected entry work, and
can be replaced atomically. No arbitrary allocator-OOM recovery is promised.

An operating stop releases invocation scratch, leaves the event/watermark
unconsumed and preserves canonical State, reading baseline and progress. The
same machine may accept a later valid input without stale lexical/fold state;
re-presenting the unconsumed input executes its transition once per attempt.
A successful MonitorStep owns the original Results and retains the immutable
Program association, including original package/clause/subject/occurrence keys,
so equal local result fields from foreign plans do not lose their qualification.
No new PlanId or Outcome/activation-status mirror is introduced.

## Features

| Feature | Kind |
| --- | --- |
| event | operation |
| watermark | operation |
| poll | operation |

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| interface-012-AC-1 | Changed package/clause/subject/occurrence/clock/counter/unit/width/profile/order or invalid required values refuse before entry execution, with state and reading/progress baselines unchanged. | Test |
| interface-012-AC-2 | Baseline events use delta zero, later admitted readings use the declared modular width, and polls or valid inclusive watermarks never move that reading baseline; regressing or foreign progress cannot settle. | Test |
| interface-012-AC-3 | A stopped event/watermark is unconsumed, preserves all committed state and allows same-machine lexical/fold re-entry; a source invariant terminates subsequent input and remains distinct from input/resource failures. | Test |
| interface-012-AC-4 | A completed step returns original canonical Results with its retained program association after preparation drops, preserving foreign-plan qualification without a replacement identity or temporal verdict. | Test |
