//! Service theme — port contracts.

#[allow(clippy::module_inception)]
pub mod service;
pub mod service_registry;

pub use service::Service;
pub use service_registry::ServiceRegistry;
