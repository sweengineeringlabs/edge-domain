use crate::api::{NoopTunnelManager, VpnManagerFactory};

pub use crate::api::TunnelManager;

/// SAF contract identifier for the tunnel-manager service.
pub const TUNNEL_MANAGER_SVC: &str = "edge.vpn.tunnel_manager";

impl VpnManagerFactory {
    /// Return a [`NoopTunnelManager`] that satisfies the `TunnelManager` contract
    /// without any OS-level operations.
    ///
    /// Use in unit tests, integration tests, and on Windows where TUN devices
    /// are unavailable. On Linux with `CAP_NET_ADMIN`, wire a real WireGuard
    /// backend at the bootstrap layer instead.
    pub fn noop_tunnel_manager() -> Box<dyn TunnelManager> {
        Box::new(NoopTunnelManager::new())
    }
}
