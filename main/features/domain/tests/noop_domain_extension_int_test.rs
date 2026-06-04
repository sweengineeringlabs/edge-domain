//! Contract tests for NoopDomainExtension.

use edge_domain::NoopDomainExtension;

/// @covers: noop_domain_extension
#[test]
fn test_noop_domain_extension_constructs_without_args() {
    let _ext = NoopDomainExtension;
}
