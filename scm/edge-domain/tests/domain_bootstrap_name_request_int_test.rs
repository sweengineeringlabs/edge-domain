//! Integration tests for `DomainBootstrapNameRequest`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_domain::{Domain, DomainBootstrap, DomainBootstrapNameRequest};

/// @covers: DomainBootstrapNameRequest
#[test]
fn test_domain_bootstrap_name_request_is_zero_sized_happy() {
    assert_eq!(std::mem::size_of::<DomainBootstrapNameRequest>(), 0);
}

/// @covers: DomainBootstrapNameRequest
#[test]
fn test_domain_bootstrap_name_request_accepted_by_bootstrap_name_error() {
    let response = Domain.bootstrap_name(DomainBootstrapNameRequest).unwrap();
    assert_eq!(response.name, "domain");
}

/// @covers: DomainBootstrapNameRequest
#[test]
fn test_domain_bootstrap_name_request_constructible_repeatedly_edge() {
    let first = Domain.bootstrap_name(DomainBootstrapNameRequest).unwrap();
    let second = Domain.bootstrap_name(DomainBootstrapNameRequest).unwrap();
    assert_eq!(first.name, second.name);
}
