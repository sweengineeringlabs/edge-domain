#![allow(clippy::unwrap_used, clippy::expect_used, unused_imports)]
//! SAF facade smoke test — Validator is exported from the crate root.

use edge_domain::Validator;

struct NonEmpty(String);
impl Validator for NonEmpty {
    fn validate(&self) -> Result<(), String> {
        if self.0.is_empty() {
            Err("must not be empty".into())
        } else {
            Ok(())
        }
    }
}

#[test]
fn test_validator_svc_facade_valid_input_returns_ok() {
    assert!(NonEmpty("hello".into()).validate().is_ok());
}

#[test]
fn test_validator_svc_facade_empty_input_returns_err() {
    let err = NonEmpty("".into()).validate().unwrap_err();
    assert!(err.contains("empty"));
}
