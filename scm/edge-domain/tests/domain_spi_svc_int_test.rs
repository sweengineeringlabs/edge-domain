//! Integration tests — `DomainSpi` SAF facade (`domain_spi_svc`).

use edge_domain::DomainSpi;

/// @covers: DOMAIN_SPI_SVC — SAF anchor is accessible
#[test]
fn test_domain_spi_svc_anchor_is_accessible_happy() {
    let _ = edge_domain::DOMAIN_SPI_SVC;
}

/// @covers: DomainSpi re-export — trait usable from crate root
#[test]
fn test_domain_spi_re_exported_from_crate_root_happy() {
    fn accepts_spi<S: DomainSpi>(_: &S) {}
    accepts_spi(&edge_domain::NoopDomainExtension);
}

/// @covers: DomainSpi — trait bound is enforced at compile time (non-implementing type fails)
#[test]
fn test_domain_spi_bound_enforced_at_compile_time_edge() {
    // This test documents that the trait bound exists — the constraint is
    // verified by the fact that this file compiles only with conforming impls.
    fn assert_spi_bound<T: DomainSpi>() {}
    assert_spi_bound::<edge_domain::NoopDomainExtension>();
}
