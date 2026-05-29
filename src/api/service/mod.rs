//! `Service` module — named domain operation contracts.

#[allow(clippy::module_inception)]
pub mod service;
pub mod service_registry;

pub use crate::api::error::ServiceError;
pub use service::Service;
pub use service_registry::ServiceRegistry;
