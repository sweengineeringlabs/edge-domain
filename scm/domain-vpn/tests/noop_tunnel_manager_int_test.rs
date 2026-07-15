//! Tests for `api/vpn/noop/noop_tunnel_manager.rs` and `core/vpn/noop_tunnel_manager.rs`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_application_vpn::{NoopTunnelManager, TunnelManager, TunnelStatus};
use tokio::runtime::Runtime;

#[test]
fn test_noop_tunnel_manager_new_creates_instance_happy() {
    let mgr = NoopTunnelManager::new();
    assert_eq!(std::mem::size_of_val(&mgr), 0);
}

#[test]
#[allow(clippy::default_constructed_unit_structs)]
fn test_noop_tunnel_manager_default_equals_new_edge() {
    let a = NoopTunnelManager::new();
    let b = NoopTunnelManager::default();
    assert_eq!(std::mem::size_of_val(&a), std::mem::size_of_val(&b));
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
