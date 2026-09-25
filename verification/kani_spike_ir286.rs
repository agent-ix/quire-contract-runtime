//! IR-286 spike: whether Kani can prove through `exact::expression`'s
//! function-application surface (`Frame::call`), not just through
//! `crate::operators`/top-level model types the way every harness in
//! `verification/kani.rs` does.
//!
//! Kept in its own `cfg(kani)` module rather than added to
//! `verification/kani.rs` so `scripts/check_kani_harnesses.py`'s declared
//! census (which reads only that file) is untouched by a spike. Neither
//! harness here is wired into `EXPECTED_KANI_HARNESSES` or the `make kani`
//! gate; run them individually with `cargo kani --harness <name>`.
//!
//! The proposition: `f(x: Int[0,9]) = x + 1`, applied through the real
//! `PackageDeclarations` -> `CheckedPackage` -> `CheckedExpression::evaluate`
//! -> `Frame::call` path (the same path `tests/exact_function_application.rs`
//! `tc_194_evaluate_charges_one_function_call_per_reached_application`
//! exercises for `ChargePoint` bookkeeping), with `x` symbolic and
//! `evaluate_integer_arithmetic` doing the arithmetic on real `exact::Integer`
//! (`num_bigint::BigInt`) values — the same operator
//! `quire-contract-codegen`'s generated oracles call and the one
//! `verification/kani.rs`'s own module comment names as rejected for
//! CBMC-model blowup even narrowed to `i8` (see that file, top).
//!
//! `EXACT_UNLIMITED` mirrors `verification/kani.rs`: every limit is
//! `u64::MAX`, so metering never refuses and no `Stop::Refused` /
//! `LimitKind` boundary is exercised here — this spike is scoped to whether
//! the call surface itself discharges, not to accounting.

use alloc::boxed::Box;
use alloc::string::ToString;
use alloc::vec::Vec;

use crate::exact::{
    evaluate_integer_arithmetic, CheckMode, CheckedPackage, CheckingLimits, FunctionDeclaration,
    Integer, IntegerArithmetic, IntegerInterval, Meter, ObjectEnvironment, Outcome,
    PackageDeclarations, Refusal, ScalarLimits, Value, ValueType,
};

const EXACT_UNLIMITED: ScalarLimits = ScalarLimits {
    integer_bits: u64::MAX,
    decimal_digits: u64::MAX,
    scale_expansion: u64::MAX,
    text_input_bytes: u64::MAX,
    text_scalars: u64::MAX,
    normalized_scalars: u64::MAX,
    unit_edges: u64::MAX,
    value_occurrences: u64::MAX,
    work_units: u64::MAX,
    result_units: u64::MAX,
};

/// `f(x: Int[0,9]) = x + 1`, as a package function whose body runs the real
/// `evaluate_integer_arithmetic(IntegerArithmetic::Add(x, 1), None, meter)`
/// on real `exact::Integer` values. `None` bound: the domain assertion below
/// is the harness's own proposition, not enforced by a runtime `Int[1,10]`
/// result bound.
fn add_one_package() -> PackageDeclarations {
    let domain = IntegerInterval::new(Integer::from(0i64), Integer::from(9i64))
        .expect("0..=9 is a nonempty interval");
    PackageDeclarations {
        types: Default::default(),
        functions: alloc::vec![FunctionDeclaration {
            name: "f".to_string(),
            parameters: alloc::vec![("x".to_string(), ValueType::Int(domain))],
            result: ValueType::Integer,
            ieee_requirements: Vec::new(),
            integer_division_consumers: Vec::new(),
            measure_discharged: true,
            body: Box::new(|frame, arguments| {
                let Value::Integer(operand) = &arguments[0] else {
                    return Outcome::Refused(Refusal::CheckedInvariant);
                };
                let one = Integer::one();
                let charged = frame.meter(|meter| {
                    evaluate_integer_arithmetic(IntegerArithmetic::Add(operand, &one), None, meter)
                });
                match charged {
                    Ok(Outcome::Completed(sum)) => Outcome::Completed(Value::Integer(sum)),
                    Ok(Outcome::Undefined(reason)) => Outcome::Undefined(reason),
                    Ok(Outcome::Refused(reason)) => Outcome::Refused(reason),
                    Ok(Outcome::Incomplete(record)) => Outcome::Incomplete(record),
                    Err(refusal) => Outcome::Refused(refusal),
                }
            }),
        }],
    }
}

/// Root: a standalone `CheckedExpression` with one `Int[0,9]` parameter whose
/// body reaches `f` only through `Frame::call` — never `CheckedPackage::call`
/// directly — matching the ticket's "through `Frame::call`" requirement.
fn call_f_through_frame(checked: &CheckedPackage) -> crate::exact::CheckedExpression {
    let domain = IntegerInterval::new(Integer::from(0i64), Integer::from(9i64))
        .expect("0..=9 is a nonempty interval");
    checked
        .check_expression(
            alloc::vec![("x".to_string(), ValueType::Int(domain))],
            ValueType::Integer,
            Box::new(|frame, arguments| frame.call("f", arguments)),
        )
        .expect("root expression checks against its own package")
}

/// The real path: `x` symbolic in `[0,9]`, applied through `Frame::call`,
/// asserting the specification's own result bound, `[1,10]`.
#[kani::proof]
fn ir286_frame_call_add_one_bounded() {
    let checked = add_one_package()
        .check(CheckMode::Linked, CheckingLimits::default())
        .expect("add_one_package checks under CheckMode::Linked");
    let expression = call_f_through_frame(&checked);
    let objects = ObjectEnvironment::default();
    let mut meter = Meter::new(EXACT_UNLIMITED);

    let x: i64 = kani::any();
    kani::assume((0..=9).contains(&x));

    let evaluation = checked
        .evaluate(
            &expression,
            alloc::vec![Value::Integer(Integer::from(x))],
            &objects,
            &mut meter,
        )
        .expect("arity and value-kind admit a symbolic Int[0,9] argument");

    let Outcome::Completed(Value::Integer(ref result)) = evaluation.outcome else {
        panic!("f(x) did not complete with an Integer");
    };
    let lower = Integer::from(1i64);
    let upper = Integer::from(10i64);
    assert!(*result >= lower && *result <= upper);
}

/// Mutated twin: same call surface, an assertion the real behaviour must
/// violate (`x=9` applies to `f(9) = 10 > 9`), kept as the sabotage/mutation
/// witness the spike asked for — this one must report a counterexample, not
/// SUCCESSFUL, or the harness above is vacuous.
#[kani::proof]
fn ir286_frame_call_add_one_bounded_mutated() {
    let checked = add_one_package()
        .check(CheckMode::Linked, CheckingLimits::default())
        .expect("add_one_package checks under CheckMode::Linked");
    let expression = call_f_through_frame(&checked);
    let objects = ObjectEnvironment::default();
    let mut meter = Meter::new(EXACT_UNLIMITED);

    let x: i64 = kani::any();
    kani::assume((0..=9).contains(&x));

    let evaluation = checked
        .evaluate(
            &expression,
            alloc::vec![Value::Integer(Integer::from(x))],
            &objects,
            &mut meter,
        )
        .expect("arity and value-kind admit a symbolic Int[0,9] argument");

    let Outcome::Completed(Value::Integer(ref result)) = evaluation.outcome else {
        panic!("f(x) did not complete with an Integer");
    };
    let mutated_upper = Integer::from(9i64);
    assert!(*result <= mutated_upper);
}

/// Diagnostic, not part of the spike's two required harnesses: checks and
/// drops `add_one_package()` with no `Frame::call`, no `evaluate`, and no
/// symbolic input at all — isolating whether `TypeEnvironment`'s two
/// `alloc::collections::BTreeMap` fields (`src/exact/composite.rs:867-868`,
/// always present on every `PackageDeclarations`/`CheckedPackage` regardless
/// of whether any composite/enum/reference type is ever declared) are
/// themselves a distinct source of CBMC blowup, separate from the
/// `num_bigint`-backed `Integer` arithmetic `verification/kani.rs`'s module
/// comment already names. Both harnesses above build one of these
/// `TypeEnvironment`s too, so this isn't a different program, only a smaller
/// one: no `x`, no arithmetic, no `Frame::call`, just construct-check-drop.
#[kani::proof]
fn ir286_package_check_and_drop_only() {
    let _checked = add_one_package()
        .check(CheckMode::Linked, CheckingLimits::default())
        .expect("add_one_package checks under CheckMode::Linked");
}
