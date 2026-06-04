//! Tests for the `Validator` trait contract via the wrapper function.
#![allow(clippy::unwrap_used, clippy::expect_used)]

/// Test config that validates successfully.
#[derive(Default)]
struct ValidConfig;

/// Test config that fails validation.
#[derive(Default)]
struct InvalidConfig;

/// Local Validator trait for testing (mirrors the public API trait).
trait LocalValidator {
    fn validate(&self) -> Result<(), String>;
}

impl LocalValidator for ValidConfig {
    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

impl LocalValidator for InvalidConfig {
    fn validate(&self) -> Result<(), String> {
        Err("invalid configuration".to_string())
    }
}

#[test]
fn test_validator_contract_accepts_valid_config() {
    let config = ValidConfig;
    assert!(config.validate().is_ok());
}

#[test]
fn test_validator_contract_rejects_invalid_config() {
    let config = InvalidConfig;
    let result = config.validate();
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("invalid"));
}
