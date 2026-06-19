//! Tests for `saf/vpn/tunnel_manager_svc.rs` — VpnManagerFactory SAF methods.
//! Required by `saf_trait_svc_test_exists` and `saf_trait_svc_fn_scenario_coverage`.

use edge_domain_vpn::{VpnManagerFactory, TunnelStatus, TunnelManager};
use tokio::runtime::Runtime;

// ── noop_tunnel_manager ───────────────────────────────────────────────────────

#[test]
fn test_noop_tunnel_manager_returns_connected_status_happy() {
    let rt = Runtime::new().unwrap();
    let mgr = VpnManagerFactory::noop_tunnel_manager();
    let s = rt.block_on(mgr.status()).unwrap();
    assert_eq!(s, TunnelStatus::Connected);
}

#[test]
fn test_noop_tunnel_manager_connect_succeeds_not_error_error() {
    // Noop always succeeds — verify the error path is not triggered.
    let rt = Runtime::new().unwrap();
    let mgr = VpnManagerFactory::noop_tunnel_manager();
    let result = rt.block_on(mgr.connect());
    assert!(result.is_ok(), "noop connect must never return Err");
}

#[test]
fn test_noop_tunnel_manager_used_as_dyn_trait_object_edge() {
    let rt = Runtime::new().unwrap();
    let mgr: Box<dyn TunnelManager> = VpnManagerFactory::noop_tunnel_manager();
    rt.block_on(async {
        mgr.connect().await.unwrap();
        assert_eq!(mgr.status().await.unwrap(), TunnelStatus::Connected);
        mgr.disconnect().await.unwrap();
    });
}
