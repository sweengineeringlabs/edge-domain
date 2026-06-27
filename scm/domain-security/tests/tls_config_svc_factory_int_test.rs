//! Integration tests for the [`tls_config_svc_factory`] SAF module.

use edge_domain_security::TLS_CONFIG_SVC_FACTORY;

/// @covers: TLS_CONFIG_SVC_FACTORY
#[test]
fn test_tls_config_svc_factory_is_unit_type_happy() {
    let marker = TLS_CONFIG_SVC_FACTORY;
    assert_eq!(
        marker,
        (),
        "TLS_CONFIG_SVC_FACTORY anchor must be unit type"
    );
}

/// @covers: TLS_CONFIG_SVC_FACTORY
#[test]
fn test_tls_config_svc_factory_matches_unit_pattern_error() {
    let marker = TLS_CONFIG_SVC_FACTORY;
    assert!(
        matches!(marker, ()),
        "TLS_CONFIG_SVC_FACTORY must match unit pattern"
    );
}

/// @covers: TLS_CONFIG_SVC_FACTORY
#[test]
fn test_tls_config_svc_factory_is_zero_sized_edge() {
    assert_eq!(
        std::mem::size_of_val(&TLS_CONFIG_SVC_FACTORY),
        0,
        "TLS_CONFIG_SVC_FACTORY anchor must be zero-sized"
    );
}
