//! Tests for `api/vpn/types/vpn_client_config.rs` — VpnClientConfig validation.

use std::fs;

use base64::{Engine as _, engine::general_purpose::STANDARD};
use edge_domain_vpn::VpnClientConfig;
use swe_edge_configbuilder::{ConfigLoaderFactory, FeatureState, OptionalSection};

fn loader(dir: &tempfile::TempDir, toml: &str) -> swe_edge_configbuilder::SectionLoaderImpl {
    fs::write(dir.path().join("application.toml"), toml).unwrap();
    ConfigLoaderFactory::create_loader_for_dir(dir.path().to_str().unwrap())
}

fn valid_key() -> String {
    STANDARD.encode([0u8; 32])
}

#[test]
fn test_vpn_client_config_valid_key_parses_happy() {
    let dir = tempfile::tempdir().unwrap();
    let toml = format!("[vpn]\nendpoint=\"127.0.0.1:51820\"\nserver_public_key=\"{}\"\n", valid_key());
    let state = VpnClientConfig::load_optional(&loader(&dir, &toml)).unwrap();
    assert!(matches!(state, FeatureState::Enabled(_)));
}

#[test]
fn test_vpn_client_config_invalid_key_returns_config_error_error() {
    let dir = tempfile::tempdir().unwrap();
    let toml = "[vpn]\nendpoint=\"127.0.0.1:51820\"\nserver_public_key=\"INVALID!!!\"\n";
    assert!(VpnClientConfig::load_optional(&loader(&dir, toml)).is_err());
}

#[test]
fn test_vpn_client_config_decode_key_32_wrong_length_edge() {
    let result = VpnClientConfig::decode_key_32(&STANDARD.encode([0u8; 16]));
    assert!(result.is_err(), "16-byte key must fail 32-byte check");
}

#[test]
fn test_vpn_client_config_decode_key_32_correct_length_happy() {
    let result = VpnClientConfig::decode_key_32(&valid_key());
    assert!(result.is_ok());
    assert_eq!(result.unwrap().len(), 32);
}

#[test]
fn test_vpn_client_config_zero_keepalive_interval_rejected_error() {
    let dir = tempfile::tempdir().unwrap();
    let toml = format!("[vpn]\nendpoint=\"127.0.0.1:51820\"\nserver_public_key=\"{}\"\nkeepalive_interval=0\n", valid_key());
    assert!(VpnClientConfig::load_optional(&loader(&dir, &toml)).is_err());
}

#[test]
fn test_vpn_client_config_default_fields_applied_edge() {
    let dir = tempfile::tempdir().unwrap();
    let toml = format!("[vpn]\nendpoint=\"127.0.0.1:51820\"\nserver_public_key=\"{}\"\n", valid_key());
    let FeatureState::Enabled(cfg) = VpnClientConfig::load_optional(&loader(&dir, &toml)).unwrap() else {
        panic!("expected Enabled");
    };
    assert_eq!(cfg.keepalive_interval, VpnClientConfig::default_keepalive());
    assert_eq!(cfg.handshake_timeout, VpnClientConfig::default_handshake_timeout());
    assert_eq!(cfg.auto_reconnect, VpnClientConfig::default_auto_reconnect());
}
