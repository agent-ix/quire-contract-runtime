//! The runtime function boundary consumes the QSV checker and exact values directly.
#![cfg(feature = "exact")]

use ix_trace_rs::trace;
use quire_contract_runtime::exact::{
    BodyStep, CheckMode, CheckedPackage, CheckingLimits, Frame, FunctionDeclaration,
    PackageDeclarations,
};
use quire_exact::{Meter, Outcome, ScalarLimits, Value, ValueType};
use quire_semantic_value::declaration::TypeEnvironment;
use quire_semantic_value::object_closure::ObjectClosure;

const LIMITS: ScalarLimits = ScalarLimits {
    integer_bits: 100,
    decimal_digits: 100,
    scale_expansion: 100,
    text_input_bytes: 100,
    text_scalars: 100,
    normalized_scalars: 100,
    unit_edges: 100,
    value_occurrences: 100,
    work_units: 100,
    result_units: 100,
};

fn identity_package() -> CheckedPackage {
    PackageDeclarations {
        types: TypeEnvironment::default(),
        functions: vec![FunctionDeclaration {
            name: "identity".into(),
            parameters: vec![("value".into(), ValueType::Boolean)],
            result: ValueType::Boolean,
            ieee_requirements: Vec::new(),
            integer_division_consumers: Vec::new(),
            measure_discharged: true,
            body: Box::new(|values| {
                Box::new(move |_: &Frame<'_, '_>, _: Option<Value>| {
                    BodyStep::Return(Outcome::Completed(values[0].clone()))
                })
            }),
        }],
    }
    .check(CheckMode::Linked, CheckingLimits::default())
    .expect("a linked identity function admits")
}

#[trace("TC-194", "FR-273-AC-2")]
#[test]
fn function_application_accepts_exact_value_and_qsv_environment() {
    let package = identity_package();
    let mut meter = Meter::new(LIMITS);
    let result = package
        .call(
            "identity",
            vec![Value::Boolean(true)],
            &ObjectClosure::default(),
            &mut meter,
        )
        .expect("admitted call");
    assert!(matches!(
        result.outcome,
        Outcome::Completed(Value::Boolean(true))
    ));
}
