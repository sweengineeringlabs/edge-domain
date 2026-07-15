//! Integration tests for `NonEmptyString`.
#![cfg(feature = "valueobject")]
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::collections::HashMap;

use edge_application::NonEmptyString;

#[test]
fn test_non_empty_string_new_with_valid_value_returns_ok() {
    let result = NonEmptyString::new("hello");
    assert!(result.is_ok());
    assert_eq!(result.unwrap().as_str(), "hello");
}

#[test]
fn test_non_empty_string_new_with_empty_string_returns_err() {
    let result = NonEmptyString::new("");
    assert!(result.is_err());
}

#[test]
fn test_non_empty_string_equal_instances_are_eq() {
    let a = NonEmptyString::new("value").unwrap();
    let b = NonEmptyString::new("value").unwrap();
    assert_eq!(a, b);
}

#[test]
fn test_non_empty_string_different_content_are_not_eq() {
    let a = NonEmptyString::new("alpha").unwrap();
    let b = NonEmptyString::new("beta").unwrap();
    assert_ne!(a, b);
}

#[test]
fn test_non_empty_string_clone_equals_original() {
    let original = NonEmptyString::new("cloneable").unwrap();
    let cloned = original.clone();
    assert_eq!(original, cloned);
}

#[test]
fn test_non_empty_string_usable_as_hashmap_key() {
    let key = NonEmptyString::new("k1").unwrap();
    let mut map: HashMap<NonEmptyString, u32> = HashMap::new();
    map.insert(key.clone(), 42);
    assert_eq!(map.get(&key), Some(&42));
}

/// @covers: ValueObjectFactory::non_empty_string — happy path: valid string succeeds
#[test]
fn test_non_empty_string_valid_input_happy() {
    let s = NonEmptyString::new("hello").unwrap();
    assert_eq!(s.as_str(), "hello");
}

/// @covers: ValueObjectFactory::non_empty_string — error: empty string is rejected
#[test]
fn test_non_empty_string_empty_input_error() {
    assert!(NonEmptyString::new("").is_err());
}

/// @covers: ValueObjectFactory::non_empty_string — edge: whitespace-only is non-empty
#[test]
fn test_non_empty_string_whitespace_only_is_valid_edge() {
    let s = NonEmptyString::new("  ").unwrap();
    assert_eq!(s.as_str(), "  ");
}
