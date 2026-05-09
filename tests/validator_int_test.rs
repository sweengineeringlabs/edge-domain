//! Integration tests for the `validate` SAF function.

use edge_domain::{validate, Validator};

struct ValidConfig;
impl Validator for ValidConfig {
    fn validate(&self) -> Result<(), String> { Ok(()) }
}

struct StrictConfig { limit: u32 }
impl Validator for StrictConfig {
    fn validate(&self) -> Result<(), String> {
        if self.limit == 0 { Err("limit must be > 0".into()) } else { Ok(()) }
    }
}

/// @covers: validate
#[test]
fn test_validate_returns_ok_for_valid_config() {
    assert!(validate(&ValidConfig).is_ok());
}

/// @covers: validate
#[test]
fn test_validate_returns_err_with_reason_for_invalid_config() {
    let err = validate(&StrictConfig { limit: 0 }).unwrap_err();
    assert_eq!(err, "limit must be > 0");
}
