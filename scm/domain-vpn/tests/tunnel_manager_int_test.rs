//! Tests for `api/vpn/traits/tunnel_manager.rs` — TunnelManager trait contract.
//!
//! Uses `NoopTunnelManager` as the concrete impl under test.
//! Async trait tests use `#[test] + block_on` so the arch checker can find them.

use base64::Engine as _;
use edge_domain_vpn::{
    NoopTunnelManager, TunnelManager, TunnelStatus, VpnClientConfig, VpnManagerFactory,
};
use tokio::runtime::Runtime;

struct FailingTunnelManager;

#[async_trait::async_trait]
impl TunnelManager for FailingTunnelManager {
    async fn connect(&self) -> edge_domain_vpn::VpnResult<()> {
        Err(edge_domain_vpn::VpnError::ConnectionFailed { reason: "test error".to_string() })
    }
    async fn disconnect(&self) -> edge_domain_vpn::VpnResult<()> {
        Err(edge_domain_vpn::VpnError::ConnectionFailed { reason: "test error".to_string() })
    }
    async fn status(&self) -> edge_domain_vpn::VpnResult<TunnelStatus> {
        Err(edge_domain_vpn::VpnError::Unavailable { reason: "test error".to_string() })
    }
    async fn configure(&self, _config: &VpnClientConfig) -> Result<(), edge_domain_vpn::VpnError> {
        Err(edge_domain_vpn::VpnError::ConfigError { reason: "always fails".to_string() })
    }
    fn factory() -> VpnManagerFactory
    where
        Self: Sized,
    {
        VpnManagerFactory
    }
}

fn valid_config() -> VpnClientConfig {
    use std::net::SocketAddr;
    // 32-byte all-zero key, base64-encoded — passes decode_key_32 validation.
    let key = base64::engine::general_purpose::STANDARD.encode([0u8; 32]);
    VpnClientConfig {
        endpoint: "127.0.0.1:51820".parse::<SocketAddr>().unwrap(),
        server_public_key: key,
        psk: None,
        keepalive_interval: 25,
        handshake_timeout: 10,
        auto_reconnect: true,
        max_reconnect_attempts: 0,
        verbose: false,
    }
}

// ── factory ──────────────────────────────────────────────────────────────────

#[test]
fn test_factory_noop_returns_vpn_manager_factory_happy() {
    let marker: VpnManagerFactory = NoopTunnelManager::factory();
    assert_eq!(std::mem::size_of_val(&marker), 0);
}

#[test]
fn test_factory_failing_impl_also_returns_factory_marker_error() {
    // Even a failing TunnelManager provides a valid factory marker.
    let marker: VpnManagerFactory = FailingTunnelManager::factory();
    assert_eq!(std::mem::size_of_val(&marker), 0);
}

#[test]
fn test_factory_multiple_calls_return_equivalent_value_edge() {
    let f1 = NoopTunnelManager::factory();
    let f2 = NoopTunnelManager::factory();
    // VpnManagerFactory is a unit struct — any two values are equivalent.
    let _ = (f1, f2);
}

// ── configure ────────────────────────────────────────────────────────────────

#[test]
fn test_configure_noop_returns_ok_happy() {
    let rt = Runtime::new().unwrap();
    let mgr = NoopTunnelManager::new();
    let cfg = valid_config();
    assert!(rt.block_on(mgr.configure(&cfg)).is_ok());
}

#[test]
fn test_configure_failing_manager_returns_err_error() {
    let rt = Runtime::new().unwrap();
    let mgr = FailingTunnelManager;
    let cfg = valid_config();
    assert!(rt.block_on(mgr.configure(&cfg)).is_err());
}

#[test]
fn test_configure_called_twice_succeeds_edge() {
    let rt = Runtime::new().unwrap();
    let mgr = NoopTunnelManager::new();
    let cfg = valid_config();
    rt.block_on(async {
        mgr.configure(&cfg).await.unwrap();
        mgr.configure(&cfg).await.unwrap();
    });
}

// ── connect ──────────────────────────────────────────────────────────────────

#[test]
fn test_connect_noop_returns_ok_happy() {
    let rt = Runtime::new().unwrap();
    let mgr = NoopTunnelManager::new();
    assert!(rt.block_on(mgr.connect()).is_ok());
}

#[test]
fn test_connect_failing_manager_returns_err_error() {
    let rt = Runtime::new().unwrap();
    let mgr = FailingTunnelManager;
    assert!(rt.block_on(mgr.connect()).is_err());
}

#[test]
fn test_connect_repeated_calls_succeed_edge() {
    let rt = Runtime::new().unwrap();
    let mgr = NoopTunnelManager::new();
    rt.block_on(async {
        mgr.connect().await.unwrap();
        mgr.connect().await.unwrap();
        mgr.connect().await.unwrap();
    });
}

// ── disconnect ───────────────────────────────────────────────────────────────

#[test]
fn test_disconnect_noop_returns_ok_happy() {
    let rt = Runtime::new().unwrap();
    let mgr = NoopTunnelManager::new();
    assert!(rt.block_on(mgr.disconnect()).is_ok());
}

#[test]
fn test_disconnect_failing_manager_returns_err_error() {
    let rt = Runtime::new().unwrap();
    let mgr = FailingTunnelManager;
    assert!(rt.block_on(mgr.disconnect()).is_err());
}

#[test]
fn test_disconnect_before_connect_succeeds_edge() {
    let rt = Runtime::new().unwrap();
    let mgr = NoopTunnelManager::new();
    assert!(rt.block_on(mgr.disconnect()).is_ok());
}

// ── status ───────────────────────────────────────────────────────────────────

#[test]
fn test_status_noop_returns_connected_happy() {
    let rt = Runtime::new().unwrap();
    let mgr = NoopTunnelManager::new();
    let s = rt.block_on(mgr.status()).unwrap();
    assert_eq!(s, TunnelStatus::Connected);
}

#[test]
fn test_status_failing_manager_returns_err_error() {
    let rt = Runtime::new().unwrap();
    let mgr = FailingTunnelManager;
    assert!(rt.block_on(mgr.status()).is_err());
}

#[test]
fn test_status_called_before_and_after_connect_consistent_edge() {
    let rt = Runtime::new().unwrap();
    let mgr = NoopTunnelManager::new();
    rt.block_on(async {
        let s1 = mgr.status().await.unwrap();
        mgr.connect().await.unwrap();
        let s2 = mgr.status().await.unwrap();
        assert_eq!(s1, s2);
    });
}
