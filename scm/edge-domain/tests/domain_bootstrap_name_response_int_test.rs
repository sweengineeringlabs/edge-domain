//! Integration tests for `DomainBootstrapNameResponse`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_domain::{
    Domain, DomainBootstrap, DomainBootstrapNameRequest, DomainBootstrapNameResponse,
};

/// @covers: DomainBootstrapNameResponse
#[test]
fn test_domain_bootstrap_name_response_carries_expected_name_happy() {
    let response: DomainBootstrapNameResponse =
        Domain.bootstrap_name(DomainBootstrapNameRequest).unwrap();
    assert_eq!(response.name, "domain");
}

/// @covers: DomainBootstrapNameResponse
#[test]
fn test_domain_bootstrap_name_response_name_is_nonempty_error() {
    let response = Domain.bootstrap_name(DomainBootstrapNameRequest).unwrap();
    assert!(!response.name.is_empty());
}

/// @covers: DomainBootstrapNameResponse
#[test]
fn test_domain_bootstrap_name_response_constructible_directly_edge() {
    let response = DomainBootstrapNameResponse { name: "custom" };
    assert_eq!(response.name, "custom");
}
