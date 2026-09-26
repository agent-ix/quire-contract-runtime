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

// ---- Spike 2/3 diagnostics (IR-286). Each isolates one step of the call
// path; the bisection's other harnesses (minimal enums, layout controls) are
// in this file's git history.

#[inline(never)]
fn wrap_ok(value: Value) -> Result<Value, u8> {
    Ok(value)
}

fn checked_add_one() -> CheckedPackage {
    add_one_package()
        .check(CheckMode::Linked, CheckingLimits::default())
        .expect("checks")
}

/// `add_one_package()` built and dropped, never checked.
#[kani::proof]
fn ir286_diag_package_build_drop() {
    let package = add_one_package();
    core::mem::drop(package);
}

/// `ValueType::Integer` in a `Box`, dropped.
#[kani::proof]
fn ir286_diag_value_type_unit_box_drop() {
    let boxed: Box<ValueType> = Box::new(ValueType::Integer);
    core::mem::drop(boxed);
}

/// Check, then `check_expression`, then drop: no evaluation.
#[kani::proof]
fn ir286_diag_check_expression_drop() {
    let checked = add_one_package()
        .check(CheckMode::Linked, CheckingLimits::default())
        .expect("checks");
    let expression = call_f_through_frame(&checked);
    core::mem::drop(expression);
}

/// The real add-one path with a concrete `x = 5`: isolates whether the
/// symbolic argument's path split is what defeats dispatch resolution.
#[kani::proof]
fn ir286_diag_add_one_concrete() {
    let checked = add_one_package()
        .check(CheckMode::Linked, CheckingLimits::default())
        .expect("add_one_package checks under CheckMode::Linked");
    let expression = call_f_through_frame(&checked);
    let objects = ObjectEnvironment::default();
    let mut meter = Meter::new(EXACT_UNLIMITED);
    let evaluation = checked
        .evaluate(
            &expression,
            alloc::vec![Value::Integer(Integer::from(5i64))],
            &objects,
            &mut meter,
        )
        .expect("admitted");
    let Outcome::Completed(Value::Integer(ref result)) = evaluation.outcome else {
        panic!("f(5) did not complete with an Integer");
    };
    assert!(*result == Integer::from(6i64));
}

/// `CheckedPackage::call("f", [5])` directly: one dyn dispatch, no root.
#[kani::proof]
fn ir286_diag_direct_call_concrete() {
    let checked = add_one_package()
        .check(CheckMode::Linked, CheckingLimits::default())
        .expect("checks");
    let objects = ObjectEnvironment::default();
    let mut meter = Meter::new(EXACT_UNLIMITED);
    let evaluation = checked
        .call(
            "f",
            alloc::vec![Value::Integer(Integer::from(5i64))],
            &objects,
            &mut meter,
        )
        .expect("admitted");
    assert!(matches!(evaluation.outcome, Outcome::Completed(_)));
}

/// `evaluate` with a root that returns its argument, never calling `f`.
#[kani::proof]
fn ir286_diag_evaluate_identity_root() {
    let checked = add_one_package()
        .check(CheckMode::Linked, CheckingLimits::default())
        .expect("checks");
    let domain = IntegerInterval::new(Integer::from(0i64), Integer::from(9i64)).expect("nonempty");
    let expression = checked
        .check_expression(
            alloc::vec![("x".to_string(), ValueType::Int(domain))],
            ValueType::Integer,
            Box::new(|_frame, arguments| Outcome::Completed(arguments[0].clone())),
        )
        .expect("checks");
    let objects = ObjectEnvironment::default();
    let mut meter = Meter::new(EXACT_UNLIMITED);
    let evaluation = checked
        .evaluate(
            &expression,
            alloc::vec![Value::Integer(Integer::from(5i64))],
            &objects,
            &mut meter,
        )
        .expect("admitted");
    assert!(matches!(evaluation.outcome, Outcome::Completed(_)));
}

/// `evaluate`: no parameters, no arguments, root returns a constant.
#[kani::proof]
fn ir286_diag3_evaluate_constant_no_args() {
    let checked = checked_add_one();
    let expression = checked
        .check_expression(
            Vec::new(),
            ValueType::Boolean,
            Box::new(|_frame, _arguments| Outcome::Completed(Value::Boolean(true))),
        )
        .expect("checks");
    let objects = ObjectEnvironment::default();
    let mut meter = Meter::new(EXACT_UNLIMITED);
    let evaluation = checked
        .evaluate(&expression, Vec::new(), &objects, &mut meter)
        .expect("admitted");
    assert!(matches!(
        evaluation.outcome,
        Outcome::Completed(Value::Boolean(true))
    ));
}

/// `Value` into `Result<Value, u8>` and back out, then forgotten.
#[kani::proof]
fn ir286_diag3_value_result_wrap_forget() {
    let value = match wrap_ok(Value::Boolean(true)) {
        Ok(value) => value,
        Err(_) => Value::Boolean(false),
    };
    assert!(matches!(value, Value::Boolean(true)));
    core::mem::forget(value);
}

/// `plan_call` (argument validation only) against a checked package.
#[kani::proof]
fn ir286_diag3_plan_call_checked() {
    let checked = checked_add_one();
    let objects = ObjectEnvironment::default();
    let arguments = alloc::vec![Value::Integer(Integer::from(5i64))];
    assert!(crate::exact::plan_call(&checked, "f", &arguments, &objects).is_ok());
}

/// `function()`'s lookup alone, reached through the public
/// `ieee_requirements` accessor.
#[kani::proof]
fn ir286_diag4_function_lookup() {
    let checked = checked_add_one();
    assert!(checked.ieee_requirements("f").is_some());
}

/// Control: the same read from the *unchecked* package.
#[kani::proof]
fn ir286_diag4_unchecked_declaration_admits() {
    let package = add_one_package();
    let value = Value::Integer(Integer::from(5i64));
    assert!(package.functions[0].parameters[0].1.admits(&value));
}

/// `Int[0,9].admits(5)` with everything on the stack.
#[kani::proof]
fn ir286_diag4_stack_admits() {
    let domain = IntegerInterval::new(Integer::from(0i64), Integer::from(9i64)).expect("nonempty");
    let declared = ValueType::Int(domain);
    let value = Value::Integer(Integer::from(5i64));
    assert!(declared.admits(&value));
}

/// `IntegerInterval::contains` with everything on the stack.
#[kani::proof]
fn ir286_diag4_stack_contains() {
    let domain = IntegerInterval::new(Integer::from(0i64), Integer::from(9i64)).expect("nonempty");
    assert!(domain.contains(&Integer::from(5i64)));
}

/// Stack interval; the value read out of `Value::Integer`.
#[kani::proof]
fn ir286_diag4_value_side_contains() {
    let domain = IntegerInterval::new(Integer::from(0i64), Integer::from(9i64)).expect("nonempty");
    let value = Value::Integer(Integer::from(5i64));
    let Value::Integer(integer) = &value else {
        panic!("not an integer");
    };
    assert!(domain.contains(integer));
}

/// Interval read out of `ValueType::Int`; stack value.
#[kani::proof]
fn ir286_diag4_type_side_contains() {
    let domain = IntegerInterval::new(Integer::from(0i64), Integer::from(9i64)).expect("nonempty");
    let declared = ValueType::Int(domain);
    let ValueType::Int(interval) = &declared else {
        panic!("not Int");
    };
    assert!(interval.contains(&Integer::from(5i64)));
}

#[allow(dead_code)]
#[repr(u8)]
enum W1 {
    A(bool),
    B(i64),
}

/// One enum level around a bare `i64`.
#[kani::proof]
fn ir286_diag4_w1_i64_in_enum() {
    let w = W1::B(5);
    let W1::B(x) = &w else { panic!("not B") };
    assert!(*x >= 0 && *x <= 9);
}

#[allow(dead_code)]
#[repr(u8)]
enum W2 {
    A(bool),
    B(Integer),
}

/// One enum level around an `Integer` (itself an enum).
#[kani::proof]
fn ir286_diag4_w2_integer_in_enum() {
    let w = W2::B(Integer::from(5i64));
    let W2::B(x) = &w else { panic!("not B") };
    assert!(*x >= Integer::zero() && *x <= Integer::from(9i64));
}

#[allow(dead_code)]
#[derive(Clone)]
struct S3 {
    small: i64,
    big: Option<Box<num_bigint::BigInt>>,
}

#[allow(dead_code)]
#[repr(u8)]
enum W3 {
    A(bool),
    B(S3),
}

/// One enum level around a struct `{ i64, Option<Box<BigInt>> }`.
#[kani::proof]
fn ir286_diag4_w3_struct_integer_in_enum() {
    let w = W3::B(S3 {
        small: 5,
        big: None,
    });
    let W3::B(x) = &w else { panic!("not B") };
    assert!(x.big.is_none() && x.small >= 0 && x.small <= 9);
}

// ---- Spike 5 diagnostics: struct `Integer` read back from the heap.

/// `Integer` inside a `Value` inside a heap `Vec<Value>`, compared.
#[kani::proof]
fn ir286_diag5_vec_value_integer_cmp() {
    let values = alloc::vec![Value::Integer(Integer::from(5i64))];
    let Value::Integer(integer) = &values[0] else {
        panic!("not an integer");
    };
    assert!(Integer::zero() <= *integer);
}

/// `Integer` in a heap `Vec<Integer>`, compared.
#[kani::proof]
fn ir286_diag5_vec_integer_cmp() {
    let values = alloc::vec![Integer::from(5i64)];
    assert!(Integer::zero() <= values[0]);
}

/// `Integer` in a `Box<Value>`, compared.
#[kani::proof]
fn ir286_diag5_box_value_integer_cmp() {
    let value = Box::new(Value::Integer(Integer::from(5i64)));
    let Value::Integer(integer) = &*value else {
        panic!("not an integer");
    };
    assert!(Integer::zero() <= *integer);
}

#[allow(dead_code)]
#[derive(Clone)]
struct S4 {
    small: i64,
    big: num_bigint::BigInt,
}

#[allow(dead_code)]
#[derive(Clone)]
#[repr(u8)]
enum W4 {
    A(bool),
    B(S4),
    C(Box<i128>),
}

/// Model: `{ i64, BigInt }` in a tagged enum in a `Box`, read, cloned, dropped.
#[kani::proof]
fn ir286_diag5_w4_box_struct_bigint() {
    let w = Box::new(W4::B(S4 {
        small: 5,
        big: num_bigint::BigInt::ZERO,
    }));
    let copy = (*w).clone();
    let W4::B(x) = &copy else { panic!("not B") };
    assert!(x.big.sign() == num_bigint::Sign::NoSign && x.small >= 0 && x.small <= 9);
}

#[allow(dead_code)]
#[derive(Clone)]
#[repr(u8)]
enum W5 {
    A(bool),
    B(S3),
    C(Box<i128>),
}

/// Control: `{ i64, Option<Box<BigInt>> }` in the same shape (expected unfolded).
#[kani::proof]
fn ir286_diag5_w5_box_struct_option_box() {
    let w = Box::new(W5::B(S3 {
        small: 5,
        big: None,
    }));
    let W5::B(x) = &*w else { panic!("not B") };
    assert!(x.big.is_none() && x.small >= 0 && x.small <= 9);
}

#[allow(dead_code)]
#[derive(Clone)]
struct S6 {
    small: i64,
    big: Vec<num_bigint::BigInt>,
}

#[allow(dead_code)]
#[derive(Clone)]
#[repr(u8)]
enum W6 {
    A(bool),
    B(S6),
    C(Box<i128>),
}

/// Model: `{ i64, Vec<BigInt> }` (empty when small) in a tagged enum in a
/// `Box`: read, cloned, dropped.
#[kani::proof]
fn ir286_diag5_w6_box_struct_vec() {
    let w = Box::new(W6::B(S6 {
        small: 5,
        big: Vec::new(),
    }));
    let copy = (*w).clone();
    let W6::B(x) = &copy else { panic!("not B") };
    assert!(x.big.is_empty() && x.small >= 0 && x.small <= 9);
}

/// Matrix: `Box<W1>`, read the `i64`.
#[kani::proof]
fn ir286_diag5_t1_box_w1() {
    let w = Box::new(W1::B(5));
    let W1::B(x) = &*w else { panic!("not B") };
    assert!(*x >= 0 && *x <= 9);
}

/// Matrix: `Box<W5>`, read `small` only.
#[kani::proof]
fn ir286_diag5_t2_box_w5_small() {
    let w = Box::new(W5::B(S3 {
        small: 5,
        big: None,
    }));
    let W5::B(x) = &*w else { panic!("not B") };
    assert!(x.small >= 0 && x.small <= 9);
}

/// Matrix: `Box<W6>`, read `is_empty` only.
#[kani::proof]
fn ir286_diag5_t3_box_w6_is_empty() {
    let w = Box::new(W6::B(S6 {
        small: 5,
        big: Vec::new(),
    }));
    let W6::B(x) = &*w else { panic!("not B") };
    assert!(x.big.is_empty());
}

/// Matrix: `Box<S6>` (no enum), cloned.
#[kani::proof]
fn ir286_diag5_t4_box_s6_clone() {
    let w = Box::new(S6 {
        small: 5,
        big: Vec::new(),
    });
    let copy = (*w).clone();
    assert!(copy.big.is_empty());
}

/// Matrix: `Box<S3>` (no enum), `is_none`.
#[kani::proof]
fn ir286_diag5_t5_box_s3_is_none() {
    let w = Box::new(S3 {
        small: 5,
        big: None,
    });
    assert!(w.big.is_none());
}

#[allow(dead_code)]
#[repr(u64)]
enum W9 {
    A(bool),
    B(i64),
}

/// Matrix: `Box<W9>` (u64 tag: payload at the union's offset 0, and `B`
/// covers the whole union), read the `i64`.
#[kani::proof]
fn ir286_diag5_t6_box_w9_u64_tag() {
    let w = Box::new(W9::B(5));
    let W9::B(x) = &*w else { panic!("not B") };
    assert!(*x >= 0 && *x <= 9);
}

#[allow(dead_code)]
#[repr(u64)]
enum W10 {
    A(bool),
    B(i64),
    C(i64, i64),
}

/// Matrix: `Box<W10>` (u64 tag; `B` does not cover the union), read the `i64`.
#[kani::proof]
fn ir286_diag5_t7_box_w10_partial_cover() {
    let w = Box::new(W10::B(5));
    let W10::B(x) = &*w else { panic!("not B") };
    assert!(*x >= 0 && *x <= 9);
}

#[allow(dead_code)]
#[repr(u64)]
enum W11 {
    A(bool),
    B(Integer),
    C(Box<i128>),
    D(u8, u64),
}

/// Matrix: `Integer` covering a `u64`-tagged union, in a heap `Vec`, compared.
#[kani::proof]
fn ir286_diag5_t8_vec_w11_integer() {
    let values = alloc::vec![W11::B(Integer::from(5i64))];
    let W11::B(x) = &values[0] else {
        panic!("not B")
    };
    assert!(Integer::zero() <= *x);
}

#[allow(dead_code)]
#[repr(u64)]
enum W14 {
    A(bool),
    B(W11),
    C(i64, i64, i64, i64, i64),
}

/// Matrix: a covering `u64`-tagged enum (`W11`) inside a `u64`-tagged enum
/// it does not cover, on the stack; read the inner tag and payload.
#[kani::proof]
fn ir286_diag5_t9_stack_nested_partial() {
    let w = W14::B(W11::B(Integer::from(5i64)));
    let W14::B(W11::B(x)) = &w else {
        panic!("not B")
    };
    assert!(Integer::zero() <= *x);
}

/// Matrix: `Outcome<Value>` holding `Value::Integer`, on the stack.
#[kani::proof]
fn ir286_diag5_t10_stack_outcome_value() {
    let outcome = Outcome::Completed(Value::Integer(Integer::from(5i64)));
    let Outcome::Completed(Value::Integer(x)) = &outcome else {
        panic!("not an integer")
    };
    assert!(Integer::zero() <= *x);
}

#[allow(dead_code)]
#[repr(u64)]
enum W15 {
    A(bool),
    B(W11),
}

/// Matrix: a covering `u64`-tagged enum (`W11`) inside a `u64`-tagged enum it
/// also covers, on the stack.
#[kani::proof]
fn ir286_diag5_t11_stack_nested_covering() {
    let w = W15::B(W11::B(Integer::from(5i64)));
    let W15::B(W11::B(x)) = &w else {
        panic!("not B")
    };
    assert!(Integer::zero() <= *x);
}

/// Diagnostic: `call("f", [5])` with the `Evaluation` forgotten, never read
/// or dropped: isolates everything before the `Outcome<Value>` read.
#[kani::proof]
fn ir286_diag5_direct_call_forget() {
    let checked = checked_add_one();
    let objects = ObjectEnvironment::default();
    let mut meter = Meter::new(EXACT_UNLIMITED);
    let evaluation = checked
        .call(
            "f",
            alloc::vec![Value::Integer(Integer::from(5i64))],
            &objects,
            &mut meter,
        )
        .expect("admitted");
    core::mem::forget(evaluation);
}

/// Diagnostic: the symbolic `evaluate` through `Frame::call`, `Evaluation`
/// forgotten: whether the symbolic path converges up to the result read.
#[kani::proof]
fn ir286_diag5_symbolic_evaluate_forget() {
    let checked = checked_add_one();
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
        .expect("admitted");
    core::mem::forget(evaluation);
}

/// Diagnostic: symbolic `x` in a heap `Vec<Value>`, read back, then
/// `evaluate_integer_arithmetic(x + 1)` on it.
#[kani::proof]
fn ir286_diag5_symbolic_heap_add() {
    let x: i64 = kani::any();
    kani::assume((0..=9).contains(&x));
    let values = alloc::vec![Value::Integer(Integer::from(x))];
    let Value::Integer(operand) = &values[0] else {
        panic!("not an integer");
    };
    let mut meter = Meter::new(EXACT_UNLIMITED);
    let one = Integer::one();
    let outcome =
        evaluate_integer_arithmetic(IntegerArithmetic::Add(operand, &one), None, &mut meter);
    let Outcome::Completed(sum) = outcome else {
        panic!("not completed");
    };
    assert!(sum >= Integer::one() && sum <= Integer::from(10i64));
}

/// Diagnostic: the same with `x` on the stack.
#[kani::proof]
fn ir286_diag5_symbolic_stack_add() {
    let x: i64 = kani::any();
    kani::assume((0..=9).contains(&x));
    let operand = Integer::from(x);
    let mut meter = Meter::new(EXACT_UNLIMITED);
    let one = Integer::one();
    let outcome =
        evaluate_integer_arithmetic(IntegerArithmetic::Add(&operand, &one), None, &mut meter);
    let Outcome::Completed(sum) = outcome else {
        panic!("not completed");
    };
    assert!(sum >= Integer::one() && sum <= Integer::from(10i64));
}
