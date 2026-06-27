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

/// @covers: verify
#[test]
fn test_token_verifier_verify_happy() {
    let verifier = OkVerifier;
    let result = verifier.verify("valid-token");
    assert!(result.is_ok(), "verify must succeed with valid token");
    assert_eq!(result.unwrap(), Claims::default());
}

/// @covers: verify
#[test]
fn test_token_verifier_verify_error() {
    let verifier = FailVerifier;
    let result = verifier.verify("invalid-token");
    assert!(result.is_err(), "verify must fail with invalid token");
}

/// @covers: verify
#[test]
fn test_token_verifier_verify_edge_empty_token() {
    let verifier = OkVerifier;
    let result = verifier.verify("");
    assert!(result.is_ok(), "verify must handle empty token");
}
