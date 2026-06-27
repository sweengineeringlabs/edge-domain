//! Integration tests for TokenVerifier trait.

use edge_domain_security::{Claims, TokenVerifier, SecurityError};

struct SuccessTokenVerifier;
impl TokenVerifier for SuccessTokenVerifier {
    fn verify(&self, _token: &str) -> Result<Claims, SecurityError> {
        Ok(Claims::default())
    }
}

struct FailTokenVerifier;
impl TokenVerifier for FailTokenVerifier {
    fn verify(&self, _token: &str) -> Result<Claims, SecurityError> {
        Err(SecurityError::Auth("invalid token".to_string()))
    }
}

#[test]
fn test_verify_valid_happy() {
    let verifier = SuccessTokenVerifier;
    let result = verifier.verify("valid-token");
    let claims = result.unwrap();
    assert_eq!(claims, Claims::default());
}

#[test]
fn test_verify_invalid_error() {
    let verifier = FailTokenVerifier;
    assert!(verifier.verify("invalid-token").is_err());
}

#[test]
fn test_verify_empty_edge() {
    let verifier = SuccessTokenVerifier;
    assert!(verifier.verify("").is_ok());
}
