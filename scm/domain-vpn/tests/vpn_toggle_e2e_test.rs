//! E2E toggle tests — verify `[vpn]` TOML section on/off behaviour.

use std::fs;

use edge_domain_vpn::{VpnClientConfig, VpnError, VpnManagerFactory, TunnelStatus};
use swe_edge_configbuilder::{ConfigLoaderFactory, FeatureState, OptionalSection};
use tokio::runtime::Runtime;

fn write_toml(dir: &tempfile::TempDir, content: &str) -> swe_edge_configbuilder::SectionLoaderImpl {
    fs::write(dir.path().join("application.toml"), content).unwrap();
    ConfigLoaderFactory::create_loader_for_dir(dir.path().to_str().unwrap())
}

fn key_b64(seed: u8) -> String {
    use base64::{Engine as _, engine::general_purpose::STANDARD};
    STANDARD.encode([seed; 32])
}

#[test]
fn test_load_optional_vpn_absent_section_returns_disabled() {
    let dir = tempfile::tempdir().unwrap();
    let loader = write_toml(&dir, "# no [vpn] section\n");
    let state = VpnClientConfig::load_optional(&loader).unwrap();
    assert!(matches!(state, FeatureState::Disabled));
}

#[test]
fn test_load_optional_vpn_present_valid_returns_enabled_with_correct_fields() {
    let dir = tempfile::tempdir().unwrap();
    let toml = format!(
        "[vpn]\nendpoint = \"127.0.0.1:51820\"\nserver_public_key = \"{}\"\nkeepalive_interval = 30\nhandshake_timeout = 15\nauto_reconnect = false\nmax_reconnect_attempts = 3\nverbose = true\n",
        key_b64(0)
    );
    let loader = write_toml(&dir, &toml);
    let FeatureState::Enabled(cfg) = VpnClientConfig::load_optional(&loader).unwrap() else {
        panic!("expected Enabled");
    };
    assert_eq!(cfg.endpoint.to_string(), "127.0.0.1:51820");
    assert_eq!(cfg.keepalive_interval, 30);
    assert_eq!(cfg.handshake_timeout, 15);
    assert!(!cfg.auto_reconnect);
    assert_eq!(cfg.max_reconnect_attempts, 3);
    assert!(cfg.verbose);
    assert!(cfg.psk.is_none());
}

#[test]
fn test_load_optional_vpn_present_with_psk_returns_enabled() {
    let dir = tempfile::tempdir().unwrap();
    let toml = format!(
        "[vpn]\nendpoint = \"10.0.0.1:51820\"\nserver_public_key = \"{}\"\npsk = \"{}\"\n",
        key_b64(1), key_b64(2)
    );
    let loader = write_toml(&dir, &toml);
    let FeatureState::Enabled(cfg) = VpnClientConfig::load_optional(&loader).unwrap() else {
        panic!("expected Enabled");
    };
    assert!(cfg.psk.is_some());
}

#[test]
fn test_load_optional_vpn_defaults_applied_when_optional_fields_absent() {
    let dir = tempfile::tempdir().unwrap();
    let toml = format!("[vpn]\nendpoint = \"127.0.0.1:51820\"\nserver_public_key = \"{}\"\n", key_b64(0));
    let loader = write_toml(&dir, &toml);
    let FeatureState::Enabled(cfg) = VpnClientConfig::load_optional(&loader).unwrap() else {
        panic!("expected Enabled");
    };
    assert_eq!(cfg.keepalive_interval, 25);
    assert_eq!(cfg.handshake_timeout, 10);
    assert!(cfg.auto_reconnect);
    assert_eq!(cfg.max_reconnect_attempts, 0);
    assert!(!cfg.verbose);
}

#[test]
fn test_load_optional_vpn_bad_server_public_key_not_base64_returns_error() {
    let dir = tempfile::tempdir().unwrap();
    let toml = "[vpn]\nendpoint = \"127.0.0.1:51820\"\nserver_public_key = \"NOT_VALID!!!\"\n";
    let loader = write_toml(&dir, toml);
    assert!(VpnClientConfig::load_optional(&loader).is_err());
}

#[test]
fn test_load_optional_vpn_server_public_key_wrong_length_returns_error() {
    let dir = tempfile::tempdir().unwrap();
    use base64::{Engine as _, engine::general_purpose::STANDARD};
    let short_key = STANDARD.encode([0u8; 16]);
    let toml = format!("[vpn]\nendpoint = \"127.0.0.1:51820\"\nserver_public_key = \"{short_key}\"\n");
    let loader = write_toml(&dir, &toml);
    assert!(VpnClientConfig::load_optional(&loader).is_err());
}

#[test]
fn test_load_optional_vpn_zero_keepalive_interval_returns_error() {
    let dir = tempfile::tempdir().unwrap();
    let toml = format!("[vpn]\nendpoint = \"127.0.0.1:51820\"\nserver_public_key = \"{}\"\nkeepalive_interval = 0\n", key_b64(0));
    let loader = write_toml(&dir, &toml);
    assert!(VpnClientConfig::load_optional(&loader).is_err());
}

#[test]
fn test_load_optional_vpn_zero_handshake_timeout_returns_error() {
    let dir = tempfile::tempdir().unwrap();
    let toml = format!("[vpn]\nendpoint = \"127.0.0.1:51820\"\nserver_public_key = \"{}\"\nhandshake_timeout = 0\n", key_b64(0));
    let loader = write_toml(&dir, &toml);
    assert!(VpnClientConfig::load_optional(&loader).is_err());
}

#[test]
fn test_noop_tunnel_manager_connect_returns_ok() {
    let rt = Runtime::new().unwrap();
    let mgr = VpnManagerFactory::noop_tunnel_manager();
    assert!(rt.block_on(mgr.connect()).is_ok());
}

#[test]
fn test_noop_tunnel_manager_disconnect_returns_ok() {
    let rt = Runtime::new().unwrap();
    let mgr = VpnManagerFactory::noop_tunnel_manager();
    assert!(rt.block_on(mgr.disconnect()).is_ok());
}

#[test]
fn test_noop_tunnel_manager_status_returns_connected() {
    let rt = Runtime::new().unwrap();
    let mgr = VpnManagerFactory::noop_tunnel_manager();
    let status = rt.block_on(mgr.status()).unwrap();
    assert_eq!(status, TunnelStatus::Connected);
}

#[test]
fn test_vpn_error_config_error_display_includes_reason() {
    let e = VpnError::ConfigError { reason: "bad key".to_string() };
    assert!(e.to_string().contains("bad key"));
}

#[test]
fn test_vpn_error_connection_failed_display_includes_reason() {
    let e = VpnError::ConnectionFailed { reason: "timeout".to_string() };
    assert!(e.to_string().contains("timeout"));
}

#[test]
fn test_vpn_error_tunnel_error_display_includes_reason() {
    let e = VpnError::TunnelError { reason: "TUN open".to_string() };
    assert!(e.to_string().contains("TUN open"));
}

#[test]
fn test_vpn_error_unavailable_display_includes_reason() {
    let e = VpnError::Unavailable { reason: "Windows".to_string() };
    assert!(e.to_string().contains("Windows"));
}
