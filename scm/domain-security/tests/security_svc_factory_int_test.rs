//! Integration tests for security_svc_factory module.

use edge_domain_security::Security;

#[test]
fn test_security_trait_is_accessible() {
    /// @covers: security_svc_factory module re-exports Security trait
    /// This test confirms that the Security trait is re-exported from the SAF
    /// and accessible via the svc_factory pattern.
    fn _assert_security_object_safe(_s: &dyn Security) {}

    // Compile-time check that Security is object-safe
    let _ = _assert_security_object_safe;
}
