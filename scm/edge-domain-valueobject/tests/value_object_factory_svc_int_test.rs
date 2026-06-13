//! Integration tests for the `ValueObjectFactory` SAF facade and `non_empty_string` factory fn.
#![allow(clippy::unwrap_used)]

use edge_domain_valueobject::{ValueObjectFactory, non_empty_string};

struct TestVo;
impl ValueObjectFactory for TestVo {}

/// @covers: non_empty_string
#[test]
fn test_non_empty_string_valid_input_happy() {
    let r = non_empty_string("hello".to_string());
    assert!(r.is_ok());
    assert_eq!(r.unwrap().as_str(), "hello");
}

/// @covers: non_empty_string
#[test]
fn test_non_empty_string_empty_input_error() {
    let r = non_empty_string(String::new());
    assert!(r.is_err());
}

/// @covers: non_empty_string
#[test]
fn test_non_empty_string_whitespace_only_edge() {
    let r = non_empty_string("  ".to_string());
    assert!(r.is_ok());
}

/// @covers: ValueObjectFactory
#[test]
fn test_value_object_factory_trait_method_valid_happy() {
    let r = TestVo::non_empty_string("world".to_string());
    assert!(r.is_ok());
}

/// @covers: ValueObjectFactory
#[test]
fn test_value_object_factory_trait_method_empty_error() {
    let r = TestVo::non_empty_string(String::new());
    assert!(r.is_err());
}

/// @covers: ValueObjectFactory
#[test]
fn test_value_object_factory_trait_method_whitespace_edge() {
    let r = TestVo::non_empty_string("  ".to_string());
    assert!(r.is_ok());
}
