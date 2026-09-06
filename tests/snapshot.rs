use quire_contract_runtime::{CampaignReport, ContractIdentity, RequirementId, RevisionId};

fn report<'a>(requirement: &'a str, revision: &'a str) -> CampaignReport<'a> {
    CampaignReport::new(ContractIdentity::new(
        RequirementId::new(requirement),
        RevisionId::new(revision),
    ))
}

/// Trace: TC-015, FR-004-AC-4
#[test]
fn tc_015_snapshot_is_a_complete_immutable_capture() {
    let mut report = report("FR-\"é", "007");
    let snapshot = report.snapshot();
    report.record_discard();
    assert_eq!(snapshot.counts().total(), 0);
    assert_eq!(report.snapshot().counts().discarded(), 1);
    assert_eq!(snapshot.identity().requirement.as_str(), "FR-\"é");
    assert_eq!(snapshot.identity().revision.as_str(), "007");
    assert!(!snapshot.at_limit());
}

#[cfg(feature = "snapshot-json")]
mod json {
    use super::report;
    use quire_contract_runtime::{
        decode_campaign_snapshot as decode, encode_campaign_snapshot as encode, ClauseId,
        ExecutionPoint, FailureDetail, FailureKind, SnapshotError, Verdict, VerdictContext,
    };

    const EMPTY: &str = concat!(
        r#"{"schemaVersion":"runtime.campaign-snapshot/v1","requirement":"","revision":"","#,
        r#""counterSemantics":"saturating-u64-v1","counts":{"accepted":0,"rejected":0,"failed":0,"discarded":0}}"#
    );

    /// Trace: TC-015, FR-004-AC-4, FR-004-AC-5
    #[test]
    fn tc_015_actual_mixed_report_has_independent_exact_json() {
        let mut report = report("FR-\"é", "007");
        let context = VerdictContext::new(report.identity(), ExecutionPoint::new("post"), &[]);
        let detail = FailureDetail::new(ClauseId::new("c"), FailureKind::Postcondition, 1, None);
        for verdict in [
            Verdict::passed(context),
            Verdict::failed_postcondition(context, detail),
            Verdict::rejected_precondition(context, detail),
        ] {
            report.record_verdict(&verdict).unwrap();
        }
        report.record_discard();
        let expected = concat!(
            r#"{"schemaVersion":"runtime.campaign-snapshot/v1","requirement":"FR-\"é","revision":"007","#,
            r#""counterSemantics":"saturating-u64-v1","counts":{"accepted":2,"rejected":1,"failed":1,"discarded":1}}"#
        );
        assert_eq!(encode(&report.snapshot()).unwrap(), expected.as_bytes());
        let imported = decode(expected.as_bytes()).unwrap();
        assert_eq!(imported.snapshot(), report.snapshot());
        assert_eq!(imported.snapshot().counts().total(), 4);
        assert_eq!(
            encode(&super::report("", "").snapshot()).unwrap(),
            EMPTY.as_bytes()
        );
        assert_eq!(
            decode(EMPTY.as_bytes())
                .unwrap()
                .snapshot()
                .counts()
                .total(),
            0
        );
    }

    /// Trace: TC-015, FR-004-AC-5
    #[test]
    fn tc_015_every_member_is_required_unique_closed_and_typed() {
        let value: serde_json::Value = serde_json::from_str(EMPTY).unwrap();
        for field in [
            "schemaVersion",
            "requirement",
            "revision",
            "counterSemantics",
            "counts",
        ] {
            let mut missing = value.clone();
            missing.as_object_mut().unwrap().remove(field);
            assert!(
                decode(&serde_json::to_vec(&missing).unwrap()).is_err(),
                "missing {field}"
            );
            for replacement in [
                serde_json::Value::Null,
                serde_json::json!(42),
                serde_json::json!([]),
            ] {
                let mut wrong = value.clone();
                wrong[field] = replacement;
                assert!(
                    decode(&serde_json::to_vec(&wrong).unwrap()).is_err(),
                    "wrong {field}"
                );
            }
            let duplicate = EMPTY.replacen('{', &format!("{{\"{field}\":{},", value[field]), 1);
            assert!(decode(duplicate.as_bytes()).is_err(), "duplicate {field}");
        }
        for field in ["accepted", "rejected", "failed", "discarded"] {
            let mut missing = value.clone();
            missing["counts"].as_object_mut().unwrap().remove(field);
            assert!(decode(&serde_json::to_vec(&missing).unwrap()).is_err());
            let duplicate = EMPTY.replace(
                &format!("\"{field}\":0"),
                &format!("\"{field}\":0,\"{field}\":0"),
            );
            assert!(decode(duplicate.as_bytes()).is_err());
            for token in [
                "null",
                "true",
                "\"0\"",
                "[]",
                "{}",
                "-1",
                "-0",
                "0.0",
                "0e0",
                "18446744073709551616",
            ] {
                let wrong =
                    EMPTY.replace(&format!("\"{field}\":0"), &format!("\"{field}\":{token}"));
                assert!(decode(wrong.as_bytes()).is_err(), "{field} {token}");
            }
        }
        for wrong in [
            EMPTY.replacen('{', "{\"unexpected\":0,", 1),
            EMPTY.replace("\"accepted\":0", "\"unexpected\":0,\"accepted\":0"),
            EMPTY.replace("\"accepted\":0", "\"accepted\":0,\"\\u0061ccepted\":0"),
            EMPTY.replace(
                "\"revision\":\"\"",
                "\"revision\":\"\",\"\\u0072evision\":\"\"",
            ),
            format!("{EMPTY} null"),
            format!("{EMPTY}{EMPTY}"),
            EMPTY.replace(
                "runtime.campaign-snapshot/v1",
                "runtime.campaign-snapshot/v2",
            ),
            EMPTY.replace("saturating-u64-v1", "exact-u64-v1"),
            EMPTY.replace("\"failed\":0", "\"failed\":1"),
        ] {
            assert!(decode(wrong.as_bytes()).is_err(), "{wrong}");
        }
        assert!(decode(EMPTY.as_bytes()).is_ok());
    }

    /// Trace: TC-015, FR-004-AC-5, FR-004-AC-6
    #[test]
    fn tc_015_exact_maximum_and_sum_limit_are_not_overflow_history() {
        for (accepted, rejected, at_limit) in [
            (u64::MAX, 0, true),
            (u64::MAX - 1, 1, true),
            (u64::MAX - 1, 2, true),
            (u64::MAX - 2, 1, false),
        ] {
            let raw = EMPTY
                .replace("\"accepted\":0", &format!("\"accepted\":{accepted}"))
                .replace("\"rejected\":0", &format!("\"rejected\":{rejected}"));
            let imported = decode(raw.as_bytes()).unwrap();
            assert_eq!(imported.snapshot().at_limit(), at_limit);
            assert_eq!(imported.snapshot().counts().accepted(), accepted);
            assert_eq!(encode(&imported.snapshot()).unwrap(), raw.as_bytes());
        }
    }

    /// Trace: TC-015, FR-004-AC-5, FR-004-AC-7
    #[test]
    fn tc_015_byte_identity_unicode_and_depth_boundaries() {
        let mut padded = EMPTY.as_bytes().to_vec();
        padded.resize(65536, b' ');
        assert!(decode(&padded).is_ok());
        padded.push(b' ');
        assert_eq!(decode(&padded).unwrap_err(), SnapshotError::ResourceLimit);
        for field in ["requirement", "revision"] {
            let exact = EMPTY.replace(
                &format!("\"{field}\":\"\""),
                &format!("\"{field}\":\"{}\"", "\\u00e9".repeat(2048)),
            );
            assert!(decode(exact.as_bytes()).is_ok());
            let over = EMPTY.replace(
                &format!("\"{field}\":\"\""),
                &format!("\"{field}\":\"{}\"", "é".repeat(2049)),
            );
            assert_eq!(
                decode(over.as_bytes()).unwrap_err(),
                SnapshotError::ResourceLimit
            );
        }
        let exact = "\0".repeat(4096);
        assert!(
            encode(&super::report(&exact, &exact).snapshot())
                .unwrap()
                .len()
                < 65536
        );
        let over = "x".repeat(4097);
        assert_eq!(
            encode(&super::report(&over, "").snapshot()).unwrap_err(),
            SnapshotError::ResourceLimit
        );
        assert_eq!(
            encode(&super::report("", &over).snapshot()).unwrap_err(),
            SnapshotError::ResourceLimit
        );
        let nested = format!("{}0{}", "[".repeat(10000), "]".repeat(10000));
        assert!(decode(nested.as_bytes()).is_err());
        assert!(decode(
            EMPTY
                .replace("\"accepted\":0", &format!("\"accepted\":{nested}"))
                .as_bytes()
        )
        .is_err());
        let mut invalid_utf8 = EMPTY.as_bytes().to_vec();
        invalid_utf8[2] = 255;
        assert!(decode(&invalid_utf8).is_err());
        assert!(decode(
            EMPTY
                .replace("\"revision\":\"\"", "\"revision\":\"\\ud800\"")
                .as_bytes()
        )
        .is_err());
        assert!(decode(format!(" \n{EMPTY}\r\t").as_bytes()).is_ok());
    }
}
