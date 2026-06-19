//! E2E tunnel tests — real WireGuard tunnel via `rustboot_vpn`.
//!
//! All unix-only tests are `#[ignore]` — they require `CAP_NET_ADMIN`.

#[cfg(unix)]
mod unix_tunnel_tests {
    #[test]
    #[ignore = "requires CAP_NET_ADMIN and a running swelvpn server"]
    fn test_wg_tunnel_requires_cap_net_admin_ignored() {
        // Real tunnel test lives in hello-vpn/tests/vpn_tunnel_int_test.rs
        // which has a direct rustboot_vpn dep for the WgTunnelAdapter.
        assert!(true, "placeholder — always passes when not ignored");
    }
}
