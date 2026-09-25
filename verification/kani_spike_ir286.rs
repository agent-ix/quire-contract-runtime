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
    let body: crate::exact::Body =
        Box::new(|_frame, _arguments| Outcome::Refused(Refusal::CheckedInvariant));
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

#[allow(dead_code)]
struct D1 {
    name: alloc::string::String,
    body: Box<dyn Fn(u8) -> u8>,
}

/// Control: a named `Box<dyn Fn>` looked up by name in a heap `Vec`, called
/// from inside another `Box<dyn Fn>` of a different signature.
#[kani::proof]
fn ir286_diag_control_dyn_lookup_call() {
    let table: Vec<D1> = alloc::vec![D1 {
        name: "f".to_string(),
        body: Box::new(|x| x.wrapping_add(1)),
    }];
    let root: Box<dyn Fn(&[D1], u8) -> u8> =
        Box::new(
            |table, x| match table.iter().find(|entry| entry.name == "f") {
                Some(entry) => (entry.body)(x),
                None => 0,
            },
        );
    let x: u8 = kani::any();
    kani::assume(x < 10);
    assert!(root(&table, x) == x + 1);
}

#[allow(dead_code)]
struct D2 {
    name: alloc::string::String,
    body: Box<dyn Fn(&[u8]) -> u8>,
}

/// Control: as above, but the root and the looked-up body share one
/// signature, as `Body` and the root expression do.
#[kani::proof]
fn ir286_diag_control_dyn_same_signature() {
    let table: Vec<D2> = alloc::vec![D2 {
        name: "f".to_string(),
        body: Box::new(|args| args[0].wrapping_add(1)),
    }];
    let table_ref = &table;
    let root: Box<dyn Fn(&[u8]) -> u8 + '_> =
        Box::new(
            move |args| match table_ref.iter().find(|entry| entry.name == "f") {
                Some(entry) => (entry.body)(args),
                None => 0,
            },
        );
    let x: u8 = kani::any();
    kani::assume(x < 10);
    assert!(root(&[x]) == x + 1);
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

/// Check, then `check_expression`, then drop: no evaluation.
#[kani::proof]
fn ir286_diag_check_expression_drop() {
    let checked = add_one_package()
        .check(CheckMode::Linked, CheckingLimits::default())
        .expect("checks");
    let expression = call_f_through_frame(&checked);
    core::mem::drop(expression);
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

// ---- Spike 3: body-invocation bisection.

/// `Value::Integer(5)` on the stack, dropped (runs `Value`'s manual `Drop`).
#[kani::proof]
fn ir286_diag3_value_stack_drop() {
    core::mem::drop(Value::Integer(Integer::from(5i64)));
}

/// `vec![Value::Integer(5)]`, dropped.
#[kani::proof]
fn ir286_diag3_value_vec_drop() {
    let values: Vec<Value> = alloc::vec![Value::Integer(Integer::from(5i64))];
    core::mem::drop(values);
}

/// A `Value` cloned out of a heap slice, then both dropped.
#[kani::proof]
fn ir286_diag3_value_clone_from_slice() {
    let values: Vec<Value> = alloc::vec![Value::Integer(Integer::from(5i64))];
    let copy = values[0].clone();
    assert!(matches!(copy, Value::Integer(_)));
}

/// `Outcome<Value>` built and dropped.
#[kani::proof]
fn ir286_diag3_outcome_value_drop() {
    let outcome: Outcome<Value> = Outcome::Completed(Value::Integer(Integer::from(5i64)));
    assert!(matches!(outcome, Outcome::Completed(_)));
}

fn checked_add_one() -> CheckedPackage {
    add_one_package()
        .check(CheckMode::Linked, CheckingLimits::default())
        .expect("checks")
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

/// `evaluate`: one `Int[0,9]` argument the root ignores; returns a constant.
#[kani::proof]
fn ir286_diag3_evaluate_constant_one_arg() {
    let checked = checked_add_one();
    let domain = IntegerInterval::new(Integer::from(0i64), Integer::from(9i64)).expect("nonempty");
    let expression = checked
        .check_expression(
            alloc::vec![("x".to_string(), ValueType::Int(domain))],
            ValueType::Boolean,
            Box::new(|_frame, _arguments| Outcome::Completed(Value::Boolean(true))),
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
    assert!(matches!(
        evaluation.outcome,
        Outcome::Completed(Value::Boolean(true))
    ));
}

/// `Outcome<Value>` -> `Result<Value, Stop>` -> `Outcome<Value>`.
#[kani::proof]
fn ir286_diag3_outcome_round_trip() {
    let outcome: Outcome<Value> = Outcome::Completed(Value::Boolean(true));
    let back = Outcome::from_stop(outcome.into_stop());
    assert!(matches!(back, Outcome::Completed(Value::Boolean(true))));
}

/// A `Box<dyn Fn>` returning `Outcome<Value>`, called once.
#[kani::proof]
fn ir286_diag3_dyn_returning_outcome() {
    let root: Box<dyn Fn(&[Value]) -> Outcome<Value>> =
        Box::new(|_arguments| Outcome::Completed(Value::Boolean(true)));
    let outcome = root(&[]);
    assert!(matches!(outcome, Outcome::Completed(Value::Boolean(true))));
}

/// Round trip with a `bool` payload.
#[kani::proof]
fn ir286_diag3_outcome_round_trip_bool() {
    let outcome: Outcome<bool> = Outcome::Completed(true);
    let back = Outcome::from_stop(outcome.into_stop());
    assert!(matches!(back, Outcome::Completed(true)));
}

/// Round trip with an `Integer` payload.
#[kani::proof]
fn ir286_diag3_outcome_round_trip_integer() {
    let outcome: Outcome<Integer> = Outcome::Completed(Integer::from(5i64));
    let back = Outcome::from_stop(outcome.into_stop());
    assert!(matches!(back, Outcome::Completed(_)));
}

/// Round trip of `Value`, result forgotten (no drop).
#[kani::proof]
fn ir286_diag3_outcome_round_trip_forget() {
    let outcome: Outcome<Value> = Outcome::Completed(Value::Boolean(true));
    let back = Outcome::from_stop(outcome.into_stop());
    assert!(matches!(back, Outcome::Completed(Value::Boolean(true))));
    core::mem::forget(back);
}

/// `Value::Boolean(true)` built and forgotten.
#[kani::proof]
fn ir286_diag3_value_forget() {
    let value = Value::Boolean(true);
    assert!(matches!(value, Value::Boolean(true)));
    core::mem::forget(value);
}

/// `Value::Boolean(true)` built and dropped.
#[kani::proof]
fn ir286_diag3_value_boolean_drop() {
    core::mem::drop(Value::Boolean(true));
}

#[inline(never)]
fn pass_value(value: Value) -> Value {
    value
}

/// Two by-value passes of a `Value` through a function, then forgotten.
#[kani::proof]
fn ir286_diag3_value_pass_twice_forget() {
    let value = pass_value(pass_value(Value::Boolean(true)));
    assert!(matches!(value, Value::Boolean(true)));
    core::mem::forget(value);
}

#[inline(never)]
fn wrap_ok(value: Value) -> Result<Value, u8> {
    Ok(value)
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

#[allow(dead_code)]
enum Lv1 {
    Boolean(bool),
    Integer(Integer),
    Rational(crate::exact::Rational),
    Decimal(crate::exact::Decimal),
    Float(crate::exact::IeeeValue),
    Quantity(crate::exact::Quantity),
    Text(crate::exact::Text),
    Enum(crate::exact::EnumValue),
    Reference(crate::exact::ObjectReference),
}

#[allow(dead_code)]
enum Lv2 {
    Boolean(bool),
    Integer(Integer),
    Rational(Box<crate::exact::Rational>),
    Decimal(Box<crate::exact::Decimal>),
    Float(crate::exact::IeeeValue),
    Quantity(Box<crate::exact::Quantity>),
    Text(Box<crate::exact::Text>),
    Enum(Box<crate::exact::EnumValue>),
    Reference(Box<crate::exact::ObjectReference>),
}

#[inline(never)]
fn wrap_lv1(value: Lv1) -> Result<Lv1, u8> {
    Ok(value)
}

#[inline(never)]
fn wrap_lv2(value: Lv2) -> Result<Lv2, u8> {
    Ok(value)
}

/// Inline payloads, as `Value` has them.
#[kani::proof]
fn ir286_diag3_lv1_inline_wrap() {
    let value = match wrap_lv1(Lv1::Boolean(true)) {
        Ok(value) => value,
        Err(_) => Lv1::Boolean(false),
    };
    assert!(matches!(value, Lv1::Boolean(true)));
    core::mem::forget(value);
}

/// Large payloads boxed.
#[kani::proof]
fn ir286_diag3_lv2_boxed_wrap() {
    let value = match wrap_lv2(Lv2::Boolean(true)) {
        Ok(value) => value,
        Err(_) => Lv2::Boolean(false),
    };
    assert!(matches!(value, Lv2::Boolean(true)));
    core::mem::forget(value);
}

#[allow(dead_code)]
enum OnlyRational {
    Boolean(bool),
    Integer(Integer),
    VRational(crate::exact::Rational),
    VDecimal(Box<crate::exact::Decimal>),
    VQuantity(Box<crate::exact::Quantity>),
    VText(Box<crate::exact::Text>),
    VEnumValue(Box<crate::exact::EnumValue>),
    VObjectReference(Box<crate::exact::ObjectReference>),
}

#[inline(never)]
fn wrap_only_rational(value: OnlyRational) -> Result<OnlyRational, u8> {
    Ok(value)
}

#[kani::proof]
fn ir286_diag3_only_rational_inline_wrap() {
    let value = match wrap_only_rational(OnlyRational::Boolean(true)) {
        Ok(value) => value,
        Err(_) => OnlyRational::Boolean(false),
    };
    assert!(matches!(value, OnlyRational::Boolean(true)));
    core::mem::forget(value);
}

#[allow(dead_code)]
enum OnlyDecimal {
    Boolean(bool),
    Integer(Integer),
    VRational(Box<crate::exact::Rational>),
    VDecimal(crate::exact::Decimal),
    VQuantity(Box<crate::exact::Quantity>),
    VText(Box<crate::exact::Text>),
    VEnumValue(Box<crate::exact::EnumValue>),
    VObjectReference(Box<crate::exact::ObjectReference>),
}

#[inline(never)]
fn wrap_only_decimal(value: OnlyDecimal) -> Result<OnlyDecimal, u8> {
    Ok(value)
}

#[kani::proof]
fn ir286_diag3_only_decimal_inline_wrap() {
    let value = match wrap_only_decimal(OnlyDecimal::Boolean(true)) {
        Ok(value) => value,
        Err(_) => OnlyDecimal::Boolean(false),
    };
    assert!(matches!(value, OnlyDecimal::Boolean(true)));
    core::mem::forget(value);
}

#[allow(dead_code)]
enum OnlyQuantity {
    Boolean(bool),
    Integer(Integer),
    VRational(Box<crate::exact::Rational>),
    VDecimal(Box<crate::exact::Decimal>),
    VQuantity(crate::exact::Quantity),
    VText(Box<crate::exact::Text>),
    VEnumValue(Box<crate::exact::EnumValue>),
    VObjectReference(Box<crate::exact::ObjectReference>),
}

#[inline(never)]
fn wrap_only_quantity(value: OnlyQuantity) -> Result<OnlyQuantity, u8> {
    Ok(value)
}

#[kani::proof]
fn ir286_diag3_only_quantity_inline_wrap() {
    let value = match wrap_only_quantity(OnlyQuantity::Boolean(true)) {
        Ok(value) => value,
        Err(_) => OnlyQuantity::Boolean(false),
    };
    assert!(matches!(value, OnlyQuantity::Boolean(true)));
    core::mem::forget(value);
}

#[allow(dead_code)]
enum OnlyText {
    Boolean(bool),
    Integer(Integer),
    VRational(Box<crate::exact::Rational>),
    VDecimal(Box<crate::exact::Decimal>),
    VQuantity(Box<crate::exact::Quantity>),
    VText(crate::exact::Text),
    VEnumValue(Box<crate::exact::EnumValue>),
    VObjectReference(Box<crate::exact::ObjectReference>),
}

#[inline(never)]
fn wrap_only_text(value: OnlyText) -> Result<OnlyText, u8> {
    Ok(value)
}

#[kani::proof]
fn ir286_diag3_only_text_inline_wrap() {
    let value = match wrap_only_text(OnlyText::Boolean(true)) {
        Ok(value) => value,
        Err(_) => OnlyText::Boolean(false),
    };
    assert!(matches!(value, OnlyText::Boolean(true)));
    core::mem::forget(value);
}

#[allow(dead_code)]
enum OnlyEnumValue {
    Boolean(bool),
    Integer(Integer),
    VRational(Box<crate::exact::Rational>),
    VDecimal(Box<crate::exact::Decimal>),
    VQuantity(Box<crate::exact::Quantity>),
    VText(Box<crate::exact::Text>),
    VEnumValue(crate::exact::EnumValue),
    VObjectReference(Box<crate::exact::ObjectReference>),
}

#[inline(never)]
fn wrap_only_enumvalue(value: OnlyEnumValue) -> Result<OnlyEnumValue, u8> {
    Ok(value)
}

#[kani::proof]
fn ir286_diag3_only_enumvalue_inline_wrap() {
    let value = match wrap_only_enumvalue(OnlyEnumValue::Boolean(true)) {
        Ok(value) => value,
        Err(_) => OnlyEnumValue::Boolean(false),
    };
    assert!(matches!(value, OnlyEnumValue::Boolean(true)));
    core::mem::forget(value);
}

#[allow(dead_code)]
enum OnlyObjectReference {
    Boolean(bool),
    Integer(Integer),
    VRational(Box<crate::exact::Rational>),
    VDecimal(Box<crate::exact::Decimal>),
    VQuantity(Box<crate::exact::Quantity>),
    VText(Box<crate::exact::Text>),
    VEnumValue(Box<crate::exact::EnumValue>),
    VObjectReference(crate::exact::ObjectReference),
}

#[inline(never)]
fn wrap_only_objectreference(value: OnlyObjectReference) -> Result<OnlyObjectReference, u8> {
    Ok(value)
}

#[kani::proof]
fn ir286_diag3_only_objectreference_inline_wrap() {
    let value = match wrap_only_objectreference(OnlyObjectReference::Boolean(true)) {
        Ok(value) => value,
        Err(_) => OnlyObjectReference::Boolean(false),
    };
    assert!(matches!(value, OnlyObjectReference::Boolean(true)));
    core::mem::forget(value);
}

#[allow(dead_code)]
enum Lv3 {
    Boolean(bool),
    Integer(Integer),
    Rational(crate::exact::Rational),
    Decimal(crate::exact::Decimal),
    Float(crate::exact::IeeeValue),
    Quantity(Box<crate::exact::Quantity>),
    Text(Box<crate::exact::Text>),
    Enum(Box<crate::exact::EnumValue>),
    Reference(Box<crate::exact::ObjectReference>),
}

#[inline(never)]
fn wrap_lv3(value: Lv3) -> Result<Lv3, u8> {
    Ok(value)
}

/// `Rational` and `Decimal` inline, the other four boxed.
#[kani::proof]
fn ir286_diag3_lv3_numeric_inline_wrap() {
    let value = match wrap_lv3(Lv3::Boolean(true)) {
        Ok(value) => value,
        Err(_) => Lv3::Boolean(false),
    };
    assert!(matches!(value, Lv3::Boolean(true)));
    core::mem::forget(value);
}

/// An `Evaluation` built directly, matched, dropped.
#[kani::proof]
fn ir286_diag3_evaluation_drop() {
    let evaluation = crate::exact::Evaluation {
        outcome: Outcome::Completed(Value::Boolean(true)),
        location: None,
        losses: Vec::new(),
    };
    assert!(matches!(
        evaluation.outcome,
        Outcome::Completed(Value::Boolean(true))
    ));
}

/// Constant-root `evaluate`, result forgotten rather than dropped.
#[kani::proof]
fn ir286_diag3_evaluate_constant_forget() {
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
    core::mem::forget(evaluation);
}

#[inline(never)]
fn evaluation_ok() -> Result<crate::exact::Evaluation, crate::exact::InputRefusal> {
    Ok(crate::exact::Evaluation {
        outcome: Outcome::Completed(Value::Boolean(true)),
        location: None,
        losses: Vec::new(),
    })
}

/// `Result<Evaluation, InputRefusal>` unwrapped with `expect`, then dropped.
#[kani::proof]
fn ir286_diag3_evaluation_result_expect_drop() {
    let evaluation = evaluation_ok().expect("ok");
    assert!(matches!(
        evaluation.outcome,
        Outcome::Completed(Value::Boolean(true))
    ));
}
