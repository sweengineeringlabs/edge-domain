//! `NoopTunnelManager` — a no-op `TunnelManager` struct.
//!
//! The `TunnelManager` implementation is in `core/vpn/noop_tunnel_manager.rs`.

/// A `TunnelManager` that accepts all calls without performing any OS-level
/// operations.  Always reports `TunnelStatus::Connected` after `connect()`.
///
/// Use in:
/// - Unit and integration tests that verify toggle logic without a real kernel.
/// - Windows bootstrap where TUN devices are unavailable.
pub struct NoopTunnelManager;

impl NoopTunnelManager {
    /// Construct a new `NoopTunnelManager`.
    pub fn new() -> Self {
        Self
    }
}

impl Default for NoopTunnelManager {
    fn default() -> Self {
        Self::new()
    }
}
