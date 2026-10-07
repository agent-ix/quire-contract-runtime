---
id: SR-2392
title: Independent IR-647 scope-boundary review
type: SpecReview
analysis: scope-boundary
scope: agent-ix/quire-contract-runtime@ae702513b86ab0978d9425e6b24a91e105aca6b7; full
  diff 3cc88c480d4efe5179fe91acf4634def5d2d82fd..HEAD; reviews/sr-2380-ir-647-code-review.md,
  reviews/sr-2381-ir-647-spec-review.md, reviews/sr-2382-ir-647-ears-conformance.md,
  reviews/sr-2383-ir-647-integrity.md, reviews/sr-2384-ir-647-matrix.md, spec/exact/functional/FR-006-exact-outcomes-and-accounting.md,
  spec/exact/functional/FR-007-exact-scalar-families.md, spec/exact/functional/FR-011-meter-state-at-a-stop.md,
  spec/exact/matrix/TC-016-exact-outcome-envelope.md, spec/exact/matrix/TC-017-exact-metering.md,
  spec/exact/matrix/TC-018-integer-division-agreement.md, spec/exact/matrix/TC-019-decimal-agreement.md,
  spec/exact/matrix/TC-020-ieee-agreement.md, spec/exact/matrix/TC-021-text-enum-agreement.md,
  spec/exact/matrix/TC-022-quantity-agreement.md, spec/exact/matrix/TC-023-metered-arithmetic.md,
  spec/exact/matrix/TC-032-meter-state-at-a-stop.md, spec/exact/matrix/TC-035-boxed-debug-parity.md,
  spec/exact/matrix/tests.md
review_set: subset
---

## Summary

Ticket: IR-647. RT core owns lazy right-closure invocation/stop propagation (FR-011-AC-3/AC-8) and backend negotiation; quire-exact core owns outcome, meter, numeric operations and already-decided Boolean retention. QSpec is the semantic/accounting dependency and QSL the independent authority. Kernel contracts are consumed/assumed at RT’s end state, rather than guaranteed by retained RT kernel tests; current RT binders record local pre-removal evidence only. IR-669 owns cross-repo agreement composition, currently absent, without a guarantee claim. IR-349 step 2/IR-583 own residue removal. No new external component or relationship edges are introduced; retained contract boundaries are explicit and no compatibility path is added.

## Verdict

PASS for this method; overall candidate FAIL on SR-2386/SR-2388, with SR-2385 hygiene.

## Scope

```yaml
scope:
- id: SR-2380 Verdict 1.1
  path: reviews/sr-2380-ir-647-code-review.md
  role: context_only
  excerpt: '**PASS** — no code defect: the change is spec-only, removes no implementation
    or test, and contains no copied file, vendored vector or digest/pin.'
- id: SR-2380 Findings 1.1
  path: reviews/sr-2380-ir-647-code-review.md
  role: context_only
  excerpt: '| ID | Severity | Summary | Refs |

    | --- | --- | --- | --- |

    | FND-001 | low | No findings (placeholder) | - |'
- id: SR-2381 Verdict 1.1
  path: reviews/sr-2381-ir-647-spec-review.md
  role: context_only
  excerpt: '**FAIL** — one finding here; the contradiction in SR-2383 (high) and the
    matrix findings in SR-2384 also block review-clean.'
- id: SR-2381 Findings 1.1
  path: reviews/sr-2381-ir-647-spec-review.md
  role: context_only
  excerpt: '| ID | Severity | Summary | Refs |

    | --- | --- | --- | --- |

    | FND-001 | medium | TC-019 step 3 now only records which D09/D13/D20–D23 charges
    can be compared instead of checking they agree, so no outcome of the step can
    fail while FR-007-AC-2 still requires D20–D21 ordering and D22–D23 upscale charges
    to agree with TC-185; the weakening is outside IR-647''s ownership scope and should
    either keep a pass/fail check for the authority-metered vectors or be justified
    against FR-007-AC-2. | spec/exact/matrix/TC-019-decimal-agreement.md:32-35 |'
- id: SR-2382 Verdict 1.1
  path: reviews/sr-2382-ir-647-ears-conformance.md
  role: context_only
  excerpt: '**PASS** — each amended statement has one system subject, one `shall`
    response and, where present, a single `When` trigger.'
- id: SR-2382 Findings 1.1
  path: reviews/sr-2382-ir-647-ears-conformance.md
  role: context_only
  excerpt: '| ID | Severity | Summary | Refs |

    | --- | --- | --- | --- |

    | FND-001 | low | No findings (placeholder) | - |'
- id: SR-2383 Verdict 1.1
  path: reviews/sr-2383-ir-647-integrity.md
  role: context_only
  excerpt: '**FAIL** — FR-006 now forbids what unchanged FR-011-AC-5 requires and
    current code implements.'
- id: SR-2383 Findings 1.1
  path: reviews/sr-2383-ir-647-integrity.md
  role: context_only
  excerpt: '| ID | Severity | Summary | Refs |

    | --- | --- | --- | --- |

    | FND-001 | high | FR-006 Behavior now states unconditionally that RT shall not
    require a production log or define a local `CHARGE_LOG_CAPACITY`, contradicting
    unchanged FR-011-AC-5 (Past `CHARGE_LOG_CAPACITY` admitted charges the log holds
    exactly the first 4096 points), FR-006''s own Outputs (admitted charge sequence
    on the `Meter`) and FR-006 Behavior line 60, and the current code (`pub const
    CHARGE_LOG_CAPACITY` at src/exact/accounting.rs:478) whose rows stay ✅ implemented;
    condition it on IR-349 completion or amend FR-011-AC-5'
- id: SR-2383 Findings 1.2
  path: reviews/sr-2383-ir-647-integrity.md
  role: context_only
  excerpt: 'consistently. | spec/exact/functional/FR-006-exact-outcomes-and-accounting.md:47-49
    |

    | FND-002 | low | The amendment assigns the remaining RT evaluation residue to
    QSL-358 work ("residue is QSL-358 work", "remain QSL-358 residue until removed",
    "removal tracked under QSL-358"), but QSL-358 is Done (2026-10-02) and covered
    QSL-side extraction; RT''s own residue deletion has no open owner named. | spec/exact/functional/FR-006-exact-outcomes-and-accounting.md:89-91
    |'
- id: SR-2384 Verdict 1.1
  path: reviews/sr-2384-ir-647-matrix.md
  role: context_only
  excerpt: '**FAIL** — the RT-owned lazy-connective clause has no binding TC, and
    the planned agreement work is attributed to a closed ticket.'
- id: SR-2384 Findings 1.1
  path: reviews/sr-2384-ir-647-matrix.md
  role: context_only
  excerpt: '| ID | Severity | Summary | Refs |

    | --- | --- | --- | --- |

    | FND-001 | medium | FR-007-AC-7 now adds an RT-owned lazy-connective clause but
    its Verification still names only TC-023, which the amendment says leaves RT in
    IR-349 and whose step 3 "is not its retained evidence", while tests.md says TC-032
    is not a binder for FR-007-AC-7; after IR-349 the permanent RT-owned clause has
    no verifying TC. Add TC-032 to AC-7''s verification (and tag the retained test)
    or split the lazy clause into its own criterion. | spec/exact/functional/FR-007-exact-scalar-families.md:101
    |

    | FND-002 | medium |'
- id: SR-2384 Findings 1.2
  path: reviews/sr-2384-ir-647-matrix.md
  role: context_only
  excerpt: 'Changed status cells and new ownership text say QSL shared-corpus agreement
    "remains planned (Linear IR-430)" and "stays separate under IR-430", but IR-430
    is Done (2026-09-30) and tracked RT PR #88''s removal of qsl-agreement, not its
    recreation; no open Linear ticket owns the quire-integration agreement oracle,
    so the planned status names a closed, non-owning ticket. | spec/exact/matrix/tests.md:17
    |'
- id: FR-006 Description 1.1
  path: spec/exact/functional/FR-006-exact-outcomes-and-accounting.md
  role: examined
  excerpt: 'When a generated oracle evaluates a complete-V1 scalar operator through
    the optional `exact`

    feature, the runtime shall consume the authoritative `quire-exact` outcome and
    metering

    contracts for every charge named by

    `quire.value.accounting/v1` in agent-ix/quire-specification

    (`proposals/quire-v1/definitions/value-accounting.md`). The runtime consumes the
    kernel

    implementation of that definition: values and outcome kinds agree with the

    quire-spec-language authority (FR-007), and charge schedules are taken from the
    QSpec definition.'
- id: FR-006 Inputs 1.1
  path: spec/exact/functional/FR-006-exact-outcomes-and-accounting.md
  role: examined
  excerpt: "- Operands of one exact scalar family, a declared result domain where\
    \ the family has one, and a\n  `ScalarLimits` tuple with the ten counters in `ScalarLimitsV1`\
    \ field order.\n- Optionally, one injected denial naming a charge point and a\
    \ 1-based occurrence."
- id: FR-006 Outputs 1.1
  path: spec/exact/functional/FR-006-exact-outcomes-and-accounting.md
  role: examined
  excerpt: "- `Outcome<T>`: `Completed(T)`, `Undefined(Undefined)`, `Refused(Refusal)`\
    \ or\n  `Incomplete(Incomplete)`, plus the admitted charge sequence and consumed\
    \ counters on the `Meter`.\n- Ill-typed operand combinations are reported before\
    \ evaluation, beside the outcome, as `IllTyped`."
- id: FR-006 Behavior 1.1
  path: spec/exact/functional/FR-006-exact-outcomes-and-accounting.md
  role: examined
  excerpt: "- A completed `false` is a value, never a refusal. `Undefined`, `Refused`\
    \ and `Incomplete` carry\n  closed, typed reasons; no variant carries a message\
    \ string.\n- A `Refusal` carries a normative `refused { code }` spelling exactly\
    \ where the language\n  defines one. The language defines four — `ieee_nan_payload_not_representable`,\n\
    \  `ieee_rational_out_of_domain`, `foreign_reference` (FR-149) and `cardinality_out_of_bound`\n\
    \  (FR-272) — and `Refusal::code()` is `Some` for exactly those four variants\
    \ and `None` for every\n  other. `None` means \"the language names no code for\
    \ this refusal\", not \"this"
- id: FR-006 Behavior 1.2
  path: spec/exact/functional/FR-006-exact-outcomes-and-accounting.md
  role: examined
  excerpt: "refusal has no\n  reason\": the typed variant is always the reason. An\
    \ oracle that must emit `refused { code }` for\n  a variant the language does\
    \ not spell shall report the absence rather than invent a code, and\n  the normative\
    \ vocabulary remains an obligation when kernel evidence leaves RT.\n- Each charge\
    \ is decided before the work it pays for, and each size amount is derived before\
    \ the\n  value it measures is materialized. A denied charge consumes nothing and\
    \ exposes no partial value.\n- The kernel owns meter storage and accounting. Its\
    \ bounded diagnostic charge log is exposed\n  under"
- id: FR-006 Behavior 1.3
  path: spec/exact/functional/FR-006-exact-outcomes-and-accounting.md
  role: examined
  excerpt: "`quire-exact`'s `test-support` feature. Until IR-349 removes the local\
    \ meter, its\n  admitted-charge sequence and `CHARGE_LOG_CAPACITY` remain governed\
    \ by FR-011-AC-5. After\n  that removal, RT shall consume the kernel's test-support\
    \ diagnostic log rather than require\n  a production log or define a local capacity\
    \ constant. Consumed counters remain exact.\n- Size counters are high-water marks;\
    \ `work_units` and `result_units` are cumulative. The\n  `Incomplete` record names\
    \ the first unavailable counter in field order, its limit, the consumed\n  amount\
    \ before the charge, the exact denied amount (a"
- id: FR-006 Behavior 1.4
  path: spec/exact/functional/FR-006-exact-outcomes-and-accounting.md
  role: examined
  excerpt: "mathematical integer) and the charge point.\n- Result-domain membership\
    \ is decided without a charge, after the arithmetic charges and before\n  retention;\
    \ an out-of-domain result is refused with no result unit.\n- An injected denial\
    \ at `(point, occurrence)` yields `Incomplete` on `work_units` at that point with\n\
    \  counters unchanged by the denied charge. Its precedence over a real short counter,\
    \ the contents\n  of its record and how occurrences are counted are FR-010.\n\
    - What the meter holds at an `Undefined` or `Refused` stop, the atomicity of one\
    \ charge, the\n  field-order scan over a charge's own"
- id: FR-006 Behavior 1.5
  path: spec/exact/functional/FR-006-exact-outcomes-and-accounting.md
  role: examined
  excerpt: "size vector, the value and truncation behavior of\n  `CHARGE_LOG_CAPACITY`,\
    \ and which derived amounts saturate are FR-011.\n- The `exact` surface uses no\
    \ host floating point, no `std`, no `unsafe` and no intentional panic\n  path,\
    \ and is re-exported from private modules only."
- id: FR-006 Acceptance Criteria 1.1
  path: spec/exact/functional/FR-006-exact-outcomes-and-accounting.md
  role: examined
  excerpt: '| ID | Criteria | Verification |

    |----|----------|--------------|

    | FR-006-AC-1 | The four outcome dispositions are distinct; completed `false`
    is a value; refusal, undefined and incomplete reasons are closed enums. | Test
    (TC-016) |

    | FR-006-AC-2 | Ill-typed operand combinations are reported before evaluation
    with zero charges, and provenance-bearing refusals (invalid UTF-8 offset, stale
    identity) are typed. | Test (TC-020, TC-021, TC-022) |

    | FR-006-AC-3 | Every charge point and limit kind round-trips its QSpec spelling;
    charges precede work; size counters are high-water, work/result'
- id: FR-006 Acceptance Criteria 1.2
  path: spec/exact/functional/FR-006-exact-outcomes-and-accounting.md
  role: examined
  excerpt: 'cumulative; the first short counter in field order is reported with the
    exact denied amount. | Test (TC-016, TC-017) |

    | FR-006-AC-4 | An injected denial at any admitted charge point yields `Incomplete`
    on `work_units` naming that point, with no result units and no partial value.
    | Test (TC-017) |

    | FR-006-AC-6 | `Refusal::code()` is `Some` for exactly `IeeeNanPayloadNotRepresentable`,
    `IeeeRationalOutOfDomain`, `ForeignReference` and `CardinalityOutOfBound` with
    their normative spellings, and `None` for all nine other variants. | Test (TC-016)
    |'
- id: FR-006 Kernel ownership 1.1
  path: spec/exact/functional/FR-006-exact-outcomes-and-accounting.md
  role: examined
  excerpt: 'The runtime shall consume `quire-exact` directly for kernel outcomes,
    charges and scalar

    operations; it shall define no replacement kernel or re-export path ([FR-275](./FR-275-single-exact-kernel.md)).

    The criteria above remain obligations on that consumed behavior. They do not require
    RT-local

    kernel tests. [The exact matrix](../matrix/tests.md) records current local evidence
    separately

    from its disposition after the implementation removes the copy.'
- id: FR-006 Kernel ownership 2.1
  path: spec/exact/functional/FR-006-exact-outcomes-and-accounting.md
  role: examined
  excerpt: 'Owner references cover only matching subsets: `ix://agent-ix/quire-exact/FR-358`
    owns one-shot

    injected denial and the bounded test-support log; `ix://agent-ix/quire-exact/FR-359`
    owns

    cumulative-counter boundary refusal and atomicity; `ix://agent-ix/quire-exact/FR-361`
    owns the

    specified allocation bounds on denied large work. These references do not establish
    the full

    outcome/reason vocabulary, every charge spelling, or the ill-typed agreement claim.
    Unmapped

    obligations remain explicit in TC-016/TC-017; no upstream verification status
    is inferred.'
- id: FR-006 Kernel ownership 3.1
  path: spec/exact/functional/FR-006-exact-outcomes-and-accounting.md
  role: examined
  excerpt: 'This amendment deletes no implementation or test. The remaining local
    kernel copy is IR-349

    work; remaining RT evaluation-residue deletion is open IR-349 step 2 and IR-583
    (backlog) work. QSL-358 is

    Done and covered QSL-side extraction, not the remaining RT deletion. Neither copy
    is RT-owned

    by remaining present. IR-430 is Done and covered removal of RT agreement tests;
    the remaining

    quire-integration agreement work has open quire-integration ticket IR-669.'
- id: FR-006 Dependencies 1.1
  path: spec/exact/functional/FR-006-exact-outcomes-and-accounting.md
  role: examined
  excerpt: "- **Upstream**: [FR-002](../../core/functional/FR-002-safe-operators.md);\n\
    \  `ix://agent-ix/quire-specification` (`value-accounting.md`)."
- id: FR-007 Description 1.1
  path: spec/exact/functional/FR-007-exact-scalar-families.md
  role: examined
  excerpt: 'When the `exact` feature is enabled, the runtime shall consume `quire-exact`
    for kernel-owned

    operations in the scalar families of

    complete V1 in agent-ix/quire-specification with values and outcome kinds equal
    to the

    quire-spec-language authority on every shared-corpus vector, and charges equal
    to the QSpec

    accounting schedule.'
- id: FR-007 Inputs 1.1
  path: spec/exact/functional/FR-007-exact-scalar-families.md
  role: examined
  excerpt: "- Operands of one family and its declared type: integer division profiles,\
    \ `Decimal[..]`,\n  IEEE 754-2019 profiles, `Text[..]` under a Unicode 17 profile,\
    \ enum declarations, unit graphs\n  and quantities, `Integer`/`Int[..]`, `Rational[..]`\
    \ and `Boolean`."
- id: FR-007 Outputs 1.1
  path: spec/exact/functional/FR-007-exact-scalar-families.md
  role: examined
  excerpt: '- An FR-006 outcome for each operation.'
- id: FR-007 Behavior 1.1
  path: spec/exact/functional/FR-007-exact-scalar-families.md
  role: examined
  excerpt: "- Integer division and modulus follow the truncating, floor and Euclidean\
    \ definitions. The three\n  laws are selectable for `div`/`rem` only. `mod` is\
    \ **always** the Euclidean remainder, whatever\n  law a `div`/`rem` in the same\
    \ package selected, and is charged only at the four\n  `integer-modulus.*` points.\
    \ A quotient/remainder pair is refused as a pair: when either member\n  is outside\
    \ the consumer domain the refusal names which members were admitted and exposes\n\
    \  neither.\n- Decimal arithmetic, rounding and ordering follow `Decimal[..]`.\
    \ Two representations are\n  distinguished and each governs"
- id: FR-007 Behavior 1.2
  path: spec/exact/functional/FR-007-exact-scalar-families.md
  role: examined
  excerpt: "a different thing. *Value* comparison — `Decimal::compare` and\n  `numerically_equal`\
    \ — is mathematical and is computed on the normalized representation, so\n  `1.10`\
    \ and `1.1` are equal and neither orders before the other; `Decimal` deliberately\
    \ has no\n  structural `PartialEq`. *Charges* are sized on the retained representation,\
    \ never a normalized\n  one, so a value retained at a larger scale costs more\
    \ to order and to retain. The retained\n  representation is kept as provenance\
    \ and is never silently normalized away.\n- Decimal rounding has six spellings,\
    \ and an omitted spelling is strict"
- id: FR-007 Behavior 1.3
  path: spec/exact/functional/FR-007-exact-scalar-families.md
  role: examined
  excerpt: "`Exact`, which refuses any\n  discarded nonzero digit rather than rounding.\
    \ On an exact tie the tie-break is fixed per mode:\n  `NearestEven` takes whichever\
    \ of the two neighbours is even; `NearestAway` takes the neighbour\n  away from\
    \ zero, so the floor for a negative value and the ceiling otherwise; `TowardZero`,\n\
    \  `Floor` and `Ceiling` are decided by the sign of the exact quotient, not by\
    \ the sign of the\n  retained coefficient. No mode consults the host's floating\
    \ point.\n- IEEE operations follow the default profile's exact-intermediate and\
    \ rounding rules, and the\n  profile's exceptional"
- id: FR-007 Behavior 1.4
  path: spec/exact/functional/FR-007-exact-scalar-families.md
  role: examined
  excerpt: "semantics are fixed rather than left to a backend:\n  - NaN propagation\
    \ is **leftmost-wins**: the result NaN is the first NaN operand in operand order,\n\
    \    quieted, with its sign and payload preserved. A later NaN operand never displaces\
    \ it.\n  - The `invalid` flag is raised when **any** operand is a signaling NaN,\
    \ whichever operand\n    position it is in and whichever NaN is propagated.\n\
    \  - A NaN payload that does not fit an explicit conversion's target width is\
    \ refused as\n    `IeeeNanPayloadNotRepresentable`; it is never truncated to fit.\n\
    \  - Exact conversion out of IEEE loses the sign of a"
- id: FR-007 Behavior 1.5
  path: spec/exact/functional/FR-007-exact-scalar-families.md
  role: examined
  excerpt: "negative zero, because the exact value is\n    zero and the exact families\
    \ have no signed zero. The loss is reported as\n    `discarded_negative_zero`\
    \ rather than being silent, and `-0.0` and `+0.0` convert to the same\n    exact\
    \ value.\n  - `total_order_key` provides the profile's total order over every\
    \ bit pattern, including NaNs\n    and both zeros, so an ordering is defined where\
    \ IEEE comparison is not.\n- Text admission, comparison and normalization follow\
    \ `value-text-unicode-17.md`; enum\n  comparison is by declaration identity and\
    \ ordering only for ordered enums.\n- Quantities convert, combine"
- id: FR-007 Behavior 1.6
  path: spec/exact/functional/FR-007-exact-scalar-families.md
  role: examined
  excerpt: "and compare through the declared unit graph; topology faults are\n  typed\
    \ refusals, decided before evaluation and in one fixed cause order per operation.\n\
    \  `Add` and `Subtract` check incompatible dimensions, then affine-unit arithmetic,\
    \ then distinct\n  units, first failure wins. `Multiply`, `Divide` and `Power`\
    \ check affine-unit arithmetic only:\n  their dimensions combine rather than having\
    \ to match, so a dimension difference is not a fault\n  there. Every one of these\
    \ is `IllTyped`, reported beside the outcome with zero charges.\n- `Rational[lo..hi;\
    \ dmin..dmax]` membership is componentwise on"
- id: FR-007 Behavior 1.7
  path: spec/exact/functional/FR-007-exact-scalar-families.md
  role: examined
  excerpt: "the **reduced** form: the reduced\n  numerator must lie in the numerator\
    \ interval and the reduced denominator in the denominator\n  interval. It is therefore\
    \ not a numeric interval — a domain whose numeric span plainly contains\n  `1/3`\
    \ still refuses it when the denominator interval excludes `3` — and a consumer\
    \ that wants a\n  numeric range must declare a denominator interval that admits\
    \ every denominator it will meet.\n  An evaluation given no result domain performs\
    \ no membership decision at all and retains the\n  result, so absent is not the\
    \ same as unbounded-but-checked.\n- The canonical form of"
- id: FR-007 Behavior 1.8
  path: spec/exact/functional/FR-007-exact-scalar-families.md
  role: examined
  excerpt: "a rational is fixed: the sign lives in the numerator, the denominator\
    \ is\n  strictly positive, the reduced form is unique, and zero is exactly `0/1`\
    \ — never `-0/1` and\n  never `0/n` for any other `n`. Every rational exposed\
    \ by the runtime is in this form.\n- Integer arithmetic, rational arithmetic,\
    \ numeric ordering and Boolean connectives charge the\n  QSpec `integer-arithmetic.*`,\
    \ `rational-arithmetic.*`, `ordering.*` and\n  `boolean.result-retain` points;\
    \ a connective evaluates its right operand only when the left\n  does not decide\
    \ it.\n- Every arithmetic, normalize, rounding, retain-upscale,"
- id: FR-007 Behavior 1.9
  path: spec/exact/functional/FR-007-exact-scalar-families.md
  role: examined
  excerpt: "unit-event and target-domain amount is\n  derived from operand bit lengths\
    \ and scales only and is charged before any intermediate or result\n  is allocated;\
    \ no charge is sized from a computed result.\n- Model domains are out of scope\
    \ (agent-ix/quire-spec-language#120), as is replay (#121)."
- id: FR-007 Acceptance Criteria 1.1
  path: spec/exact/functional/FR-007-exact-scalar-families.md
  role: examined
  excerpt: '| ID | Criteria | Verification |

    |----|----------|--------------|

    | FR-007-AC-1 | Integer division and modulus agree with QSpec TC-192 in value,
    outcome kind and charges; the arithmetic amount `max(bits(a), bits(b))` is charged
    before the quotient exists. | Test (TC-018) |

    | FR-007-AC-2 | Decimal arithmetic, rounding and ordering agree with QSpec TC-185,
    including D20–D21 ordering charges and D22–D23 result-retain upscale charges;
    every operand-derived amount is exact at its limit and denied one under before
    allocation. | Test (TC-019) |

    | FR-007-AC-3 | IEEE profile operations agree with'
- id: FR-007 Acceptance Criteria 1.2
  path: spec/exact/functional/FR-007-exact-scalar-families.md
  role: examined
  excerpt: 'QSpec TC-193. | Test (TC-020) |

    | FR-007-AC-4 | Text and enum operations agree with QSpec TC-186. | Test (TC-021)
    |

    | FR-007-AC-5 | Quantity and unit-graph operations agree with QSpec TC-187. |
    Test (TC-022) |

    | FR-007-AC-6 | Every evaluated shared-corpus vector is executed on both the runtime
    and the quire-spec-language authority with equal Debug renderings; admission-only
    vectors and charges not yet metered by the authority are listed by name. | Test
    (TC-018, TC-019, TC-020, TC-021, TC-022) |

    | FR-007-AC-7 | Integer arithmetic, rational arithmetic, ordering and Boolean
    connectives match an'
- id: FR-007 Acceptance Criteria 1.3
  path: spec/exact/functional/FR-007-exact-scalar-families.md
  role: examined
  excerpt: 'independent `i128` oracle and the QSpec TC-191 P11 and TC-190 Q11 atom
    charges, with denial behavior at every kernel point; operand-derived arithmetic
    and normalize amounts are charged before any intermediate or result is allocated.
    | Test (TC-023) |

    | FR-007-AC-8 | The six rounding spellings round every exact tie to the stated
    neighbour for both signs; an omitted spelling is `Exact` and refuses a discarded
    nonzero digit. | Test (TC-034) |

    | FR-007-AC-9 | IEEE NaN propagation is leftmost-wins with sign and payload preserved
    and the result quieted; `invalid` is raised when any operand is'
- id: FR-007 Acceptance Criteria 1.4
  path: spec/exact/functional/FR-007-exact-scalar-families.md
  role: examined
  excerpt: 'signaling; an unrepresentable NaN payload is refused, never truncated;
    `-0.0` and `+0.0` convert to the same exact value with `discarded_negative_zero`
    reported; `total_order_key` totally orders every bit pattern including both zeros
    and NaNs. | Test (TC-034) |

    | FR-007-AC-10 | `Rational` membership admits exactly the reduced pairs inside
    both intervals — including refusing a value whose numeric magnitude is inside
    the numerator interval but whose reduced denominator is outside the denominator
    interval — an absent domain decides no membership and retains, and every exposed
    rational is in'
- id: FR-007 Acceptance Criteria 1.5
  path: spec/exact/functional/FR-007-exact-scalar-families.md
  role: examined
  excerpt: 'canonical form with zero as `0/1`. | Test (TC-034) |

    | FR-007-AC-11 | `Decimal` value comparison is on the normalized representation
    and charges are sized on the retained one, demonstrated by a pair equal in value
    whose ordering and retain charges differ. | Test (TC-034) |

    | FR-007-AC-12 | `mod` returns the Euclidean remainder for every operand sign
    whatever `div`/`rem` law is selected; a quotient/remainder pair outside the consumer
    domain is refused as a pair naming which members were admitted, exposing neither;
    quantity `IllTyped` causes appear in the stated per-operation order with zero'
- id: FR-007 Acceptance Criteria 1.6
  path: spec/exact/functional/FR-007-exact-scalar-families.md
  role: examined
  excerpt: 'charges, and `Multiply`/`Divide`/`Power` raise no dimension fault. | Test
    (TC-034) |

    | FR-007-AC-13 | Every hand-written `Debug` impl for a boxed value or type struct
    (`Rational`, `RationalDomain`, `Decimal`, `DecimalType`, `Quantity`, `Text`, `EnumValue`,
    `ObjectReference`, `CompoundUnit`) renders exactly the fields its `*Fields` struct
    declares, in declaration order, against a fixed expected string. | Test (TC-035)
    |'
- id: FR-007 Kernel ownership 1.1
  path: spec/exact/functional/FR-007-exact-scalar-families.md
  role: examined
  excerpt: 'The runtime shall consume kernel scalar operations directly from `quire-exact`,
    without a

    local implementation or re-export ([FR-275](./FR-275-single-exact-kernel.md)).
    The criteria

    remain obligations on consumed behavior; they do not require RT to retest the
    kernel.'
- id: FR-007 Kernel ownership 2.1
  path: spec/exact/functional/FR-007-exact-scalar-families.md
  role: examined
  excerpt: 'For FR-007-AC-7, `ix://agent-ix/quire-exact/FR-362` owns integer/rational
    arithmetic and

    ordering, atom charges and Boolean truth tables over already-decided operands;

    `ix://agent-ix/quire-exact/FR-363` owns retained-representation decimal ordering;
    and

    `ix://agent-ix/quire-exact/FR-361` owns the specified denied-work allocation bounds.

    `FR-362` explicitly leaves expression evaluation and the decision to skip a right
    operand to

    the caller. RT therefore retains `evaluate_boolean_short_circuit` and `ShortCircuitConnective`,

    with lazy invocation and stop propagation required by FR-011-AC-3 and'
- id: FR-007 Kernel ownership 2.2
  path: spec/exact/functional/FR-007-exact-scalar-families.md
  role: examined
  excerpt: 'FR-011-AC-8 and

    evidenced by TC-032,

    not by the kernel''s already-decided Boolean tests.'
- id: FR-007 Kernel ownership 3.1
  path: spec/exact/functional/FR-007-exact-scalar-families.md
  role: examined
  excerpt: 'FR-007-AC-1''s division/modulus allocation subset maps to `ix://agent-ix/quire-exact/FR-361-AC-3`;

    FR-007-AC-2''s decimal allocation subset maps to `FR-361-AC-4` through `FR-361-AC-6`,
    and

    its metered ordering subset to `ix://agent-ix/quire-exact/FR-363`. These owner
    contracts do not

    prove QSpec/QSL shared-corpus agreement or the whole decimal family. `FR-360`
    concerns

    `Integer::abs(i64::MIN)` only and is not evidence for those broader claims.'
- id: FR-007 Kernel ownership 4.1
  path: spec/exact/functional/FR-007-exact-scalar-families.md
  role: examined
  excerpt: '[The exact matrix](../matrix/tests.md) separates current RT evidence,
    future kernel-owned

    evidence, IR-669 agreement work and RT evaluation residue under IR-349 step 2
    and IR-583. This spec amendment removes

    no code or tests and does not claim IR-349''s implementation is complete.'
- id: FR-007 Dependencies 1.1
  path: spec/exact/functional/FR-007-exact-scalar-families.md
  role: examined
  excerpt: "- **Upstream**: [FR-006](./FR-006-exact-outcomes-and-accounting.md);\n\
    \  `ix://agent-ix/quire-specification`; quire-spec-language."
- id: FR-011 Description 1.1
  path: spec/exact/functional/FR-011-meter-state-at-a-stop.md
  role: examined
  excerpt: 'When an evaluation stops without a completed value, the runtime shall
    leave the `Meter` in a state

    the caller can read and rely on: FR-006 states that a *denied* charge consumes
    nothing, but an

    oracle consumer also needs to know what the meter holds when the stop is `Undefined`
    or `Refused`,

    and what the meter holds at all is what this requirement fixes.'
- id: FR-011 Inputs 1.1
  path: spec/exact/functional/FR-011-meter-state-at-a-stop.md
  role: examined
  excerpt: "- Any `Meter` that has admitted zero or more charges, at the moment an\
    \ evaluation returns\n  `Outcome::Undefined`, `Outcome::Refused` or `Outcome::Incomplete`."
- id: FR-011 Outputs 1.1
  path: spec/exact/functional/FR-011-meter-state-at-a-stop.md
  role: examined
  excerpt: '- `Meter::consumed(kind)` for each of the ten `ScalarLimitsV1` counters.

    - `Meter::admitted_charges()` and `Meter::charge_log_truncated()`.'
- id: FR-011 Behavior 1.1
  path: spec/exact/functional/FR-011-meter-state-at-a-stop.md
  role: examined
  excerpt: "- **A stop is not a rollback.** Every charge admitted before the stop\
    \ stays consumed and stays in\n  the admitted-charge log. The four dispositions\
    \ differ only in what the *stopping* step costs:\n  - `Incomplete` — the denied\
    \ charge itself consumed nothing and appended nothing.\n  - `Undefined` — the\
    \ operation has no mathematical value, and the charges already paid for reading\n\
    \    and sizing the operands stay consumed. An `Undefined` is therefore never\
    \ free.\n  - `Refused` — the result exists but is not admitted. Every charge up\
    \ to and including the\n    arithmetic that produced it stays consumed,"
- id: FR-011 Behavior 1.2
  path: spec/exact/functional/FR-011-meter-state-at-a-stop.md
  role: examined
  excerpt: "and the result-retain charge is never admitted, so\n    a refusal costs\
    \ no result unit.\n- **Undefinedness is detected at a stated point, not at an\
    \ unstated one.** A divisor is tested for\n  zero after the operation's `*.operands`\
    \ charge — whose size is derived from the divisor's bit\n  length, so the charge\
    \ must precede the test — and before its `*.arithmetic` charge. An\n  `Undefined::DivisionByZero`\
    \ therefore leaves exactly the operands charge consumed.\n- A quantity `power`\
    \ with a zero base and a negative exponent is `Undefined::DivisionByZero`: the\n\
    \  value is `1/0^|n|`, a division by zero, and"
- id: FR-011 Behavior 1.3
  path: spec/exact/functional/FR-011-meter-state-at-a-stop.md
  role: examined
  excerpt: "the vocabulary carries no separate cause for it.\n  Zero-divisor and zero-base-under-negative-exponent\
    \ are tested in that order, first match wins.\n- **A Boolean connective's stop\
    \ propagates unchanged.** When the right operand stops, the\n  connective returns\
    \ that stop verbatim and admits no `boolean.result-retain` charge, so a stopped\n\
    \  connective costs no result unit. The left operand enters as an already-decided\
    \ `bool` and is\n  charged for by whoever produced it, never by the connective.\n\
    - **One charge is all-or-nothing.** Within a single charge every semantic size\
    \ is checked against\n  its"
- id: FR-011 Behavior 1.4
  path: spec/exact/functional/FR-011-meter-state-at-a-stop.md
  role: examined
  excerpt: "limit and both cumulative counters are checked for availability before\
    \ any counter is\n  written. A charge that fails any check writes no counter,\
    \ appends no log entry and advances no\n  occurrence counter.\n- **The first short\
    \ counter is found in `ScalarLimitsV1` field order over the whole charge.** A\n\
    \  charge's own size vector is sorted by `LimitKind` field-order index before\
    \ it is scanned, so the\n  reported counter does not depend on the order in which\
    \ the evaluator happened to attach the\n  sizes, and every semantic-size counter\
    \ is scanned before `work_units` and `result_units`.\n- **The"
- id: FR-011 Behavior 1.5
  path: spec/exact/functional/FR-011-meter-state-at-a-stop.md
  role: examined
  excerpt: "admitted-charge log is a bounded diagnostic, not accounting.** It holds\
    \ the first\n  `CHARGE_LOG_CAPACITY` admitted points in admission order, `CHARGE_LOG_CAPACITY`\
    \ is 4096, and\n  `charge_log_truncated()` is true once a charge was admitted\
    \ that the log could not hold. Past the\n  cap every counter stays exact and every\
    \ limit is still enforced.\n- **Cumulative counters are exact; derived amounts\
    \ saturate.** `work_units` and `result_units` are\n  added with a checked addition\
    \ and a charge that would overflow one is denied, never wrapped. By\n  contrast\
    \ the amounts *derived* from operand shape — a"
- id: FR-011 Behavior 1.6
  path: spec/exact/functional/FR-011-meter-state-at-a-stop.md
  role: examined
  excerpt: "`usize` length widened to `u64`, an\n  exponent sum, a bit-length sum,\
    \ and the injected-denial occurrence counter — saturate at\n  `u64::MAX`. Saturation\
    \ can only make a charge more conservative, i.e. more likely to be denied,\n \
    \ and never admits a charge that the exact amount would have denied.\n- **Every\
    \ reader is total.** `Meter::consumed` answers for every `LimitKind` without a\
    \ panic path;\n  it is a total function over the enum and reports zero rather\
    \ than failing for an index outside the\n  ten counters, which the closed `LimitKind`\
    \ vocabulary cannot produce.\n- **Every exposed order is"
- id: FR-011 Behavior 1.7
  path: spec/exact/functional/FR-011-meter-state-at-a-stop.md
  role: examined
  excerpt: "deterministic and stated.** IEEE flags iterate in `IeeeFlag::ALL`\n  vocabulary\
    \ order; `Dimension` and `CompoundUnit` terms iterate ascending by node key;\n\
    \  `UnitGraph::admit` checks node well-formedness, then duplicate keys, then graph\
    \ topology, and its\n  refusal names the first failed check; `check_terms` refuses\
    \ zero exponent, then duplicate term,\n  then unsorted terms, in that order. No\
    \ exposed order depends on a hash, an address or an\n  insertion order the caller\
    \ cannot see."
- id: FR-011 Acceptance Criteria 1.1
  path: spec/exact/functional/FR-011-meter-state-at-a-stop.md
  role: examined
  excerpt: '| ID | Criteria | Verification |

    |----|----------|--------------|

    | FR-011-AC-1 | For each of `Undefined`, `Refused` and `Incomplete`, the meter
    after the stop holds exactly the charges admitted before it: an `Undefined` division
    by zero retains the operands charge and no arithmetic charge; a refused result
    retains the arithmetic charge and no result unit; a denied charge retains neither.
    | Test (TC-032) |

    | FR-011-AC-2 | A quantity `power` with zero base and negative exponent is `Undefined::DivisionByZero`,
    and a divide by zero is reported in preference to it when both hold. | Test (TC-032)'
- id: FR-011 Acceptance Criteria 1.2
  path: spec/exact/functional/FR-011-meter-state-at-a-stop.md
  role: examined
  excerpt: '|

    | FR-011-AC-3 | A lazy connective skips its right operand when the left decides
    the result; for every connective kind and every decided operand pair, its completed
    result admits exactly one `boolean.result-retain` charge. | Test (TC-032) |

    | FR-011-AC-4 | A charge whose second-scanned counter is short writes no counter,
    appends no log entry and advances no occurrence counter; and a charge presented
    with its size vector in either order reports the same first short counter in `ScalarLimitsV1`
    field order. | Test (TC-032) |

    | FR-011-AC-5 | Past `CHARGE_LOG_CAPACITY` admitted charges the log'
- id: FR-011 Acceptance Criteria 1.3
  path: spec/exact/functional/FR-011-meter-state-at-a-stop.md
  role: examined
  excerpt: 'holds exactly the first 4096 points in admission order, `charge_log_truncated()`
    is true, and the counters are still exact and still enforced. | Test (TC-032)
    |

    | FR-011-AC-6 | A cumulative counter at `u64::MAX - 1` denies rather than wraps;
    a derived amount that exceeds `u64::MAX` saturates and the resulting charge is
    denied rather than admitted. | Test (TC-032) |

    | FR-011-AC-7 | `Meter::consumed` answers for all ten `LimitKind` members with
    no panic path, and IEEE flag iteration, dimension and compound-unit term iteration,
    and the `UnitGraph::admit` and `check_terms` refusal orders are the'
- id: FR-011 Acceptance Criteria 1.4
  path: spec/exact/functional/FR-011-meter-state-at-a-stop.md
  role: examined
  excerpt: 'stated ones for every input permutation. | Test (TC-032) |

    | FR-011-AC-8 | A connective whose right operand stops returns that stop unchanged,
    admits no `boolean.result-retain` charge and consumes no result unit. | Test (TC-032)
    |'
- id: FR-011 Kernel ownership 1.1
  path: spec/exact/functional/FR-011-meter-state-at-a-stop.md
  role: examined
  excerpt: 'The behaviour above that `quire-exact` implements (interface-001, "Exact
    kernel surface") is the

    behaviour of that one kernel, which the runtime consumes and does not copy ([FR-275](./FR-275-single-exact-kernel.md)).

    This requirement stays in force and binds the `exact` feature as a whole; where
    its evidence

    moves to the kernel''s repository the matrix says so and keeps the row, without
    deleting any

    acceptance criterion.'
- id: FR-011 Dependencies 1.1
  path: spec/exact/functional/FR-011-meter-state-at-a-stop.md
  role: examined
  excerpt: "- **Upstream**: [FR-006](./FR-006-exact-outcomes-and-accounting.md);\n\
    \  `ix://agent-ix/quire-specification`."
- id: TC-016 Description 1.1
  path: spec/exact/matrix/TC-016-exact-outcome-envelope.md
  role: examined
  excerpt: 'Check the typed outcome envelope and the closed reason and vocabulary
    enums of the `exact`

    surface. Evidence: `tests/exact_outcomes.rs` (`--features exact`).'
- id: TC-016 Ownership and evidence 1.1
  path: spec/exact/matrix/TC-016-exact-outcome-envelope.md
  role: examined
  excerpt: 'Kernel-owned after IR-349 removes the local copy. The evidence path above
    is current

    RT evidence, not a retained RT test obligation. The outcome envelope, reason variants
    and

    spelling census remain required; FR-359–363 do not establish them. An exact owner
    criterion

    mapping for those checks remains to be identified; no upstream pass is claimed.'
- id: TC-016 Test Procedure 1.1
  path: spec/exact/matrix/TC-016-exact-outcome-envelope.md
  role: examined
  excerpt: "1. Build one outcome of each disposition, including completed `true` and\
    \ `false`; compare every\n   pair for equality and check that only completed outcomes\
    \ yield a value.\n2. Enumerate every `Refusal` and `Undefined` variant and check\
    \ that codes are distinct.\n3. Round-trip every `ChargePoint` and `LimitKind`\
    \ through its spelling; check the eleven QSpec\n   families are present and `equality.plan-form`\
    \ is absent."
- id: TC-016 Expected Results 1.1
  path: spec/exact/matrix/TC-016-exact-outcome-envelope.md
  role: examined
  excerpt: Four distinct dispositions and closed vocabularies matching QSpec.
- id: TC-017 Description 1.1
  path: spec/exact/matrix/TC-017-exact-metering.md
  role: examined
  excerpt: 'Check charge ordering, counter semantics, first-short-counter reporting
    and injected denials on

    the public meter. Evidence: `tests/exact_outcomes.rs` (`--features exact`).'
- id: TC-017 Ownership and evidence 1.1
  path: spec/exact/matrix/TC-017-exact-metering.md
  role: examined
  excerpt: 'Kernel-owned after IR-349 removes the local copy. The evidence path above
    remains present

    in this spec-only amendment. `ix://agent-ix/quire-exact/FR-358` owns step 5''s
    one-shot denial

    and step 6''s diagnostic-log bound under `test-support`; `ix://agent-ix/quire-exact/FR-359`

    owns cumulative-boundary and atomic-refusal checks, not the entire step 3 counter
    census.

    No reference is a claim that every procedure step is mapped or verified upstream.'
- id: TC-017 Test Procedure 1.1
  path: spec/exact/matrix/TC-017-exact-metering.md
  role: examined
  excerpt: "1. Multiply `2^64 × 2^64` under `integer_bits` 128; expect `Incomplete`\
    \ at\n   `integer-arithmetic.arithmetic` with consumed 65 and denied amount 129.\n\
    2. Run two operations on one meter; check size counters keep the high-water mark\
    \ and\n   `work_units`/`result_units` accumulate.\n3. Exhaust two counters at\
    \ once; check the first in `ScalarLimitsV1` field order is reported.\n4. Evaluate\
    \ an out-of-domain result; check the refusal precedes retention and no result\
    \ unit is\n   charged.\n5. Inject a denial at each admitted point; check the record\
    \ and that counters are unchanged.\n6. Admit more charges than"
- id: TC-017 Test Procedure 1.2
  path: spec/exact/matrix/TC-017-exact-metering.md
  role: examined
  excerpt: "`CHARGE_LOG_CAPACITY`; check the log is capped and marked truncated,\n\
    \   counters stay exact, and an injected denial past the cap still fires."
- id: TC-017 Expected Results 1.1
  path: spec/exact/matrix/TC-017-exact-metering.md
  role: examined
  excerpt: 'Every `Incomplete` record names the exact counter, limit, consumed amount,
    denied amount and

    point; no denied charge consumes anything.'
- id: TC-018 Description 1.1
  path: spec/exact/matrix/TC-018-integer-division-agreement.md
  role: examined
  excerpt: 'Execute every QSpec TC-192 vector on the runtime and on quire-spec-language.

    Evidence: The QSL agreement oracle is removed from this repository; recreating
    it in agent-ix/quire-integration is planned with open quire-integration ticket
    IR-669 (IR-430 removal is Done). Steps 4-5 keep evidence in the

    arithmetic allocation bound in `tests/exact_allocation.rs` (`--features exact`).'
- id: TC-018 Ownership and evidence 1.1
  path: spec/exact/matrix/TC-018-integer-division-agreement.md
  role: examined
  excerpt: 'Steps 1–3 remain a QSL agreement gap with open quire-integration ticket
    IR-669. Steps 4–5 currently have RT allocation

    coverage; that local kernel evidence leaves in IR-349, without replacement tests
    in RT.

    `ix://agent-ix/quire-exact/FR-361-AC-3` owns injected division/modulus denial
    before large

    allocation only. It does not establish the generated sweep, exact/one-under amounts
    or

    TC-192 agreement; those remaining obligations require their own evidence mapping.'
- id: TC-018 Test Procedure 1.1
  path: spec/exact/matrix/TC-018-integer-division-agreement.md
  role: examined
  excerpt: "1. Check the evaluated and admission-only lists together equal the TC-192\
    \ vector census.\n2. For each evaluated vector, evaluate both sides under the\
    \ same limits and compare Debug renderings\n   of value, outcome kind, charge\
    \ sequence and consumed counters.\n3. Deny every admitted charge and compare the\
    \ `Incomplete` records.\n4. Sweep truncating, floor and Euclidean division and\
    \ modulus over generated operands.\n5. Charge division and modulus arithmetic\
    \ at `max(bits(a), bits(b))` exact and one under, and deny\n   it by injection\
    \ with no quotient or remainder allocation."
- id: TC-018 Expected Results 1.1
  path: spec/exact/matrix/TC-018-integer-division-agreement.md
  role: examined
  excerpt: 12 vectors agree exactly; DIV-03 and DIV-09 are admission-only and named.
- id: TC-019 Description 1.1
  path: spec/exact/matrix/TC-019-decimal-agreement.md
  role: examined
  excerpt: 'Execute every QSpec TC-185 vector on the runtime and on quire-spec-language.

    Evidence: The QSL agreement oracle is removed from this repository; recreating
    it in agent-ix/quire-integration is planned with open quire-integration ticket
    IR-669 (IR-430 removal is Done). Step 5 keeps evidence in the

    upscale allocation bound in `tests/exact_allocation.rs` (`--features exact`).'
- id: TC-019 Ownership and evidence 1.1
  path: spec/exact/matrix/TC-019-decimal-agreement.md
  role: examined
  excerpt: 'Steps 1–3 remain a QSL agreement gap with open quire-integration ticket
    IR-669. Step 5 currently has RT allocation

    coverage, which leaves in IR-349. `ix://agent-ix/quire-exact/FR-361-AC-6` owns
    that denied

    retain-upscale allocation bound; FR-361-AC-4/AC-5 cover scale-expansion/arithmetic
    denial.

    `ix://agent-ix/quire-exact/FR-363` owns retained-representation decimal ordering
    only.

    Neither contract establishes TC-185 agreement or all rounding/arithmetic cases
    in step 4.

    No local allocation check constitutes an agreement pass.'
- id: TC-019 Test Procedure 1.1
  path: spec/exact/matrix/TC-019-decimal-agreement.md
  role: examined
  excerpt: "1. Check the evaluated list equals D01–D23.\n2. For every vector with\
    \ an authority run, compare value and outcome kind on both sides; for\n   vectors\
    \ whose charges the authority meters, also compare charges, counters and every\
    \ injected\n   denial.\n3. For D09, D13 and D20–D23, check the runtime's admitted\
    \ point sequence, charge amounts\n   and consumed counters against the QSpec TC-185\
    \ expectations at the exact limit tuple;\n   repeat one under the first short\
    \ counter and require `Incomplete` with that counter,\n   exact denied amount\
    \ and charge point, with no later charge or retained result.\n  "
- id: TC-019 Test Procedure 1.2
  path: spec/exact/matrix/TC-019-decimal-agreement.md
  role: examined
  excerpt: "D20–D21 must short `integer_bits` at `ordering.arithmetic`; D22–D23 must\
    \ charge\n   result-retain upscale before materialization. For every authority-metered\
    \ vector, also\n   require equal value, outcome kind, charge sequence and consumed\
    \ counters on both sides;\n   any difference fails. D23 currently has no authority\
    \ run: its local QSpec charge check\n   remains mandatory, while its two-sided\
    \ agreement is unavailable and cannot count as a pass.\n   Name any other unavailable\
    \ or unmetered authority case; it remains an agreement gap,\n   not permission\
    \ to omit the local QSpec assertions.\n4. Check"
- id: TC-019 Test Procedure 1.3
  path: spec/exact/matrix/TC-019-decimal-agreement.md
  role: examined
  excerpt: "each operand-derived decimal amount exact and one under: add alignment,\
    \ subtract\n   cancellation, multiply, divide, negate with rounding and retain\
    \ upscale.\n5. Deny the digits charge of a `2^20` scale upscale and check no allocation\
    \ reaches 4096 bytes."
- id: TC-019 Expected Results 1.1
  path: spec/exact/matrix/TC-019-decimal-agreement.md
  role: examined
  excerpt: 'The planned agreement run shall account for all 23 vectors and name every
    admission-only,

    unmetered or unavailable authority case, including D23. Value/outcome and charge
    agreement

    shall be claimed only for vectors actually executed on both sides. Step 3 fails
    on any

    local TC-185 charge/counter mismatch or any mismatch in an executed authority
    comparison. Local allocation evidence

    alone does not meet this expected result (Linear IR-669, open quire-integration
    owner).'
- id: TC-020 Description 1.1
  path: spec/exact/matrix/TC-020-ieee-agreement.md
  role: examined
  excerpt: 'Execute every QSpec TC-193 vector on the runtime and on quire-spec-language.

    Evidence: none in this repository. The QSL agreement oracle is removed from this
    repository; recreating it in agent-ix/quire-integration is planned with open quire-integration
    ticket IR-669 (IR-430 removal is Done).'
- id: TC-020 Test Procedure 1.1
  path: spec/exact/matrix/TC-020-ieee-agreement.md
  role: examined
  excerpt: "1. Check the evaluated and admission-only lists together equal the TC-193\
    \ census.\n2. Compare value, outcome kind, charges and counters for arithmetic,\
    \ rounding, non-finite,\n   NaN-payload and rational-domain vectors on both sides.\n\
    3. Deny every admitted charge and compare the records; sweep generated operands\
    \ per profile."
- id: TC-020 Expected Results 1.1
  path: spec/exact/matrix/TC-020-ieee-agreement.md
  role: examined
  excerpt: 33 vectors agree exactly; semantic admission of the IEEE definition is
    admission-only and named.
- id: TC-021 Description 1.1
  path: spec/exact/matrix/TC-021-text-enum-agreement.md
  role: examined
  excerpt: 'Execute every QSpec TC-186 vector on the runtime and on quire-spec-language.

    Evidence: none in this repository. The QSL agreement oracle is removed from this
    repository; recreating it in agent-ix/quire-integration is planned with open quire-integration
    ticket IR-669 (IR-430 removal is Done).'
- id: TC-021 Test Procedure 1.1
  path: spec/exact/matrix/TC-021-text-enum-agreement.md
  role: examined
  excerpt: "1. Check the evaluated and admission-only lists together equal the TC-186\
    \ census.\n2. Compare admission lengths, invalid UTF-8 provenance, profile and\
    \ declaration mismatches,\n   ordered and unordered enum comparison, charges and\
    \ counters on both sides.\n3. Deny every named charge; sweep six profiles × twelve\
    \ sequences² × six operators, and every enum\n   declaration × member × operator."
- id: TC-021 Expected Results 1.1
  path: spec/exact/matrix/TC-021-text-enum-agreement.md
  role: examined
  excerpt: 17 vectors agree exactly; T05b and the T09 stale-key half are admission-only
    and named.
- id: TC-022 Description 1.1
  path: spec/exact/matrix/TC-022-quantity-agreement.md
  role: examined
  excerpt: 'Execute every QSpec TC-187 vector on the runtime and on quire-spec-language.

    Evidence: none in this repository. The QSL agreement oracle is removed from this
    repository; recreating it in agent-ix/quire-integration is planned with open quire-integration
    ticket IR-669 (IR-430 removal is Done).'
- id: TC-022 Test Procedure 1.1
  path: spec/exact/matrix/TC-022-quantity-agreement.md
  role: examined
  excerpt: "1. Check the evaluated and admission-only lists together equal the TC-187\
    \ census.\n2. Compare dimension algebra, ill-typed combinations, graph topology\
    \ refusals (zero scale,\n   duplicate root, cross-dimension target, target cycle,\
    \ unknown target, missing root), decimal\n   and integer targets, compound units\
    \ and values on both sides; compare charges and counters where\n   the authority\
    \ meters them.\n3. For U10, U13, U15, U16, U19, U20, U22–U24, U26, U28 and U29,\
    \ agree with the authority on the\n   value, charge schedule and consumed counters,\
    \ metered under the QSpec limit tuple and one\n   under"
- id: TC-022 Test Procedure 1.2
  path: spec/exact/matrix/TC-022-quantity-agreement.md
  role: examined
  excerpt: "its first short counter: `unit.rational-arithmetic` per event operands,\
    \ power\n   `max(1, |n| × maxparts)`, and `unit.target-domain` from the operand\
    \ and target scale.\n4. Deny every named charge; sweep conversions across two\
    \ unit families against an `i128` fraction\n   oracle, and add/subtract/multiply/divide/compare\
    \ over five units × 25 pairs."
- id: TC-022 Expected Results 1.1
  path: spec/exact/matrix/TC-022-quantity-agreement.md
  role: examined
  excerpt: '30 vectors agree in value, outcome kind and charges, U10, U13, U15, U16,
    U19, U20, U22–U24, U26, U28

    and U29 included: QSL `quire-spec-language#119` is fixed, so every vector''s charges
    are asserted

    against the authority like every other vector. U11 owner selection and stale keys
    are admission-only

    and named.'
- id: TC-023 Description 1.1
  path: spec/exact/matrix/TC-023-metered-arithmetic.md
  role: examined
  excerpt: 'The local kernel tests check charges against QSpec and values against
    an independent `i128`

    oracle. They do not establish quire-spec-language shared-corpus agreement (Linear
    IR-669, open quire-integration owner; IR-430 removal is Done). Evidence:

    `tests/exact_arithmetic.rs` and `tests/exact_allocation.rs` (`--features exact`).'
- id: TC-023 Ownership and evidence 1.1
  path: spec/exact/matrix/TC-023-metered-arithmetic.md
  role: examined
  excerpt: 'Kernel arithmetic, ordering, atom charges and already-decided Boolean
    retention leave RT

    in IR-349: `ix://agent-ix/quire-exact/FR-362` owns those subsets; decimal ordering
    belongs to

    `ix://agent-ix/quire-exact/FR-363`, and the specified denied-work allocation bounds
    to

    `ix://agent-ix/quire-exact/FR-361`. These are owner references, not copied tests
    or a claim

    that the P11/Q11 expression workloads have been evaluated upstream. Lazy operand
    evaluation

    is RT-owned and remains in TC-032; step 3''s caller simulation in this file is
    not its retained

    evidence. Shared-authority agreement remains'
- id: TC-023 Ownership and evidence 1.2
  path: spec/exact/matrix/TC-023-metered-arithmetic.md
  role: examined
  excerpt: separate, with open quire-integration ticket IR-669; IR-430 removal is
    Done.
- id: TC-023 Test Procedure 1.1
  path: spec/exact/matrix/TC-023-metered-arithmetic.md
  role: examined
  excerpt: "1. TC-191 P11 atoms: `2 > 0` and `2 - 1` each charge three points at `integer_bits`\
    \ 2; the scalar\n   atoms of `down(2)` cost 15 work and 5 result units and stop\
    \ at `0 > 0` retention under 14 work.\n2. `3/2` charges four rational points at\
    \ `integer_bits` 2; TC-190 Q11 `acc + x` charges\n   `integer-arithmetic.*` per\
    \ step.\n3. `implies`: left true costs 7 work and 3 results; left false skips\
    \ the right operand and costs 4\n   and 2. A stopped right operand is returned\
    \ unchanged without `boolean.result-retain`.\n4. Operand-derived rational amounts\
    \ (`×` and `÷` sum cross parts, `+`/`-` add one to the"
- id: TC-023 Test Procedure 1.2
  path: spec/exact/matrix/TC-023-metered-arithmetic.md
  role: examined
  excerpt: "larger\n   cross product), zero divisors (undefined after operands only),\
    \ domain refusal before retention;\n   rational `cross_bits` and retained-decimal\
    \ `sbits`/`sdigits` ordering amounts, including an\n   aligned coefficient beyond\
    \ `u64`.\n5. Exact and one-under limits for every arithmetic and normalize charge\
    \ (`2/3 × 3/2`, `3/4 ÷ 5/7`,\n   `5/7 - 4/7`, `1000 - 999`, `255 × 255`, negation\
    \ at the operand amount); amounts one below and\n   at a power of two and under\
    \ total cancellation equal the operand formula, not the result size;\n   squaring,\
    \ `(2^k-1)(2^k+1)`, `2^k - (2^k-1)` and `2^k × 1/2^k`"
- id: TC-023 Test Procedure 1.3
  path: spec/exact/matrix/TC-023-metered-arithmetic.md
  role: examined
  excerpt: "denied at the operand-derived\n   amount make no allocation request larger\
    \ than an eighth of one operand.\n6. Generated sweeps: 12² integer pairs × add/subtract/multiply/negate/four\
    \ orderings, and 10²\n   fraction pairs × five operations and four orderings,\
    \ with every named denial."
- id: TC-023 Expected Results 1.1
  path: spec/exact/matrix/TC-023-metered-arithmetic.md
  role: examined
  excerpt: 'Every value equals the oracle, every charge schedule equals QSpec, and
    every denial

    names its point with no result unit.'
- id: TC-032 Description 1.1
  path: spec/exact/matrix/TC-032-meter-state-at-a-stop.md
  role: examined
  excerpt: 'Check what the meter holds at `Undefined`, `Refused` and `Incomplete`
    stops, charge atomicity, the

    field-order scan, the bounded log, saturation and the exposed deterministic orders.
    Evidence:

    `tests/exact_meter_state.rs` (`--features exact`).'
- id: TC-032 Ownership and evidence 1.1
  path: spec/exact/matrix/TC-032-meter-state-at-a-stop.md
  role: examined
  excerpt: 'Step 5''s lazy connective behavior remains RT-owned after IR-349: skipping
    the right closure

    when the left decides, propagating each right-operand stop unchanged with no retention,
    and

    retaining a completed connective result exactly once. Current evidence is

    `tc_032_ac3_ac8_short_circuit_propagates_a_stop_and_retains_exactly_once` in the
    file above

    (FR-011-AC-3 and FR-011-AC-8). Its future API home is `scalar`, as interface-001
    requires.

    The already-decided truth-table test is kernel-owned; `ix://agent-ix/quire-exact/FR-362`

    does not test lazy closure invocation. Steps 1–3 and 6–8 and the'
- id: TC-032 Ownership and evidence 1.2
  path: spec/exact/matrix/TC-032-meter-state-at-a-stop.md
  role: examined
  excerpt: 'kernel part of step 9 leave

    with the kernel copy; the quantity/environment parts of steps 4 and 9 remain RT
    evaluation residue

    owned by open IR-349 step 2 and backlog IR-583 until removed; QSL-358''s QSL-side
    extraction is Done. This amendment changes no test or implementation.'
- id: TC-032 Test Procedure 1.1
  path: spec/exact/matrix/TC-032-meter-state-at-a-stop.md
  role: examined
  excerpt: "1. Divide by zero; check the operands charge is consumed and the arithmetic\
    \ charge is not.\n2. Produce an out-of-domain result; check the arithmetic charge\
    \ is consumed and no result unit is.\n3. Deny a charge; check no counter, log\
    \ entry or occurrence counter moved.\n4. Evaluate a quantity `power` with zero\
    \ base and negative exponent, and a divide by zero that also\n   satisfies it;\
    \ check the reported cause and its precedence.\n5. Evaluate each connective with\
    \ a stopping right operand and with a short-circuiting left operand;\n   check\
    \ the retain charge is admitted exactly when the result is"
- id: TC-032 Test Procedure 1.2
  path: spec/exact/matrix/TC-032-meter-state-at-a-stop.md
  role: examined
  excerpt: "decided by the connective.\n6. Present one charge's size vector in both\
    \ orders under two short counters; check the same\n   `ScalarLimitsV1`-field-order\
    \ counter is reported and that nothing was written.\n7. Admit more than `CHARGE_LOG_CAPACITY`\
    \ charges; check the log holds the first 4096 in order, is\n   marked truncated,\
    \ and the counters stay exact and enforced.\n8. Drive a cumulative counter to\
    \ `u64::MAX - 1` and a derived amount past `u64::MAX`; check the\n   charge is\
    \ denied, not wrapped.\n9. Read `consumed` for all ten `LimitKind` members; iterate\
    \ IEEE flags, dimension terms and\n  "
- id: TC-032 Test Procedure 1.3
  path: spec/exact/matrix/TC-032-meter-state-at-a-stop.md
  role: examined
  excerpt: "compound-unit terms built in several orders; drive `UnitGraph::admit`\
    \ and `check_terms` into\n   inputs that fail more than one check."
- id: TC-032 Expected Results 1.1
  path: spec/exact/matrix/TC-032-meter-state-at-a-stop.md
  role: examined
  excerpt: 'A stop is never a rollback and never free; a denied charge is atomic;
    every exposed order is the

    stated one for every permutation.'
- id: TC-035 Description 1.1
  path: spec/exact/matrix/TC-035-boxed-debug-parity.md
  role: examined
  excerpt: '`Rational`, `RationalDomain`, `Decimal`, `DecimalType`, `Quantity`, `Text`,
    `EnumValue`,

    `ObjectReference` and `CompoundUnit` each hold their fields behind one `Box` (a
    Kani-provability

    layout constraint; see `AD-002`''s risk section), which turned their derived `Debug`
    into a

    hand-written impl that lists each `*Fields` field explicitly. A field added to
    a `*Fields` struct

    but not to its `Debug` impl would disappear from the rendering with nothing to
    notice, since

    FR-007-AC-6''s own Debug-parity oracle (equal renderings against the quire-spec-language
    authority) is removed from

    this repository;'
- id: TC-035 Description 1.2
  path: spec/exact/matrix/TC-035-boxed-debug-parity.md
  role: examined
  excerpt: 'recreating it in agent-ix/quire-integration is planned with open quire-integration
    ticket IR-669 (IR-430 removal is Done).

    Evidence: `tests/exact_debug_parity.rs` (`--features exact`).'
- id: TC-035 Description 2.1
  path: spec/exact/matrix/TC-035-boxed-debug-parity.md
  role: examined
  excerpt: 'This is not FR-007-AC-6''s oracle: it makes no comparison against the
    quire-spec-language authority, and it

    covers only these nine boxed structs, not the shared corpus. It is a same-tree
    regression pin.'
- id: TC-035 Test Procedure 1.1
  path: spec/exact/matrix/TC-035-boxed-debug-parity.md
  role: examined
  excerpt: "1. Construct one instance of each of the nine boxed structs through its\
    \ public API.\n2. Render each with `{:?}` and compare the result to a fixed string\
    \ listing every field the\n   corresponding `*Fields` struct declares, in declaration\
    \ order."
- id: TC-035 Expected Results 1.1
  path: spec/exact/matrix/TC-035-boxed-debug-parity.md
  role: examined
  excerpt: 'Every rendering matches its pinned string exactly. A field added to a
    `*Fields` struct without a

    matching addition to its `Debug` impl changes the rendering and fails the comparison.'
- id: TM-001 Functional Requirement Coverage 1.1
  path: spec/exact/matrix/tests.md
  role: examined
  excerpt: '| Functional Req | Acceptance Criteria | Test Cases | Status |

    |---|---|---|---|

    | FR-006 | FR-006-AC-1, FR-006-AC-6 | TC-016 | ✅ implemented |

    | FR-006 | FR-006-AC-2 | TC-020, TC-021, TC-022 | 🚧 planned (Linear IR-669, open
    quire-integration owner; IR-430 removal is Done): the QSL agreement oracle is
    removed from this repository, and its evidence with it |

    | FR-006 | FR-006-AC-3 | TC-016, TC-017 | ✅ implemented |

    | FR-006 | FR-006-AC-4 | TC-017 | ✅ implemented |

    | FR-007 | FR-007-AC-1 | TC-018 | 🚧 partly evidenced: local allocation checks
    are implemented; QSL shared-corpus agreement remains'
- id: TM-001 Functional Requirement Coverage 1.2
  path: spec/exact/matrix/tests.md
  role: examined
  excerpt: 'planned (Linear IR-669, open quire-integration owner; IR-430 removal is
    Done) |

    | FR-007 | FR-007-AC-2 | TC-019 | 🚧 partly evidenced: local allocation checks
    are implemented; QSL shared-corpus agreement remains planned (Linear IR-669, open
    quire-integration owner; IR-430 removal is Done) |

    | FR-007 | FR-007-AC-3 | TC-020 | 🚧 planned (Linear IR-669, open quire-integration
    owner; IR-430 removal is Done): the QSL agreement oracle is removed from this
    repository, and its evidence with it |

    | FR-007 | FR-007-AC-4 | TC-021 | 🚧 planned (Linear IR-669, open quire-integration
    owner; IR-430 removal is'
- id: TM-001 Functional Requirement Coverage 1.3
  path: spec/exact/matrix/tests.md
  role: examined
  excerpt: 'Done): the QSL agreement oracle is removed from this repository, and its
    evidence with it |

    | FR-007 | FR-007-AC-5 | TC-022 | 🚧 planned (Linear IR-669, open quire-integration
    owner; IR-430 removal is Done): the QSL agreement oracle is removed from this
    repository, and its evidence with it |

    | FR-007 | FR-007-AC-6 | TC-018, TC-019, TC-020, TC-021, TC-022 | 🚧 partly evidenced:
    the runtime-side Debug pins remain (TC-035); the QSL Debug-parity half is removed
    from this repository, recreation in agent-ix/quire-integration is planned with
    open quire-integration ticket IR-669 (IR-430 removal is'
- id: TM-001 Functional Requirement Coverage 1.4
  path: spec/exact/matrix/tests.md
  role: examined
  excerpt: 'Done) |

    | FR-007 | FR-007-AC-7 | TC-023 | ✅ implemented |

    | FR-008 | FR-008-AC-1, FR-008-AC-2 | TC-024 | ✅ implemented |

    | FR-008 | FR-008-AC-3, FR-008-AC-4 | TC-025 | ✅ implemented |

    | FR-008 | FR-008-AC-5, FR-008-AC-6 | TC-026 | ✅ implemented |

    | FR-008 | FR-008-AC-7, FR-008-AC-8 | TC-024, TC-025, TC-026 | ✅ implemented |

    | FR-008 | FR-008-AC-9 | TC-024, TC-025 | ✅ implemented |

    | FR-008 | FR-008-AC-13 | TC-024 | ✅ implemented: deep metadata clone, equality,
    formatting and drop in a small-stack thread and default-stack child process |

    | FR-008 | FR-008-AC-10, FR-008-AC-11, FR-008-AC-12 |'
- id: TM-001 Functional Requirement Coverage 1.5
  path: spec/exact/matrix/tests.md
  role: examined
  excerpt: 'TC-025 | 🚧 sequence and ordered-set cases not yet in tests/exact_collection.rs
    |

    | FR-007 | FR-007-AC-8, FR-007-AC-9, FR-007-AC-10, FR-007-AC-11, FR-007-AC-12
    | TC-034 | ✅ implemented |

    | FR-007 | FR-007-AC-13 | TC-035 | ✅ implemented |

    | FR-009 | FR-009-AC-1, FR-009-AC-2, FR-009-AC-3, FR-009-AC-4 | TC-030 | ✅ implemented
    |

    | FR-009 | FR-009-AC-5 | TC-195 | ✅ implemented (compile_fail doctest on `IeeeDisposition`,
    `src/exact/ieee.rs`) |

    | FR-010 | FR-010-AC-1, FR-010-AC-2, FR-010-AC-3, FR-010-AC-4, FR-010-AC-5, FR-010-AC-6
    | TC-031 | ✅ implemented |

    | FR-011 | FR-011-AC-1, FR-011-AC-2,'
- id: TM-001 Functional Requirement Coverage 1.6
  path: spec/exact/matrix/tests.md
  role: examined
  excerpt: 'FR-011-AC-3, FR-011-AC-4, FR-011-AC-5, FR-011-AC-6, FR-011-AC-7, FR-011-AC-8
    | TC-032 | ✅ implemented |

    | FR-012 | FR-012-AC-1, FR-012-AC-2, FR-012-AC-3, FR-012-AC-4, FR-012-AC-5 | TC-033
    | ✅ implemented |

    | FR-273 | FR-273-AC-1, FR-273-AC-2, FR-273-AC-3, FR-273-AC-6 | TC-194 | ✅ implemented:
    AC-1''s linked-only application is proved by `tc_194_kernel_check_is_refused_so_no_kernel_package_is_applicable`
    (a package `check` rejects under `CheckMode::Kernel` is never applicable — that
    is AC-6''s inspection too) together with `tc_194_linked_package_applies_every_declared_function`
    and the rest of'
- id: TM-001 Functional Requirement Coverage 1.7
  path: spec/exact/matrix/tests.md
  role: examined
  excerpt: 'this corpus''s `Linked`-application tests, which call only a package `check`
    admitted under `CheckMode::Linked`. AC-2/AC-3''s "arity before any per-argument
    check, all before the `function.call` charge, before the body" ordering is covered
    for both `call` and `Frame::call` |

    | FR-273 | FR-273-AC-7 | TC-194 | ✅ implemented: re-entry into a checked package
    through `CheckedPackage::call`, `CheckedPackage::evaluate` or `Frame::call` is
    bounded by `CheckingLimits::depth` on a shared counter — not only `Frame::call`
    — including the direct-re-entry attack a body holding its own `Rc<CheckedPackage>`'
- id: TM-001 Functional Requirement Coverage 1.8
  path: spec/exact/matrix/tests.md
  role: examined
  excerpt: 'could otherwise use to bypass it, proved by `tc_194_recursion_beyond_the_depth_limit_is_a_checked_invariant_refusal`,
    `tc_194_direct_reentrant_package_call_is_bounded_like_frame_call` and `tc_194_checking_limits_refuses_a_depth_above_the_maximum`.
    The bound is per-`CheckedPackage`, not universal: a host body that builds a *fresh*
    `CheckedPackage` at each hop gets a fresh budget and can still overflow the host
    stack — but so does a body that recurses without touching this crate''s runtime
    at all, since under AD-002 a body is arbitrary host Rust and its own stack usage
    is the host''s concern, not'
- id: TM-001 Functional Requirement Coverage 1.9
  path: spec/exact/matrix/tests.md
  role: examined
  excerpt: 'this crate''s |

    | FR-273 | FR-273-AC-5 | TC-194 | 🚧 partly evidenced: the QSL shared-corpus half
    is removed from this repository, recreation in agent-ix/quire-integration is planned
    with open quire-integration ticket IR-669 (IR-430 removal is Done); the runtime-only
    ordering tests remain. AC-5 quantifies over shared-corpus function-application
    vectors only, and the shared corpus agrees on all five of them — the closed `InputRefusal`
    vocabulary (with codes and causes), the charge count of one admitted call, and
    — via AP01–AP04''s `charges == 0` assertions on each refusal path (the removed
    QSL'
- id: TM-001 Functional Requirement Coverage 1.10
  path: spec/exact/matrix/tests.md
  role: examined
  excerpt: 'shared-corpus vectors; see Evidence Locations) — that every refusal precedes
    the `function.call` charge, agreed on both sides. Relative order *among* the four
    checks themselves (arity, value kind, dangling reference, unknown function) is
    not something any vector needs to discriminate for AC-5 to be met, since each
    corpus vector isolates exactly one violation by design; that ordering is instead
    verified by the runtime-only tests in `tests/exact_function_application.rs` (see
    Evidence Locations), which AC-2/AC-3 already cover. Body semantics have no shared
    corpus either, for the same reason:'
- id: TM-001 Functional Requirement Coverage 1.11
  path: spec/exact/matrix/tests.md
  role: examined
  excerpt: 'AC-5 does not claim them. |

    | FR-273 | FR-273-AC-4 | TC-195 | ✅ implemented: `negotiate_ieee(&[IeeeItemRequirement],
    &IeeeBackendCapabilities)` receives no `Meter` at all, so no application-time
    charge is reachable from it by construction — the evidence is that signature plus
    the `compile_fail` doctest on `IeeeDisposition` (`src/exact/ieee.rs`) proving
    no conversion path from a disposition into `Outcome`/`InputRefusal` exists. `tc_195_negotiate_ieee_takes_no_meter_by_signature`
    inspects that signature and confirms negotiation still runs and reports one disposition
    per requirement; it carries'
- id: TM-001 Functional Requirement Coverage 1.12
  path: spec/exact/matrix/tests.md
  role: examined
  excerpt: 'no `Meter` assertion of its own, since a `Meter` never passed to `negotiate_ieee`
    cannot be evidence of anything the call did |

    | FR-275 | FR-275-AC-1, FR-275-AC-2, FR-275-AC-3, FR-275-AC-4, FR-275-AC-5, FR-275-AC-6,
    FR-275-AC-12, FR-275-AC-13, FR-275-AC-14, FR-275-AC-15, FR-275-AC-16, FR-275-AC-17,
    FR-275-AC-18, FR-275-AC-19 | TC-197 | 🚧 planned (Linear IR-349; AC-16 and AC-18
    are unmet while the residue exists, and the residue is not authorized: no exception,
    no expiry, no approval; AC-17 can be checked today): the runtime still holds its
    own copy of the kernel and of the residue. The'
- id: TM-001 Functional Requirement Coverage 1.13
  path: spec/exact/matrix/tests.md
  role: examined
  excerpt: IR-349 foundation slice (floor, dependency, bans, one copy; it deletes
    nothing) made AC-3 to AC-6 true (the optional `quire-exact` git dependency at
    `branch = "main"`, one lock entry, `make deny`'s one-copy check); AC-1, AC-2,
    AC-12 and AC-13 need the copy deleted; interface-001-AC-7 is likewise unmet while
    the runtime's copy of `Frame`, `Body`, `CheckedPackage`, `Evaluation` and `plan_call`
    exists. AC-1 and AC-16 now assert the end state of no `exact` module and no re-export
    (AD-004 step 1) and are unmet today, planned until the IR-349 code steps land;
    AC-17 restates its exception as the
- id: TM-001 Functional Requirement Coverage 1.14
  path: spec/exact/matrix/tests.md
  role: examined
  excerpt: '`scalar` items and stays checkable today. Per interface-001 criterion,
    see the interface-001 rows in the core matrix |

    | FR-275 | FR-275-AC-7, FR-275-AC-8 | TC-198 | ✅ implemented (IR-349 foundation
    slice; evidence is a gate script, not a `tc_NNN` test): `deny.toml` bans every
    listed QSL crate, and `make deny-mutations` (`scripts/check_deny_bans.sh`) adds
    `qsl-eval`, `qsl-replay`, `qsl-semantics` and `quire-spec-language` as normal
    and as dev dependencies in a scratch copy and requires cargo-deny''s `banned`
    error for each |

    | FR-275 | FR-275-AC-9, FR-275-AC-10, FR-275-AC-11, FR-275-AC-20,'
- id: TM-001 Functional Requirement Coverage 1.15
  path: spec/exact/matrix/tests.md
  role: examined
  excerpt: 'FR-275-AC-21 | TC-199 | ✅ implemented (IR-349 foundation slice; evidence
    is gate targets, not a `tc_NNN` test): `make test-features` row `build-exact-no-std-msrv`
    builds `exact` without `std` for `thumbv7em-none-eabi` on 1.98.1 with `quire-exact`
    in the graph; `make msrv` and `make size` run on 1.98.1; `make size` fails when
    the footprint graph holds `quire-exact` and when the linked size leaves the band
    |'
- id: TM-001 Test Case Summary 1.1
  path: spec/exact/matrix/tests.md
  role: examined
  excerpt: '| Test ID | Title | Type | Priority | Traces To | Status |

    |---|---|---|---|---|---|

    | TC-016 | Inspect the exact outcome envelope and vocabulary | Unit | P0 | FR-006-AC-1,
    FR-006-AC-3, FR-006-AC-6 | ✅ implemented |

    | TC-017 | Meter charges before work and deny them without effect | Unit | P0
    | FR-006-AC-3, FR-006-AC-4 | ✅ implemented |

    | TC-018 | Agree with the authority on integer division vectors | Integration
    | P0 | FR-007-AC-1, FR-007-AC-6 | 🚧 partly evidenced: local allocation checks
    are implemented; QSL shared-corpus agreement remains planned (Linear IR-669, open
    quire-integration'
- id: TM-001 Test Case Summary 1.2
  path: spec/exact/matrix/tests.md
  role: examined
  excerpt: 'owner; IR-430 removal is Done) |

    | TC-019 | Agree with the authority on exact decimal vectors | Integration | P0
    | FR-007-AC-2, FR-007-AC-6 | 🚧 partly evidenced: local allocation checks are implemented;
    QSL shared-corpus agreement remains planned (Linear IR-669, open quire-integration
    owner; IR-430 removal is Done) |

    | TC-020 | Agree with the authority on IEEE profile vectors | Integration | P0
    | FR-006-AC-2, FR-007-AC-3, FR-007-AC-6 | 🚧 planned (Linear IR-669, open quire-integration
    owner; IR-430 removal is Done): the QSL agreement oracle is removed from this
    repository, no test evidences it'
- id: TM-001 Test Case Summary 1.3
  path: spec/exact/matrix/tests.md
  role: examined
  excerpt: 'here |

    | TC-021 | Agree with the authority on text and enum vectors | Integration | P0
    | FR-006-AC-2, FR-007-AC-4, FR-007-AC-6 | 🚧 planned (Linear IR-669, open quire-integration
    owner; IR-430 removal is Done): the QSL agreement oracle is removed from this
    repository, no test evidences it here |

    | TC-022 | Agree with the authority on quantity and unit vectors | Integration
    | P0 | FR-006-AC-2, FR-007-AC-5, FR-007-AC-6 | 🚧 planned (Linear IR-669, open
    quire-integration owner; IR-430 removal is Done): the QSL agreement oracle is
    removed from this repository, no test evidences it here |

    | TC-023 |'
- id: TM-001 Test Case Summary 1.4
  path: spec/exact/matrix/tests.md
  role: examined
  excerpt: 'Meter integer, rational, ordering and Boolean operations | Property |
    P0 | FR-007-AC-7 | ✅ implemented |

    | TC-024 | Construct composite values and their declaration environment | Unit
    | P0 | FR-008-AC-1, FR-008-AC-2, FR-008-AC-7, FR-008-AC-8, FR-008-AC-9, FR-008-AC-13
    | ✅ implemented |

    | TC-025 | Construct collections and order them by the canonical key | Property
    | P0 | FR-008-AC-3, FR-008-AC-4, FR-008-AC-7, FR-008-AC-8, FR-008-AC-9, FR-008-AC-10,
    FR-008-AC-11, FR-008-AC-12 | 🚧 steps 1–2 and 4–7 implemented in `tests/exact_collection.rs`;
    steps 3 and 5 cover the set and bag only, so'
- id: TM-001 Test Case Summary 1.5
  path: spec/exact/matrix/tests.md
  role: examined
  excerpt: 'FR-008-AC-10 through FR-008-AC-12''s sequence and ordered-set cases are
    not yet tested |

    | TC-026 | Evaluate the equality matrix and terminal references | Unit | P0 |
    FR-008-AC-5, FR-008-AC-6, FR-008-AC-7, FR-008-AC-8 | ✅ implemented |

    | TC-030 | Dispose negotiation items independently and in input order | Unit |
    P0 | FR-009-AC-1, FR-009-AC-2, FR-009-AC-3, FR-009-AC-4 | ✅ implemented |

    | TC-031 | Fire one injected denial with a limit-independent record | Unit | P0
    | FR-010-AC-1, FR-010-AC-2, FR-010-AC-3, FR-010-AC-4, FR-010-AC-5, FR-010-AC-6
    | ✅ implemented |

    | TC-032 | Read a determinate'
- id: TM-001 Test Case Summary 1.6
  path: spec/exact/matrix/tests.md
  role: examined
  excerpt: 'meter state at every stop | Unit | P0 | FR-011-AC-1, FR-011-AC-2, FR-011-AC-3,
    FR-011-AC-4, FR-011-AC-5, FR-011-AC-6, FR-011-AC-7, FR-011-AC-8 | ✅ implemented
    |

    | TC-033 | Carry the compiler vocabulary byte-exactly | Unit | P0 | FR-012-AC-1,
    FR-012-AC-2, FR-012-AC-3, FR-012-AC-4, FR-012-AC-5 | ✅ implemented |

    | TC-034 | Pin the exact semantics the agreement corpus does not reach | Unit
    | P0 | FR-007-AC-8, FR-007-AC-9, FR-007-AC-10, FR-007-AC-11, FR-007-AC-12 | ✅
    implemented |

    | TC-035 | Pin boxed value and type structs'' hand-written Debug rendering | Unit
    | P1 | FR-007-AC-13 | ✅ implemented'
- id: TM-001 Test Case Summary 1.7
  path: spec/exact/matrix/tests.md
  role: examined
  excerpt: '|

    | TC-194 | Apply checked functions totally, before any charge | Unit | P0 | FR-273-AC-1,
    FR-273-AC-2, FR-273-AC-3, FR-273-AC-5, FR-273-AC-6, FR-273-AC-7 | ✅ implemented
    |

    | TC-195 | Negotiate a function''s undischargeable capability as unsupported |
    Unit | P0 | FR-273-AC-4, FR-009-AC-5 | ✅ implemented |

    | TC-197 | Inspect that the runtime holds one kernel and no copy | Integration
    | P0 | FR-275-AC-1, FR-275-AC-2, FR-275-AC-3, FR-275-AC-4, FR-275-AC-5, FR-275-AC-6,
    FR-275-AC-12, FR-275-AC-13, FR-275-AC-14, FR-275-AC-15, FR-275-AC-16, FR-275-AC-17,
    FR-275-AC-18, FR-275-AC-19 | 🚧 planned'
- id: TM-001 Test Case Summary 1.8
  path: spec/exact/matrix/tests.md
  role: examined
  excerpt: '(Linear IR-349): the kernel copy is deleted in part 1 and the residue
    in part 2 (remaining RT deletion owned by open IR-349 step 2 and IR-583; QSL-358
    extraction is Done); step 7 fails while the residue exists |

    | TC-198 | Fail the build on a dependency on a guarded QSL crate | Integration
    | P0 | FR-275-AC-7, FR-275-AC-8 | ✅ implemented (IR-349 foundation slice): `deny.toml`
    entries and `make deny-mutations` |

    | TC-199 | Build the exact profile no_std and keep the default footprint | Integration
    | P0 | FR-275-AC-9, FR-275-AC-10, FR-275-AC-11, FR-275-AC-20, FR-275-AC-21 | ✅
    implemented (IR-349'
- id: TM-001 Test Case Summary 1.9
  path: spec/exact/matrix/tests.md
  role: examined
  excerpt: 'foundation slice): the exact profile builds on 1.98.1 with `quire-exact`
    in the graph and the footprint graph holds none; the runtime''s own copy is still
    deleted by later parts |'
- id: TM-001 Test Case Summary 2.1
  path: spec/exact/matrix/tests.md
  role: examined
  excerpt: 'FR-009-AC-5, FR-010-AC-6 and FR-273-AC-4 are verified by `compile_fail`
    doctests, and TC-198 and

    TC-199 by gate targets (`make deny-mutations`, `make test-features`, `make msrv`,
    `make size`;

    see Evidence Locations). Every other row is backed by a `tc_NNN` Rust test; executable
    semantic

    claims retain direct acceptance-criterion trace tags. Rows marked planned or partly
    evidenced above

    are the exceptions.'
- id: TM-001 Test Case Summary 3.1
  path: spec/exact/matrix/tests.md
  role: examined
  excerpt: '`FR-010-AC-5` is verified at both `check_injected` call sites: `Meter::charge`

    (`tc_031_further_charges_after_the_injected_denial_meter_normally`,

    `tc_031_work_accounting_is_correct_before_and_after_the_injected_denial`) and

    `Meter::charge_plan`

    (`tc_031_further_charge_plan_calls_after_the_injected_denial_meter_normally`,

    `tc_031_charge_plan_reservation_is_unaffected_by_the_injected_denial`).'
- id: TM-001 Evidence at the kernel move 1.1
  path: spec/exact/matrix/tests.md
  role: examined
  excerpt: 'FR-275 requires RT to consume `quire-exact` directly and remove its local
    kernel copy in

    IR-349, followed by RT evaluation-residue deletion owned by open IR-349 step 2
    and backlog IR-583.

    QSL-358 is Done and covered QSL-side extraction. This amendment is spec-only:

    all current test files and production definitions remain present. The status columns
    describe

    current evidence only. TC-016/017/023 and their local kernel criteria remain implemented;

    TC-018/019 and FR-007-AC-1/2 are partly evidenced because their allocation checks
    exist but

    the shared-corpus agreement oracle is absent. Future'
- id: TM-001 Evidence at the kernel move 1.2
  path: spec/exact/matrix/tests.md
  role: examined
  excerpt: 'deletion does not change those statuses

    before the code PR removes the tests.

    The existing vocabulary has no "verified upstream" status; no such status or upstream
    pass is

    claimed. Requirements, criteria and TC IDs remain, including obligations not yet
    mapped to an

    exact owner criterion. The computed `quire matrix` still finds current local trace
    binders;

    `tagged` reports their presence, not a semantic pass or future ownership. TC-032
    is not a

    binder for FR-007-AC-7: its retained lazy connective evidence is directly tagged
    only to

    FR-011-AC-3/AC-8, which own the permanent lazy behavior.'
- id: TM-001 Evidence at the kernel move 1.3
  path: spec/exact/matrix/tests.md
  role: examined
  excerpt: 'FR-007-AC-7 owns kernel

    operations and keeps TC-023''s current evidence disposition; it has no permanent
    lazy clause.'
- id: TM-001 Evidence at the kernel move 2.1
  path: spec/exact/matrix/tests.md
  role: examined
  excerpt: 'When code PR #95 removes the local kernel tests, its matrix update shall
    mark the affected

    rows planned with a stated reason under FR-275-AC-15, keeping any remaining partial
    evidence

    explicit. The file dispositions below describe that future state; no deletion
    or upstream

    verification is claimed by this amendment.'
- id: TM-001 Evidence at the kernel move 3.1
  path: spec/exact/matrix/tests.md
  role: examined
  excerpt: 'The owner mappings in TC-016/017/018/019/023 identify only matching contracts
    in

    `ix://agent-ix/quire-exact`. RT shall retain no substitute kernel tests, copied
    vectors or

    agreement tests (FR-275-AC-12). The lazy connective in TC-032 is RT-owned behavior,
    with

    FR-011-AC-3/AC-8 evidence retained; already-decided Boolean truth tables are kernel-owned.

    QSL-358 is Done (QSL-side extraction); open IR-349 step 2 and IR-583 own remaining
    RT residue deletion.

    IR-430 is Done (RT agreement-test removal). The remaining QSL agreement oracle
    in

    `quire-integration` has open quire-integration ticket IR-669;'
- id: TM-001 Evidence at the kernel move 3.2
  path: spec/exact/matrix/tests.md
  role: examined
  excerpt: 'an upstream

    arithmetic or allocation test does not close that gap. Kernel ownership mappings
    leave both workstreams distinct.'
- id: TM-001 Evidence at the kernel move 4.1
  path: spec/exact/matrix/tests.md
  role: examined
  excerpt: '| Test file | Test cases | Disposition | Reason |

    |---|---|---|---|

    | `tests/exact_function_application.rs` | TC-194, TC-195 | splits | the TC-194
    function-application tests are residue (FR-273, AD-002; not authorized): they
    run over the `quire-exact` `Value`, `Meter` and `Outcome`, then leave with the
    code; rows stay planned until QSL''s evidence exists. The TC-195 tests exercise
    `negotiate_ieee`, which is runtime-owned and stays (FR-275-AC-19, FR-009-AC-5)
    |

    | `tests/exact_negotiation.rs` | TC-030 | stays | the `negotiate_*` predicates
    are runtime-owned, not a QSL port (FR-009, FR-275) |

    |'
- id: TM-001 Evidence at the kernel move 4.2
  path: spec/exact/matrix/tests.md
  role: examined
  excerpt: '`tests/exact_vocabulary.rs` | TC-033 | leaves with the residue | residue
    unless `quire-exact` exports the vocabulary (FR-012) |

    | `tests/exact_equality.rs` | TC-026 | leaves with the residue | residue: `CheckedEquality`
    and the checking environment; its equality-plan assertions on a kernel item leave
    in step 1 |

    | `tests/exact_composite.rs` | TC-024 | splits | construction of kernel `Value`s
    leaves in step 1; declaration-environment checks leave with the residue |

    | `tests/exact_arithmetic.rs`, `tests/exact_allocation.rs` | TC-016, TC-017, TC-018,
    TC-019, TC-023 | leaves in step 1 | the files'
- id: TM-001 Evidence at the kernel move 4.3
  path: spec/exact/matrix/tests.md
  role: examined
  excerpt: 'test kernel behaviour (they also use `CHARGE_LOG_CAPACITY`, which `quire-exact`
    does not export, so that constant''s assertions go with the kernel''s accounting
    evidence): evidence belongs to the QSL repository, which this repository does
    not track |

    | `tests/exact_outcomes.rs` | TC-016, TC-017, TC-031 | splits | the kernel `Meter`,
    `Outcome` and injected-denial tests leave in step 1; the cases that use `TypeEnvironment`,
    `CheckedEquality`, `CheckedPackage`, `ObjectEnvironment`, `UnitGraph`, `EnumDeclaration`
    and `evaluate_quantity`, which `quire-exact` does not export, leave with the residue'
- id: TM-001 Evidence at the kernel move 4.4
  path: spec/exact/matrix/tests.md
  role: examined
  excerpt: '|

    | `tests/exact_collection.rs` | TC-025 | splits | the collection-algebra and canonical-key
    cases leave in step 1; the cases built on `TypeEnvironment` and `CompositeDeclaration`
    leave with the residue |

    | `tests/exact_meter_state.rs` | TC-032 | splits | `evaluate_boolean_short_circuit`
    lazy invocation/stop propagation (FR-011-AC-3/AC-8, step 5) stays in RT; already-decided
    Boolean and meter cases leave in step 1; `UnitGraph::admit`, `CompoundUnit` and
    `evaluate_quantity` cases leave with RT residue under open IR-349 step 2 and IR-583
    |

    | `tests/exact_semantics.rs` | TC-034 | splits | the'
- id: TM-001 Evidence at the kernel move 4.5
  path: spec/exact/matrix/tests.md
  role: examined
  excerpt: '`UnitGraph`, `Dimension` and `evaluate_quantity` cases leave with the
    residue; the rest leave in step 1 |

    | `tests/exact_debug_parity.rs` | TC-035 | splits | the `CompoundUnit`, `Dimension`
    and `EnumDeclaration` Debug pins leave with the residue; the rest leave in step
    1 |

    | `src/exact_accounting_tests.rs`, `src/exact_integer_tests.rs` | TC-023, TC-031,
    TC-032 | leaves in step 1 | current in-crate tests of kernel `Meter` and `Integer`
    internals that IR-349 will remove |'
- id: TM-001 Evidence at the kernel move 5.1
  path: spec/exact/matrix/tests.md
  role: examined
  excerpt: 'For the kernel, RT keeps the consumption checks TC-197 to TC-199 in the
    end state: direct

    consumption, no copied code and guarded edges. RT also keeps evidence of its own
    lazy connective

    in TC-032 and backend negotiation in TC-030/TC-195; kernel deletion shall not
    remove those checks.'
- id: TM-001 Evidence Locations 1.1
  path: spec/exact/matrix/tests.md
  role: examined
  excerpt: "- TC-198: `deny.toml` (the `[bans]` list and `[graph] all-features = true`)\
    \ and\n  `scripts/check_deny_bans.sh`, run by `make deny-mutations`, which adds\
    \ each banned crate to a\n  scratch copy of the tracked files and requires cargo-deny's\
    \ `banned` error. TC-199:\n  `scripts/run_feature_matrix.py` (row `build-exact-no-std-msrv`,\
    \ `make test-features`),\n  `make msrv` and `make size` (linked band, panic relocations\
    \ and the footprint-graph check that\n  `quire-exact` is absent). Neither has\
    \ a Rust test.\n- TC-194, TC-195: `tests/exact_function_application.rs` (`--features\
    \ exact`), landed under\n "
- id: TM-001 Evidence Locations 1.2
  path: spec/exact/matrix/tests.md
  role: examined
  excerpt: "agent-ix/quire-contract-runtime#34. FR-273-AC-4's evidence is a `compile_fail`\
    \ doctest on\n  `IeeeDisposition` (`src/exact/ieee.rs`), mirroring `InjectedDenial`'s.\n\
    - FR-273-AC-5 (TC-194's shared-corpus row): the QSL agreement oracle is removed\
    \ from this repository;\n  recreating it in agent-ix/quire-integration is planned\
    \ with open quire-integration ticket IR-669 (IR-430 removal is Done). The removed\
    \ oracle agreed on the closed `InputRefusal` vocabulary (`UnknownFunction`, `Arity`,\
    \ `WrongValueKind`,\n  `DanglingReference`) with its codes and causes, and the\
    \ charge count of one admitted call"
- id: TM-001 Evidence Locations 1.3
  path: spec/exact/matrix/tests.md
  role: examined
  excerpt: "(AP05).\n  It does **not** agree on check *ordering*: each of its five\
    \ vectors (AP01 through AP05) triggers\n  exactly one refusal in isolation — no\
    \ vector supplies a call violating two checks at once — so the\n  corpus cannot\
    \ distinguish an implementation that checks arity, then per-argument kind and\n\
    \  reference, then charges `function.call`, from one that checks in some other\
    \ order and happens to\n  agree on each single-violation vector's result. That\
    \ ordering claim is instead backed only by the\n  runtime-only tests in `tests/exact_function_application.rs`\n\
    \ "
- id: TM-001 Evidence Locations 1.4
  path: spec/exact/matrix/tests.md
  role: examined
  excerpt: "(`tc_194_arity_is_decided_before_any_per_argument_check`,\n  `tc_194_earlier_parameter_refusal_wins_over_a_later_dangling_reference`,\n\
    \  `tc_194_function_call_precedes_the_body`,\n  `tc_194_frame_call_charges_function_call_before_the_body_it_invokes`),\
    \ which construct vectors\n  that do carry two simultaneous violations specifically\
    \ to discriminate check order. Nor does the\n  corpus agree on function-body semantics:\
    \ AD-002 draws the runtime's boundary at typed values and\n  operators, so `Body`\
    \ is an opaque Rust closure while the authority's function bodies are a typed\n\
    \  `Expression` AST an"
- id: TM-001 Evidence Locations 1.5
  path: spec/exact/matrix/tests.md
  role: examined
  excerpt: "interpreter runs — there is no `Debug` rendering that could compare the\
    \ two,\n  so no shared corpus exists for arithmetic, `let`, `if`, recursion or\
    \ any other body form. Those\n  are covered by `tests/exact_function_application.rs`\
    \ against the runtime alone. AC-5 quantifies\n  over shared-corpus function-application\
    \ vectors only, so neither the check-ordering gap nor the\n  absent body-semantics\
    \ corpus is a gap in AC-5 itself — the corpus agrees on every vector it\n  supplies,\
    \ which is all AC-5 claims — and AC-5 is recorded as fully implemented.\n- TC-030:\
    \ `tests/exact_negotiation.rs`; TC-032"
- id: TM-001 Evidence Locations 1.6
  path: spec/exact/matrix/tests.md
  role: examined
  excerpt: "(current evidence; lazy FR-011-AC-3/AC-8 stays): `tests/exact_meter_state.rs`\
    \ and the in-crate\n  `src/exact_accounting_tests.rs` for the cumulative-counter\
    \ boundary no public operator can\n  reach;\n  TC-033: `tests/exact_vocabulary.rs`;\
    \ TC-034: `tests/exact_semantics.rs`; TC-035:\n  `tests/exact_debug_parity.rs`.\n\
    - TC-016, TC-017, TC-031: `tests/exact_outcomes.rs` and, for TC-031's check-before-mutate\
    \ ordering\n  across every counter, the in-crate `src/exact_accounting_tests.rs`\
    \ (needs `Charge`'s\n  crate-private builders to construct a charge that moves\
    \ every counter at once, so is reachable\n "
- id: TM-001 Evidence Locations 1.7
  path: spec/exact/matrix/tests.md
  role: examined
  excerpt: "only from inside the crate); TC-023: `tests/exact_arithmetic.rs`; allocation\
    \ bounds for\n  TC-016, TC-018, TC-019 and TC-023: `tests/exact_allocation.rs`.\
    \ All run with\n  `--features exact`. FR-010-AC-6's evidence is a `compile_fail`\
    \ doctest on `InjectedDenial`\n  (`src/exact/accounting.rs`): `occurrence: 0`\
    \ does not compile, so the malformed request cannot be\n  written.\n- TC-018 through\
    \ TC-022: The QSL agreement oracle is removed from this repository; recreating\
    \ it in agent-ix/quire-integration is planned with open quire-integration ticket\
    \ IR-669 (IR-430 removal is Done). TC-018/TC-019"
- id: TM-001 Evidence Locations 1.8
  path: spec/exact/matrix/tests.md
  role: examined
  excerpt: "currently retain only local allocation-bound evidence; it leaves in IR-349\n\
    \  and does not close the remaining agreement gap. Owner mappings are in the individual\
    \ TC documents.\n- TC-024: `tests/exact_composite.rs`; TC-025: `tests/exact_collection.rs`;\
    \ TC-026:\n  `tests/exact_equality.rs`. All run with `--features exact`."
```

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Checks and limitations

Quoin 0.28.1, Quire CLI 0.36.1 (engine 0.50.1), runtime 0.1.0 on luna. Spec validation exits 0: 56/56 grammar-clean, no grammar findings. Static matrix exits 0; it establishes tags, not passing execution. Strict coverage exits 1 at both base and head: 56 unbacked rows and five contradicted statuses; no new coverage regression. git diff --check exits 2 due eight inherited whitespace lines. No applicable AssuranceProfile. gh repo view returns HTTP 401; unauthenticated GitHub metadata returns HTTP 403. Visibility is unavailable; markers use private conservatively per the dispatch’s private-source instruction, not as verified repository metadata. Repo identity is established from origin. quoin write --types SpecReview fails exit 1, “write requires <repo_dir>”; documented quoin write . --types SpecReview --json succeeds. Installed module warnings: duplicate archetypes/inverse edge and inline-data-schema advisory, retained in logs; validation still succeeds. Owner FR358/359/361/362/363 examined as read-only local review-custody context; no upstream verification or release claim.
