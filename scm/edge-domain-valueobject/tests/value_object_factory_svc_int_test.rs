//! Integration tests for the `ValueObjectFactory` SAF facade.
#![allow(clippy::unwrap_used)]

use edge_domain_valueobject::{ValueObjectFactory};

struct TestValueObjects;
impl ValueObjectFactory for TestValueObjects {}

/// @covers ValueObjectFactory::non_empty_string — happy path: valid string returns Ok
#[test]
fn test_non_empty_string_via_factory_valid_input_happy() {
    let r = TestValueObjects::non_empty_string("hello".to_string());
    assert!(r.is_ok());
    assert_eq!(r.unwrap().as_str(), "hello");
}

/// @covers ValueObjectFactory::non_empty_string — error: empty string returns Err
#[test]
fn test_non_empty_string_via_factory_empty_returns_err_error() {
    let r = TestValueObjects::non_empty_string(String::new());
    assert!(r.is_err());
}

/// @covers ValueObjectFactory::non_empty_string — edge: whitespace-only string is non-empty
#[test]
fn test_non_empty_string_via_factory_whitespace_is_valid_edge() {
    let r = TestValueObjects::non_empty_string("  ".to_string());
    assert!(r.is_ok());
}
