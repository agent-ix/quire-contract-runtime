// SPDX-License-Identifier: AGPL-3.0-or-later
//! QSpec TC-186 text and enum identity semantics: runtime versus authority.
//!
//! Admission-only vectors are compiler work and are not evaluated here:
//! T05b (source type `Text` without bounds is `invalid_syntax` at source
//! recognition) and the stale-key half of T09 (retained declaration/member
//! keys that no longer match their preimage are `invalid_semantic_graph` at
//! owner/key admission). The recomputed-key half of T09 is evaluated.

#[macro_use]
mod support;

use support::rt_side::*;

/// Vectors evaluated through both boundaries.
const EVALUATED: [&str; 17] = [
    "T01", "T02", "T03", "T04", "T05", "T06", "T06b", "T07", "T08", "T09", "T10", "T11", "T12",
    "T13", "T14", "T15", "T16",
];
/// Compiler-owned admission vectors.
const ADMISSION_ONLY: [&str; 2] = ["T05b", "T09 stale-key half"];

/// Trace: TC-021, FR-007-AC-4, FR-007-AC-6
#[test]
fn tc_021_every_tc186_vector_is_evaluated_or_admission_only() {
    let mut expected: Vec<String> = (1..=16).map(|n| format!("T{n:02}")).collect();
    expected.insert(5, "T05b".into());
    expected.insert(7, "T06b".into());
    let mut all: Vec<&str> = EVALUATED.to_vec();
    all.insert(5, "T05b");
    assert_eq!(all, expected);
    assert!(ADMISSION_ONLY.iter().all(|id| id.starts_with("T05b") || id.starts_with("T09")));
    println!("TC-186 agreement: {} evaluated, {} admission-only", EVALUATED.len(), ADMISSION_ONLY.len());
}

const E_ACUTE: &str = "\u{e9}";
const E_COMBINING: &str = "e\u{301}";

/// `TextProfile::ALL` indexes.
const SCALARS: usize = 0;
const NFC: usize = 1;
const NFD: usize = 2;
const NFKC: usize = 3;
const NFKD: usize = 4;
const BINARY: usize = 5;

/// Trace: TC-021, FR-007-AC-4
#[test]
fn tc_021_t01_t02_t03_profiles_select_the_equality_domain() {
    let equal = |left: &'static str, right: &'static str, p: usize| {
        agree! {
            compare_text(ComparisonOperator::Equal, &text(left, TextProfile::ALL[p]), &text(right, TextProfile::ALL[p]), &mut Meter::new(UNLIMITED))
        }
        .unwrap()
        .completed()
        .unwrap()
    };
    assert!(!equal(E_ACUTE, E_COMBINING, SCALARS));
    assert!(equal(E_ACUTE, E_COMBINING, NFC));
    assert!(equal(E_ACUTE, E_COMBINING, NFD));
    let retained = agree! {
        [TextProfile::Nfc, TextProfile::Nfd].map(|profile| (text(E_ACUTE, profile).retained().to_owned(), text(E_COMBINING, profile).retained().to_owned()))
    };
    assert_eq!(
        retained,
        [(E_ACUTE.to_owned(), E_ACUTE.to_owned()), (E_COMBINING.to_owned(), E_COMBINING.to_owned())]
    );
    assert!(equal("\u{fb00}", "ff", NFKC));
    assert!(equal("\u{fb00}", "ff", NFKD));
    assert!(!equal("\u{fb00}", "ff", NFC));
}

/// Trace: TC-021, FR-007-AC-4
#[test]
fn tc_021_t04_t05_t13_admission_bounds_use_the_retained_profile_length() {
    let admit = |input: &'static str, min: u64, max: u64, p: usize| {
        agree! {
            admitted_length(admit_text(&payload(input), &text_type(min, max, TextProfile::ALL[p]), &mut Meter::new(UNLIMITED)))
        }
    };
    let refused = Outcome::Refused(Refusal::TextLengthOutOfDomain);
    // T04
    assert_eq!(admit(E_ACUTE, 1, 1, NFC), Outcome::Completed(1));
    assert_eq!(admit(E_ACUTE, 1, 1, BINARY), refused);
    // T05
    assert_eq!(admit("", 0, 0, NFC), Outcome::Completed(0));
    assert_eq!(admit("a", 0, 0, NFC), refused);
    assert_eq!(admit("a", 1, 1, NFC), Outcome::Completed(1));
    // T13
    assert_eq!(admit(E_COMBINING, 1, 1, NFC), Outcome::Completed(1));
    assert_eq!(admit(E_COMBINING, 1, 1, NFD), refused);
    assert_eq!(admit(E_COMBINING, 1, 1, SCALARS), refused);
    assert_eq!(admit(E_ACUTE, 2, 2, NFD), Outcome::Completed(2));
}

/// Trace: TC-021, FR-007-AC-4
#[test]
fn tc_021_t06_t06b_utf8_reader_and_source_provenance() {
    let invalid = agree! { TextPayload::from_utf8(&[0xc3, 0x28]).map(|_| ()) };
    assert_eq!(invalid, Err(InvalidUtf8 { valid_up_to: 0 }));

    let (payloads, equal) = agree! {{
        let payloads = [
            TextPayload::from_source_literal("\"\u{e9}\"").unwrap(),
            TextPayload::from_source_literal("\"\\u00e9\"").unwrap(),
            TextPayload::from_utf8(&[0xc3, 0xa9]).unwrap(),
        ];
        let binary = text_type(0, 8, TextProfile::BinaryUtf8);
        let texts: Vec<Text> = payloads
            .iter()
            .map(|p| admit_text(p, &binary, &mut Meter::new(UNLIMITED)).completed().unwrap())
            .collect();
        let equal: Vec<Result<Outcome<bool>, IllTyped>> = [(0, 1), (0, 2), (1, 2)]
            .into_iter()
            .map(|(a, b)| compare_text(ComparisonOperator::Equal, &texts[a], &texts[b], &mut Meter::new(UNLIMITED)))
            .collect();
        (payloads, equal)
    }};
    assert!(payloads.iter().all(|p| p.bytes() == [0xc3, 0xa9]));
    assert_eq!(
        payloads.each_ref().map(TextPayload::provenance),
        [
            &TextProvenance::SourceLiteral("\"\u{e9}\"".into()),
            &TextProvenance::SourceLiteral("\"\\u00e9\"".into()),
            &TextProvenance::Runtime,
        ]
    );
    assert!(equal.into_iter().all(|e| e == Ok(Outcome::Completed(true))));
}

/// Trace: TC-021, FR-007-AC-4, FR-006-AC-2
#[test]
fn tc_021_t07_t09_t15_enum_identity_is_the_declaration_node_never_the_spelling() {
    let (t07, t09, t15) = agree! {{
        let a = enum_declaration("A", true, &["READY", "DONE"]).unwrap();
        let b = enum_declaration("B", true, &["READY", "DONE"]).unwrap();
        let reordered = enum_declaration("A", true, &["DONE", "READY"]).unwrap();
        let zero = || Meter::new(limits([0; 10]));
        let mut t15 = zero();
        let t07 = compare_enum(ComparisonOperator::Equal, &a.value("READY").unwrap(), &b.value("READY").unwrap(), &mut t15);
        let t09 = compare_enum(ComparisonOperator::Equal, &a.value("READY").unwrap(), &reordered.value("READY").unwrap(), &mut zero());
        (t07, t09, t15.admitted_charges().to_vec())
    }};
    let distinct = Err(IllTyped { cause: IllTypedCause::DistinctEnumDeclarations });
    assert_eq!(t07, distinct);
    assert_eq!(t09, distinct);
    assert_eq!(t15, []);
}

/// Trace: TC-021, FR-007-AC-4, FR-006-AC-2
#[test]
fn tc_021_t08_t15_ordered_members_order_and_unordered_ordering_refuses() {
    let (ordered, unordered, charges) = agree! {{
        let ordered = enum_declaration("A", true, &["READY", "DONE"]).unwrap();
        let unordered = enum_declaration("U", false, &["DONE", "READY"]).unwrap();
        let mut zero = Meter::new(limits([0; 10]));
        (
            compare_enum(ComparisonOperator::Less, &ordered.value("READY").unwrap(), &ordered.value("DONE").unwrap(), &mut Meter::new(UNLIMITED)),
            compare_enum(ComparisonOperator::Less, &unordered.value("READY").unwrap(), &unordered.value("DONE").unwrap(), &mut zero),
            zero.admitted_charges().to_vec(),
        )
    }};
    assert_eq!(ordered, Ok(Outcome::Completed(true)));
    assert_eq!(unordered, Err(IllTyped { cause: IllTypedCause::UnorderedEnumOrdering }));
    assert_eq!(charges, []);
}

/// Trace: TC-021, FR-007-AC-4
#[test]
fn tc_021_t10_distinct_profiles_are_ill_typed_not_false() {
    let outcome = agree! {
        compare_text(ComparisonOperator::Equal, &text("abc", TextProfile::Nfc), &text("abc", TextProfile::BinaryUtf8), &mut Meter::new(limits([0; 10])))
    };
    assert_eq!(outcome, Err(IllTyped { cause: IllTypedCause::DistinctTextProfiles }));
}

const T11: [u64; 10] = [0, 0, 0, 5, 3, 2, 0, 2, 6, 1];

/// Trace: TC-021, FR-007-AC-4, FR-006-AC-3, FR-006-AC-4
#[test]
fn tc_021_t11_nfc_exact_tuple_and_named_denials() {
    let (exact, denied) = agree! {{
        let left = payload(E_ACUTE);
        let right = payload(E_COMBINING);
        let run = |m: &mut Meter| {
            let texts = [&left, &right].map(|p| admit_text(p, &text_type(0, 4, TextProfile::Nfc), &mut Meter::new(UNLIMITED)).completed().unwrap());
            compare_text(ComparisonOperator::Equal, &texts[0], &texts[1], m)
        };
        (metered(limits(T11), run), denials(limits(T11), run))
    }};
    use ChargePoint::*;
    assert_eq!(exact.0, Ok(Outcome::Completed(true)));
    assert_eq!(
        exact.1,
        [TextInputBytes, TextDecodeScalars, TextNormalizeInput, TextNormalizeOutput, TextNormalizeOutput, TextResultRetain]
    );
    assert_eq!(exact.2, T11);
    let named: Vec<_> = denied
        .into_iter()
        .filter(|(point, occurrence, ..)| (*point, *occurrence) == (TextNormalizeOutput, 2) || *point == TextResultRetain)
        .collect();
    assert_eq!(named.len(), 2);
    for (work, (point, _, outcome, results)) in [4, 5].into_iter().zip(named) {
        assert_eq!(outcome, Ok(Outcome::Incomplete(work_denied(work, point))));
        assert_eq!(results, 0);
    }
}

/// Trace: TC-021, FR-007-AC-4
#[test]
fn tc_021_t12_scalar_and_unsigned_byte_lexicographic_order() {
    let less = agree! {
        [
            compare_text(ComparisonOperator::Less, &text("a", TextProfile::UnicodeScalars), &text("b", TextProfile::UnicodeScalars), &mut Meter::new(UNLIMITED)),
            compare_text(ComparisonOperator::Less, &text("\u{7f}", TextProfile::BinaryUtf8), &text("\u{80}", TextProfile::BinaryUtf8), &mut Meter::new(UNLIMITED)),
        ]
    };
    assert_eq!(less, [Ok(Outcome::Completed(true)), Ok(Outcome::Completed(true))]);
    assert_eq!(payload("\u{80}").bytes(), [0xc2, 0x80]);
}

const T14: [u64; 10] = [0, 0, 0, 5, 3, 0, 0, 2, 3, 1];

/// Trace: TC-021, FR-007-AC-4, FR-006-AC-3
#[test]
fn tc_021_t14_non_normalizing_profiles_charge_no_normalization() {
    for p in [SCALARS, BINARY] {
        let (exact, short) = agree! {{
            let run = |m: &mut Meter| {
                compare_text(ComparisonOperator::Equal, &text(E_ACUTE, TextProfile::ALL[p]), &text(E_COMBINING, TextProfile::ALL[p]), m)
            };
            let mut work = T14;
            work[8] = 2;
            (metered(limits(T14), run), metered(limits(work), run))
        }};
        use ChargePoint::*;
        assert_eq!(exact.0, Ok(Outcome::Completed(false)));
        assert_eq!(exact.1, [TextInputBytes, TextDecodeScalars, TextResultRetain]);
        assert_eq!(exact.2, T14);
        assert_eq!(
            short.0,
            Ok(Outcome::Incomplete(incomplete(LimitKind::WorkUnits, 2, 2, int(1), TextResultRetain)))
        );
    }
}

/// Trace: TC-021, FR-007-AC-4, FR-006-AC-3
#[test]
fn tc_021_t16_length_refusal_follows_the_profile_charge_that_measures_it() {
    let run = |input: &'static str, p: usize, tuple: [u64; 10]| {
        agree! {
            metered(limits(tuple), |m| admitted_length(admit_text(&payload(input), &text_type(1, 1, TextProfile::ALL[p]), m)))
        }
    };
    let refused = Outcome::Refused(Refusal::TextLengthOutOfDomain);
    use ChargePoint::*;

    let nfd = [0, 0, 0, 3, 2, 2, 0, 1, 5, 1];
    let exact = run(E_COMBINING, NFD, nfd);
    assert_eq!(exact.0, refused);
    assert_eq!(exact.1, [TextInputBytes, TextDecodeScalars, TextNormalizeInput, TextNormalizeOutput, TextNormalizeOutput]);
    let mut short = nfd;
    short[5] = 1;
    let short = run(E_COMBINING, NFD, short);
    assert_eq!(short.1, [TextInputBytes, TextDecodeScalars, TextNormalizeInput, TextNormalizeOutput]);
    let expected = incomplete(LimitKind::NormalizedScalars, 1, 1, int(2), TextNormalizeOutput);
    assert_eq!(short.0, Outcome::Incomplete(expected));

    let scalars = [0, 0, 0, 3, 2, 0, 0, 1, 2, 1];
    let exact = run(E_COMBINING, SCALARS, scalars);
    assert_eq!(exact.0, refused);
    assert_eq!(exact.1, [TextInputBytes, TextDecodeScalars]);
    let mut short = scalars;
    short[4] = 1;
    let expected = incomplete(LimitKind::TextScalars, 1, 0, int(2), TextDecodeScalars);
    assert_eq!(run(E_COMBINING, SCALARS, short).0, Outcome::Incomplete(expected));

    let binary = [0, 0, 0, 2, 0, 0, 0, 1, 1, 1];
    let exact = run(E_ACUTE, BINARY, binary);
    assert_eq!(exact.0, refused);
    assert_eq!(exact.1, [TextInputBytes]);
    let mut short = binary;
    short[3] = 1;
    let expected = incomplete(LimitKind::TextInputBytes, 1, 0, int(2), TextInputBytes);
    assert_eq!(run(E_ACUTE, BINARY, short).0, Outcome::Incomplete(expected));
}

const SEQUENCES: [&str; 12] = [
    "", "a", "b", "ab", E_ACUTE, E_COMBINING, "\u{fb00}", "ff", "\u{7f}", "\u{80}", "\u{1e0a}\u{323}", "\u{1e0c}\u{307}",
];

/// Trace: TC-021, FR-007-AC-4, FR-006-AC-4
#[test]
fn tc_021_generated_profiles_operators_and_denials_agree() {
    let mut vectors = 0_u32;
    for p in 0..TextProfile::ALL.len() {
        for left in SEQUENCES {
            for right in SEQUENCES {
                for o in 0..ComparisonOperator::ALL.len() {
                    let outcome = agree! {{
                        let run = |m: &mut Meter| compare_text(ComparisonOperator::ALL[o], &text(left, TextProfile::ALL[p]), &text(right, TextProfile::ALL[p]), m);
                        (metered(UNLIMITED, run), if o == 0 { denials(UNLIMITED, run) } else { Vec::new() })
                    }};
                    let exact = outcome.0 .0.unwrap().completed().unwrap();
                    let (l, r) = (text(left, TextProfile::ALL[p]), text(right, TextProfile::ALL[p]));
                    let ordering = if p == BINARY {
                        l.retained().as_bytes().cmp(r.retained().as_bytes())
                    } else {
                        l.retained().chars().cmp(r.retained().chars())
                    };
                    let expected = match ComparisonOperator::ALL[o] {
                        ComparisonOperator::Equal => ordering.is_eq(),
                        ComparisonOperator::NotEqual => ordering.is_ne(),
                        ComparisonOperator::Less => ordering.is_lt(),
                        ComparisonOperator::LessOrEqual => ordering.is_le(),
                        ComparisonOperator::Greater => ordering.is_gt(),
                        ComparisonOperator::GreaterOrEqual => ordering.is_ge(),
                    };
                    assert_eq!(exact, expected, "{left:?} {right:?} {p} {o}");
                    for (point, _, denied, results) in outcome.1 {
                        assert!(matches!(denied, Ok(Outcome::Incomplete(ref record)) if record.charge_point == point));
                        assert_eq!(results, 0);
                    }
                    vectors += 1;
                }
            }
        }
    }
    assert_eq!(vectors, 6 * 12 * 12 * 6);

    let declarations: [(&str, bool, &[&str]); 4] = [
        ("One", true, &["ONLY"]),
        ("Ordered", true, &["C", "A", "B"]),
        ("Unordered", false, &["A", "B", "C"]),
        ("Other", true, &["A", "B", "C"]),
    ];
    let mut enum_vectors = 0_u32;
    for (dl, (ln, lo, lm)) in declarations.into_iter().enumerate() {
        for (dr, (rn, ro, rm)) in declarations.into_iter().enumerate() {
            for lc in lm {
                for rc in rm {
                    for o in 0..ComparisonOperator::ALL.len() {
                        let outcome = agree! {{
                            let left = enum_declaration(ln, lo, lm).unwrap().value(lc).unwrap();
                            let right = enum_declaration(rn, ro, rm).unwrap().value(rc).unwrap();
                            metered(UNLIMITED, |m| compare_enum(ComparisonOperator::ALL[o], &left, &right, m))
                        }};
                        let operator = ComparisonOperator::ALL[o];
                        match outcome.0 {
                            Err(IllTyped { cause: IllTypedCause::DistinctEnumDeclarations }) => assert_ne!(dl, dr),
                            Err(IllTyped { cause: IllTypedCause::UnorderedEnumOrdering }) => {
                                assert!(!lo && operator.is_ordering());
                                assert!(outcome.1.is_empty());
                            }
                            Ok(Outcome::Completed(_)) => {
                                assert_eq!(dl, dr);
                                assert_eq!(
                                    outcome.1,
                                    [ChargePoint::EnumIdentityRead, ChargePoint::EnumIdentityRead, ChargePoint::EnumResultRetain]
                                );
                            }
                            other => panic!("unexpected {other:?}"),
                        }
                        enum_vectors += 1;
                    }
                }
            }
        }
    }
    assert_eq!(enum_vectors, (1 + 3 + 3 + 3) * (1 + 3 + 3 + 3) * 6);
}
