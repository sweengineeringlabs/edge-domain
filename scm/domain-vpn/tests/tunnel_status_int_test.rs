//! Tests for `api/vpn/types/tunnel_status.rs` — TunnelStatus enum.

use edge_application_vpn::TunnelStatus;

#[test]
fn test_tunnel_status_connected_eq_connected_happy() {
    let s1 = TunnelStatus::Connected;
    let s2 = TunnelStatus::Connected;
    assert_eq!(s1, s2, "Connected status should equal another Connected status");
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
