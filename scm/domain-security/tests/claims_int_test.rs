//! Integration tests for [`Claims`] type.

use edge_domain_security::Claims;

#[test]
fn test_claims_default_happy() {
    let claims = Claims::default();
    assert_eq!(claims, Claims::default(), "Default claims must be equal");
}

#[test]
fn test_claims_clone_happy() {
    let claims = Claims::default();
    let cloned = claims.clone();
    assert_eq!(claims, cloned, "Cloned claims must equal original");
}

#[test]
fn test_claims_debug_happy() {
    let claims = Claims::default();
    let debug_str = format!("{:?}", claims);
    assert!(!debug_str.is_empty(), "Debug output must not be empty");
    assert!(
        debug_str.contains("Claims"),
        "Debug output must include type name"
    );
}

#[test]
fn test_claims_equality_edge() {
    let c1 = Claims::default();
    let c2 = Claims::default();
    assert_eq!(c1, c2, "Default claims must be equal");
}

#[test]
fn test_claims_partial_eq_happy() {
    let mut c1 = Claims::default();
    let mut c2 = Claims::default();
    c1.sub = Some("user1".to_string());
    c2.sub = Some("user1".to_string());
    assert_eq!(c1, c2, "Claims with same sub must be equal");
}
