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

    /// Trace: TC-015, FR-004-AC-5
    #[test]
    fn tc_015_structured_refusals_remain_distinct() {
        assert_eq!(decode(b"{}").unwrap_err(), SnapshotError::Malformed);
        assert_eq!(
            decode(EMPTY.replace("snapshot/v1", "snapshot/v2").as_bytes()).unwrap_err(),
            SnapshotError::UnsupportedVersion
        );
        assert_eq!(
            decode(EMPTY.replace("\"failed\":0", "\"failed\":1").as_bytes()).unwrap_err(),
            SnapshotError::InvalidCounts
        );
        // Accepted wire order and escape spelling need not equal deterministic encoder order.
        let reordered = r#"{"counts":{"discarded":0,"failed":0,"rejected":0,"accepted":0},"revision":"","requirement":"","counterSemantics":"saturating-u64-v1","\u0073chemaVersion":"runtime.campaign-snapshot/v1"}"#;
        assert_eq!(
            encode(&decode(reordered.as_bytes()).unwrap().snapshot()).unwrap(),
            EMPTY.as_bytes()
        );
    }

    /// Trace: TC-015, FR-004-AC-4, FR-004-AC-5
    #[test]
    fn tc_015_generated_unicode_identities_roundtrip_without_normalization() {
        use proptest::prelude::*;
        let strategy = (
            proptest::collection::vec(any::<char>(), 0..100),
            proptest::collection::vec(any::<char>(), 0..100),
            0u16..1000,
        );
        proptest::test_runner::TestRunner::default()
            .run(&strategy, |(left, right, discards)| {
                let left: String = left.into_iter().collect();
                let right: String = right.into_iter().collect();
                let mut report = report(&left, &right);
                for _ in 0..discards {
                    report.record_discard();
                }
                let encoded = encode(&report.snapshot()).unwrap();
                let imported = decode(&encoded).unwrap();
                prop_assert_eq!(imported.snapshot(), report.snapshot());
                Ok(())
            })
            .unwrap();
    }

    /// Trace: TC-015, FR-004-AC-7
    #[cfg(target_os = "linux")]
    #[test]
    fn tc_015_memory_ceiling_is_process_failure_not_semantic_refusal() {
        use std::io::Write;
        use std::os::unix::process::ExitStatusExt;
        const CHILD_ENV: &str = "QUIRE_SNAPSHOT_MEMORY_PROBE";
        if let Ok(mode) = std::env::var(CHILD_ENV) {
            let raw = EMPTY.replace(
                "\"requirement\":\"\"",
                &format!("\"requirement\":\"{}\"", "\\u00e9".repeat(2048)),
            );
            let mut stderr = std::io::stderr().lock();
            if mode == "pressure" {
                // Keep metadata and the valid input allocated before applying pressure.
                // All filler reservations are fallible. The real decoder's allocating path
                // is invoked only after the capped process cannot reserve another small block.
                let mut held: Vec<Vec<u8>> = Vec::with_capacity(65536);
                for size in [8192, 1024, 64] {
                    loop {
                        let mut block = Vec::new();
                        if block.try_reserve_exact(size).is_err() {
                            break;
                        }
                        block.resize(size, 0);
                        assert!(held.len() < held.capacity());
                        held.push(block);
                    }
                }
                stderr
                    .write_all(b"snapshot-probe: entering real decoder under pressure\n")
                    .unwrap();
                let outcome = decode(raw.as_bytes());
                std::hint::black_box(&held);
                // These explicit returns are NOT a successful resource-failure control.
                std::process::exit(if outcome.is_ok() { 32 } else { 31 });
            }
            assert_eq!(mode, "healthy");
            assert!(decode(raw.as_bytes()).is_ok());
            std::process::exit(0);
        }
        let executable = std::env::current_exe().unwrap();
        let run = |mode: &str| {
            std::process::Command::new("prlimit")
                .args(["--as=134217728", "--core=0", "--"])
                .arg(&executable)
                .args([
                    "--exact",
                    "json::tc_015_memory_ceiling_is_process_failure_not_semantic_refusal",
                    "--nocapture",
                ])
                .env(CHILD_ENV, mode)
                .output()
                .expect("native prlimit must be available; absence is not skipped success")
        };
        let healthy = run("healthy");
        assert!(healthy.status.success(), "healthy: {:?}", healthy);
        let failure = run("pressure");
        assert!(
            String::from_utf8_lossy(&failure.stderr)
                .contains("snapshot-probe: entering real decoder under pressure"),
            "pressure must reach the real decoder: {:?}",
            failure
        );
        assert_eq!(
            failure.status.signal(),
            Some(6),
            "expected allocator abort, not semantic refusal: {:?}",
            failure
        );
    }
}
