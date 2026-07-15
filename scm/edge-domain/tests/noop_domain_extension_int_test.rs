//! Contract tests for NoopDomainExtension.

use edge_application::NoopDomainExtension;

/// @covers: noop_domain_extension
#[test]
fn test_noop_domain_extension_constructs_without_args() {
    let ext = NoopDomainExtension;
    assert_eq!(std::mem::size_of_val(&ext), 0);
}
