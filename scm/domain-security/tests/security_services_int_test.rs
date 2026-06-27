//! Integration tests for [`SecurityServices`] type.

use edge_domain_security::SecurityServices;

#[test]
fn test_security_services_copy_happy() {
    let services = SecurityServices;
    let services_copy = services;
    assert_eq!(services, services_copy, "SecurityServices must be Copy");
}

#[test]
fn test_security_services_clone_happy() {
    let services = SecurityServices;
    let cloned = services.clone();
    assert_eq!(
        services, cloned,
        "SecurityServices clone must equal original"
    );
}

#[test]
fn test_security_services_debug_happy() {
    let services = SecurityServices;
    let debug_str = format!("{:?}", services);
    assert!(!debug_str.is_empty(), "Debug output must not be empty");
}

#[test]
fn test_security_services_equality_happy() {
    let s1 = SecurityServices;
    let s2 = SecurityServices;
    assert_eq!(s1, s2, "All SecurityServices instances must be equal");
}

#[test]
fn test_security_services_zero_sized_edge() {
    assert_eq!(
        std::mem::size_of::<SecurityServices>(),
        0,
        "SecurityServices must be zero-sized"
    );
}
