//! Structural coverage for [`BootstrapNameRequest`].

use edge_domain_command::BootstrapNameRequest;

/// @covers: BootstrapNameRequest
#[test]
fn test_bootstrap_name_request_is_zero_sized_happy() {
    assert_eq!(std::mem::size_of::<BootstrapNameRequest>(), 0);
}

/// @covers: BootstrapNameRequest
#[test]
#[allow(clippy::default_constructed_unit_structs)]
fn test_bootstrap_name_request_default_equals_unit_value_happy() {
    assert_eq!(BootstrapNameRequest::default(), BootstrapNameRequest);
}

/// @covers: BootstrapNameRequest
#[test]
fn test_bootstrap_name_request_is_copy_edge() {
    let a = BootstrapNameRequest;
    let b = a;
    assert_eq!(a, b);
}
