//! [`DomainBootstrap`] — constructor contract for domain building-block types.

use crate::api::domain::errors::DomainError;
use crate::api::domain::types::domain::Domain;
use crate::api::domain::types::domain_bootstrap_name_request::DomainBootstrapNameRequest;
use crate::api::domain::types::domain_bootstrap_name_response::DomainBootstrapNameResponse;
use crate::api::domain::types::noop_domain_extension::NoopDomainExtension;
use crate::api::domain::types::outbound_registry::OutboundRegistry;

/// Bootstrap trait for the core domain building-block types.
pub trait DomainBootstrap {
    /// Identifies this bootstrap implementation.
    fn bootstrap_name(
        &self,
        _req: DomainBootstrapNameRequest,
    ) -> Result<DomainBootstrapNameResponse, DomainError> {
        Ok(DomainBootstrapNameResponse { name: "domain" })
    }

    /// Construct the [`Domain`] factory handle.
    fn domain() -> Domain
    where
        Self: Sized,
    {
        Domain
    }

    /// Construct a [`NoopDomainExtension`] placeholder.
    fn noop_extension() -> NoopDomainExtension
    where
        Self: Sized,
    {
        NoopDomainExtension
    }

    /// Construct an empty [`OutboundRegistry`].
    fn outbound_registry<H: Clone + Send + Sync + 'static>() -> OutboundRegistry<H>
    where
        Self: Sized,
    {
        OutboundRegistry::new()
    }
}
