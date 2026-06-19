//! Tests for `api/vpn/types/tunnel_status.rs` — TunnelStatus enum.

use edge_domain_vpn::TunnelStatus;

#[test]
fn test_tunnel_status_connected_eq_connected_happy() {
    assert_eq!(TunnelStatus::Connected, TunnelStatus::Connected);
}

#[test]
fn test_tunnel_status_failed_variant_carries_reason_error() {
    let s = TunnelStatus::Failed("handshake timeout".to_string());
    assert!(matches!(s, TunnelStatus::Failed(_)));
}

#[test]
fn test_tunnel_status_all_variants_ne_connected_edge() {
    let others = vec![
        TunnelStatus::Disconnected,
        TunnelStatus::Connecting,
        TunnelStatus::Handshaking,
        TunnelStatus::Reconnecting,
        TunnelStatus::Failed("x".to_string()),
    ];
    for s in others {
        assert_ne!(s, TunnelStatus::Connected);
    }
}
