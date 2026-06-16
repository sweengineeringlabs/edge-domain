//! Integration tests for `ExpectedVersion`.

use edge_domain_event::ExpectedVersion;

/// @covers: ExpectedVersion — Any variant equality
#[test]
fn test_expected_version_any_equals_any_happy() {
    assert_eq!(ExpectedVersion::Any, ExpectedVersion::Any);
}

/// @covers: ExpectedVersion — NoStream not equal to Any
#[test]
fn test_expected_version_no_stream_not_equal_any_error() {
    assert_ne!(ExpectedVersion::NoStream, ExpectedVersion::Any);
}

/// @covers: ExpectedVersion — Exact wraps and exposes value
#[test]
fn test_expected_version_exact_wraps_value_edge() {
    let v = ExpectedVersion::Exact(42);
    assert_eq!(v, ExpectedVersion::Exact(42));
    assert_ne!(v, ExpectedVersion::Exact(0));
}
