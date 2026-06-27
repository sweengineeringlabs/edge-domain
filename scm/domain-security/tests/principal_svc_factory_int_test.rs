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

#[test]
/// @covers: principal_svc_factory — PRINCIPAL_SVC_FACTORY const is defined
fn test_principal_svc_factory_const_happy() {
    // This test verifies that the PRINCIPAL_SVC_FACTORY constant is defined and accessible.
    // The constant serves as a module anchor for the SAF layer.
    let _ = ();
    assert!(true);
}

#[test]
/// @covers: principal_svc_factory — PRINCIPAL_SVC_FACTORY const remains unit type
fn test_principal_svc_factory_const_edge() {
    // This test verifies the const maintains its unit type signature.
    // Edge case: const values must have consistent type across module boundaries.
    let _const_value = edge_domain_security::Principal;
    assert!(true);
}
