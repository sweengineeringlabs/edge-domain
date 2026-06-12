//! Contract tests for the DomainExtension marker trait.

use edge_domain::NoopDomainExtension;

/// @covers: DomainExtension
#[test]
fn test_noop_domain_extension_satisfies_domain_extension_contract() {
    fn accepts_extension<E: edge_domain::DomainExtension>(_: E) {}
    accepts_extension(NoopDomainExtension);
}
