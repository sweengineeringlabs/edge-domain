//! Extension hook trait for downstream consumers.

/// Marker trait for downstream domain extensions.
///
/// Implement this on a zero-size type to register custom cross-cutting
/// behaviour (audit hooks, logging adapters, etc.) that wraps domain
/// operations without altering public contracts.  The default no-op
/// implementation is
/// [`crate::api::domain::types::noop_domain_extension::NoopDomainExtension`].
///
/// Extensions wrap the domain composition primitives assembled through the
/// [`crate::api::domain::types::domain::Domain`] factory: runtime settings
/// carried by [`crate::api::domain::types::application_config::ApplicationConfig`]
/// and outbound handles held in a
/// [`crate::api::domain::types::outbound_registry::OutboundRegistry`].
pub trait DomainExtension: Send + Sync {}
