//! Small no_std runtime support for generated contract oracles and harness verdicts.

/// Placeholder entry point.
pub fn hello() -> &'static str {
    "hello from quire_contract_runtime"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hello_returns_greeting() {
        assert!(hello().contains("quire_contract_runtime"));
    }
}
