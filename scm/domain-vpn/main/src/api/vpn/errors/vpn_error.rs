//! `VpnError` — VPN tunnel error taxonomy.

use thiserror::Error;

/// Errors that can occur during VPN tunnel operations.
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum VpnError {
    /// Config is structurally invalid (bad key encoding, missing required field).
    #[error("vpn config error: {reason}")]
    ConfigError {
        /// Human-readable description of the config problem.
        reason: String,
    },

    /// Handshake or TCP connect attempt failed.
    #[error("vpn connection failed: {reason}")]
    ConnectionFailed {
        /// Human-readable description of the failure.
        reason: String,
    },

    /// TUN device setup, routing, or kernel error.
    #[error("vpn tunnel error: {reason}")]
    TunnelError {
        /// Human-readable description of the tunnel problem.
        reason: String,
    },

    /// Requested operation is unavailable on this platform or in this state.
    #[error("vpn unavailable: {reason}")]
    Unavailable {
        /// Human-readable description of why it is unavailable.
        reason: String,
    },
}
