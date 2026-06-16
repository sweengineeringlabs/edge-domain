//! Integration tests for the `ValueObjectFactory` SAF facade.
//!
//! Construction is via the factory trait method on the implementing type
//! (`NonEmptyString::non_empty_string`), not a free function (SEA rule 191).
#![allow(clippy::unwrap_used)]

use edge_domain_valueobject::{NonEmptyString, ValueObjectFactory, VALUE_OBJECT_FACTORY_SVC};

struct TestVo;
impl ValueObjectFactory for TestVo {}

/// @covers: VALUE_OBJECT_FACTORY_SVC
#[test]
fn test_value_object_factory_svc_identifier_is_stable_happy() {
    assert_eq!(VALUE_OBJECT_FACTORY_SVC, "value_object_factory");
}

/// @covers: ValueObjectFactory
#[test]
fn test_non_empty_string_factory_method_valid_input_happy() {
    let r = NonEmptyString::non_empty_string("hello".to_string());
    assert!(r.is_ok());
    assert_eq!(r.unwrap().as_str(), "hello");
}

/// @covers: ValueObjectFactory
#[test]
fn test_non_empty_string_factory_method_empty_input_error() {
    let r = NonEmptyString::non_empty_string(String::new());
    assert!(r.is_err());
}

/// @covers: ValueObjectFactory
#[test]
fn test_non_empty_string_factory_method_whitespace_only_edge() {
    // Whitespace is non-empty: the factory accepts it (boundary of the empty check).
    let r = NonEmptyString::non_empty_string("  ".to_string());
    assert!(r.is_ok());
    assert_eq!(r.unwrap().as_str(), "  ");
}

/// @covers: ValueObjectFactory
#[test]
fn test_value_object_factory_default_method_works_for_any_implementor_happy() {
    // Proves `non_empty_string` is a real trait method, usable by any implementor.
    let r = TestVo::non_empty_string("world".to_string());
    assert!(r.is_ok());
}
