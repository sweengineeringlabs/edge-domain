//! [`DomainFactory`] — constructor contract for domain building-block types.

use crate::api::domain::types::domain::Domain;
use crate::api::domain::types::noop_domain_extension::NoopDomainExtension;
use crate::api::domain::types::outbound_registry::OutboundRegistry;

/// Factory trait for the core domain building-block types.
pub trait DomainFactory {
    /// Construct the [`Domain`] factory handle.
    fn domain() -> Domain {
        Domain
    }

    /// Construct a [`NoopDomainExtension`] placeholder.
    fn noop_extension() -> NoopDomainExtension {
        NoopDomainExtension
    }

    /// Construct an empty [`OutboundRegistry`].
    fn outbound_registry<H: Clone + Send + Sync + 'static>() -> OutboundRegistry<H> {
        OutboundRegistry::new()
    }
}
