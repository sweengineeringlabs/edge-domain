//! `TunnelStatus` — observable state of a VPN tunnel.

/// Current lifecycle state of a managed VPN tunnel.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TunnelStatus {
    /// No active tunnel; `connect()` has not been called or was never attempted.
    Disconnected,
    /// TCP/UDP connect to the server endpoint is in progress.
    Connecting,
    /// Transport layer connected; WireGuard Noise handshake in progress.
    Handshaking,
    /// Tunnel fully established; traffic is flowing.
    Connected,
    /// Previous connection was lost; automatic reconnect is pending.
    Reconnecting,
    /// Terminal failure — reconnect limit reached or non-recoverable error.
    Failed(String),
}
