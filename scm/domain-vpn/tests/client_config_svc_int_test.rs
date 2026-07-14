//! Tests for `saf/vpn/client_config_svc.rs` — VpnClientConfig SAF exports.
//! Required by `saf_trait_svc_test_exists`.

use std::fs;

use base64::{Engine as _, engine::general_purpose::STANDARD};
use edge_application_vpn::{CLIENT_CONFIG_SVC, VpnClientConfig};
use swe_edge_configbuilder::{ConfigLoaderFactory, FeatureState, OptionalSection};

fn loader(dir: &tempfile::TempDir, toml: &str) -> swe_edge_configbuilder::SectionLoaderImpl {
    fs::write(dir.path().join("application.toml"), toml).unwrap();
    ConfigLoaderFactory::create_loader_for_dir(dir.path().to_str().unwrap())
}

#[test]
fn test_client_config_svc_constant_is_non_empty_happy() {
    assert!(!CLIENT_CONFIG_SVC.is_empty());
}

#[test]
fn test_client_config_svc_constant_starts_with_edge_vpn_happy() {
    assert!(CLIENT_CONFIG_SVC.starts_with("edge.vpn"), "expected 'edge.vpn.' prefix in {CLIENT_CONFIG_SVC}");
}

#[test]
fn test_vpn_client_config_load_optional_absent_returns_disabled_edge() {
    let dir = tempfile::tempdir().unwrap();
    let state = VpnClientConfig::load_optional(&loader(&dir, "")).unwrap();
    assert!(matches!(state, FeatureState::Disabled));
}

#[test]
fn test_vpn_client_config_load_optional_present_returns_enabled_happy() {
    let dir = tempfile::tempdir().unwrap();
    let key = STANDARD.encode([7u8; 32]);
    let toml = format!("[vpn]\nendpoint=\"127.0.0.1:51820\"\nserver_public_key=\"{key}\"\n");
    let state = VpnClientConfig::load_optional(&loader(&dir, &toml)).unwrap();
    assert!(matches!(state, FeatureState::Enabled(_)));
}
