//! `VpnManagerFactory` — marker type for VPN manager factory methods.
//!
//! Methods are added in `saf/vpn/tunnel_manager_svc.rs` following the
//! domain-observe pattern of extending api/ types in saf/ with factory fns.

/// Marker type whose factory methods are implemented in the SAF layer.
///
/// Use `VpnManagerFactory::noop_tunnel_manager()` to obtain a `TunnelManager`
/// that satisfies the contract without any OS-level side effects.
pub struct VpnManagerFactory;
