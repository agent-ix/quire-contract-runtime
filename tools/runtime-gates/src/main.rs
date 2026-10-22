//! CLI for the runtime's feature and Kani mutation gates.

#![forbid(unsafe_code)]

mod gates;

use gates::{collect_features, collect_mutations, trusted_home, Outcome, SystemRunner};

fn main() {
    let Some(gate) = std::env::args().nth(1) else {
        eprintln!("usage: runtime-gates <feature-matrix|kani-mutations>");
        std::process::exit(2);
    };
    let root = match std::env::current_dir() {
        Ok(root) => root,
        Err(error) => {
            eprintln!("GATE_UNAVAILABLE: {error}");
            std::process::exit(2);
        }
    };
    let home = match trusted_home() {
        Ok(home) => home,
        Err(error) => {
            eprintln!("GATE_UNAVAILABLE: {error}");
            std::process::exit(2);
        }
    };
    let mut runner = SystemRunner;
    let (prefix, success_noun, rows) = match gate.as_str() {
        "feature-matrix" => (
            "FEATURE_MATRIX",
            "feature sets",
            collect_features(&mut runner, &root, &home),
        ),
        "kani-mutations" => (
            "KANI_MUTATION",
            "Kani semantic mutation controls",
            collect_mutations(&mut runner, &root, &home),
        ),
        _ => {
            eprintln!("unknown gate: {gate}");
            std::process::exit(2);
        }
    };
    for row in rows.iter().filter(|row| row.outcome != Outcome::Pass) {
        eprintln!(
            "{prefix}_{}: {} phase={} traces={} {}",
            row.outcome.label(),
            row.symbol,
            row.phase.map(|phase| phase.label()).unwrap_or("none"),
            row.trace_ids.join(","),
            row.detail.as_deref().unwrap_or("")
        );
    }
    if rows.iter().any(|row| row.outcome != Outcome::Pass) {
        std::process::exit(1);
    }
    println!("verified {} {success_noun}", rows.len());
}
