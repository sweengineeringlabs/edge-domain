//! Structural coverage for [`BootstrapNameResponse`].

use edge_domain_command::BootstrapNameResponse;

/// @covers: BootstrapNameResponse
#[test]
fn test_bootstrap_name_response_field_returns_configured_value_happy() {
    let response = BootstrapNameResponse { name: "command" };
    assert_eq!(response.name, "command");
}

/// @covers: BootstrapNameResponse
#[test]
fn test_bootstrap_name_response_default_is_empty_name_edge() {
    assert_eq!(
        BootstrapNameResponse::default(),
        BootstrapNameResponse { name: "" }
    );
}

/// @covers: BootstrapNameResponse
#[test]
fn test_bootstrap_name_response_clone_is_independent_edge() {
    let a = BootstrapNameResponse { name: "a" };
    let b = a.clone();
    assert_eq!(a, b);
}
