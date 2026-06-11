//! Coverage for api/event/types/expected_version.rs
use edge_domain::ExpectedVersion;

#[test]
fn test_expected_version_any_equals_any() {
    assert_eq!(ExpectedVersion::Any, ExpectedVersion::Any);
}

#[test]
fn test_expected_version_nostream_not_equal_to_any() {
    assert_ne!(ExpectedVersion::NoStream, ExpectedVersion::Any);
}

#[test]
fn test_expected_version_exact_carries_version_number() {
    let v = ExpectedVersion::Exact(42);
    assert_eq!(v, ExpectedVersion::Exact(42));
    assert_ne!(v, ExpectedVersion::Exact(0));
}
