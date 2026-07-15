//! Umbrella-level integration tests that exercise `edge-domain-valueobject` as a
//! dependency — verifying the sub-crate contract is accessible end-to-end.
#![allow(clippy::unwrap_used)]

use edge_application_valueobject::{NonEmptyString, ValueObject};

fn accepts_value_object<V: ValueObject>(_v: V) {}

/// @covers: edge-domain-valueobject::NonEmptyString — accessible as a direct dep of the umbrella
#[test]
fn test_value_object_sub_crate_non_empty_string_accessible_as_umbrella_dep_happy() {
    let name = NonEmptyString::new("umbrella").unwrap();
    accepts_value_object(name.clone());
    assert_eq!(name.as_str(), "umbrella");
}

/// @covers: edge-domain-valueobject::NonEmptyString::new — empty string returns Err
#[test]
fn test_value_object_sub_crate_non_empty_string_rejects_empty_string_error() {
    assert!(NonEmptyString::new("").is_err());
}
