//! Integration tests — `DomainSpi` trait.

use edge_domain::{DomainSpi, NoopDomainExtension};

/// @covers: DomainSpi — NoopDomainExtension satisfies the DomainSpi contract
#[test]
fn test_noop_extension_satisfies_domain_spi_happy() {
    fn accepts_spi<S: DomainSpi>(_: S) {}
    accepts_spi(NoopDomainExtension);
}

/// @covers: DomainSpi — trait is object-safe when wrapped in Arc
#[test]
fn test_domain_spi_is_object_safe_error() {
    use std::sync::Arc;
    let _: Arc<dyn DomainSpi> = Arc::new(NoopDomainExtension);
}

/// @covers: DomainSpi — Send + Sync bounds hold for NoopDomainExtension
#[test]
fn test_domain_spi_is_send_sync_edge() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<NoopDomainExtension>();
}
