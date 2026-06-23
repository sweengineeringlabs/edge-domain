//! Tests for `api/vpn/errors/vpn_result.rs` — VpnResult alias.

use edge_domain_vpn::{VpnError, VpnResult};

#[test]
fn test_vpn_result_ok_variant_happy() {
    let r: VpnResult<u32> = Ok(42);
    assert_eq!(r.unwrap(), 42);
}

#[test]
fn test_vpn_result_err_variant_carries_vpn_error_error() {
    let r: VpnResult<u32> = Err(VpnError::Unavailable { reason: "test".to_string() });
    assert!(r.is_err());
}

#[test]
fn test_vpn_result_unit_ok_edge() {
    let r: VpnResult<()> = Ok(());
    assert_eq!(r, Ok(()), "unit result should equal Ok(())");
}
