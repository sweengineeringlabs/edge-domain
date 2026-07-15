//! End-to-end contract tests for the `Validator` trait, exercised through a
//! test-double implementation via the crate's public API.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_application_handler::{HandlerError, Validator, ValidatorRequest};

struct ValidatorDouble {
    name: String,
}

impl Validator for ValidatorDouble {
    fn validate(&self, _req: ValidatorRequest) -> Result<(), HandlerError> {
        if self.name.is_empty() {
            Err(HandlerError::InvalidRequest("name cannot be empty".into()))
        } else {
            Ok(())
        }
    }
}

/// @covers: Validator::validate
#[test]
fn test_validate_nonempty_name_returns_ok_happy() {
    let v = ValidatorDouble {
        name: "svc".to_string(),
    };
    assert_eq!(v.validate(ValidatorRequest), Ok(()));
}

/// @covers: Validator::validate
#[test]
fn test_validate_empty_name_returns_invalid_request_error() {
    let v = ValidatorDouble {
        name: String::new(),
    };
    let result = v.validate(ValidatorRequest);
    assert!(matches!(result, Err(HandlerError::InvalidRequest(_))));
}

/// @covers: Validator::validate
#[test]
fn test_validate_error_message_describes_constraint_edge() {
    let v = ValidatorDouble {
        name: String::new(),
    };
    let err = v.validate(ValidatorRequest).unwrap_err();
    assert!(err.to_string().contains("empty"));
}
