//! Tests for `api/vpn/vo/vpn_manager_factory.rs` — VpnManagerFactory.

use edge_application_vpn::{VpnManagerFactory, TunnelStatus, TunnelManager};
use tokio::runtime::Runtime;

#[test]
fn test_vpn_manager_factory_noop_tunnel_manager_returns_working_manager_happy() {
    let rt = Runtime::new().unwrap();
    let mgr = VpnManagerFactory::noop_tunnel_manager();
    rt.block_on(async {
        mgr.connect().await.unwrap();
        let s = mgr.status().await.unwrap();
        assert_eq!(s, TunnelStatus::Connected);
        mgr.disconnect().await.unwrap();
    });
}

#[test]
fn test_vpn_manager_factory_noop_tunnel_manager_is_dyn_compatible_edge() {
    // Verify factory returns Box<dyn TunnelManager> — dyn object works cross-module.
    let mgr: Box<dyn TunnelManager> = VpnManagerFactory::noop_tunnel_manager();
    let rt = Runtime::new().unwrap();
    rt.block_on(async { mgr.connect().await }).unwrap();
}
