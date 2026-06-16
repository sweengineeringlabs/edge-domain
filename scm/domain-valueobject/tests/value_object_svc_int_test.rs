//! SAF facade integration tests for `ValueObject` and `NonEmptyString`.
#![allow(clippy::unwrap_used)]

use edge_domain_valueobject::{NonEmptyString, ValueObject};

fn accepts_value_object<V: ValueObject>(_v: V) {}

/// @covers: ValueObject — NonEmptyString satisfies the ValueObject bound
#[test]
fn test_value_object_svc_non_empty_string_satisfies_value_object_bound_happy() {
    let v = NonEmptyString::new("test").unwrap();
    accepts_value_object(v);
}

/// @covers: ValueObject — NonEmptyString is accessible from crate root
#[test]
fn test_value_object_svc_non_empty_string_accessible_from_crate_root_happy() {
    let v = NonEmptyString::new("accessible").unwrap();
    assert_eq!(v.as_str(), "accessible");
}

/// @covers: ValueObjectError — error type is accessible from crate root
#[test]
fn test_value_object_svc_value_object_error_accessible_via_public_api_edge() {
    use edge_domain_valueobject::ValueObjectError;
    let e = ValueObjectError::Empty;
    assert_eq!(e.to_string(), "value must not be empty");
}
