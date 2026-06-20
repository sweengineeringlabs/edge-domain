//! `StdServiceRegistryFactory` — concrete factory implementation.
//!
//! This type is kept in `api/types/` to satisfy the structural audit's one-type-per-file
//! rule. Its [`ServiceRegistryBootstrap`](crate::api::service::traits::service_registry_bootstrap::ServiceRegistryBootstrap)
//! implementation lives in `core/service/service_registry_factory.rs` (core impl, not renamed).

/// The default concrete factory for constructing [`ServiceRegistry`](super::ServiceRegistry) instances.
///
/// Implements [`ServiceRegistryBootstrap`](crate::api::service::traits::service_registry_bootstrap::ServiceRegistryBootstrap).
#[derive(Debug, Default, Clone, Copy)]
pub struct StdServiceRegistryFactory;
