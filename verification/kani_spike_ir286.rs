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

// ---- Spike 2 bisection diagnostics (IR-286 follow-up). Each isolates one
// ingredient of `ir286_package_check_and_drop_only`, so the report can
// attribute cost to a representation rather than to the whole path.

/// Only an empty `TypeEnvironment`, built and dropped.
#[kani::proof]
fn ir286_diag_type_environment_default_drop() {
    let environment = crate::exact::TypeEnvironment::default();
    core::mem::drop(environment);
}

/// Only one concrete `Integer` (a `BigInt`), built and dropped.
#[kani::proof]
fn ir286_diag_integer_from_drop() {
    let value = Integer::from(9i64);
    core::mem::drop(value);
}

/// Only `IntegerInterval::new(0, 9)`: two `Integer`s and one comparison.
#[kani::proof]
fn ir286_diag_interval_new() {
    let interval = IntegerInterval::new(Integer::from(0i64), Integer::from(9i64));
    assert!(interval.is_ok());
}

/// `add_one_package()` built and dropped, never checked.
#[kani::proof]
fn ir286_diag_package_build_drop() {
    let package = add_one_package();
    core::mem::drop(package);
}

/// Only the parameter list: `vec![("x", Int[0,9])]`, built and dropped.
#[kani::proof]
fn ir286_diag_parameters_drop() {
    let domain = IntegerInterval::new(Integer::from(0i64), Integer::from(9i64))
        .expect("0..=9 is a nonempty interval");
    let parameters: Vec<(alloc::string::String, ValueType)> =
        alloc::vec![("x".to_string(), ValueType::Int(domain))];
    core::mem::drop(parameters);
}

/// Only a `Body` (`Box<dyn Fn>`), built and dropped.
#[kani::proof]
fn ir286_diag_body_drop() {
    let body: crate::exact::Body = Box::new(|_frame, _arguments| {
        Outcome::Refused(Refusal::CheckedInvariant)
    });
    core::mem::drop(body);
}

/// `ValueType::Int(0..=9)` on the stack, dropped.
#[kani::proof]
fn ir286_diag_value_type_int_stack_drop() {
    let domain = IntegerInterval::new(Integer::from(0i64), Integer::from(9i64))
        .expect("0..=9 is a nonempty interval");
    core::mem::drop(ValueType::Int(domain));
}

/// `vec![ValueType::Integer]`, dropped: a heap slot, no BigInt payload.
#[kani::proof]
fn ir286_diag_value_type_unit_vec_drop() {
    let types: Vec<ValueType> = alloc::vec![ValueType::Integer];
    core::mem::drop(types);
}

/// `vec![ValueType::Int(0..=9)]`, dropped: a heap slot with a BigInt payload.
#[kani::proof]
fn ir286_diag_value_type_int_vec_drop() {
    let domain = IntegerInterval::new(Integer::from(0i64), Integer::from(9i64))
        .expect("0..=9 is a nonempty interval");
    let types: Vec<ValueType> = alloc::vec![ValueType::Int(domain)];
    core::mem::drop(types);
}

/// Control: a local two-variant recursive enum in a `Vec`, dropped.
#[allow(dead_code)]
enum Recursive {
    Leaf,
    Node(Box<Recursive>),
}

#[kani::proof]
fn ir286_diag_control_recursive_vec_drop() {
    let values: Vec<Recursive> = alloc::vec![Recursive::Leaf];
    core::mem::drop(values);
}

/// `ValueType::Integer` pushed into a `Vec::with_capacity(1)`, dropped.
#[kani::proof]
fn ir286_diag_value_type_unit_push_drop() {
    let mut types: Vec<ValueType> = Vec::with_capacity(1);
    types.push(ValueType::Integer);
    core::mem::drop(types);
}

/// `ValueType::Integer` in a `Box`, dropped.
#[kani::proof]
fn ir286_diag_value_type_unit_box_drop() {
    let boxed: Box<ValueType> = Box::new(ValueType::Integer);
    core::mem::drop(boxed);
}

#[allow(dead_code)]
enum M1 {
    Leaf,
    Coll(Box<C1>),
}
#[allow(dead_code)]
struct C1 {
    element: M1,
    bound: u64,
}
#[kani::proof]
fn ir286_diag_m1_struct_recursion_box_drop() {
    core::mem::drop(Box::new(M1::Leaf));
}

#[allow(dead_code)]
enum M2 {
    Leaf,
    Big(Integer),
    Coll(Box<C2>),
}
#[allow(dead_code)]
struct C2 {
    element: M2,
    bound: u64,
}
#[kani::proof]
fn ir286_diag_m2_bigint_variant_box_drop() {
    core::mem::drop(Box::new(M2::Leaf));
}

#[allow(dead_code)]
enum M3 {
    Leaf,
    Big(Integer),
    Opt(Box<M3>),
    Coll(Box<C3>),
}
#[allow(dead_code)]
struct C3 {
    element: M3,
    bound: u64,
}
#[kani::proof]
fn ir286_diag_m3_two_recursions_box_drop() {
    core::mem::drop(Box::new(M3::Leaf));
}

#[allow(dead_code)]
#[repr(u8)]
enum M4 {
    Leaf,
    Big(Integer),
    Coll(Box<C4>),
}
#[allow(dead_code)]
struct C4 {
    element: M4,
    bound: u64,
}
#[kani::proof]
fn ir286_diag_m4_repr_u8_box_drop() {
    core::mem::drop(Box::new(M4::Leaf));
}

#[allow(dead_code)]
enum M5 {
    Leaf,
    Big(Vec<u64>),
    Coll(Box<C5>),
}
#[allow(dead_code)]
struct C5 {
    element: M5,
    bound: u64,
}
#[kani::proof]
fn ir286_diag_m5_vec_variant_box_drop() {
    core::mem::drop(Box::new(M5::Leaf));
}

#[allow(dead_code)]
enum M6 {
    Leaf,
    Big(Integer),
    Coll(Box<C6>),
}
#[allow(dead_code)]
struct C6 {
    element: M6,
    bound: u64,
}
/// Same shape as M2, but the value lives on the stack.
#[kani::proof]
fn ir286_diag_m6_bigint_variant_stack_drop() {
    core::mem::drop(M6::Leaf);
}

#[allow(dead_code)]
#[repr(u8)]
enum M7 {
    Leaf,
    Big([u64; 10]),
    Coll(Box<C7>),
}
#[allow(dead_code)]
struct C7 {
    element: M7,
    bound: u64,
}
/// 88-byte repr(u8) recursive enum on the heap.
#[kani::proof]
fn ir286_diag_m7_large_box_drop() {
    core::mem::drop(Box::new(M7::Leaf));
}

#[allow(dead_code)]
#[repr(u8)]
enum M8 {
    Leaf,
    Big([u64; 6]),
    Coll(Box<C8>),
}
#[allow(dead_code)]
struct C8 {
    element: M8,
    bound: u64,
}
/// 56-byte repr(u8) recursive enum on the heap.
#[kani::proof]
fn ir286_diag_m8_small_box_drop() {
    core::mem::drop(Box::new(M8::Leaf));
}

/// Prints nothing; only here so `size_of::<ValueType>()` is checkable.
#[kani::proof]
fn ir286_diag_value_type_size() {
    assert!(core::mem::size_of::<ValueType>() <= 64);
}
