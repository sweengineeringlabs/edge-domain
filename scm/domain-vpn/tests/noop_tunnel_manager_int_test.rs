//! Tests for `api/vpn/noop/noop_tunnel_manager.rs` and `core/vpn/noop_tunnel_manager.rs`.

use edge_domain_vpn::{NoopTunnelManager, TunnelManager, TunnelStatus};
use tokio::runtime::Runtime;

#[test]
fn test_noop_tunnel_manager_new_creates_instance_happy() {
    let _mgr = NoopTunnelManager::new();
}

#[test]
fn test_noop_tunnel_manager_default_equals_new_edge() {
    let _a = NoopTunnelManager::new();
    let _b = NoopTunnelManager::default();
}

#[test]
fn test_noop_tunnel_manager_connect_always_ok_happy() {
    let rt = Runtime::new().unwrap();
    let mgr = NoopTunnelManager::new();
    assert!(rt.block_on(mgr.connect()).is_ok());
}

#[test]
fn test_noop_tunnel_manager_disconnect_always_ok_happy() {
    let rt = Runtime::new().unwrap();
    let mgr = NoopTunnelManager::new();
    assert!(rt.block_on(mgr.disconnect()).is_ok());
}

#[test]
fn test_noop_tunnel_manager_status_always_connected_happy() {
    let rt = Runtime::new().unwrap();
    let mgr = NoopTunnelManager::new();
    let s = rt.block_on(mgr.status()).unwrap();
    assert_eq!(s, TunnelStatus::Connected);
}

#[test]
fn test_noop_tunnel_manager_connect_then_status_still_connected_edge() {
    let rt = Runtime::new().unwrap();
    let mgr = NoopTunnelManager::new();
    rt.block_on(async {
        mgr.connect().await.unwrap();
        assert_eq!(mgr.status().await.unwrap(), TunnelStatus::Connected);
    });
}
