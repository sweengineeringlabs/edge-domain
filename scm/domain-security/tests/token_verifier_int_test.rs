//! Integration tests for [`TokenVerifier`] trait.

use edge_domain_security::{Claims, SecurityError, TokenVerifier};

struct OkVerifier;
impl TokenVerifier for OkVerifier {
    fn verify(&self, _token: &str) -> Result<Claims, SecurityError> {
        Ok(Claims::default())
    }
}

struct FailVerifier;
impl TokenVerifier for FailVerifier {
    fn verify(&self, _token: &str) -> Result<Claims, SecurityError> {
        Err(SecurityError::Verification("invalid".to_string()))
    }
}

/// @covers: TokenVerifier::verify
#[test]
fn test_verify_valid_happy() {
    let verifier = OkVerifier;
    let result = verifier.verify("valid-token");
    assert!(result.is_ok(), "verify must succeed with valid token");
    assert_eq!(result.unwrap(), Claims::default());
}

/// @covers: TokenVerifier::verify
#[test]
fn test_verify_invalid_error() {
    let verifier = FailVerifier;
    let result = verifier.verify("invalid-token");
    assert!(result.is_err(), "verify must fail with invalid token");
}

/// @covers: TokenVerifier::verify
#[test]
fn test_verify_empty_edge() {
    let verifier = OkVerifier;
    let result = verifier.verify("");
    assert!(result.is_ok(), "verify must handle empty token");
    assert_eq!(result.unwrap(), Claims::default(), "verify must return default claims");
}
