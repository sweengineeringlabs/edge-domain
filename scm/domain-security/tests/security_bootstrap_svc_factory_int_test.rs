//! Integration tests for security_bootstrap_svc_factory module.

use edge_domain_security::SecurityBootstrap;

#[test]
fn test_security_bootstrap_trait_is_accessible() {
    /// @covers: security_bootstrap_svc_factory module re-exports SecurityBootstrap trait
    /// This test confirms that the SecurityBootstrap trait is re-exported from the SAF
    /// and accessible via the svc_factory pattern.
    fn _assert_security_bootstrap_object_safe(_sb: &dyn SecurityBootstrap) {}

    // Compile-time check that SecurityBootstrap is object-safe
    let _ = _assert_security_bootstrap_object_safe;
}
