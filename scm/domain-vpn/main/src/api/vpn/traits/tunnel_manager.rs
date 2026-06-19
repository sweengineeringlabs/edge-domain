//! `TunnelManager` — VPN tunnel lifecycle contract.

use async_trait::async_trait;

use crate::api::TunnelStatus;
use crate::api::VpnClientConfig;
use crate::api::VpnError;
use crate::api::VpnManagerFactory;
use crate::api::VpnResult;

/// Manages the lifecycle of a VPN tunnel connection.
///
/// Implementations establish and tear down the underlying transport (e.g. a
/// WireGuard TUN device) and report the current tunnel state.
///
/// # Platform note
///
/// On Linux, the real WireGuard implementation requires `CAP_NET_ADMIN`.
/// On Windows and in unit tests, use `NoopTunnelManager` which satisfies
/// the trait contract without any OS-level side effects.
#[async_trait]
pub trait TunnelManager: Send + Sync {
    /// Establish the VPN tunnel.
    ///
    /// Blocks until the handshake completes or the `handshake_timeout` expires.
    /// Returns `Err` on the first failure — the caller decides whether to retry.
    async fn connect(&self) -> VpnResult<()>;

    /// Gracefully close the VPN tunnel.
    ///
    /// Sends a WireGuard `SESSION_END` or equivalent before destroying the TUN
    /// device so the server can reclaim the peer slot immediately.
    async fn disconnect(&self) -> VpnResult<()>;

    /// Return the current observable tunnel state.
    ///
    /// Non-blocking; does not initiate any connection or disconnection.
    async fn status(&self) -> VpnResult<TunnelStatus>;

    /// Apply runtime configuration to the tunnel.
    ///
    /// Called after construction to push updated `VpnClientConfig` values
    /// (keepalive, reconnect policy, etc.) without tearing down the tunnel.
    async fn configure(&self, config: &VpnClientConfig) -> Result<(), VpnError>;

    /// Return the `VpnManagerFactory` marker associated with this implementation.
    ///
    /// Excluded from vtables via `where Self: Sized` — use on concrete types only.
    fn factory() -> VpnManagerFactory
    where
        Self: Sized;
}
