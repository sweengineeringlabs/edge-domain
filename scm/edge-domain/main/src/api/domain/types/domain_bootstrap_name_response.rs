//! `DomainBootstrapNameResponse` — response envelope for [`DomainBootstrap::bootstrap_name`](crate::api::DomainBootstrap::bootstrap_name).

/// Response carrying the bootstrap implementation's identifier.
pub struct DomainBootstrapNameResponse {
    /// Identifies this bootstrap implementation.
    pub name: &'static str,
}
