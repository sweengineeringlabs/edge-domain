//! Tests for `api/vpn/errors/vpn_error.rs` — VpnError display and variants.

use edge_domain_vpn::VpnError;

#[test]
fn test_vpn_error_config_error_variant_display_happy() {
    let e = VpnError::ConfigError { reason: "bad encoding".to_string() };
    assert!(e.to_string().contains("bad encoding"));
}

#[test]
fn test_vpn_error_config_error_empty_reason_error() {
    let e = VpnError::ConfigError { reason: String::new() };
    assert!(e.to_string().contains("config error"));
}

#[test]
fn test_vpn_error_all_variants_display_edge() {
    let variants: Vec<Box<dyn std::fmt::Display>> = vec![
        Box::new(VpnError::ConfigError { reason: "r".to_string() }),
        Box::new(VpnError::ConnectionFailed { reason: "r".to_string() }),
        Box::new(VpnError::TunnelError { reason: "r".to_string() }),
        Box::new(VpnError::Unavailable { reason: "r".to_string() }),
    ];
    for v in variants {
        assert!(!v.to_string().is_empty());
    }
}
