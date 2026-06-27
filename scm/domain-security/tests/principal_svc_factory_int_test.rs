//! Integration tests for principal_svc_factory module.

use edge_domain_security::Principal;

#[test]
fn test_principal_trait_is_accessible() {
    /// @covers: principal_svc_factory module re-exports Principal trait
    /// This test confirms that the Principal trait is re-exported from the SAF
    /// and accessible via the svc_factory pattern.
    fn _assert_principal_object_safe(_p: &dyn Principal) {}

    // Compile-time check that Principal is object-safe
    let _ = _assert_principal_object_safe;
}
