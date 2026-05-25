//! `Service` module — named domain operation contracts.

#[allow(clippy::module_inception)]
pub mod service;
pub mod service_error;
pub mod service_registry;
pub mod service_registry_trait;

pub use service::Service;
pub use service_error::ServiceError;
pub use service_registry_trait::ServiceRegistry;
