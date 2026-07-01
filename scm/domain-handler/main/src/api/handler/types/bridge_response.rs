//! [`BridgeResponse`] — response for [`RegistryBridge::bridge`](crate::api::handler::traits::RegistryBridge::bridge).

/// The number of services transferred from a service registry into a handler registry.
pub struct BridgeResponse {
    /// The number of services transferred.
    pub transferred: usize,
}
