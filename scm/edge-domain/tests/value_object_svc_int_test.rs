//! SAF facade integration tests for `ValueObject` and `NonEmptyString`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_application::NonEmptyString;
use edge_application::ValueObject;

fn accepts_value_object<V: ValueObject>(_v: V) {}

/// @covers: ValueObject SAF re-export
#[test]
fn test_value_object_svc_non_empty_string_satisfies_value_object_bound() {
    let v = NonEmptyString::new("test").unwrap();
    accepts_value_object(v);
}

/// @covers: ValueObject SAF re-export
#[test]
fn test_value_object_svc_non_empty_string_accessible_from_crate_root() {
    let v = NonEmptyString::new("accessible").unwrap();
    assert_eq!(v.as_str(), "accessible");
}
