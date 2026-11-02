---
id: SR-20261102072217
title: "IR-722 event-position EARS amendment review"
type: SpecReview
analysis: ears-conformance
scope: "agent-ix/quire-contract-runtime; IR-722; FR-276 event-position amendment"
review_set: subset
---

## Summary

Reviewed new event-position SHALL statements and AC-11. The responses are concrete and distinguish count from tick time; one new sentence combines two obligations. Engine validation of the exact amended FR snapshot succeeded and reported one non-singular warning at pre-existing Reporting line 85; that baseline warning is context only. The new sentence below is a semantic EARS judgment, not attributed to the engine.

## Verdict

**CONDITIONAL** — split the new compound sentence. Execution acceptance remains open.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The new event-position sentence combines initializer and admitted-event obligations with two SHALL clauses; split it into independently stated obligations so requirement grammar and verification attribution stay singular. | spec/monitor/functional/FR-276-consume-timed-monitor-plan.md:87 |

## Examined scope

```yaml
scope:
- id: FR-276
  path: spec/monitor/functional/FR-276-consume-timed-monitor-plan.md
  role: examined
  excerpt: "`init` SHALL set it to 0, and\n  `on_event` SHALL add 1 per admitted event,\
    \ including the baseline reading's\n  `on_event(0)`."
- id: FR-276-AC-11
  path: spec/monitor/functional/FR-276-consume-timed-monitor-plan.md
  role: examined
  excerpt: Build observes event_position 0 from init. With width 16 and maximum gap
    100, the baseline event reading 65530 calls on_event(0) and leaves event_position
    1; a watermark and poll leave it 1; reading 65500 faults through on_gap and leaves
    it 1; admitted reading 65510 calls on_event(10) and leaves it 2.
```

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Verified separate init and on_event obligation statements in 23a4cf5523644e3c4ca31065d7cffbe216fefe4d. |

## Final event-position review carry

Final event-position carry inspected 23a4cf5523644e3c4ca31065d7cffbe216fefe4d. The initializer and admitted-event increment are separate obligation statements with unchanged semantics. EARS FND-001 is fixed. The only engine grammar warning in supplied validation remains the pre-existing Reporting sentence at line 85, not this amendment. No RT production change after reviewed 8e0ff8fb; the closed State validator and shared-field ownership remain unchanged. All prior RT code findings remain fixed; no new defect found.

Integration final ordinary aggregate evidence at 33020725707bd6e559c00090fc33138f6a0ec5a3 reports exit 0, 84 passing, zero failed and six existing ignored; fourteen construction controls pass. Ignored proofs are not credited. State initialization literal retention does not prove event increments: FR-276-AC-1 through AC-11 remain open execution obligations on IR-722. Full-delivery gap stays deferred/open. RT strict-coverage authorization and stopped heavy IEEE proof remain unresolved; user merge hold persists. This is a construction review result, not all-gates-green or merge authorization. Reviewer ran no Cargo/Kani and edited no source.

Current method verdict: **PASS** for construction amendment review, subject to unresolved aggregate gates and merge hold.

```yaml
dispositions:
- fnd: FND-001
  outcome: fixed
  after_excerpt: "- `init` SHALL set `event_position` to 0.\n- `on_event` SHALL add\
    \ 1 to `event_position` per admitted event, including the baseline reading's\n\
    \  `on_event(0)`."
```
